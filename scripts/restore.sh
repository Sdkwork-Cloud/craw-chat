#!/bin/bash
# 文件: scripts/restore.sh
# 描述: SDKWork IM 灾难恢复脚本，从 S3 备份恢复 PostgreSQL、Redis 和应用配置
# 用法: ./scripts/restore.sh <YYYYMMDD_HHMMSS> [--target s3://bucket] [--skip-redis] [--skip-config]
# 创建日期: 2026-07-03

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

S3_BUCKET="${SDKWORK_IM_BACKUP_BUCKET:-s3://backup-sdkwork-im}"
DATABASE_URL="${SDKWORK_IM_DATABASE_URL:-}"
REDIS_NODES="${SDKWORK_IM_REDIS_CLUSTER_NODES:-${SDKWORK_IM_REDIS_URL:-}}"
TMP_DIR="${TMPDIR:-/tmp}"

BACKUP_DATE="${1:-}"
SKIP_REDIS=false
SKIP_CONFIG=false

if [ -z "$BACKUP_DATE" ]; then
    echo "Usage: $0 <YYYYMMDD_HHMMSS> [--target s3://bucket] [--skip-redis] [--skip-config]"
    echo ""
    echo "Examples:"
    echo "  $0 20260703_020000"
    echo "  $0 latest --skip-redis"
    exit 1
fi
shift

while [[ $# -gt 0 ]]; do
    case "$1" in
        --target) S3_BUCKET="$2"; shift 2 ;;
        --skip-redis) SKIP_REDIS=true; shift ;;
        --skip-config) SKIP_CONFIG=true; shift ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

log_info()  { echo -e "${BLUE}[INFO]${NC}  $1"; }
log_pass()  { echo -e "${GREEN}[PASS]${NC}  $1"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC}  $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

log_info "=== SDKWork IM Restore Start ==="
log_info "Backup timestamp: $BACKUP_DATE"
log_info "Source: $S3_BUCKET"
log_info "Timestamp: $(date)"
echo ""

# ============================================================================
# 0. 解析 latest 标记
# ============================================================================
if [ "$BACKUP_DATE" == "latest" ]; then
    log_info "Resolving latest backup..."
    if command -v aws >/dev/null 2>&1; then
        BACKUP_DATE=$(aws s3 ls "${S3_BUCKET}/db-full/" 2>/dev/null \
            | sort -k1,2 \
            | tail -n1 \
            | awk '{print $4}' \
            | sed -E 's|sdkwork-im-db_([0-9]{8}_[0-9]{6})\.dump|\1|')
        if [ -z "$BACKUP_DATE" ]; then
            log_error "No database backup found in ${S3_BUCKET}/db-full/"
            exit 1
        fi
        log_pass "Latest backup resolved: $BACKUP_DATE"
    else
        log_error "aws-cli required to resolve 'latest'; pass explicit timestamp"
        exit 1
    fi
fi
echo ""

# ============================================================================
# 1. 恢复数据库
# ============================================================================
log_info "1. Restoring PostgreSQL database..."

if [ -z "$DATABASE_URL" ]; then
    log_error "SDKWORK_IM_DATABASE_URL not set; cannot restore database"
    exit 1
fi

require_tool() {
    if ! command -v "$1" >/dev/null 2>&1; then
        log_error "Required tool not installed: $1"
        exit 1
    fi
}
require_tool pg_restore

DB_ARCHIVE="${TMP_DIR}/sdkwork-im-db_${BACKUP_DATE}.dump"
if [ -f "$DB_ARCHIVE" ]; then
    log_info "Using local archive: $DB_ARCHIVE"
else
    require_tool aws
    log_info "Downloading database backup from S3..."
    if aws s3 cp "${S3_BUCKET}/db-full/sdkwork-im-db_${BACKUP_DATE}.dump" "$DB_ARCHIVE" 2>/dev/null; then
        log_pass "Database backup downloaded"
    else
        log_error "Failed to download database backup for ${BACKUP_DATE}"
        exit 1
    fi
fi

log_info "Restoring database (this may take a while)..."
if pg_restore -d "$DATABASE_URL" -Fc -c --if-exists "$DB_ARCHIVE" 2>"${TMP_DIR}/restore_warnings.log"; then
    log_pass "Database restore completed"
else
    RESTORE_EXIT=$?
    # pg_restore 对已存在对象会发出 warning，但非致命
    if grep -qi "FATAL\|could not" "${TMP_DIR}/restore_warnings.log" 2>/dev/null; then
        log_error "Database restore encountered fatal errors"
        cat "${TMP_DIR}/restore_warnings.log"
        rm -f "$DB_ARCHIVE" "${TMP_DIR}/restore_warnings.log"
        exit $RESTORE_EXIT
    else
        log_warn "Database restore completed with non-fatal warnings"
    fi
fi
rm -f "$DB_ARCHIVE" "${TMP_DIR}/restore_warnings.log"
echo ""

