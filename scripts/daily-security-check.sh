#!/bin/bash
# 文件: scripts/daily-security-check.sh
# 描述: SDKWork IM 每日安全巡检，覆盖签名校验、HTTPS、证书有效期、异常登录、审计日志完整性
# 用法: ./scripts/daily-security-check.sh [--profile production]
# 创建日期: 2026-07-03

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
PROFILE="${1:-production}"

GATEWAY_URL="${SDKWORK_IM_PUBLIC_URL:-http://localhost:18079}"
AUDIT_LOG_PATH="${SDKWORK_IM_AUDIT_LOG_PATH:-/var/log/sdkwork-im/audit.log}"
SSL_CERT_PATH="${SDKWORK_IM_SSL_CERT_PATH:-}"
FAILED_LOGIN_THRESHOLD="${SDKWORK_IM_FAILED_LOGIN_THRESHOLD:-100}"

ERRORS=0
WARNINGS=0
PASSED=0

print_header() {
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

check_pass() { echo -e "${GREEN}✅ PASS${NC}: $1"; PASSED=$((PASSED + 1)); }
check_fail() { echo -e "${RED}❌ FAIL${NC}: $1"; echo -e "${RED}         $2${NC}"; ERRORS=$((ERRORS + 1)); }
check_warn() { echo -e "${YELLOW}⚠️  WARN${NC}: $1"; echo -e "${YELLOW}         $2${NC}"; WARNINGS=$((WARNINGS + 1)); }

echo -e "${BLUE}=== SDKWork IM Daily Security Check ===${NC}"
echo "Profile: $PROFILE"
echo "Timestamp: $(date)"
echo ""

# 先复用 check-security-config.sh 的配置审计
if [ -f "$SCRIPT_DIR/check-security-config.sh" ]; then
    print_header "0. Reusing configuration audit from check-security-config.sh"
    bash "$SCRIPT_DIR/check-security-config.sh" "$PROFILE" || {
        echo -e "${YELLOW}⚠️  Configuration audit reported issues; continuing runtime checks${NC}"
    }
    echo ""
fi

# ============================================================================
# 1. SSL 证书有效期
# ============================================================================
print_header "1. SSL Certificate Validity"

if [ -n "$SSL_CERT_PATH" ] && [ -f "$SSL_CERT_PATH" ]; then
    CERT_EXPIRY=$(openssl x509 -in "$SSL_CERT_PATH" -noout -enddate 2>/dev/null | cut -d= -f2)
    if [ -n "$CERT_EXPIRY" ]; then
        # 兼容 GNU date 和 BSD date
        EXPIRY_EPOCH=$(date -d "$CERT_EXPIRY" +%s 2>/dev/null || date -j -f "%b %d %T %Y %Z" "$CERT_EXPIRY" +%s 2>/dev/null || echo 0)
        if [ "$EXPIRY_EPOCH" -gt 0 ]; then
            DAYS_LEFT=$(( (EXPIRY_EPOCH - $(date +%s)) / 86400 ))
            if [ "$DAYS_LEFT" -ge 30 ]; then
                check_pass "SSL certificate valid for $DAYS_LEFT days"
            elif [ "$DAYS_LEFT" -gt 0 ]; then
                check_warn "SSL certificate expires in $DAYS_LEFT days" "Schedule renewal immediately"
            else
                check_fail "SSL certificate has expired" "Renew immediately to restore HTTPS"
            fi
        else
            check_warn "Unable to parse certificate expiry" "Inspect $SSL_CERT_PATH manually"
        fi
    fi
elif command -v openssl >/dev/null 2>&1 && [ -n "$SDKWORK_IM_FORCE_HTTPS" ] && [ "$SDKWORK_IM_FORCE_HTTPS" == "true" ]; then
    # 尝试通过 TLS 握手探测
    HOST=$(echo "$GATEWAY_URL" | sed -E 's|https?://([^:/]+).*|\1|')
    PORT=$(echo "$GATEWAY_URL" | sed -E 's|https?://[^:]+:([0-9]+).*|\1|; t; s|.*|443|')
    if [ "$PORT" != "443" ] && [ "$PORT" == "$GATEWAY_URL" ]; then PORT=443; fi
    CERT_EXPIRY=$(echo | openssl s_client -connect "${HOST}:${PORT}" -servername "$HOST" 2>/dev/null \
        | openssl x509 -noout -enddate 2>/dev/null | cut -d= -f2)
    if [ -n "$CERT_EXPIRY" ]; then
        EXPIRY_EPOCH=$(date -d "$CERT_EXPIRY" +%s 2>/dev/null || date -j -f "%b %d %T %Y %Z" "$CERT_EXPIRY" +%s 2>/dev/null || echo 0)
        if [ "$EXPIRY_EPOCH" -gt 0 ]; then
            DAYS_LEFT=$(( (EXPIRY_EPOCH - $(date +%s)) / 86400 ))
            if [ "$DAYS_LEFT" -ge 30 ]; then
                check_pass "TLS certificate valid for $DAYS_LEFT days ($HOST:$PORT)"
            else
                check_warn "TLS certificate expires in $DAYS_LEFT days" "Plan renewal"
            fi
        fi
    else
        check_warn "Unable to probe TLS certificate for $HOST:$PORT" "Verify HTTPS is enforced"
    fi
else
    check_warn "SSL certificate path not configured" "Set SDKWORK_IM_SSL_CERT_PATH for direct inspection"
fi

# ============================================================================
# 2. 审计日志完整性
# ============================================================================
print_header "2. Audit Log Integrity"

if [ -f "$AUDIT_LOG_PATH" ]; then
    LOG_SIZE=$(du -h "$AUDIT_LOG_PATH" | cut -f1)
    check_pass "Audit log exists (size: $LOG_SIZE)"

    if command -v wc >/dev/null 2>&1; then
        LINE_COUNT=$(wc -l < "$AUDIT_LOG_PATH" 2>/dev/null || echo 0)
        if [ "$LINE_COUNT" -lt 100 ]; then
            check_warn "Audit log entries too few ($LINE_COUNT)" "Verify audit pipeline is shipping events"
        else
            check_pass "Audit log has $LINE_COUNT entries"
        fi
    fi

    # 异常登录检测
    if command -v grep >/dev/null 2>&1; then
        FAILED_LOGINS=$(grep -c "LOGIN_FAILED\|login_failed\|auth_failure" "$AUDIT_LOG_PATH" 2>/dev/null || echo 0)
        if [ "$FAILED_LOGINS" -gt "$FAILED_LOGIN_THRESHOLD" ]; then
            check_fail "High failed login attempts: $FAILED_LOGINS" "Investigate potential brute-force attack"
        elif [ "$FAILED_LOGINS" -gt 0 ]; then
            check_pass "Failed login attempts within threshold: $FAILED_LOGINS"
        else
            check_pass "No failed login attempts detected in current audit log"
        fi
    fi
else
    check_warn "Audit log file not found: $AUDIT_LOG_PATH" "Set SDKWORK_IM_AUDIT_LOG_PATH to the deployed log location"
fi

# ============================================================================
# 3. 防火墙与端口暴露
# ============================================================================
print_header "3. Firewall and Port Exposure"

if command -v iptables >/dev/null 2>&1; then
    GATEWAY_PORT=$(echo "$GATEWAY_URL" | sed -E 's|.*:([0-9]+).*|\1|; t; s|.*|80|')
    if iptables -L -n 2>/dev/null | grep -qE "DROP.*${GATEWAY_PORT}|REJECT.*${GATEWAY_PORT}"; then
        check_pass "Gateway port $GATEWAY_PORT has DROP/REJECT rule"
    else
        check_warn "No explicit DROP rule for gateway port $GATEWAY_PORT" "Ensure ingress controller restricts access"
    fi
else
    check_warn "iptables not available" "Verify firewall rules via cloud provider console"
fi

# ============================================================================
# 4. 数据库连接异常
# ============================================================================
print_header "4. Database Connection Anomaly"

DB_URL="${SDKWORK_IM_DATABASE_URL:-}"
if [ -n "$DB_URL" ] && command -v psql >/dev/null 2>&1; then
    ACTIVE_CONN=$(psql "$DB_URL" -t -c "SELECT count(*) FROM pg_stat_activity WHERE usename NOT IN ('postgres', 'replication')" 2>/dev/null | tr -d '[:space:]')
    MAX_CONN=$(psql "$DB_URL" -t -c "SELECT setting::int FROM pg_settings WHERE name='max_connections'" 2>/dev/null | tr -d '[:space:]')
    if [ -n "$ACTIVE_CONN" ] && [ -n "$MAX_CONN" ] && [ "$MAX_CONN" -gt 0 ]; then
        RATIO=$((ACTIVE_CONN * 100 / MAX_CONN))
        if [ "$RATIO" -gt 80 ]; then
            check_warn "Database connections high: $ACTIVE_CONN/$MAX_CONN ($RATIO%)" "Investigate connection leaks"
        else
            check_pass "Database connections healthy: $ACTIVE_CONN/$MAX_CONN ($RATIO%)"
        fi
    fi
fi

# ============================================================================
# Summary
# ============================================================================
print_header "Daily Security Check Summary"
echo -e "${GREEN}Passed:${NC}    $PASSED"
echo -e "${YELLOW}Warnings:${NC}  $WARNINGS"
echo -e "${RED}Errors:${NC}    $ERRORS"
echo ""

if [ "$ERRORS" -gt 0 ]; then
    echo -e "${RED}❌ Daily security check found $ERRORS critical issue(s)${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Daily security check completed${NC}"
exit 0
