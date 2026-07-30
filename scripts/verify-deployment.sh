#!/bin/bash
# 鏂囦欢: scripts/verify-deployment.sh
# 鎻忚堪: SDKWork IM 閮ㄧ讲鍚庡姛鑳介獙璇佽剼鏈紝瑕嗙洊鍋ュ悍妫€鏌ャ€佷緷璧栬繛閫氭€с€丄PI 涓?WebSocket 鍙敤鎬?
# 鐢ㄦ硶: ./scripts/verify-deployment.sh [--gateway-url http://localhost:18079] [--ws-url ws://localhost:18079]
# 鍒涘缓鏃ユ湡: 2026-07-03

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

GATEWAY_URL="http://localhost:18079"
WS_URL="ws://localhost:18079"
DATABASE_URL="${SDKWORK_DATABASE_URL:-}"
REDIS_NODES="${SDKWORK_IM_REDIS_CLUSTER_NODES:-${SDKWORK_IM_REDIS_URL:-}}"

# 瑙ｆ瀽鍙傛暟
while [[ $# -gt 0 ]]; do
    case "$1" in
        --gateway-url) GATEWAY_URL="$2"; shift 2 ;;
        --ws-url) WS_URL="$2"; shift 2 ;;
        --database-url) DATABASE_URL="$2"; shift 2 ;;
        --redis-nodes) REDIS_NODES="$2"; shift 2 ;;
        -h|--help)
            echo "Usage: $0 [--gateway-url URL] [--ws-url URL] [--database-url URL] [--redis-nodes URL]"
            exit 0
            ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

ERRORS=0
PASSED=0

print_header() {
    echo -e "${BLUE}鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣鈹佲攣${NC}"
}

check_pass() { echo -e "${GREEN}鉁?PASS${NC}: $1"; PASSED=$((PASSED + 1)); }
check_fail() { echo -e "${RED}鉂?FAIL${NC}: $1"; echo -e "${RED}         $2${NC}"; ERRORS=$((ERRORS + 1)); }
check_warn() { echo -e "${YELLOW}鈿狅笍  WARN${NC}: $1"; echo -e "${YELLOW}         $2${NC}"; }

echo -e "${BLUE}=== SDKWork IM Deployment Verification ===${NC}"
echo "Gateway URL: $GATEWAY_URL"
echo "WebSocket URL: $WS_URL"
echo "Timestamp: $(date)"
echo ""

# ============================================================================
# 1. 鍋ュ悍妫€鏌ョ鐐?
# ============================================================================
print_header "1. Health Endpoint Checks"

if curl -sf "${GATEWAY_URL}/healthz" >/dev/null 2>&1; then
    check_pass "Liveness endpoint /healthz reachable"
else
    check_fail "Liveness endpoint /healthz unreachable" "Verify gateway service is running on $GATEWAY_URL"
fi

if curl -sf "${GATEWAY_URL}/readyz" >/dev/null 2>&1; then
    check_pass "Readiness endpoint /readyz reachable"
else
    check_fail "Readiness endpoint /readyz unreachable" "Check PostgreSQL, Redis, and IAM dependencies"
fi

# ============================================================================
# 2. 鏁版嵁搴撹繛閫氭€?
# ============================================================================
print_header "2. Database Connectivity"

if [ -n "$DATABASE_URL" ]; then
    if command -v psql >/dev/null 2>&1; then
        if psql "$DATABASE_URL" -c "SELECT 1" >/dev/null 2>&1; then
            check_pass "PostgreSQL connection successful"
            ACTIVE=$(psql "$DATABASE_URL" -t -c "SELECT count(*) FROM pg_stat_activity WHERE datname=current_database()" 2>/dev/null | tr -d ' ')
            check_pass "Active database connections: ${ACTIVE:-unknown}"
        else
            check_fail "PostgreSQL connection failed" "Verify SDKWORK_DATABASE_URL and credentials"
        fi
    else
        check_warn "psql client not installed" "Cannot verify database connectivity"
    fi
else
    check_warn "SDKWORK_DATABASE_URL not set" "Export it or pass --database-url"
