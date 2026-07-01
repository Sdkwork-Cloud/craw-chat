#!/bin/bash
# 文件: scripts/check-security-config.sh
# 描述: SDKWork IM生产环境安全配置验证脚本
# 用法: ./scripts/check-security-config.sh [--profile production|staging|development]
# 创建日期: 2026-06-30

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 配置
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CONFIG_DIR="$ROOT_DIR/configs/topology"

# 默认profile
PROFILE="${1:-production}"

# 计数器
ERRORS=0
WARNINGS=0
PASSED=0

echo -e "${BLUE}=== SDKWork IM Security Configuration Check ===${NC}"
echo "Profile: $PROFILE"
echo "Timestamp: $(date)"
echo ""

# 函数定义
print_header() {
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

check_pass() {
    echo -e "${GREEN}✅ PASS${NC}: $1"
    PASSED=$((PASSED + 1))
}

check_fail() {
    echo -e "${RED}❌ FAIL${NC}: $1"
    echo -e "${RED}         $2${NC}"
    ERRORS=$((ERRORS + 1))
}

check_warning() {
    echo -e "${YELLOW}⚠️  WARN${NC}: $1"
    echo -e "${YELLOW}         $2${NC}"
    WARNINGS=$((WARNINGS + 1))
}

get_config_value() {
    local key=$1
    local config_file=$2
    
    if [ -f "$config_file" ]; then
        grep "^${key}=" "$config_file" | cut -d'=' -f2 | tr -d '"' | tr -d "'"
    else
        echo ""
    fi
}

# 查找配置文件
find_config_file() {
    local profile=$1
    
    case "$profile" in
        production)
            # 查找生产配置文件
            if [ -f "$CONFIG_DIR/cloud.split-services.production.env" ]; then
                echo "$CONFIG_DIR/cloud.split-services.production.env"
            elif [ -f "$CONFIG_DIR/standalone.unified-process.production.env" ]; then
                echo "$CONFIG_DIR/standalone.unified-process.production.env"
            else
                echo "$ROOT_DIR/.env"
            fi
            ;;
        staging)
            if [ -f "$CONFIG_DIR/cloud.split-services.staging.env" ]; then
                echo "$CONFIG_DIR/cloud.split-services.staging.env"
            else
                echo "$ROOT_DIR/.env"
            fi
            ;;
        development)
            echo "$ROOT_DIR/.env"
            ;;
        *)
            echo "$ROOT_DIR/.env"
            ;;
    esac
}

# 主配置文件
CONFIG_FILE=$(find_config_file "$PROFILE")

if [ ! -f "$CONFIG_FILE" ]; then
    echo -e "${RED}❌ ERROR: Configuration file not found: $CONFIG_FILE${NC}"
    exit 1
fi

echo "Configuration file: $CONFIG_FILE"
echo ""

# ============================================================================
# 1. CRITICAL - 安全配置检查
# ============================================================================

print_header "1. CRITICAL Security Configuration Checks"

# 1.1 JWT签名验证
JWT_SIG=$(get_config_value "SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE" "$CONFIG_FILE")
if [ "$PROFILE" == "production" ]; then
    if [ "$JWT_SIG" == "true" ]; then
        check_pass "JWT signature verification enabled"
    else
        check_fail "JWT signature verification disabled" \
            "Set SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE=true in production"
    fi
else
    if [ "$JWT_SIG" == "true" ]; then
        check_pass "JWT signature verification enabled"
    else
        check_warning "JWT signature verification disabled" \
            "Recommended to enable for all environments"
    fi
fi

# 1.2 Runtime Profile
RUNTIME_PROFILE=$(get_config_value "SDKWORK_IM_RUNTIME_PROFILE" "$CONFIG_FILE")
if [ "$PROFILE" == "production" ]; then
    if [ "$RUNTIME_PROFILE" == "production" ]; then
        check_pass "Runtime profile set to production"
    else
        check_fail "Runtime profile not set to production" \
            "Set SDKWORK_IM_RUNTIME_PROFILE=production"
    fi
else
    if [ -n "$RUNTIME_PROFILE" ]; then
        check_pass "Runtime profile configured: $RUNTIME_PROFILE"
    else
        check_warning "Runtime profile not configured" \
            "Set SDKWORK_IM_RUNTIME_PROFILE=$PROFILE"
    fi
fi