# ============================================================================
# 2. 恢复 Redis
# ============================================================================
if [ "$SKIP_REDIS" != "true" ]; then
    log_info "2. Restoring Redis..."

    if [ -n "$REDIS_NODES" ]; then
        require_tool redis-cli
        FIRST_NODE=$(echo "$REDIS_NODES" | cut -d',' -f1)
        HOST=$(echo "$FIRST_NODE" | sed -E 's|redis(s?)://([^:]+):.*|\2|; s|redis(s?)://([^:]+)|\2|')
        PORT=$(echo "$FIRST_NODE" | sed -E 's|.*:([0-9]+).*|\1|; t; s|.*||')
        PORT="${PORT:-6379}"

        RDB_ARCHIVE="${TMP_DIR}/sdkwork-im-redis_${BACKUP_DATE}.rdb"
        if [ -f "$RDB_ARCHIVE" ]; then
            log_info "Using local RDB: $RDB_ARCHIVE"
        else
            if command -v aws >/dev/null 2>&1; then
                if aws s3 cp "${S3_BUCKET}/redis/sdkwork-im-redis_${BACKUP_DATE}.rdb" "$RDB_ARCHIVE" 2>/dev/null; then
                    log_pass "Redis RDB downloaded"
                else
                    log_warn "Redis RDB backup not found for ${BACKUP_DATE}; skipping Redis restore"
                    RDB_ARCHIVE=""
                fi
            else
                log_warn "aws-cli not installed; cannot download Redis backup"
                RDB_ARCHIVE=""
            fi
        fi

        if [ -n "$RDB_ARCHIVE" ] && [ -f "$RDB_ARCHIVE" ]; then
            # Redis 不支持热加载 RDB；需要停止节点替换文件后重启
            log_warn "Redis RDB restore requires node restart"
            log_info "To restore: stop Redis, replace dump.rdb with $RDB_ARCHIVE, then restart Redis"
            log_info "Manual step: cp $RDB_ARCHIVE /var/lib/redis/dump.rdb && systemctl restart redis"
            log_pass "Redis RDB staged for manual restore"
        fi
    else
        log_warn "Redis nodes not configured; skipping Redis restore"
    fi
    echo ""
fi

# ============================================================================
# 3. 恢复应用配置
# ============================================================================
if [ "$SKIP_CONFIG" != "true" ]; then
    log_info "3. Restoring application configuration..."

    CONFIG_ARCHIVE="${TMP_DIR}/sdkwork-im-config_${BACKUP_DATE}.tar.gz"
    if [ -f "$CONFIG_ARCHIVE" ]; then
        log_info "Using local archive: $CONFIG_ARCHIVE"
    else
        if command -v aws >/dev/null 2>&1; then
            if aws s3 cp "${S3_BUCKET}/config/sdkwork-im-config_${BACKUP_DATE}.tar.gz" "$CONFIG_ARCHIVE" 2>/dev/null; then
                log_pass "Configuration backup downloaded"
            else
                log_warn "Configuration backup not found for ${BACKUP_DATE}; skipping"
                CONFIG_ARCHIVE=""
            fi
        else
            log_warn "aws-cli not installed; cannot download configuration backup"
            CONFIG_ARCHIVE=""
        fi
    fi

    if [ -n "$CONFIG_ARCHIVE" ] && [ -f "$CONFIG_ARCHIVE" ]; then
        STAGE_DIR="${TMP_DIR}/sdkwork-im-config-restore_${BACKUP_DATE}"
        mkdir -p "$STAGE_DIR"
        if tar -xzf "$CONFIG_ARCHIVE" -C "$STAGE_DIR"; then
            log_pass "Configuration extracted to $STAGE_DIR"
            log_info "Review and copy required files to $ROOT_DIR before starting services"
            log_info "Manual step: cp -r $STAGE_DIR/configs/* $ROOT_DIR/configs/"
        else
            log_error "Failed to extract configuration archive"
        fi
        rm -f "$CONFIG_ARCHIVE"
    fi
    echo ""
fi

# ============================================================================
# 4. 验证恢复
# ============================================================================
log_info "4. Verifying recovery..."

if command -v curl >/dev/null 2>&1; then
    GATEWAY_URL="${SDKWORK_IM_PUBLIC_URL:-http://localhost:18079}"
    log_info "Waiting for services to become ready..."
    for i in {1..30}; do
        if curl -sf "${GATEWAY_URL}/healthz" >/dev/null 2>&1; then
            log_pass "Health check passed after ${i}0 seconds"
            break
        fi
        if [ "$i" -eq 30 ]; then
            log_warn "Services not ready after 300 seconds; start services manually then re-check"
        fi
        sleep 10
    done
fi

log_info "=== SDKWork IM Restore Completed ==="
log_pass "Restore finished at $(date)"
log_info "Next steps:"
log_info "  1. Start application services: scripts/restart-services.sh"
log_info "  2. Run deployment verification: scripts/verify-deployment.sh"
log_info "  3. Run security check: scripts/check-security-config.sh"
exit 0