fi

# ============================================================================
# 3. Redis 杩為€氭€?
# ============================================================================
print_header "3. Redis Connectivity"

if [ -n "$REDIS_NODES" ]; then
    if command -v redis-cli >/dev/null 2>&1; then
        FIRST_NODE=$(echo "$REDIS_NODES" | cut -d',' -f1)
        HOST=$(echo "$FIRST_NODE" | sed -E 's|redis(s?)://([^:]+):.*|\2|; s|redis(s?)://([^:]+)|\2|')
        PORT=$(echo "$FIRST_NODE" | sed -E 's|.*:([0-9]+).*|\1|; t; s|.*||')
        PORT="${PORT:-6379}"

        if redis-cli -h "$HOST" -p "$PORT" PING >/dev/null 2>&1; then
            check_pass "Redis PING successful ($HOST:$PORT)"
            CLUSTER_STATE=$(redis-cli -c -h "$HOST" -p "$PORT" CLUSTER INFO 2>/dev/null | grep "^cluster_state:" | cut -d: -f2 | tr -d '[:space:]')
            if [ "$CLUSTER_STATE" == "ok" ]; then
                check_pass "Redis cluster state: ok"
            elif [ -n "$CLUSTER_STATE" ]; then
                check_warn "Redis cluster state: $CLUSTER_STATE" "Investigate cluster health"
            fi
        else
            check_fail "Redis PING failed" "Verify Redis connectivity to $HOST:$PORT"
        fi
    else
        check_warn "redis-cli not installed" "Cannot verify Redis connectivity"
    fi
else
    check_warn "Redis nodes not configured" "Set SDKWORK_IM_REDIS_CLUSTER_NODES or SDKWORK_IM_REDIS_URL"
fi

# ============================================================================
# 4. WebSocket 鍙揪鎬?
# ============================================================================
print_header "4. WebSocket Reachability"

if command -v wscat >/dev/null 2>&1; then
    if timeout 5 wscat -c "$WS_URL" -x '{"type":"ping"}' >/dev/null 2>&1; then
        check_pass "WebSocket endpoint reachable"
    else
        check_warn "WebSocket handshake timed out or rejected" "Auth may be required; verify ingress and port"
    fi
else
    # 鍥為€€鍒?HTTP Upgrade 鎺㈡祴
    if curl -sf -o /dev/null -w "%{http_code}" \
        -H "Connection: Upgrade" -H "Upgrade: websocket" \
        -H "Sec-WebSocket-Version: 13" -H "Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==" \
        "$WS_URL" 2>/dev/null | grep -qE "101|4xx|5xx"; then
        check_pass "WebSocket upgrade endpoint responds"
    else
        check_warn "WebSocket endpoint probe inconclusive" "Install wscat for accurate test"
    fi
fi

# ============================================================================
# 5. API 鐑熼浘娴嬭瘯
# ============================================================================
print_header "5. API Smoke Test"

HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "${GATEWAY_URL}/im/v3/api/health" 2>/dev/null || echo "000")
case "$HTTP_STATUS" in
    200|204) check_pass "API health endpoint returned $HTTP_STATUS" ;;
    401|403) check_pass "API endpoint reachable (auth required: $HTTP_STATUS)" ;;
    404)     check_warn "API health endpoint returned 404" "Route may differ; verify OpenAPI manifest" ;;
    000)     check_fail "API endpoint unreachable" "Gateway not responding" ;;
    *)       check_warn "API endpoint returned $HTTP_STATUS" "Investigate non-standard response" ;;
esac

# ============================================================================
# Summary
# ============================================================================
print_header "Deployment Verification Summary"
echo -e "${GREEN}Passed:${NC} $PASSED"
echo -e "${RED}Errors:${NC} $ERRORS"
echo ""

if [ "$ERRORS" -gt 0 ]; then
    echo -e "${RED}鉂?Deployment verification failed with $ERRORS error(s)${NC}"
    exit 1
fi

echo -e "${GREEN}鉁?All deployment checks passed${NC}"
exit 0