# 1.3 IAM数据库配置
IAM_DB=$(get_config_value "SDKWORK_IM_IAM_DATABASE_URL" "$CONFIG_FILE")
if [ "$PROFILE" == "production" ]; then
    if [ -n "$IAM_DB" ]; then
        # 测试数据库连接
        if command -v psql >/dev/null 2>&1; then
            if psql "$IAM_DB" -c "SELECT 1" >/dev/null 2>&1; then
                check_pass "IAM database connection successful"
            else
                check_warning "IAM database connection failed" \
                    "Verify database URL and credentials"
            fi
        else
            check_pass "IAM database URL configured (connection test skipped)"
        fi
    else
        check_fail "IAM database not configured" \
            "Set SDKWORK_IM_IAM_DATABASE_URL in production"
    fi
else
    if [ -n "$IAM_DB" ]; then
        check_pass "IAM database URL configured"
    else
        check_warning "IAM database not configured" \
            "Recommended for all environments"
    fi
fi

# 1.4 HTTPS强制
HTTPS=$(get_config_value "SDKWORK_IM_FORCE_HTTPS" "$CONFIG_FILE")
if [ "$PROFILE" == "production" ]; then
    if [ "$HTTPS" == "true" ]; then
        check_pass "HTTPS forced in production"
    else
        check_fail "HTTPS not forced in production" \
            "Set SDKWORK_IM_FORCE_HTTPS=true"
    fi
else
    if [ "$HTTPS" == "true" ]; then
        check_pass "HTTPS forced"
    else
        check_warning "HTTPS not forced" \
            "Recommended to force HTTPS in all environments"
    fi
fi

# ============================================================================
# 2. HIGH - Infrastructure Configuration Checks
# ============================================================================

print_header "2. HIGH Infrastructure Configuration Checks"

# 2.1 Redis Cluster配置
REDIS_NODES=$(get_config_value "SDKWORK_IM_REDIS_CLUSTER_NODES" "$CONFIG_FILE")
if [ -n "$REDIS_NODES" ]; then
    NODE_COUNT=$(echo "$REDIS_NODES" | tr ',' '\n' | wc -l)
    if [ "$NODE_COUNT" -ge 3 ]; then
        check_pass "Redis Cluster configured with $NODE_COUNT nodes"
        
        # 测试Redis连接
        if command -v redis-cli >/dev/null 2>&1; then
            FIRST_NODE=$(echo "$REDIS_NODES" | cut -d',' -f1)
            HOST=$(echo "$FIRST_NODE" | sed 's/redis:\/\/\(.*\):.*/\1/')
            PORT=$(echo "$FIRST_NODE" | sed 's/redis:\/\/.*:\(.*\)/\1/')
            
            if redis-cli -h "$HOST" -p "$PORT" PING >/dev/null 2>&1; then
                check_pass "Redis connection successful"
                
                # 检查集群状态
                CLUSTER_STATE=$(redis-cli -c -h "$HOST" -p "$PORT" CLUSTER INFO | grep "cluster_state:" | cut -d':' -f2)
                if [ "$CLUSTER_STATE" == "ok" ]; then
                    check_pass "Redis Cluster state is OK"
                else
                    check_fail "Redis Cluster state is FAIL" \
                        "Check Redis Cluster health: redis-cli CLUSTER INFO"
                fi
            else
                check_warning "Redis connection failed" \
                    "Verify Redis Cluster nodes configuration"
            fi
        fi
    else
        check_warning "Redis Cluster has only $NODE_COUNT nodes" \
            "Recommended minimum 3 nodes for HA"
    fi
else
    # 检查单节点Redis
    REDIS_URL=$(get_config_value "SDKWORK_IM_REDIS_URL" "$CONFIG_FILE")
    if [ -n "$REDIS_URL" ]; then
        if [ "$PROFILE" == "production" ]; then
            check_warning "Single Redis node configured in production" \
                "Recommended to use Redis Cluster for HA"
        else
            check_pass "Redis URL configured"
        fi
    else
        check_warning "Redis not configured" \
            "Set SDKWORK_IM_REDIS_CLUSTER_NODES or SDKWORK_IM_REDIS_URL"
    fi
fi

# 2.2 PostgreSQL连接池配置
MAX_CONN=$(get_config_value "SDKWORK_IM_DATABASE_MAX_CONNECTIONS" "$CONFIG_FILE")
if [ -n "$MAX_CONN" ]; then
    if [ "$PROFILE" == "production" ]; then
        if [ "$MAX_CONN" -ge 50 ]; then
            check_pass "Database connection pool size adequate: $MAX_CONN"
        else
            check_warning "Database connection pool size low: $MAX_CONN" \
                "Recommended minimum 50 connections in production"
        fi
    else
        if [ "$MAX_CONN" -ge 10 ]; then
            check_pass "Database connection pool configured: $MAX_CONN"
        else
            check_warning "Database connection pool size very low: $MAX_CONN"
        fi
    fi
