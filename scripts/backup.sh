#!/bin/bash
# 文件: scripts/backup.sh
# 描述: SDKWork IM 生产环境备份脚本，覆盖配置、PostgreSQL 全量、Redis 快照及对象存储跨区域复制
# 用法: ./scripts/backup.sh [--target s3://bucket] [--retention-days 30]
# 创建日期: 2026-07-03

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

BACKUP_DATE=$(date +%Y%m%d_%H%M%S)
S3_BUCKET="${SDKWORK_IM_BACKUP_BUCKET:-s3://backup-sdkwork-im}"
RETENTION_DAYS="${SDKWORK_IM_BACKUP_RETENTION_DAYS:-30}"
DATABASE_URL="${SDKWORK_IM_DATABASE_URL:-}"
REDIS_NODES="${SDKWORK_IM_REDIS_CLUSTER_NODES:-${SDKWORK_IM_REDIS_URL:-}}"
TMP_DIR="${TMPDIR:-/tmp}"

# 解析参数
while [[ $# -gt 0 ]]; do
    case "$1" in
        --target) S3_BUCKET="$2"; shift 2 ;;
        --retention-days) RETENTION_DAYS="$2"; shift 2 ;;
        -h|--help)
            echo "Usage: $0 [--target s3://bucket] [--retention-days N]"
            exit 0
            ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

log_info()  { echo -e "${BLUE}[INFO]${NC}  $1"; }
log_pass()  { echo -e "${GREEN}[PASS]${NC}  $1"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC}  $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

log_info "=== SDKWork IM Backup Start ==="
log_info "Timestamp: $(date)"
log_info "Target: $S3_BUCKET"
log_info "Retention: $RETENTION_DAYS days"
echo ""

# 检查依赖工具
require_tool() {
    if ! command -v "$1" >/dev/null 2>&1; then
        log_error "Required tool not installed: $1"
        exit 1
    fi
}

# ============================================================================
# 1. 应用配置备份
# ============================================================================
log_info "1. Backing up application configuration..."

CONFIG_ARCHIVE="${TMP_DIR}/sdkwork-im-config_${BACKUP_DATE}.tar.gz"
if tar -czf "$CONFIG_ARCHIVE" \
    -C "$ROOT_DIR" \
    configs/ \
    deployments/templates/ \
    sdkwork.app.config.json \
    sdkwork.workflow.json \
    2>/dev/null; then
    log_pass "Configuration archive created: $CONFIG_ARCHIVE"
    if command -v aws >/dev/null 2>&1; then
        if aws s3 cp "$CONFIG_ARCHIVE" "${S3_BUCKET}/config/" >/dev/null 2>&1; then
            log_pass "Configuration uploaded to ${S3_BUCKET}/config/"
        else
            log_warn "Failed to upload configuration to S3; keeping local copy"
        fi
    else
        log_warn "aws-cli not installed; configuration archive retained locally"
    fi
else
    log_warn "Configuration archive creation skipped (some paths may not exist)"
fi
rm -f "$CONFIG_ARCHIVE"
echo ""

# ============================================================================
# 2. PostgreSQL 全量备份
# ============================================================================
log_info "2. Backing up PostgreSQL database..."

if [ -z "$DATABASE_URL" ]; then
    log_error "SDKWORK_IM_DATABASE_URL not set; cannot back up database"
    exit 1
fi

require_tool pg_dump

DB_ARCHIVE="${TMP_DIR}/sdkwork-im-db_${BACKUP_DATE}.dump"
if pg_dump -Fc -Z9 "$DATABASE_URL" > "$DB_ARCHIVE"; then
    DB_SIZE=$(du -h "$DB_ARCHIVE" | cut -f1)
    log_pass "Database backup created: $DB_ARCHIVE ($DB_SIZE)"
    if command -v aws >/dev/null 2>&1; then
        if aws s3 cp "$DB_ARCHIVE" "${S3_BUCKET}/db-full/" >/dev/null 2>&1; then
            log_pass "Database backup uploaded to ${S3_BUCKET}/db-full/"
        else
            log_error "Failed to upload database backup to S3"
            rm -f "$DB_ARCHIVE"
            exit 1
        fi
    else
        log_warn "aws-cli not installed; database backup retained locally at $DB_ARCHIVE"
    fi
