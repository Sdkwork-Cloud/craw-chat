#!/bin/bash
# 文件: scripts/migrate-database.sh
# 描述: SDKWork IM 数据库迁移脚本，按版本顺序执行 SQL 迁移并验证 schema_migrations 状态
# 用法: ./scripts/migrate-database.sh [version] [--dry-run] [--rollback version]
# 创建日期: 2026-07-03

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

DATABASE_URL="${SDKWORK_IM_DATABASE_URL:-}"
MIGRATIONS_DIR="${SDKWORK_IM_MIGRATIONS_DIR:-$ROOT_DIR/database/migrations}"
DRY_RUN=false
ROLLBACK_VERSION=""
TARGET_VERSION="${1:-}"

# 解析参数
while [[ $# -gt 0 ]]; do
    case "$1" in
        --dry-run) DRY_RUN=true; shift ;;
        --rollback) ROLLBACK_VERSION="$2"; shift 2 ;;
        -h|--help)
            echo "Usage: $0 [version] [--dry-run] [--rollback version]"
            echo ""
            echo "Examples:"
            echo "  $0                    # Apply all pending migrations"
            echo "  $0 20260701           # Apply migrations up to version 20260701"
            echo "  $0 --dry-run          # Show pending migrations without applying"
            echo "  $0 --rollback 20260601# Rollback to version 20260601"
            exit 0
            ;;
        *) TARGET_VERSION="$1"; shift ;;
    esac
done

log_info()  { echo -e "${BLUE}[INFO]${NC}  $1"; }
log_pass()  { echo -e "${GREEN}[PASS]${NC}  $1"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC}  $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

log_info "=== SDKWork IM Database Migration ==="
log_info "Timestamp: $(date)"
log_info "Database: ${DATABASE_URL:+configured}"
log_info "Migrations dir: $MIGRATIONS_DIR"
echo ""

if [ -z "$DATABASE_URL" ]; then
    log_error "SDKWORK_IM_DATABASE_URL not set"
    exit 1
fi

if ! command -v psql >/dev/null 2>&1; then
    log_error "psql client is required"
    exit 1
fi

if [ ! -d "$MIGRATIONS_DIR" ]; then
    log_error "Migrations directory not found: $MIGRATIONS_DIR"
    exit 1
fi

# ============================================================================
# 1. 检查 schema_migrations 表
# ============================================================================
log_info "1. Checking schema_migrations table..."

if ! psql "$DATABASE_URL" -t -c "SELECT 1 FROM information_schema.tables WHERE table_name='schema_migrations'" 2>/dev/null | grep -q 1; then
    log_info "Creating schema_migrations table..."
    psql "$DATABASE_URL" >/dev/null 2>&1 <<'SQL'