else
    check_warning "Database connection pool size not configured" \
        "Set SDKWORK_IM_DATABASE_MAX_CONNECTIONS"
fi

# 2.3 主数据库配置
DB_URL=$(get_config_value "SDKWORK_IM_DATABASE_URL" "$CONFIG_FILE")
if [ -n "$DB_URL" ]; then
    check_pass "Main database URL configured"
    
    # 测试数据库连接
    if command -v psql >/dev/null 2>&1; then
        if psql "$DB_URL" -c "SELECT 1" >/dev/null 2>&1; then
            check_pass "Main database connection successful"
            
            # 检查连接数
            ACTIVE_CONN=$(psql "$DB_URL" -t -c "SELECT count(*) FROM pg_stat_activity WHERE datname=current_database()")
            if [ "$PROFILE" == "production" ]; then
                if [ "$ACTIVE_CONN" -lt "$MAX_CONN" ]; then
                    check_pass "Database connections within limit: $ACTIVE_CONN / $MAX_CONN"
                else
                    check_warning "Database connections near limit: $ACTIVE_CONN / $MAX_CONN" \
                        "Monitor connection usage and optimize queries"
                fi
            fi
        else
            check_warning "Main database connection failed" \
                "Verify database URL and credentials"
        fi
    fi
else
    check_fail "Main database not configured" \
        "Set SDKWORK_IM_DATABASE_URL"
fi

# ============================================================================
# 3. MEDIUM - Monitoring and Logging Checks
# ============================================================================

print_header "3. MEDIUM Monitoring and Logging Checks"

# 3.1 Prometheus监控
PROM_ENABLED=$(get_config_value "SDKWORK_IM_PROMETHEUS_ENABLED" "$CONFIG_FILE")
if [ "$PROM_ENABLED" == "true" ]; then
    check_pass "Prometheus monitoring enabled"
    
    METRICS_PORT=$(get_config_value "SDKWORK_IM_METRICS_PORT" "$CONFIG_FILE")
    if [ -n "$METRICS_PORT" ]; then
        check_pass "Metrics port configured: $METRICS_PORT"
        
        # 测试metrics endpoint
        if curl -s "http://localhost:$METRICS_PORT/metrics" >/dev/null 2>&1; then
            check_pass "Metrics endpoint accessible"
        else
            check_warning "Metrics endpoint not accessible" \
                "Verify service is running and port is correct"
        fi
    fi
else
    check_warning "Prometheus monitoring not enabled" \
        "Recommended to enable for production visibility"
fi

# 3.2 审计日志
AUDIT_ENABLED=$(get_config_value "SDKWORK_IM_AUDIT_LOG_ENABLED" "$CONFIG_FILE")
if [ "$AUDIT_ENABLED" == "true" ]; then
    check_pass "Audit logging enabled"
    
    AUDIT_LOG_PATH=$(get_config_value "SDKWORK_IM_AUDIT_LOG_PATH" "$CONFIG_FILE")
    if [ -n "$AUDIT_LOG_PATH" ]; then
        check_pass "Audit log path configured: $AUDIT_LOG_PATH"
        
        # 检查日志文件
        if [ -f "$AUDIT_LOG_PATH" ]; then
            LOG_SIZE=$(du -h "$AUDIT_LOG_PATH" | cut -f1)
            check_pass "Audit log file exists (size: $LOG_SIZE)"
            
            # 检查最近日志条目
            if tail -10 "$AUDIT_LOG_PATH" | grep -q "AUDIT"; then
                check_pass "Audit log contains recent entries"
            else
                check_warning "Audit log missing recent entries" \
                    "Verify audit logging is working"
            fi
        else
            check_warning "Audit log file not found: $AUDIT_LOG_PATH"
        fi
    fi
else
    if [ "$PROFILE" == "production" ]; then
        check_warning "Audit logging not enabled in production" \
            "Recommended for compliance and security"
    else
        check_pass "Audit logging not enabled (acceptable for development)"
    fi
fi

# 3.3 日志级别
LOG_LEVEL=$(get_config_value "SDKWORK_IM_LOG_LEVEL" "$CONFIG_FILE")
if [ -n "$LOG_LEVEL" ]; then
    if [ "$PROFILE" == "production" ]; then
        if [ "$LOG_LEVEL" == "info" ] || [ "$LOG_LEVEL" == "warn" ] || [ "$LOG_LEVEL" == "error" ]; then
            check_pass "Log level appropriate for production: $LOG_LEVEL"
        else
            check_warning "Log level too verbose for production: $LOG_LEVEL" \
                "Recommended: info, warn, or error"
        fi
    else
        check_pass "Log level configured: $LOG_LEVEL"
    fi
