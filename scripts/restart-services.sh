#!/bin/bash
# 文件: scripts/restart-services.sh
# 描述: SDKWork IM 服务优雅重启脚本，支持 Docker Compose 和 systemd 两种部署模式
# 用法: ./scripts/restart-services.sh [--mode docker|systemd|kubernetes] [--timeout 30] [--namespace sdkwork-im]
# 创建日期: 2026-07-03

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

MODE="${SDKWORK_IM_DEPLOYMENT_MODE:-docker}"
TIMEOUT=30
NAMESPACE="sdkwork-im"
DRAIN_WAIT=30
GATEWAY_URL="${SDKWORK_IM_PUBLIC_URL:-http://localhost:18079}"

# 解析参数
while [[ $# -gt 0 ]]; do
    case "$1" in
        --mode) MODE="$2"; shift 2 ;;
        --timeout) TIMEOUT="$2"; shift 2 ;;
        --namespace) NAMESPACE="$2"; shift 2 ;;
        --drain-wait) DRAIN_WAIT="$2"; shift 2 ;;
        -h|--help)
            echo "Usage: $0 [--mode docker|systemd|kubernetes] [--timeout N] [--namespace NS]"
            exit 0
            ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

log_info()  { echo -e "${BLUE}[INFO]${NC}  $1"; }
log_pass()  { echo -e "${GREEN}[PASS]${NC}  $1"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC}  $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

log_info "=== SDKWork IM Service Restart ==="
log_info "Mode: $MODE"
log_info "Timeout: ${TIMEOUT}s"
log_info "Drain wait: ${DRAIN_WAIT}s"
log_info "Timestamp: $(date)"
echo ""

# ============================================================================
# 1. 优雅停止服务
# ============================================================================
log_info "1. Gracefully stopping services..."

case "$MODE" in
    docker)
        if ! command -v docker-compose >/dev/null 2>&1 && ! command -v docker >/dev/null 2>&1; then
            log_error "Neither docker-compose nor docker is installed"
            exit 1
        fi
        if command -v docker-compose >/dev/null 2>&1; then
            COMPOSE_CMD="docker-compose"
        else
            COMPOSE_CMD="docker compose"
        fi

        if [ -f "$ROOT_DIR/deployments/deploy.yaml" ]; then
            $COMPOSE_CMD -f "$ROOT_DIR/deployments/deploy.yaml" stop --timeout "$TIMEOUT" \
                || log_warn "docker-compose stop returned non-zero; services may already be stopped"
        else
            log_warn "deployments/deploy.yaml not found; attempting default docker-compose stop"
            $COMPOSE_CMD stop --timeout "$TIMEOUT" \
                || log_warn "docker-compose stop returned non-zero"
        fi
        ;;
    systemd)
        if ! command -v systemctl >/dev/null 2>&1; then
            log_error "systemctl not available"
            exit 1
        fi
        systemctl stop sdkwork-im-gateway.service 2>/dev/null || log_warn "sdkwork-im-gateway.service not found"
        systemctl stop sdkwork-im-session-gateway.service 2>/dev/null || true
        systemctl stop sdkwork-im-conversation-service.service 2>/dev/null || true
        systemctl stop sdkwork-im-projection-service.service 2>/dev/null || true
        ;;
    kubernetes)
        if ! command -v kubectl >/dev/null 2>&1; then
            log_error "kubectl not installed"
            exit 1
        fi
        log_info "Scaling down deployments in namespace $NAMESPACE..."
        for DEPLOY in im-gateway session-gateway conversation-service projection-service media-service streaming-service governance-service notification-service; do
            kubectl -n "$NAMESPACE" scale deployment "$DEPLOY" --replicas=0 2>/dev/null \
                && log_pass "Scaled down $DEPLOY" \
                || log_warn "Deployment $DEPLOY not found or already at 0 replicas"
        done
        ;;
    *)
        log_error "Unknown mode: $MODE (supported: docker, systemd, kubernetes)"
        exit 1
        ;;
esac

log_info "Waiting ${DRAIN_WAIT}s for in-flight requests to drain..."
sleep "$DRAIN_WAIT"
log_pass "Drain period completed"
echo ""

# ============================================================================
# 2. 启动服务
# ============================================================================
log_info "2. Starting services..."

case "$MODE" in
    docker)
        if [ -f "$ROOT_DIR/deployments/deploy.yaml" ]; then
            $COMPOSE_CMD -f "$ROOT_DIR/deployments/deploy.yaml" up -d
        else
            $COMPOSE_CMD up -d
        fi
        ;;
    systemd)
        systemctl start sdkwork-im-projection-service.service 2>/dev/null || true
        systemctl start sdkwork-im-conversation-service.service 2>/dev/null || true
        systemctl start sdkwork-im-session-gateway.service 2>/dev/null || true
        systemctl start sdkwork-im-gateway.service 2>/dev/null || log_error "Failed to start sdkwork-im-gateway.service"
        ;;
    kubernetes)
        for DEPLOY in projection-service conversation-service session-gateway im-gateway media-service streaming-service governance-service notification-service; do
            kubectl -n "$NAMESPACE" scale deployment "$DEPLOY" --replicas=1 2>/dev/null \
                && log_pass "Scaled up $DEPLOY" \
                || log_warn "Deployment $DEPLOY not found"
        done
        ;;
esac
echo ""

# ============================================================================
# 3. 健康检查轮询
# ============================================================================
log_info "3. Health check polling..."

if ! command -v curl >/dev/null 2>&1; then
    log_warn "curl not installed; skip health check polling"
    log_info "Manually verify: curl ${GATEWAY_URL}/readyz"
    exit 0
fi

MAX_RETRIES=12
RETRY_INTERVAL=5

for i in $(seq 1 "$MAX_RETRIES"); do
    if curl -sf "${GATEWAY_URL}/healthz" >/dev/null 2>&1; then
        log_pass "Liveness check passed (attempt $i/$MAX_RETRIES)"
        if curl -sf "${GATEWAY_URL}/readyz" >/dev/null 2>&1; then
            log_pass "Readiness check passed"
            log_info "=== Services Restarted Successfully ==="
            log_pass "Finished at $(date)"
            exit 0
        else
            log_warn "Readiness check not yet passing (attempt $i/$MAX_RETRIES)"
        fi
    else
        log_info "Waiting for services to start... (attempt $i/$MAX_RETRIES)"
    fi
    sleep "$RETRY_INTERVAL"
done

log_error "Services failed to become ready after $((MAX_RETRIES * RETRY_INTERVAL))s"
log_error "Check logs with:"
case "$MODE" in
    docker)      log_error "  docker logs sdkwork-im-gateway" ;;
    systemd)     log_error "  journalctl -u sdkwork-im-gateway.service -n 200" ;;
    kubernetes)  log_error "  kubectl -n $NAMESPACE logs deploy/im-gateway --tail=200" ;;
esac
exit 1