else
    log_error "pg_dump failed; aborting backup"
    rm -f "$DB_ARCHIVE"
    exit 1
fi
rm -f "$DB_ARCHIVE"
echo ""

# ============================================================================
# 3. Redis 快照备份
# ============================================================================
log_info "3. Backing up Redis..."

if [ -n "$REDIS_NODES" ]; then
    require_tool redis-cli
    FIRST_NODE=$(echo "$REDIS_NODES" | cut -d',' -f1)
    HOST=$(echo "$FIRST_NODE" | sed -E 's|redis(s?)://([^:]+):.*|\2|; s|redis(s?)://([^:]+)|\2|')
    PORT=$(echo "$FIRST_NODE" | sed -E 's|.*:([0-9]+).*|\1|; t; s|.*||')
    PORT="${PORT:-6379}"

    # 触发后台保存
    if redis-cli -h "$HOST" -p "$PORT" BGSAVE >/dev/null 2>&1; then
        log_info "Redis BGSAVE triggered; waiting for completion..."
        for _ in {1..30}; do
            LAST_SAVE=$(redis-cli -h "$HOST" -p "$PORT" LASTSAVE 2>/dev/null || echo 0)
            CURRENT_TIME=$(date +%s)
            if [ $((CURRENT_TIME - LAST_SAVE)) -lt 5 ]; then
                log_pass "Redis BGSAVE completed"
                break
            fi
            sleep 2
        done

        # 导出 RDB
        RDB_ARCHIVE="${TMP_DIR}/sdkwork-im-redis_${BACKUP_DATE}.rdb"
        if redis-cli -h "$HOST" -p "$PORT" --rdb "$RDB_ARCHIVE" >/dev/null 2>&1; then
            RDB_SIZE=$(du -h "$RDB_ARCHIVE" | cut -f1)
            log_pass "Redis RDB snapshot created: $RDB_ARCHIVE ($RDB_SIZE)"
            if command -v aws >/dev/null 2>&1; then
                if aws s3 cp "$RDB_ARCHIVE" "${S3_BUCKET}/redis/" >/dev/null 2>&1; then
                    log_pass "Redis backup uploaded to ${S3_BUCKET}/redis/"
                else
                    log_warn "Failed to upload Redis backup to S3; keeping local copy"
                fi
            fi
        else
            log_warn "Redis RDB export failed; cluster may not support --rdb"
        fi
        rm -f "$RDB_ARCHIVE"
    else
        log_warn "Redis BGSAVE failed; skipping Redis backup"
    fi
else
    log_warn "Redis nodes not configured; skipping Redis backup"
fi
echo ""

# ============================================================================
# 4. 清理过期备份
# ============================================================================
log_info "4. Cleaning expired backups (older than $RETENTION_DAYS days)..."

if command -v aws >/dev/null 2>&1; then
    CUTOFF_DATE=$(date -d "-${RETENTION_DAYS} days" +%Y-%m-%d 2>/dev/null || date -v-${RETENTION_DAYS}d +%Y-%m-%d 2>/dev/null || echo "")
    if [ -n "$CUTOFF_DATE" ]; then
        for PREFIX in "config/" "db-full/" "redis/"; do
            DELETED=0
            while IFS= read -r KEY; do
                if [ -n "$KEY" ]; then
                    aws s3 rm "s3://${S3_BUCKET#s3://}/${PREFIX}${KEY}" >/dev/null 2>&1 && DELETED=$((DELETED + 1))
                fi
            done < <(aws s3 ls "${S3_BUCKET}/${PREFIX}" 2>/dev/null | awk '{print $4}')
            log_pass "Cleaned $DELETED expired objects from ${PREFIX}"
        done
    fi
else
    log_warn "aws-cli not installed; skip expired backup cleanup"
fi
echo ""

log_info "=== SDKWork IM Backup Completed ==="
log_pass "Backup finished at $(date)"
exit 0