else
    check_warning "Log level not configured" \
        "Set SDKWORK_IM_LOG_LEVEL (recommended: info)"
fi

# ============================================================================
# 4. LOW - Additional Security Recommendations
# ============================================================================

print_header "4. LOW Additional Security Recommendations"

# 4.1 CORS配置
CORS_ORIGIN=$(get_config_value "SDKWORK_IM_CORS_ALLOWED_ORIGINS" "$CONFIG_FILE")
if [ -n "$CORS_ORIGIN" ]; then
    check_pass "CORS origins configured"
    
    if [ "$PROFILE" == "production" ]; then
        if echo "$CORS_ORIGIN" | grep -q "*"; then
            check_warning "CORS allows all origins in production" \
                "Recommended to restrict to specific domains"
        else
            check_pass "CORS origins restricted in production"
        fi
    fi
else
    check_warning "CORS origins not configured" \
        "Set SDKWORK_IM_CORS_ALLOWED_ORIGINS"
fi

# 4.2 Rate Limiting
RATE_LIMIT_ENABLED=$(get_config_value "SDKWORK_IM_RATE_LIMIT_ENABLED" "$CONFIG_FILE")
if [ "$RATE_LIMIT_ENABLED" == "true" ]; then
    check_pass "Rate limiting enabled"
    
    RATE_LIMIT_RPM=$(get_config_value "SDKWORK_IM_RATE_LIMIT_RPM" "$CONFIG_FILE")
    if [ -n "$RATE_LIMIT_RPM" ]; then
        check_pass "Rate limit configured: $RATE_LIMIT_RPM requests/min"
    fi
else
    if [ "$PROFILE" == "production" ]; then
        check_warning "Rate limiting not enabled in production" \
            "Recommended for security protection"
    fi
fi

# 4.3 Trusted Proxies
TRUSTED_PROXIES=$(get_config_value "SDKWORK_IM_GATEWAY_TRUSTED_PROXIES" "$CONFIG_FILE")
if [ -n "$TRUSTED_PROXIES" ]; then
    check_pass "Trusted proxies configured: $TRUSTED_PROXIES"
else
    if [ "$PROFILE" == "production" ]; then
        check_warning "Trusted proxies not configured" \
            "Configure for accurate IP extraction behind proxies"
    fi
fi

