#!/usr/bin/env bash
# File: scripts/backup.sh
# Description: SDKWork IM backup for configuration, PostgreSQL, and Redis.
# Usage: ./scripts/backup.sh [--target s3://bucket[/prefix]] [--retention-days N] [--dry-run] [--delete-limit N]

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
DELETE_LIMIT="${SDKWORK_IM_BACKUP_DELETE_LIMIT:-100}"
MAX_DELETE_LIMIT=1000
MAX_RETENTION_DAYS=36500
DATABASE_URL="${SDKWORK_IM_DATABASE_URL:-}"
REDIS_NODES="${SDKWORK_IM_REDIS_CLUSTER_NODES:-${SDKWORK_IM_REDIS_URL:-}}"
TMP_DIR="${TMPDIR:-/tmp}"
DRY_RUN=false
SHOW_HELP=false

# Parsed only after validating S3_BUCKET. Keeping these distinct prevents a
# bucket URI with a key prefix from being passed to the s3api --bucket option.
S3_BUCKET_NAME=""
S3_KEY_PREFIX=""
CUTOFF_EPOCH=0
LATEST_DB_KEY=""
LATEST_DB_TIMESTAMP=""
LATEST_CONFIG_KEY=""
LATEST_REDIS_KEY=""
LATEST_DB_EPOCH=0
LATEST_CONFIG_EPOCH=0
LATEST_REDIS_EPOCH=0
CANDIDATE_COUNT=0
DELETED_COUNT=0

log_info()  { echo -e "${BLUE}[INFO]${NC}  $1"; }
log_pass()  { echo -e "${GREEN}[PASS]${NC}  $1"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC}  $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

require_tool() {
    if ! command -v "$1" >/dev/null 2>&1; then
        log_error "Required tool not installed: $1"
        return 1
    fi
}

require_option_value() {
    if [[ $# -lt 2 || -z "${2:-}" ]]; then
        log_error "Missing value for $1"
        return 1
    fi
}

parse_arguments() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --target)
                require_option_value "$@" || return 1
                S3_BUCKET="$2"
                shift 2
                ;;
            --retention-days)
                require_option_value "$@" || return 1
                RETENTION_DAYS="$2"
                shift 2
                ;;
            --delete-limit)
                require_option_value "$@" || return 1
                DELETE_LIMIT="$2"
                shift 2
                ;;
            --dry-run)
                DRY_RUN=true
                shift
                ;;
            -h|--help)
                cat <<'USAGE'
Usage: scripts/backup.sh [options]

Options:
  --target s3://bucket[/prefix]  Backup bucket and optional key prefix.
  --retention-days N             Retain objects newer than N days (default: 30).
  --delete-limit N               Maximum cleanup deletions per run, 1-1000 (default: 100).
  --dry-run                      Report eligible cleanup deletions without deleting objects.
USAGE
                SHOW_HELP=true
                return 0
                ;;
            *)
                log_error "Unknown option: $1"
                return 1
                ;;
        esac
    done
}