CREATE TABLE IF NOT EXISTS schema_migrations (
    version VARCHAR(255) PRIMARY KEY,
    applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
SQL
    log_pass "schema_migrations table ready"
else
    log_pass "schema_migrations table exists"
fi

APPLIED_VERSIONS=$(psql "$DATABASE_URL" -t -c "SELECT version FROM schema_migrations ORDER BY version" 2>/dev/null | tr -d '[:space:]' | sort)
log_info "Applied versions: $(echo "$APPLIED_VERSIONS" | tr '\n' ' ')"
echo ""

# ============================================================================
# 2. 收集待执行迁移
# ============================================================================
log_info "2. Scanning migrations directory..."

# 期望目录结构: $MIGRATIONS_DIR/<version>/up.sql, down.sql
MIGRATIONS_FOUND=0
PENDING_MIGRATIONS=()

while IFS= read -r MIGRATION_DIR; do
    if [ -d "$MIGRATION_DIR" ]; then
        VERSION=$(basename "$MIGRATION_DIR")
        MIGRATIONS_FOUND=$((MIGRATIONS_FOUND + 1))

        # 检查是否已应用
        if echo "$APPLIED_VERSIONS" | grep -qxF "$VERSION"; then
            continue
        fi

        # 如果指定了目标版本，跳过超过目标的迁移
        if [ -n "$TARGET_VERSION" ] && [ "$VERSION" \> "$TARGET_VERSION" ]; then
            continue
        fi

        PENDING_MIGRATIONS+=("$VERSION|$MIGRATION_DIR")
    fi
done < <(find "$MIGRATIONS_DIR" -mindepth 1 -maxdepth 1 -type d | sort)

log_info "Found $MIGRATIONS_FOUND migration directories"
log_info "Pending migrations: ${#PENDING_MIGRATIONS[@]}"

if [ "${#PENDING_MIGRATIONS[@]}" -eq 0 ]; then
    log_pass "No pending migrations"
    exit 0
fi

# 显示待执行迁移
for entry in "${PENDING_MIGRATIONS[@]}"; do
    VERSION="${entry%%|*}"
    log_info "  - $VERSION"
done
echo ""

# ============================================================================
# 3. Dry-run 模式
# ============================================================================
if [ "$DRY_RUN" == "true" ]; then
    log_info "Dry-run mode: no changes will be applied"
    for entry in "${PENDING_MIGRATIONS[@]}"; do
        VERSION="${entry%%|*}"
        DIR="${entry#*|}"
        UP_SQL="$DIR/up.sql"
        if [ -f "$UP_SQL" ]; then
            log_info "--- $VERSION/up.sql ---"
            head -n 20 "$UP_SQL"
            echo "..."
        fi
    done
    log_pass "Dry-run completed"
    exit 0
fi

# ============================================================================
# 4. 执行迁移
# ============================================================================
log_info "3. Applying migrations..."

# 备份提示
log_warn "Ensure you have a recent backup before proceeding"
log_info "Run scripts/backup.sh to create a backup now"
echo ""

for entry in "${PENDING_MIGRATIONS[@]}"; do
    VERSION="${entry%%|*}"
    DIR="${entry#*|}"
    UP_SQL="$DIR/up.sql"

    if [ ! -f "$UP_SQL" ]; then
        log_error "Missing up.sql for migration $VERSION"
        exit 1
    fi

    log_info "Applying $VERSION..."
    if psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f "$UP_SQL" 2>&1 | tee "${TMPDIR:-/tmp}/migration_${VERSION}.log"; then
        psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -c "INSERT INTO schema_migrations (version) VALUES ('$VERSION')" >/dev/null 2>&1
        log_pass "Migration $VERSION applied"
    else
        log_error "Migration $VERSION failed"
        log_error "Rollback with: $0 --rollback $VERSION"
        exit 1
    fi
done
echo ""

# ============================================================================
# 5. 回滚处理
# ============================================================================
if [ -n "$ROLLBACK_VERSION" ]; then
    log_info "Rolling back to version $ROLLBACK_VERSION..."

    # 获取需要回滚的版本（大于 ROLLBACK_VERSION 的已应用版本）
    ROLLBACK_TARGETS=$(psql "$DATABASE_URL" -t -c \
        "SELECT version FROM schema_migrations WHERE version > '$ROLLBACK_VERSION' ORDER BY version DESC" \
        2>/dev/null | tr -d '[:space:]' | tac)

    for VERSION in $ROLLBACK_TARGETS; do
        DIR="$MIGRATIONS_DIR/$VERSION"
        DOWN_SQL="$DIR/down.sql"
        if [ ! -f "$DOWN_SQL" ]; then
            log_error "Missing down.sql for migration $VERSION"
            exit 1
        fi
        log_info "Rolling back $VERSION..."
        if psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f "$DOWN_SQL"; then
            psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -c "DELETE FROM schema_migrations WHERE version='$VERSION'" >/dev/null 2>&1
            log_pass "Migration $VERSION rolled back"
        else
            log_error "Rollback of $VERSION failed"
            exit 1
        fi
    done
fi

# ============================================================================
# 6. 验证迁移状态
# ============================================================================
log_info "4. Verifying migration status..."

FINAL_VERSIONS=$(psql "$DATABASE_URL" -t -c "SELECT version FROM schema_migrations ORDER BY version" 2>/dev/null | tr -d '[:space:]')
log_pass "Applied versions: $(echo "$FINAL_VERSIONS" | tr '\n' ' ')"
echo ""

log_info "=== Database Migration Completed ==="
log_pass "Finished at $(date)"
exit 0