# 4.4 Secret管理
SECRET_KEY=$(get_config_value "SDKWORK_IM_SECRET_KEY" "$CONFIG_FILE")
if [ -n "$SECRET_KEY" ]; then
    # 检查是否为弱密钥
    if [ "$SECRET_KEY" == "secret" ] || [ "$SECRET_KEY" == "test" ] || [ "$SECRET_KEY" == "development" ]; then
        check_fail "Weak secret key detected" \
            "Use a strong, unique secret key for production"
    else
        KEY_LENGTH=${#SECRET_KEY}
        if [ "$KEY_LENGTH" -ge 32 ]; then
            check_pass "Secret key length adequate: $KEY_LENGTH characters"
        else
            check_warning "Secret key length insufficient: $KEY_LENGTH characters" \
                "Recommended minimum 32 characters"
        fi
    fi
else
    check_warning "Secret key not configured" \
        "Set SDKWORK_IM_SECRET_KEY for encryption and signing"
fi

# ============================================================================
# 5. Environment-specific Checks
# ============================================================================

print_header "5. Environment-specific Checks ($PROFILE)"

if [ "$PROFILE" == "production" ]; then
    # 生产环境特殊检查
    
    # 5.1 SSL证书检查
    echo -e "${BLUE}Checking SSL certificate...${NC}"
    SSL_CERT_PATH=$(get_config_value "SDKWORK_IM_SSL_CERT_PATH" "$CONFIG_FILE")
    SSL_KEY_PATH=$(get_config_value "SDKWORK_IM_SSL_KEY_PATH" "$CONFIG_FILE")
    
    if [ -n "$SSL_CERT_PATH" ] && [ -f "$SSL_CERT_PATH" ]; then
        check_pass "SSL certificate file found"
        
        # 检查证书有效期
        CERT_EXPIRY=$(openssl x509 -in "$SSL_CERT_PATH" -noout -enddate 2>/dev/null | cut -d= -f2)
        if [ -n "$CERT_EXPIRY" ]; then
            EXPIRY_DATE=$(date -d "$CERT_EXPIRY" +%s 2>/dev/null || date -j -f "%b %d %T %Y %Z" "$CERT_EXPIRY" +%s 2>/dev/null)
            CURRENT_DATE=$(date +%s)
            DAYS_LEFT=$(( (EXPIRY_DATE - CURRENT_DATE) / 86400 ))
            
            if [ "$DAYS_LEFT" -gt 30 ]; then
                check_pass "SSL certificate valid for $DAYS_LEFT days"
            elif [ "$DAYS_LEFT" -gt 0 ]; then
                check_warning "SSL certificate expires in $DAYS_LEFT days" \
                    "Plan certificate renewal"
            else
                check_fail "SSL certificate has expired" \
                    "Immediately renew SSL certificate"
            fi
        fi
    else
        check_warning "SSL certificate not configured" \
            "Configure SSL certificates for HTTPS"
    fi
    
    # 5.2 备份配置检查
    BACKUP_ENABLED=$(get_config_value "SDKWORK_IM_BACKUP_ENABLED" "$CONFIG_FILE")
    if [ "$BACKUP_ENABLED" == "true" ]; then
        check_pass "Backup enabled"
        
        BACKUP_SCHEDULE=$(get_config_value "SDKWORK_IM_BACKUP_SCHEDULE" "$CONFIG_FILE")
        if [ -n "$BACKUP_SCHEDULE" ]; then
            check_pass "Backup schedule configured: $BACKUP_SCHEDULE"
        fi
        
        BACKUP_LOCATION=$(get_config_value "SDKWORK_IM_BACKUP_LOCATION" "$CONFIG_FILE")
        if [ -n "$BACKUP_LOCATION" ]; then
            check_pass "Backup location configured: $BACKUP_LOCATION"
            
            # 检查备份目录可访问性
            if [ -d "$BACKUP_LOCATION" ]; then
                check_pass "Backup directory accessible"
            else
                check_warning "Backup directory not found: $BACKUP_LOCATION"
            fi
        fi
    else
        check_warning "Backup not enabled in production" \
            "Recommended to enable regular backups"
    fi
    
elif [ "$PROFILE" == "staging" ]; then
    # Staging环境特殊检查
    echo -e "${BLUE}Staging environment checks...${NC}"
    
    # Staging应该接近生产配置
    if [ "$JWT_SIG" != "true" ]; then
        check_warning "JWT signature disabled in staging" \
            "Recommended to match production configuration"
    fi
    
elif [ "$PROFILE" == "development" ]; then
    # 开发环境检查
    echo -e "${BLUE}Development environment checks...${NC}"
    
    # 开发环境允许宽松配置
    check_pass "Development profile allows relaxed configuration"
    
    if [ "$JWT_SIG" == "true" ]; then
        check_pass "Good practice: JWT signature enabled even in development"
    fi
fi

# ============================================================================
# Summary
# ============================================================================

print_header "Security Configuration Check Summary"

echo -e "${GREEN}Passed:${NC} $PASSED checks"
echo -e "${YELLOW}Warnings:${NC} $WARNINGS checks"
echo -e "${RED}Errors:${NC} $ERRORS checks"
echo ""

TOTAL=$((PASSED + WARNINGS + ERRORS))
SCORE=$((PASSED * 100 / TOTAL))

echo -e "${BLUE}Security Score:${NC} $SCORE%"
echo ""

if [ "$PROFILE" == "production" ]; then
    if [ "$ERRORS" -gt 0 ]; then
        echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${RED}  ❌ CRITICAL: Production deployment blocked due to security errors${NC}"
        echo -e "${RED}  Fix all errors before deploying to production${NC}"
        echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        exit 1
    elif [ "$WARNINGS" -gt 5 ]; then
        echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${YELLOW}  ⚠️  WARNING: Many security warnings detected${NC}"
        echo -e "${YELLOW}  Recommended to address warnings before production deployment${NC}"
        echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        exit 0
    else
        echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${GREEN}  ✅ SUCCESS: Production security configuration passed${NC}"
        echo -e "${GREEN}  All critical checks passed, deployment approved${NC}"
        echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        exit 0
    fi
else
    if [ "$ERRORS" -gt 0 ]; then
        echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${YELLOW}  ⚠️  $PROFILE environment has security errors${NC}"
        echo -e "${YELLOW}  Recommended to fix errors for better security${NC}"
        echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        exit 1
    else
        echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${GREEN}  ✅ $PROFILE environment security check passed${NC}"
        echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        exit 0
    fi
fi