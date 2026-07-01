# SDKWork IM 运维手册

**版本**: v1.0  
**适用范围**: 生产环境运维、故障处理、容量规划  
**更新日期**: 2026-06-30

---

## 目录

1. [部署指南](#1-部署指南)
2. [监控告警](#2-监控告警)
3. [故障处理](#3-故障处理)
4. [容量规划](#4-容量规划)
5. [安全运维](#5-安全运维)
6. [备份恢复](#6-备份恢复)
7. [升级维护](#7-升级维护)
8. [检查清单](#8-检查清单)

---

## 1. 部署指南

### 1.1 环境准备

#### 系统要求

| 项目 | 最低要求 | 推荐配置 |
|------|---------|---------|
| CPU | 4核 | 8核+ |
| 内存 | 8GB | 16GB+ |
| 存储 | 100GB SSD | 500GB SSD |
| 网络 | 100Mbps | 1Gbps |
| 操作系统 | Ubuntu 20.04/CentOS 7+ | Ubuntu 22.04 LTS |

#### 软件依赖

```bash
# 基础软件
- Docker 20.10+
- Docker Compose 2.0+
- Kubernetes 1.24+ (可选)
- PostgreSQL 14+
- Redis 7+

# 运维工具
- Prometheus 2.40+
- Grafana 9.0+
- Node Exporter
- Prometheus Postgres Exporter
- Redis Exporter
```

#### 网络配置

```yaml
# 端口映射
services:
  sdkwork-im-gateway:
    http: 18079
    websocket: 18080
    
  session-gateway:
    grpc: 50051
    
  conversation-service:
    grpc: 50052
    
  postgres:
    port: 5432
    
  redis:
    cluster:
      - 6379-6384 (6 nodes)
```

### 1.2 配置管理

#### 环境配置文件

```bash
# 配置文件位置
configs/topology/
├── standalone.unified-process.development.env
├── standalone.unified-process.production.env
├── cloud.split-services.production.env
└── cloud.split-services.staging.env

# 使用方式
export SDKWORK_IM_RUNTIME_PROFILE=production
source configs/topology/cloud.split-services.production.env
```

#### 必需配置项

```bash
# 安全配置 (CRITICAL - 必须设置)
SDKWORK_IM_RUNTIME_PROFILE=production
SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE=true
SDKWORK_IM_IAM_DATABASE_URL=postgresql://user:pass@host:5432/iam
SDKWORK_IM_FORCE_HTTPS=true

# 数据库配置
SDKWORK_IM_DATABASE_URL=postgresql://user:pass@host:5432/im
SDKWORK_IM_DATABASE_MAX_CONNECTIONS=50
SDKWORK_IM_DATABASE_MIN_CONNECTIONS=10

# Redis配置
SDKWORK_IM_REDIS_CLUSTER_NODES=redis://node1:6379,redis://node2:6380,redis://node3:6381
SDKWORK_IM_REDIS_MAX_CONNECTIONS=20

# 监控配置
SDKWORK_IM_PROMETHEUS_ENABLED=true
SDKWORK_IM_METRICS_PORT=9090
```

### 1.3 部署步骤

#### Docker Compose部署

```bash
# Step 1: 克隆代码
git clone https://github.com/sdkwork/sdkwork-im.git
cd sdkwork-im

# Step 2: 加载配置
export SDKWORK_IM_RUNTIME_PROFILE=production
source configs/topology/cloud.split-services.production.env

# Step 3: 安全配置检查
scripts/check-security-config.sh

# Step 4: 启动依赖服务
docker-compose -f deployments/redis/redis-cluster.yml up -d
docker-compose -f deployments/postgres/postgres-cluster.yml up -d

# Step 5: 启动应用服务
docker-compose -f deployments/docker-compose.yml up -d

# Step 6: 健康检查
curl http://localhost:18079/healthz
curl http://localhost:18079/readyz
```

#### Kubernetes部署

```bash
# Step 1: 创建namespace
kubectl apply -f deployments/kubernetes/cloud-split-services/namespace.yml

# Step 2: 创建secrets
kubectl create secret generic sdkwork-im-secrets \
  --from-literal=database-url=$SDKWORK_IM_DATABASE_URL \
  --from-literal=redis-nodes=$SDKWORK_IM_REDIS_CLUSTER_NODES

# Step 3: 部署服务
kubectl apply -f deployments/kubernetes/cloud-split-services/

# Step 4: 验证部署
kubectl get pods -n sdkwork-im
kubectl logs -f deployment/sdkwork-im-gateway -n sdkwork-im
```

### 1.4 部署验证

#### 功能验证清单

```bash
#!/bin/bash
# 文件: scripts/verify-deployment.sh

echo "=== Deployment Verification ==="

# 1. 健康检查
echo "Checking health endpoints..."
curl -f http://localhost:18079/healthz || exit 1
curl -f http://localhost:18079/readyz || exit 1

# 2. 数据库连接
echo "Checking database connection..."
psql $SDKWORK_IM_DATABASE_URL -c "SELECT 1" || exit 1

# 3. Redis连接
echo "Checking Redis connection..."
redis-cli -c -h localhost -p 6379 PING || exit 1

# 4. WebSocket测试
echo "Testing WebSocket..."
wscat -c ws://localhost:18080 -x '{"type":"auth.init","token":"test"}'

# 5. API测试
echo "Testing API..."
curl -X POST http://localhost:18079/im/v3/api/messages \
  -H "Authorization: Bearer test" \
  -H "Content-Type: application/json" \
  -d '{"content":"test"}'

echo "✅ All deployment checks passed"
```

---

## 2. 监控告警

### 2.1 监控指标体系

#### 核心监控指标

```yaml
# 应用指标
application_metrics:
  - name: http_requests_total
    type: Counter
    description: HTTP请求总数
    
  - name: http_request_duration_seconds
    type: Histogram
    description: HTTP请求延迟
    buckets: [0.01, 0.05, 0.1, 0.2, 0.5, 1.0, 2.0, 5.0]
    
  - name: websocket_connections_active
    type: Gauge
    description: 活跃WebSocket连接数
    
  - name: messages_sent_total
    type: Counter
    description: 发送消息总数
    
  - name: message_delivery_duration_seconds
    type: Histogram
    description: 消息投递延迟
    
  - name: tenant_quota_usage_ratio
    type: Gauge
    description: 租户配额使用率

# 系统指标
system_metrics:
  - name: cpu_usage_percent
    type: Gauge
    description: CPU使用率
    
  - name: memory_usage_bytes
    type: Gauge
    description: 内存使用量
    
  - name: disk_usage_percent
    type: Gauge
    description: 磁盘使用率
    
  - name: network_io_bytes
    type: Counter
    description: 网络IO流量

# 数据库指标
database_metrics:
  - name: postgres_connections_active
    type: Gauge
    description: PostgreSQL活跃连接数
    
  - name: postgres_query_duration_seconds
    type: Histogram
    description: PostgreSQL查询延迟
    
  - name: postgres_replication_lag_seconds
    type: Gauge
    description: PostgreSQL复制延迟

# Redis指标
redis_metrics:
  - name: redis_connections_active
    type: Gauge
    description: Redis活跃连接数
    
  - name: redis_memory_usage_bytes
    type: Gauge
    description: Redis内存使用量
    
  - name: redis_cluster_state
    type: Gauge
    description: Redis集群状态 (1=ok, 0=fail)
```

### 2.2 Prometheus配置

```yaml
# 文件: deployments/prometheus/prometheus.yml

global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'sdkwork-im-gateway'
    static_configs:
      - targets: ['localhost:9090']
    metrics_path: '/metrics'
    
  - job_name: 'session-gateway'
    static_configs:
      - targets: ['session-gateway:9090']
    
  - job_name: 'conversation-service'
    static_configs:
      - targets: ['conversation-service:9090']
    
  - job_name: 'postgres'
    static_configs:
      - targets: ['postgres-exporter:9187']
    
  - job_name: 'redis'
    static_configs:
      - targets: ['redis-exporter:9121']
    
  - job_name: 'node'
    static_configs:
      - targets: ['node-exporter:9100']

alerting:
  alertmanagers:
    - static_configs:
        - targets: ['alertmanager:9093']

rule_files:
  - 'alerts/*.yml'
```

### 2.3 告警规则

```yaml
# 文件: deployments/prometheus/alerts/critical.yml

groups:
  - name: critical_alerts
    rules:
      # 应用告警
      - alert: HighErrorRate
        expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.1
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "High HTTP error rate detected"
          description: "Error rate is {{ $value }} errors/s"
          
      - alert: HighLatency
        expr: histogram_quantile(0.99, rate(http_request_duration_seconds_bucket[5m])) > 0.5
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "High P99 latency detected"
          description: "P99 latency is {{ $value }}s"
          
      - alert: WebSocketConnectionsDrop
        expr: delta(websocket_connections_active[5m]) < -100
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "WebSocket connections dropped significantly"
          description: "{{ $value }} connections dropped in 5 minutes"
          
      # 数据库告警
      - alert: PostgreSQLDown
        expr: pg_up == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "PostgreSQL is down"
          description: "PostgreSQL instance is unreachable"
          
      - alert: PostgreSQLHighConnections
        expr: pg_stat_activity_count / pg_settings_max_connections > 0.8
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "PostgreSQL connection pool nearly exhausted"
          description: "{{ $value }}% connections used"
          
      # Redis告警
      - alert: RedisClusterDown
        expr: redis_cluster_state == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Redis Cluster is down"
          description: "Redis Cluster state is FAIL"
          
      - alert: RedisHighMemory
        expr: redis_memory_used_bytes / redis_memory_max_bytes > 0.8
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "Redis memory usage high"
          description: "{{ $value }}% memory used"
          
      # 系统告警
      - alert: HighCPUUsage
        expr: cpu_usage_percent > 80
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "High CPU usage"
          description: "CPU usage is {{ $value }}%"
          
      - alert: HighMemoryUsage
        expr: memory_usage_bytes / memory_total_bytes > 0.9
        for: 10m
        labels:
          severity: critical
        annotations:
          summary: "High memory usage"
          description: "{{ $value }}% memory used"
```

### 2.4 Grafana仪表盘

#### 系统概览仪表盘

```json
{
  "dashboard": {
    "title": "SDKWork IM System Overview",
    "panels": [
      {
        "title": "HTTP Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(http_requests_total[5m])",
            "legendFormat": "{{method}} {{path}}"
          }
        ]
      },
      {
        "title": "HTTP Latency (P99)",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.99, rate(http_request_duration_seconds_bucket[5m]))",
            "legendFormat": "P99"
          }
        ]
      },
      {
        "title": "WebSocket Connections",
        "type": "gauge",
        "targets": [
          {
            "expr": "websocket_connections_active"
          }
        ]
      },
      {
        "title": "Message Throughput",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(messages_sent_total[5m])",
            "legendFormat": "messages/s"
          }
        ]
      },
      {
        "title": "Database Connections",
        "type": "graph",
        "targets": [
          {
            "expr": "pg_stat_activity_count",
            "legendFormat": "active connections"
          }
        ]
      },
      {
        "title": "Redis Cluster Status",
        "type": "stat",
        "targets": [
          {
            "expr": "redis_cluster_state"
          }
        ],
        "thresholds": [
          { "value": 0, "color": "red" },
          { "value": 1, "color": "green" }
        ]
      }
    ]
  }
}
```

---

## 3. 故障处理

### 3.1 故障诊断流程

```mermaid
graph TD
    A[故障发现] --> B[初步诊断]
    B --> C{故障类型}
    
    C -->|应用故障| D[应用诊断]
    C -->|数据库故障| E[数据库诊断]
    C -->|Redis故障| F[Redis诊断]
    C -->|网络故障| G[网络诊断]
    
    D --> D1[检查日志]
    D --> D2[检查指标]
    D --> D3[检查进程]
    
    E --> E1[检查连接]
    E --> E2[检查查询]
    E --> E3[检查复制]
    
    F --> F1[检查集群]
    F --> F2[检查连接]
    F --> F3[检查内存]
    
    G --> G1[检查连通性]
    G --> G2[检查防火墙]
    G --> G3[检查负载]
    
    D1 & D2 & D3 --> H[定位问题]
    E1 & E2 & E3 --> H
    F1 & F2 & F3 --> H
    G1 & G2 & G3 --> H
    
    H --> I[执行修复]
    I --> J[验证恢复]
    J --> K[记录报告]
```

### 3.2 常见故障处理

#### 应用故障

**故障1: 服务无法启动**

```bash
# 诊断步骤
1. 检查日志
   docker logs sdkwork-im-gateway
   
2. 检查配置
   scripts/check-security-config.sh
   
3. 检查依赖
   curl http://postgres:5432/healthz
   curl http://redis:6379/PING
   
# 常见原因
- 配置错误: 检查.env文件
- 依赖服务不可用: 启动依赖服务
- 端口冲突: 检查端口占用
- 权限问题: 检查文件权限

# 修复方案
- 修正配置文件
- 启动依赖服务
- 调整端口配置
- 修复权限问题
```

**故障2: HTTP请求错误率高**

```bash
# 诊断步骤
1. 检查错误类型
   curl http://localhost:9090/api/v1/query?query=http_requests_total{status=~"5.."}
   
2. 检查日志
   grep "ERROR" /var/log/sdkwork-im/gateway.log
   
3. 检查数据库连接
   psql $SDKWORK_IM_DATABASE_URL -c "SELECT count(*) FROM pg_stat_activity"
   
# 常见原因
- 数据库连接池耗尽: 增加连接数
- 查询超时: 优化SQL或增加超时时间
- Redis故障: 检查Redis状态
- 内存不足: 增加内存或优化内存使用

# 修复方案
- 调整数据库连接池大小
- 优化慢查询
- 修复Redis问题
- 增加系统内存
```

**故障3: WebSocket连接掉线**

```bash
# 诊断步骤
1. 检查WebSocket连接数
   curl http://localhost:9090/api/v1/query?query=websocket_connections_active
   
2. 检查掉线原因
   grep "WebSocket.*disconnect" /var/log/sdkwork-im/gateway.log
   
3. 检查网络状态
   ping client_ip
   netstat -an | grep :18080
   
# 常见原因
- 网络不稳定: 检查网络配置
- 客户端超时: 调整超时配置
- 服务器重启: 检查服务稳定性
- 负载过高: 增加服务器资源

# 修复方案
- 优化网络配置
- 调整WebSocket超时设置
- 增加服务器稳定性
- 扩容服务器资源
```

#### 数据库故障

**故障4: PostgreSQL连接失败**

```bash
# 诊断步骤
1. 检查PostgreSQL状态
   systemctl status postgresql
   docker ps | grep postgres
   
2. 检查连接
   psql -h postgres_host -p 5432 -U postgres -c "SELECT 1"
   
3. 检查日志
   tail -f /var/log/postgresql/postgresql.log
   
# 常见原因
- PostgreSQL服务停止: 重启服务
- 连接数超限: 增加max_connections
- 认证失败: 检查用户名密码
- 网络问题: 检查网络连通性

# 修复方案
- 重启PostgreSQL服务
- 调整max_connections配置
- 修正认证配置
- 修复网络问题
```

**故障5: PostgreSQL慢查询**

```bash
# 诊断步骤
1. 查找慢查询
   psql $SDKWORK_IM_DATABASE_URL -c "
   SELECT query, calls, total_time/calls as avg_time
   FROM pg_stat_statements
   ORDER BY avg_time DESC
   LIMIT 10"
   
2. 检查执行计划
   psql $SDKWORK_IM_DATABASE_URL -c "EXPLAIN ANALYZE <slow_query>"
   
3. 检查索引使用
   psql $SDKWORK_IM_DATABASE_URL -c "
   SELECT indexrelname, idx_scan, idx_tup_read
   FROM pg_stat_user_indexes
   WHERE idx_scan = 0"
   
# 常见原因
- 缺少索引: 创建合适的索引
- 查询复杂: 简化查询或分解查询
- 数据量大: 分区表或清理历史数据
- 锁竞争: 优化事务逻辑

# 修复方案
- 创建必要的索引
- 优化查询语句
- 实现数据分区
- 优化事务逻辑
```

#### Redis故障

**故障6: Redis Cluster故障**

```bash
# 诊断步骤
1. 检查集群状态
   redis-cli -c CLUSTER INFO
   redis-cli -c CLUSTER NODES
   
2. 检查节点健康
   for port in {6379..6384}; do
     redis-cli -h localhost -p $port PING
   done
   
3. 检查槽分配
   redis-cli -c CLUSTER SLOTS
   
# 常见原因
- 节点故障: 重启或替换节点
- 网络分区: 修复网络问题
- 槽未覆盖: 重新分配槽
- 配置不一致: 同步配置

# 修复方案
- 重启故障节点
- 修复网络分区
- 使用CLUSTER ADDSLOTS重新分配
- 同步集群配置
```

**故障7: Redis内存不足**

```bash
# 诊断步骤
1. 检查内存使用
   redis-cli INFO memory
   
2. 查找大key
   redis-cli --bigkeys
   
3. 检查过期策略
   redis-cli CONFIG GET maxmemory-policy
   
# 常见原因
- 数据过多: 清理过期数据
- 大key存在: 分割大key
- 内存限制过低: 增加maxmemory
- 过期策略不当: 调整maxmemory-policy

# 修复方案
- 清理过期数据: redis-cli SCAN + DEL
- 分割大key
- 增加maxmemory配置
- 设置合适的maxmemory-policy (如allkeys-lru)
```

### 3.3 故障升级策略

```yaml
# 故障等级定义
severity_levels:
  P0_critical:
    criteria:
      - 服务完全不可用
      - 数据丢失风险
      - 安全漏洞
    response_time: 5分钟
    escalation: 立即通知负责人和团队
    resolution_time: 30分钟
    
  P1_high:
    criteria:
      - 核心功能受影响
      - 性能严重下降
      - 大量用户受影响
    response_time: 15分钟
    escalation: 30分钟未解决升级
    resolution_time: 2小时
    
  P2_medium:
    criteria:
      - 部分功能受影响
      - 性能轻微下降
      - 少量用户受影响
    response_time: 30分钟
    escalation: 1小时未解决升级
    resolution_time: 4小时
    
  P3_low:
    criteria:
      - 非核心功能问题
      - UI小问题
      - 单个用户问题
    response_time: 2小时
    escalation: 4小时未解决升级
    resolution_time: 24小时

# 升级路径
escalation_path:
  P0: [值班工程师 -> 技术负责人 -> CTO]
  P1: [值班工程师 -> 技术负责人]
  P2: [值班工程师 -> 小组长]
  P3: [值班工程师]
```

---

## 4. 容量规划

### 4.1 性能基准

```yaml
# 性能基准指标
performance_benchmarks:
  single_instance:
    concurrent_users: 1000
    messages_per_minute: 6000
    api_latency_p99: 100ms
    websocket_connections: 1000
    
  cluster_5_nodes:
    concurrent_users: 5000
    messages_per_minute: 30000
    api_latency_p99: 50ms
    websocket_connections: 5000
    
  large_cluster_20_nodes:
    concurrent_users: 20000
    messages_per_minute: 120000
    api_latency_p99: 30ms
    websocket_connections: 20000

# 资源消耗基准
resource_consumption:
  per_1000_users:
    cpu: 2 cores
    memory: 4GB
    storage_growth: 1GB/day
    network: 10Mbps
    
  database:
    connections_per_instance: 20
    storage_per_user: 1MB
    query_latency_target: <50ms
    
  redis:
    memory_per_user: 100KB
    connections_per_instance: 50
```

### 4.2 扩容指标

```yaml
# 扩容触发阈值
scaling_thresholds:
  horizontal:
    cpu_usage: 70%
    memory_usage: 80%
    api_latency_p99: 100ms
    websocket_connections: 800 per instance
    
  vertical:
    cpu_usage: 90%
    memory_usage: 90%
    disk_usage: 85%
    
  database:
    connection_usage: 80%
    query_latency_p99: 200ms
    replication_lag: 10s
    
  redis:
    memory_usage: 80%
    connection_usage: 90%

# 扩容方案
scaling_strategies:
  application:
    method: Kubernetes HPA
    min_replicas: 3
    max_replicas: 20
    scale_up: add 2 instances when threshold reached
    scale_down: remove 1 instance when usage <50% for 30min
    
  database:
    method: Read replicas + connection pool scaling
    read_replicas: 2 per master
    connection_pool: 50 per instance
    
  redis:
    method: Cluster node addition
    min_nodes: 6
    max_nodes: 12
```

### 4.3 容量规划公式

```python
# 容量规划计算

def calculate_required_resources(users: int, messages_per_day: int):
    """计算所需资源"""
    
    # 应用服务器
    instances_needed = users / 1000  # 每1000用户1个实例
    cpu_cores = instances_needed * 2  # 每实例2核
    memory_gb = instances_needed * 4  # 每实例4GB
    
    # 数据库
    db_connections = instances_needed * 20
    db_storage_gb = users * 1 + messages_per_day * 0.1  # 每用户1MB + 每消息0.1MB
    
    # Redis
    redis_memory_gb = users * 0.1  # 每用户100KB
    redis_nodes = max(6, redis_memory_gb / 2)  # 每节点2GB
    
    return {
        'application': {
            'instances': round(instances_needed),
            'cpu_cores': round(cpu_cores),
            'memory_gb': round(memory_gb)
        },
        'database': {
            'connections': round(db_connections),
            'storage_gb': round(db_storage_gb)
        },
        'redis': {
            'memory_gb': round(redis_memory_gb),
            'nodes': round(redis_nodes)
        }
    }

# 示例: 5000用户，每天10万消息
resources = calculate_required_resources(5000, 100000)
print(resources)
# {
#   'application': {'instances': 5, 'cpu_cores': 10, 'memory_gb': 20},
#   'database': {'connections': 100, 'storage_gb': 15},
#   'redis': {'memory_gb': 0.5, 'nodes': 6}
# }
```

---

## 5. 安全运维

### 5.1 安全配置检查

#### 每日检查项

```bash
#!/bin/bash
# 文件: scripts/daily-security-check.sh

echo "=== Daily Security Check ==="

# 1. JWT签名验证
JWT_SIG=$(grep SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE .env | cut -d'=' -f2)
if [ "$JWT_SIG" != "true" ]; then
    echo "❌ CRITICAL: JWT signature verification disabled"
fi

# 2. HTTPS强制
HTTPS=$(grep SDKWORK_IM_FORCE_HTTPS .env | cut -d'=' -f2)
if [ "$HTTPS" != "true" ]; then
    echo "❌ CRITICAL: HTTPS not forced"
fi

# 3. 防火墙规则
iptables -L -n | grep -E "ACCEPT|DROP" > firewall_rules.txt
if ! grep -q "DROP.*18079" firewall_rules.txt; then
    echo "⚠️  WARNING: Gateway port 18079 not properly restricted"
fi

# 4. SSL证书有效期
cert_expiry=$(openssl s_client -connect localhost:443 -servername localhost 2>/dev/null | openssl x509 -noout -enddate | cut -d= -f2)
days_left=$(( ( $(date -d "$cert_expiry" +%s) - $(date +%s) ) / 86400 ))
if [ $days_left -lt 30 ]; then
    echo "⚠️  WARNING: SSL certificate expires in $days_left days"
fi

# 5. 数据库访问控制
db_connections=$(psql $SDKWORK_IM_DATABASE_URL -c "SELECT count(*) FROM pg_stat_activity WHERE usename NOT IN ('postgres', 'replication')" -t)
if [ $db_connections -gt 100 ]; then
    echo "⚠️  WARNING: High database connections: $db_connections"
fi

echo "✅ Security check completed"
```

#### 安全审计日志

```bash
# 检查审计日志完整性
audit_log_count=$(grep -c "AUDIT" /var/log/sdkwork-im/audit.log)
if [ $audit_log_count -lt 100 ]; then
    echo "⚠️  WARNING: Audit log entries too few"
fi

# 检查异常登录
failed_logins=$(grep "LOGIN_FAILED" /var/log/sdkwork-im/audit.log | wc -l)
if [ $failed_logins -gt 100 ]; then
    echo "❌ CRITICAL: High failed login attempts: $failed_logins"
fi
```

### 5.2 漏洞修复流程

```yaml
vulnerability_response:
  discovery:
    - 安全扫描发现漏洞
    - 用户报告漏洞
    - 第三方通报漏洞
    
  assessment:
    - 评估漏洞严重性 (CVSS评分)
    - 确定影响范围
    - 制定修复方案
    
  remediation:
    P0_critical:
      - 立即修复
      - 发布补丁
      - 通知所有用户
      
    P1_high:
      - 7天内修复
      - 发布补丁
      - 通知受影响用户
      
    P2_medium:
      - 30天内修复
      - 定期发布补丁
      
    P3_low:
      - 90天内修复
      - 计划性修复
      
  verification:
    - 验证修复效果
    - 回归测试
    - 安全复查
    
  communication:
    - 发布安全公告
    - 更新文档
    - 用户通知
```

### 5.3 安全事件响应

```yaml
security_incident_response:
  detection:
    - 监控告警触发
    - 用户报告异常
    - 日志分析发现
    
  containment:
    immediate_actions:
      - 隔离受影响系统
      - 暂停可疑账户
      - 阻断攻击源IP
      - 收集证据
      
    team_activation:
      - 启动安全事件团队
      - 通知管理层
      - 启动应急响应流程
      
  eradication:
    - 定位攻击源头
    - 清除恶意代码
    - 修复安全漏洞
    - 加固系统配置
    
  recovery:
    - 恢复系统服务
    - 验证数据完整性
    - 恢复用户访问
    - 监控后续异常
    
  post_incident:
    - 编写事件报告
    - 分析根本原因
    - 制定改进措施
    - 更新应急预案
```

---

## 6. 备份恢复

### 6.1 备份策略

```yaml
backup_strategy:
  application:
    config_files:
      frequency: daily
      retention: 30 days
      location: s3://backup-sdkwork-im/config/
      
    application_logs:
      frequency: hourly
      retention: 7 days
      location: s3://backup-sdkwork-im/logs/
      
  database:
    full_backup:
      frequency: daily at 2am
      method: pg_dump + compression
      retention: 30 days
      location: s3://backup-sdkwork-im/db-full/
      
    incremental_backup:
      frequency: hourly
      method: WAL archiving
      retention: 7 days
      location: s3://backup-sdkwork-im/db-wal/
      
    point_in_time_recovery:
      enabled: true
      max_retention: 7 days
      
  redis:
    rdb_snapshot:
      frequency: hourly
      retention: 24 hours
      location: s3://backup-sdkwork-im/redis/
      
  object_storage:
    media_files:
      frequency: daily
      method: cross-region replication
      retention: indefinite
```

### 6.2 备份执行脚本

```bash
#!/bin/bash
# 文件: scripts/backup.sh

set -e

BACKUP_DATE=$(date +%Y%m%d_%H%M%S)
S3_BUCKET="s3://backup-sdkwork-im"

echo "=== Starting Backup ==="

# 1. 应用配置备份
echo "Backing up application config..."
tar -czf /tmp/config_${BACKUP_DATE}.tar.gz configs/
aws s3 cp /tmp/config_${BACKUP_DATE}.tar.gz ${S3_BUCKET}/config/

# 2. 数据库全量备份
echo "Backing up database..."
pg_dump -Fc -Z9 $SDKWORK_IM_DATABASE_URL > /tmp/db_${BACKUP_DATE}.dump
aws s3 cp /tmp/db_${BACKUP_DATE}.dump ${S3_BUCKET}/db-full/

# 3. Redis备份
echo "Backing up Redis..."
redis-cli BGSAVE
sleep 10
redis-cli --rdb /tmp/redis_${BACKUP_DATE}.rdb
aws s3 cp /tmp/redis_${BACKUP_DATE}.rdb ${S3_BUCKET}/redis/

# 4. 清理过期备份
echo "Cleaning old backups..."
aws s3 ls ${S3_BUCKET}/db-full/ | awk '{print $4}' | head -n -30 | xargs -I {} aws s3 rm ${S3_BUCKET}/db-full/{}

echo "✅ Backup completed successfully"
```

### 6.3 恢复流程

```bash
#!/bin/bash
# 文件: scripts/restore.sh

BACKUP_DATE=$1  # 格式: YYYYMMDD_HHMMSS

if [ -z "$BACKUP_DATE" ]; then
    echo "Usage: scripts/restore.sh YYYYMMDD_HHMMSS"
    exit 1
fi

S3_BUCKET="s3://backup-sdkwork-im"

echo "=== Starting Restore ==="

# 1. 停止服务
echo "Stopping services..."
docker-compose down

# 2. 恢复数据库
echo "Restoring database..."
aws s3 cp ${S3_BUCKET}/db-full/db_${BACKUP_DATE}.dump /tmp/
pg_restore -d $SDKWORK_IM_DATABASE_URL -Fc /tmp/db_${BACKUP_DATE}.dump

# 3. 恢复Redis
echo "Restoring Redis..."
aws s3 cp ${S3_BUCKET}/redis/redis_${BACKUP_DATE}.rdb /tmp/
docker-compose -f deployments/redis/redis-cluster.yml down
cp /tmp/redis_${BACKUP_DATE}.rdb /var/lib/redis/dump.rdb
docker-compose -f deployments/redis/redis-cluster.yml up -d

# 4. 恢复配置
echo "Restoring config..."
aws s3 cp ${S3_BUCKET}/config/config_${BACKUP_DATE}.tar.gz /tmp/
tar -xzf /tmp/config_${BACKUP_DATE}.tar.gz -C /

# 5. 启动服务
echo "Starting services..."
docker-compose up -d

# 6. 验证恢复
echo "Verifying recovery..."
sleep 30
curl -f http://localhost:18079/healthz || exit 1
curl -f http://localhost:18079/readyz || exit 1

echo "✅ Restore completed successfully"
```

### 6.4 恢复验证清单

```yaml
recovery_verification:
  application:
    - [ ] 服务健康检查通过 (/healthz, /readyz)
    - [ ] WebSocket连接测试成功
    - [ ] API功能测试通过
    - [ ] 配置文件正确加载
    
  database:
    - [ ] 数据库连接正常
    - [ ] 数据完整性验证 (checksum)
    - [ ] 查询功能正常
    - [ ] 复制状态正常
    
  redis:
    - [ ] Redis Cluster健康
    - [ ] 数据完整性验证
    - [ ] 会话路由正常
    - [ ] 序列分配正常
    
  user_verification:
    - [ ] 用户登录测试
    - [ ] 消息发送接收测试
    - [ ] 文件上传下载测试
    - [ ] 搜索功能测试
    
  monitoring:
    - [ ] 监控系统恢复
    - [ ] 告警规则生效
    - [ ] 日志收集正常
```

---

## 7. 升级维护

### 7.1 版本升级流程

```yaml
upgrade_process:
  preparation:
    - 通知用户升级计划
    - 备份当前系统
    - 准备回滚方案
    - 测试升级流程
    
  execution:
    step1_pre_upgrade:
      - 停止旧版本服务
      - 验证备份完整性
      - 准备新版本镜像
      
    step2_database_migration:
      - 执行数据库迁移脚本
      - 验证迁移结果
      - 记录迁移日志
      
    step3_config_update:
      - 更新配置文件
      - 验证配置有效性
      - 备份新配置
      
    step4_service_startup:
      - 启动新版本服务
      - 验证服务健康
      - 执行功能测试
      
  verification:
    - API功能验证
    - WebSocket连接测试
    - 性能基准测试
    - 安全配置检查
    
  rollback:
    trigger:
      - 功能验证失败
      - 性能不达标
      - 用户严重投诉
      
    steps:
      - 停止新版本服务
      - 恢复数据库备份
      - 恢复旧版本配置
      - 启动旧版本服务
      - 验证回滚成功
```

### 7.2 数据库迁移指南

```bash
#!/bin/bash
# 文件: scripts/migrate-database.sh

NEW_VERSION=$1

echo "=== Database Migration ==="

# 1. 检查迁移脚本
ls database/migrations/ | grep -E "^${NEW_VERSION}"

# 2. 备份数据库
scripts/backup.sh

# 3. 执行迁移
for migration in database/migrations/${NEW_VERSION}/*.sql; do
    echo "Executing $migration..."
    psql $SDKWORK_IM_DATABASE_URL -f $migration || {
        echo "❌ Migration failed"
        scripts/restore.sh latest
        exit 1
    }
done

# 4. 验证迁移
psql $SDKWORK_IM_DATABASE_URL -c "SELECT * FROM schema_migrations ORDER BY version"

echo "✅ Migration completed successfully"
```

### 7.3 服务重启流程

```bash
#!/bin/bash
# 文件: scripts/restart-services.sh

echo "=== Restarting Services ==="

# 1. 优雅停止
echo "Gracefully stopping services..."
docker-compose stop --timeout 30

# 2. 等待连接断开
echo "Waiting for connections to drain..."
sleep 30

# 3. 启动服务
echo "Starting services..."
docker-compose up -d

# 4. 健康检查
echo "Checking health..."
for i in {1..10}; do
    if curl -f http://localhost:18079/readyz; then
        echo "✅ Services restarted successfully"
        exit 0
    fi
    echo "Waiting for services to be ready... ($i/10)"
    sleep 5
done

echo "❌ Services failed to start"
exit 1
```

---

## 8. 检查清单

### 8.1 部署检查清单

#### 安全配置检查 (CRITICAL)

```markdown
# 生产部署安全配置检查清单

## 必须项 (CRITICAL - 未通过拒绝部署)

- [ ] **JWT签名验证已启用**
  - 配置项: `SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE=true`
  - 验证命令: `grep SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE .env`
  - 预期结果: `true`
  
- [ ] **IAM数据库连接已配置**
  - 配置项: `SDKWORK_IM_IAM_DATABASE_URL`已设置
  - 验证: 数据库连接测试成功
  - 测试命令: `psql $SDKWORK_IM_IAM_DATABASE_URL -c "SELECT 1"`
  
- [ ] **开发环境fallback已禁用**
  - 配置项: `SDKWORK_IM_RUNTIME_PROFILE=production`
  - 验证: 启动日志显示"Production mode enforced"
  - 检查命令: `grep SDKWORK_IM_RUNTIME_PROFILE .env`
  
- [ ] **HTTPS强制启用**
  - 配置项: `SDKWORK_IM_FORCE_HTTPS=true`
  - 验证: HTTP请求自动重定向到HTTPS
  - 测试: `curl -I http://localhost:18079` 应返回301重定向

## 推荐项 (HIGH - 建议启用)

- [ ] Redis Cluster已部署 (至少3主3从)
  - 验证: `redis-cli -c CLUSTER INFO` 显示 cluster_state:ok
  - 节点数: 至少6个节点
  
- [ ] 数据库连接池已优化
  - 配置项: `SDKWORK_IM_DATABASE_MAX_CONNECTIONS >= 50`
  - 验证: 连接池监控显示充足连接
  
- [ ] 速率限制已配置
  - Layer 1: Per-IP rate limiting已启用
  - Layer 2: Per-tenant rate limiting已启用
  - 验证: 监控显示限流指标
  
- [ ] 熔断器已启用
  - 每个上游服务独立熔断器
  - 验证: 熔断器配置正确
  
- [ ] 审计日志已启用
  - 所有安全敏感操作记录审计日志
  - 验证: `/var/log/sdkwork-im/audit.log` 包含审计记录

## 执行步骤

1. 运行安全配置检查脚本
   ```bash
   scripts/check-security-config.sh
   ```

2. 修复所有检查失败项

3. 重新运行检查直到全部通过

4. 记录检查结果作为部署证据
```

#### 功能验证检查

```markdown
# 功能验证检查清单

## 核心功能验证

- [ ] **用户认证**
  - 登录成功并获取token
  - Token验证通过
  - 无效token拒绝访问
  
- [ ] **消息发送接收**
  - 发送消息成功
  - 接收方实时收到消息
  - 消息状态正确显示
  
- [ ] **WebSocket连接**
  - WebSocket连接建立成功
  - auth.init认证通过
  - 实时消息推送正常
  
- [ ] **文件上传下载**
  - 文件上传成功
  - 文件下载成功
  - 文件类型验证正确
  
- [ ] **消息搜索**
  - 全文搜索返回正确结果
  - 搜索性能满足要求
  
## 集成验证

- [ ] **多租户隔离**
  - 不同租户数据隔离
  - 跨租户访问拒绝
  
- [ ] **权限验证**
  - 权限检查正确执行
  - 无权限操作拒绝
  
- [ ] **审计日志**
  - 关键操作记录审计日志
  - 日志格式符合规范

## 性能验证

- [ ] **API延迟**
  - P99延迟 < 100ms
  - 平均延迟 < 50ms
  
- [ ] **并发性能**
  - 支持1000并发用户
  - 6000消息/分钟
  
- [ ] **WebSocket连接**
  - 支持1000并发连接
  - 连接稳定不掉线

## 执行脚本

```bash
scripts/verify-deployment.sh
```
```

### 8.2 运维日常检查清单

```markdown
# 运维日常检查清单

## 每日检查 (自动化执行)

- [ ] **系统健康检查**
  - 所有服务健康状态正常
  - 自动执行: Prometheus健康检查
  
- [ ] **资源使用监控**
  - CPU使用率 < 80%
  - 内存使用率 < 80%
  - 磁盘使用率 < 85%
  - 自动执行: Prometheus指标监控
  
- [ ] **数据库健康**
  - PostgreSQL连接数正常
  - 查询延迟正常
  - 复制延迟 < 10s
  - 自动执行: PostgreSQL exporter监控
  
- [ ] **Redis健康**
  - Redis Cluster状态正常
  - 内存使用正常
  - 连接数正常
  - 自动执行: Redis exporter监控
  
- [ ] **备份验证**
  - 每日备份执行成功
  - 备份文件完整性验证
  - 自动执行: 备份脚本
  
## 每周检查 (人工执行)

- [ ] **日志审查**
  - 检查异常日志
  - 分析告警趋势
  - 识别潜在问题
  
- [ ] **安全审计**
  - 检查安全配置
  - 审查审计日志
  - 验证访问控制
  
- [ ] **性能分析**
  - 分析性能趋势
  - 识别性能瓶颈
  - 制定优化计划
  
- [ ] **容量评估**
  - 评估资源使用趋势
  - 预测容量需求
  - 制定扩容计划
  
## 每月检查 (人工执行)

- [ ] **灾备演练**
  - 执行灾备演练脚本
  - 验证恢复流程
  - 记录演练结果
  
- [ ] **安全扫描**
  - 执行安全扫描工具
  - 修复发现漏洞
  - 记录修复结果
  
- [ ] **版本更新**
  - 检查依赖版本
  - 评估更新风险
  - 制定更新计划
  
- [ ] **文档更新**
  - 更新运维文档
  - 更新检查清单
  - 分享运维经验
```

### 8.3 故障处理检查清单

```markdown
# 故障处理检查清单

## 故障发现

- [ ] **告警触发**
  - 告警类型识别
  - 影响范围评估
  - 严重性判定
  
- [ ] **初步诊断**
  - 检查系统状态
  - 查看错误日志
  - 分析监控指标
  
## 故障定位

- [ ] **日志分析**
  - 查找关键错误日志
  - 分析错误堆栈
  - 定位错误源
  
- [ ] **指标分析**
  - 分析异常指标
  - 对比正常基准
  - 找出异常模式
  
- [ ] **依赖检查**
  - 检查数据库状态
  - 检查Redis状态
  - 检查网络连通性
  
## 故障修复

- [ ] **修复执行**
  - 执行修复方案
  - 记录修复步骤
  - 监控修复效果
  
- [ ] **验证恢复**
  - 功能验证
  - 性能验证
  - 用户验证
  
## 故障总结

- [ ] **报告编写**
  - 故障描述
  - 影响评估
  - 修复过程
  - 经验教训
  
- [ ] **改进措施**
  - 制定预防措施
  - 优化监控告警
  - 更新应急预案
```

---

**文档维护**: 运维团队  
**更新频率**: 每季度review  
**下次更新**: 2026-09-30