parse_s3_target() {
    local target="${1%/}"
    local remainder
    local key_root=""

    if [[ "$target" != s3://* ]]; then
        log_error "Backup target must use s3://bucket[/prefix], got: $1"
        return 1
    fi

    remainder="${target#s3://}"
    if [[ "$remainder" == */* ]]; then
        S3_BUCKET_NAME="${remainder%%/*}"
        key_root="${remainder#*/}"
    else
        S3_BUCKET_NAME="$remainder"
    fi

    if [[ ! "$S3_BUCKET_NAME" =~ ^[A-Za-z0-9][A-Za-z0-9.-]{1,61}[A-Za-z0-9]$ ]]; then
        log_error "Backup target has an invalid S3 bucket name: $S3_BUCKET_NAME"
        return 1
    fi

    if [[ -n "$key_root" ]]; then
        if [[ "$key_root" == /* || "$key_root" == */ || "$key_root" == *"//"* ]]; then
            log_error "Backup target key prefix must be normalized: $1"
            return 1
        fi
        case "/$key_root/" in
            */./*|*/../*)
                log_error "Backup target key prefix must not contain . or .. segments: $1"
                return 1
                ;;
        esac
        S3_KEY_PREFIX="${key_root}/"
    else
        S3_KEY_PREFIX=""
    fi

    S3_BUCKET="$target"
}

validate_cleanup_configuration() {
    if [[ ! "$RETENTION_DAYS" =~ ^[0-9]+$ ]] || (( RETENTION_DAYS > MAX_RETENTION_DAYS )); then
        log_error "Retention days must be an integer from 0 to $MAX_RETENTION_DAYS"
        return 1
    fi

    if [[ ! "$DELETE_LIMIT" =~ ^[1-9][0-9]*$ ]] || (( DELETE_LIMIT > MAX_DELETE_LIMIT )); then
        log_error "Delete limit must be an integer from 1 to $MAX_DELETE_LIMIT"
        return 1
    fi

    parse_s3_target "$S3_BUCKET"
}

last_modified_to_epoch() {
    local last_modified="$1"
    local epoch
    local utc_value

    if epoch="$(date -u -d "$last_modified" +%s 2>/dev/null)" && [[ "$epoch" =~ ^[0-9]+$ ]]; then
        printf '%s\n' "$epoch"
        return 0
    fi

    # BSD date does not accept all RFC 3339 variants accepted by GNU date. S3
    # emits UTC values, so normalize +00:00 to the BSD-compatible Z form.
    utc_value="${last_modified%+00:00}"
    if [[ "$utc_value" != "$last_modified" ]]; then
        utc_value="${utc_value}Z"
    fi
    if epoch="$(date -j -u -f '%Y-%m-%dT%H:%M:%SZ' "$utc_value" +%s 2>/dev/null)" && [[ "$epoch" =~ ^[0-9]+$ ]]; then
        printf '%s\n' "$epoch"
        return 0
    fi

    return 1
}

backup_timestamp_from_key() {
    local key="$1"
    local prefix="$2"
    local expected_prefix="${S3_KEY_PREFIX}${prefix}"
    local object_name

    if [[ "$key" != "$expected_prefix"* ]]; then
        return 1
    fi

    object_name="${key#"$expected_prefix"}"
    case "$prefix" in
        config/)
            if [[ "$object_name" =~ ^sdkwork-im-config_([0-9]{8}_[0-9]{6})\.tar\.gz$ ]]; then
                printf '%s\n' "${BASH_REMATCH[1]}"
                return 0
            fi
            ;;
        db-full/)
            if [[ "$object_name" =~ ^sdkwork-im-db_([0-9]{8}_[0-9]{6})\.dump$ ]]; then
                printf '%s\n' "${BASH_REMATCH[1]}"
                return 0
            fi
            ;;
        redis/)
            if [[ "$object_name" =~ ^sdkwork-im-redis_([0-9]{8}_[0-9]{6})\.rdb$ ]]; then
                printf '%s\n' "${BASH_REMATCH[1]}"
                return 0
            fi
            ;;
        *)
            log_error "Unsupported cleanup prefix: $prefix"
            return 1
            ;;
    esac

    return 1
}

# Enumerate one bounded S3 API page at a time. Callback arguments are:
# prefix, key, backup timestamp, LastModified epoch, LastModified value.
list_backup_objects() {
    local prefix="$1"
    local callback="$2"
    local continuation_token=""
    local previous_continuation_token=""
    local temp_base="${TMP_DIR%/}"
    local work_dir
    local page_file
    local objects_file
    local is_truncated
    local key
    local last_modified
    local timestamp
    local last_modified_epoch
    local -a list_args

    if [[ -z "$temp_base" ]]; then
        temp_base="/tmp"
    fi
    if ! work_dir="$(mktemp -d "$temp_base/sdkwork-im-backup-cleanup.XXXXXX")"; then
        log_error "Unable to create a temporary directory for backup cleanup"
        return 1
    fi
    page_file="$work_dir/page.json"
    objects_file="$work_dir/objects.tsv"

    while :; do
        list_args=(
            s3api list-objects-v2
            --bucket "$S3_BUCKET_NAME"
            --prefix "${S3_KEY_PREFIX}${prefix}"
            --max-keys 1000
            --no-paginate
            --output json
        )
        if [[ -n "$continuation_token" ]]; then
            list_args+=(--continuation-token "$continuation_token")
        fi

        if ! aws "${list_args[@]}" > "$page_file"; then
            log_error "Unable to list backup objects for ${S3_BUCKET}/${prefix}; cleanup stopped"
            rm -rf "$work_dir"
            return 1
        fi
        if ! jq -e '((.Contents? == null) or (.Contents | type == "array")) and ((.IsTruncated? == null) or (.IsTruncated | type == "boolean"))' "$page_file" >/dev/null; then
            log_error "S3 returned an invalid object listing for ${S3_BUCKET}/${prefix}; cleanup stopped"
            rm -rf "$work_dir"
            return 1
        fi
        if ! jq -r '.Contents[]? | if type == "object" then [(.Key // ""), (.LastModified // "")] else ["", ""] end | @tsv' "$page_file" > "$objects_file"; then
            log_error "Unable to parse the S3 object listing for ${S3_BUCKET}/${prefix}; cleanup stopped"
            rm -rf "$work_dir"
            return 1
        fi

        while IFS=$'\t' read -r key last_modified; do
            if ! timestamp="$(backup_timestamp_from_key "$key" "$prefix")"; then
                # Cleanup deliberately ignores every object outside the exact backup names.
                continue
            fi
            if [[ -z "$last_modified" ]] || ! last_modified_epoch="$(last_modified_to_epoch "$last_modified")"; then
                log_error "Backup object $key has an invalid LastModified value; cleanup stopped"
                rm -rf "$work_dir"
                return 1
            fi
            if ! "$callback" "$prefix" "$key" "$timestamp" "$last_modified_epoch" "$last_modified"; then
                rm -rf "$work_dir"
                return 1
            fi
        done < "$objects_file"

        if ! is_truncated="$(jq -r 'if .IsTruncated == true then "true" else "false" end' "$page_file")"; then
            log_error "Unable to read S3 pagination state for ${S3_BUCKET}/${prefix}; cleanup stopped"
            rm -rf "$work_dir"
            return 1
        fi
        if [[ "$is_truncated" != "true" ]]; then
            break
        fi
        if ! jq -e '(.NextContinuationToken | type == "string" and length > 0)' "$page_file" >/dev/null; then
            log_error "S3 returned a truncated listing without a continuation token; cleanup stopped"
            rm -rf "$work_dir"
            return 1
        fi
        continuation_token="$(jq -r '.NextContinuationToken // empty' "$page_file")"
        if [[ -z "$continuation_token" || "$continuation_token" == "$previous_continuation_token" ]]; then
            log_error "S3 returned an unsafe continuation token; cleanup stopped"
            rm -rf "$work_dir"
            return 1
        fi
        previous_continuation_token="$continuation_token"
    done

    rm -rf "$work_dir"
}

record_latest_backup_object() {
    local prefix="$1"
    local key="$2"
    local timestamp="$3"
    local epoch="$4"
    local _last_modified="$5"
    # LastModified is the authoritative remote ordering. The strict backup
    # timestamp/key tie-breaker keeps the result deterministic for equal times.
    case "$prefix" in
        db-full/)
            if [[ -z "$LATEST_DB_KEY" || "$epoch" -gt "${LATEST_DB_EPOCH:-0}" || ( "$epoch" -eq "${LATEST_DB_EPOCH:-0}" && "$key" > "$LATEST_DB_KEY" ) ]]; then
                LATEST_DB_KEY="$key"
                LATEST_DB_TIMESTAMP="$timestamp"
                LATEST_DB_EPOCH="$epoch"
            fi
            ;;
        config/)
            if [[ -z "$LATEST_CONFIG_KEY" || "$epoch" -gt "${LATEST_CONFIG_EPOCH:-0}" || ( "$epoch" -eq "${LATEST_CONFIG_EPOCH:-0}" && "$key" > "$LATEST_CONFIG_KEY" ) ]]; then
                LATEST_CONFIG_KEY="$key"
                LATEST_CONFIG_EPOCH="$epoch"
            fi
            ;;
        redis/)
            if [[ -z "$LATEST_REDIS_KEY" || "$epoch" -gt "${LATEST_REDIS_EPOCH:-0}" || ( "$epoch" -eq "${LATEST_REDIS_EPOCH:-0}" && "$key" > "$LATEST_REDIS_KEY" ) ]]; then
                LATEST_REDIS_KEY="$key"
                LATEST_REDIS_EPOCH="$epoch"
            fi
            ;;
    esac
}

is_protected_backup_object() {
    local _prefix="$1"
    local key="$2"
    local timestamp="$3"

    if [[ "$key" == "$LATEST_DB_KEY" || "$key" == "$LATEST_CONFIG_KEY" || "$key" == "$LATEST_REDIS_KEY" ]]; then
        return 0
    fi
    if [[ "$timestamp" == "$LATEST_DB_TIMESTAMP" ]]; then
        return 0
    fi
    return 1
}

is_expired_unprotected_object() {
    local prefix="$1"
    local key="$2"
    local timestamp="$3"
    local epoch="$4"

    (( epoch < CUTOFF_EPOCH )) || return 1
    ! is_protected_backup_object "$prefix" "$key" "$timestamp"
}

count_expired_backup_candidate() {
    local prefix="$1"
    local key="$2"
    local timestamp="$3"
    local epoch="$4"
    local _last_modified="$5"

    if is_expired_unprotected_object "$prefix" "$key" "$timestamp" "$epoch"; then
        CANDIDATE_COUNT=$((CANDIDATE_COUNT + 1))
        if (( CANDIDATE_COUNT > DELETE_LIMIT )); then
            log_error "Cleanup would delete more than the configured limit of $DELETE_LIMIT objects; no objects were deleted"
            return 1
        fi
    fi
}

report_expired_backup_candidate() {
    local prefix="$1"
    local key="$2"
    local timestamp="$3"
    local epoch="$4"
    local last_modified="$5"

    if is_expired_unprotected_object "$prefix" "$key" "$timestamp" "$epoch"; then
        log_info "DRY RUN: would delete s3://${S3_BUCKET_NAME}/${key} (LastModified: $last_modified)"
    fi
}

delete_expired_backup_candidate() {
    local prefix="$1"
    local key="$2"
    local timestamp="$3"
    local epoch="$4"
    local _last_modified="$5"

    if ! is_expired_unprotected_object "$prefix" "$key" "$timestamp" "$epoch"; then
        return 0
    fi
    if (( DELETED_COUNT >= DELETE_LIMIT )); then
        log_error "Cleanup candidate set changed after validation; cleanup stopped at the deletion limit"
        return 1
    fi
    if ! aws s3api delete-object --bucket "$S3_BUCKET_NAME" --key "$key" --output json >/dev/null; then
        log_error "Failed to delete expired backup object s3://${S3_BUCKET_NAME}/${key}; cleanup stopped"
        return 1
    fi
    DELETED_COUNT=$((DELETED_COUNT + 1))
}

cleanup_expired_backups() {
    local now_epoch
    local prefix

    if ! command -v aws >/dev/null 2>&1; then
        log_warn "aws-cli not installed; skip expired backup cleanup"
        return 0
    fi
    if ! command -v jq >/dev/null 2>&1; then
        log_error "jq is required for safe backup cleanup; no objects were deleted"
        return 1
    fi
    if ! command -v mktemp >/dev/null 2>&1; then
        log_error "mktemp is required for safe backup cleanup; no objects were deleted"
        return 1
    fi
    if ! now_epoch="$(date -u +%s 2>/dev/null)" || [[ ! "$now_epoch" =~ ^[0-9]+$ ]]; then
        log_error "Unable to determine the current UTC time; no objects were deleted"
        return 1
    fi

    CUTOFF_EPOCH=$((now_epoch - (RETENTION_DAYS * 86400)))
    LATEST_DB_KEY=""
    LATEST_DB_TIMESTAMP=""
    LATEST_CONFIG_KEY=""
    LATEST_REDIS_KEY=""
    LATEST_DB_EPOCH=0
    LATEST_CONFIG_EPOCH=0
    LATEST_REDIS_EPOCH=0

    # The latest database object is the restore entry point. Protect its
    # timestamp companions and the newest artifact in every backup prefix.
    for prefix in "config/" "db-full/" "redis/"; do
        list_backup_objects "$prefix" record_latest_backup_object || return 1
    done
    if [[ -z "$LATEST_DB_KEY" || -z "$LATEST_DB_TIMESTAMP" ]]; then
        log_error "No valid database backup recovery point was found; no objects were deleted"
        return 1
    fi

    CANDIDATE_COUNT=0
    for prefix in "config/" "db-full/" "redis/"; do
        list_backup_objects "$prefix" count_expired_backup_candidate || return 1
    done

    if (( CANDIDATE_COUNT == 0 )); then
        log_pass "No expired backup objects are eligible for cleanup; latest recovery point is preserved"
        return 0
    fi
    if [[ "$DRY_RUN" == "true" ]]; then
        log_info "Dry-run cleanup found $CANDIDATE_COUNT eligible object(s); no objects will be deleted"
        for prefix in "config/" "db-full/" "redis/"; do
            list_backup_objects "$prefix" report_expired_backup_candidate || return 1
        done
        log_pass "Dry-run cleanup completed without deletion"
        return 0
    fi

    DELETED_COUNT=0
    for prefix in "config/" "db-full/" "redis/"; do
        list_backup_objects "$prefix" delete_expired_backup_candidate || return 1
    done
    if (( DELETED_COUNT != CANDIDATE_COUNT )); then
        log_error "Cleanup candidate set changed during deletion; cleanup stopped after $DELETED_COUNT object(s)"
        return 1
    fi
    log_pass "Safely deleted $DELETED_COUNT expired backup object(s); latest recovery point is preserved"
}

main() {
    parse_arguments "$@" || return 1
    if [[ "$SHOW_HELP" == "true" ]]; then
        return 0
    fi
    validate_cleanup_configuration || return 1

    log_info "=== SDKWork IM Backup Start ==="
    log_info "Timestamp: $(date)"
    log_info "Target: $S3_BUCKET"
    log_info "Retention: $RETENTION_DAYS days"
    log_info "Delete limit: $DELETE_LIMIT objects"
    if [[ "$DRY_RUN" == "true" ]]; then
        log_warn "Dry-run is enabled for remote cleanup only; backup uploads still run"
    fi
    echo ""

    # ========================================================================
    # 1. Application configuration backup
    # ========================================================================
    log_info "1. Backing up application configuration..."

    local config_archive="${TMP_DIR}/sdkwork-im-config_${BACKUP_DATE}.tar.gz"
    if tar -czf "$config_archive" \
        -C "$ROOT_DIR" \
        etc/ \
        deployments/templates/ \
        sdkwork.app.config.json \
        sdkwork.workflow.json \
        2>/dev/null; then
        log_pass "Configuration archive created: $config_archive"
        if command -v aws >/dev/null 2>&1; then
            if aws s3 cp "$config_archive" "${S3_BUCKET}/config/" >/dev/null 2>&1; then
                log_pass "Configuration uploaded to ${S3_BUCKET}/config/"
            else
                log_warn "Failed to upload configuration to S3; the temporary archive will be removed"
            fi
        else
            log_warn "aws-cli not installed; configuration archive cannot be uploaded"
        fi
    else
        log_warn "Configuration archive creation skipped (some paths may not exist)"
    fi
    rm -f "$config_archive"
    echo ""

    # ========================================================================
    # 2. PostgreSQL full backup
    # ========================================================================
    log_info "2. Backing up PostgreSQL database..."

    if [[ -z "$DATABASE_URL" ]]; then
        log_error "SDKWORK_IM_DATABASE_URL not set; cannot back up database"
        return 1
    fi
    require_tool pg_dump || return 1

    local db_archive="${TMP_DIR}/sdkwork-im-db_${BACKUP_DATE}.dump"
    if pg_dump -Fc -Z9 "$DATABASE_URL" > "$db_archive"; then
        local db_size
        db_size=$(du -h "$db_archive" | cut -f1)
        log_pass "Database backup created: $db_archive ($db_size)"
        if command -v aws >/dev/null 2>&1; then
            if aws s3 cp "$db_archive" "${S3_BUCKET}/db-full/" >/dev/null 2>&1; then
                log_pass "Database backup uploaded to ${S3_BUCKET}/db-full/"
            else
                log_error "Failed to upload database backup to S3"
                rm -f "$db_archive"
                return 1
            fi
        else
            log_error "aws-cli not installed; database backup cannot be uploaded"
            rm -f "$db_archive"
            return 1
        fi
    else
        log_error "pg_dump failed; aborting backup"
        rm -f "$db_archive"
        return 1
    fi
    rm -f "$db_archive"
    echo ""

    # ========================================================================
    # 3. Redis snapshot backup
    # ========================================================================
    log_info "3. Backing up Redis..."

    if [[ -n "$REDIS_NODES" ]]; then
        require_tool redis-cli || return 1
        local first_node
        local host
        local port
        first_node=$(echo "$REDIS_NODES" | cut -d',' -f1)
        host=$(echo "$first_node" | sed -E 's|redis(s?)://([^:]+):.*|\2|; s|redis(s?)://([^:]+)|\2|')
        port=$(echo "$first_node" | sed -E 's|.*:([0-9]+).*|\1|; t; s|.*||')
        port="${port:-6379}"

        if redis-cli -h "$host" -p "$port" BGSAVE >/dev/null 2>&1; then
            log_info "Redis BGSAVE triggered; waiting for completion..."
            local last_save
            local current_time
            local rdb_archive="${TMP_DIR}/sdkwork-im-redis_${BACKUP_DATE}.rdb"
            for _ in {1..30}; do
                last_save=$(redis-cli -h "$host" -p "$port" LASTSAVE 2>/dev/null || echo 0)
                current_time=$(date +%s)
                if [[ $((current_time - last_save)) -lt 5 ]]; then
                    log_pass "Redis BGSAVE completed"
                    break
                fi
                sleep 2
            done

            if redis-cli -h "$host" -p "$port" --rdb "$rdb_archive" >/dev/null 2>&1; then
                local rdb_size
                rdb_size=$(du -h "$rdb_archive" | cut -f1)
                log_pass "Redis RDB snapshot created: $rdb_archive ($rdb_size)"
                if command -v aws >/dev/null 2>&1; then
                    if aws s3 cp "$rdb_archive" "${S3_BUCKET}/redis/" >/dev/null 2>&1; then
                        log_pass "Redis backup uploaded to ${S3_BUCKET}/redis/"
                    else
                        log_warn "Failed to upload Redis backup to S3; the temporary archive will be removed"
                    fi
                else
                    log_warn "aws-cli not installed; Redis RDB cannot be uploaded"
                fi
            else
                log_warn "Redis RDB export failed; cluster may not support --rdb"
            fi
            rm -f "$rdb_archive"
        else
            log_warn "Redis BGSAVE failed; skipping Redis backup"
        fi
    else
        log_warn "Redis nodes not configured; skipping Redis backup"
    fi
    echo ""

    # ========================================================================
    # 4. Expired backup cleanup
    # ========================================================================
    log_info "4. Evaluating backup retention (older than $RETENTION_DAYS days)..."
    cleanup_expired_backups
    echo ""

    log_info "=== SDKWork IM Backup Completed ==="
    log_pass "Backup finished at $(date)"
}

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
    main "$@"
fi
