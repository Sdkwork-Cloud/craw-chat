# SDKWork IM 杩愮淮鎵嬪唽

**鐗堟湰**: v1.0  
**閫傜敤鑼冨洿**: 鐢熶骇鐜杩愮淮銆佹晠闅滃鐞嗐€佸閲忚鍒? 
**鏇存柊鏃ユ湡**: 2026-06-30

---

## 鐩綍

1. [閮ㄧ讲鎸囧崡](#1-閮ㄧ讲鎸囧崡)
2. [鐩戞帶鍛婅](#2-鐩戞帶鍛婅)
3. [鏁呴殰澶勭悊](#3-鏁呴殰澶勭悊)
4. [瀹归噺瑙勫垝](#4-瀹归噺瑙勫垝)
5. [瀹夊叏杩愮淮](#5-瀹夊叏杩愮淮)
6. [澶囦唤鎭㈠](#6-澶囦唤鎭㈠)
7. [鍗囩骇缁存姢](#7-鍗囩骇缁存姢)
8. [妫€鏌ユ竻鍗昡(#8-妫€鏌ユ竻鍗?

---

## 1. 閮ㄧ讲鎸囧崡

### 1.1 鐜鍑嗗

#### 绯荤粺瑕佹眰

| 椤圭洰 | 鏈€浣庤姹?| 鎺ㄨ崘閰嶇疆 |
|------|---------|---------|
| CPU | 4鏍?| 8鏍? |
| 鍐呭瓨 | 8GB | 16GB+ |
| 瀛樺偍 | 100GB SSD | 500GB SSD |
| 缃戠粶 | 100Mbps | 1Gbps |
| 鎿嶄綔绯荤粺 | Ubuntu 20.04/CentOS 7+ | Ubuntu 22.04 LTS |

#### 杞欢渚濊禆

```bash
# 鍩虹杞欢
- Docker 20.10+
- Docker Compose 2.0+
- Kubernetes 1.24+ (鍙€?
- PostgreSQL 14+
- Redis 7+

# 杩愮淮宸ュ叿
- Prometheus 2.40+
- Grafana 9.0+
- Node Exporter
- Prometheus Postgres Exporter
- Redis Exporter
```

#### 缃戠粶閰嶇疆

```yaml
# 绔彛鏄犲皠
services:
  sdkwork-im-gateway:
    http: 18079
    websocket: 18079
    
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

### 1.2 閰嶇疆绠＄悊

#### 鐜閰嶇疆鏂囦欢

```bash
# 閰嶇疆鏂囦欢浣嶇疆
configs/topology/
鈹溾攢鈹€ standalone.development.env
鈹溾攢鈹€ standalone.unified-process.production.env
鈹溾攢鈹€ cloud.production.env
鈹斺攢鈹€ cloud.staging.env

# 浣跨敤鏂瑰紡
export SDKWORK_IM_RUNTIME_PROFILE=production
source configs/topology/cloud.production.env
```

#### 蹇呴渶閰嶇疆椤?

```bash
# 瀹夊叏閰嶇疆 (CRITICAL - 蹇呴』璁剧疆)
SDKWORK_IM_RUNTIME_PROFILE=production
SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE=true
SDKWORK_IM_IAM_DATABASE_URL=postgresql://user:pass@host:5432/iam
SDKWORK_IM_FORCE_HTTPS=true

# 鏁版嵁搴撻厤缃?
SDKWORK_IM_DATABASE_URL=postgresql://user:pass@host:5432/im
SDKWORK_IM_DATABASE_MAX_CONNECTIONS=50
SDKWORK_IM_DATABASE_MIN_CONNECTIONS=10

# Redis閰嶇疆
SDKWORK_IM_REDIS_CLUSTER_NODES=redis://node1:6379,redis://node2:6380,redis://node3:6381
SDKWORK_IM_REDIS_MAX_CONNECTIONS=20

# 鐩戞帶閰嶇疆
SDKWORK_IM_PROMETHEUS_ENABLED=true
SDKWORK_IM_METRICS_PORT=9090
```

### 1.3 閮ㄧ讲姝ラ

#### Docker Compose閮ㄧ讲

```bash
# Step 1: 鍏嬮殕浠ｇ爜
git clone https://github.com/sdkwork/sdkwork-im.git
cd sdkwork-im

# Step 2: 鍔犺浇閰嶇疆
export SDKWORK_IM_RUNTIME_PROFILE=production
source configs/topology/cloud.production.env

# Step 3: 瀹夊叏閰嶇疆妫€鏌?
scripts/check-security-config.sh

# Step 4: 鍚姩渚濊禆鏈嶅姟
docker-compose -f deployments/redis/redis-cluster.yml up -d
docker-compose -f deployments/postgres/postgres-cluster.yml up -d

# Step 5: 鍚姩搴旂敤鏈嶅姟
docker-compose -f deployments/docker-compose.yml up -d

# Step 6: 鍋ュ悍妫€鏌?
curl http://localhost:18079/healthz
curl http://localhost:18079/readyz
```

#### Kubernetes閮ㄧ讲

```bash
# Step 1: 鍒涘缓namespace
kubectl apply -f deployments/kubernetes/cloud/namespace.yml

# Step 2: 鍒涘缓secrets
kubectl create secret generic sdkwork-im-secrets \
  --from-literal=database-url=$SDKWORK_IM_DATABASE_URL \
  --from-literal=redis-nodes=$SDKWORK_IM_REDIS_CLUSTER_NODES

# Step 3: 閮ㄧ讲鏈嶅姟
kubectl apply -f deployments/kubernetes/cloud/

# Step 4: 楠岃瘉閮ㄧ讲
kubectl get pods -n sdkwork-im
kubectl logs -f deployment/sdkwork-im-gateway -n sdkwork-im
```

### 1.4 閮ㄧ讲楠岃瘉

#### 鍔熻兘楠岃瘉娓呭崟

```bash
#!/bin/bash
# 鏂囦欢: scripts/verify-deployment.sh

echo "=== Deployment Verification ==="

# 1. 鍋ュ悍妫€鏌?
echo "Checking health endpoints..."
curl -f http://localhost:18079/healthz || exit 1
curl -f http://localhost:18079/readyz || exit 1

# 2. 鏁版嵁搴撹繛鎺?
echo "Checking database connection..."
psql $SDKWORK_IM_DATABASE_URL -c "SELECT 1" || exit 1

# 3. Redis杩炴帴
echo "Checking Redis connection..."
redis-cli -c -h localhost -p 6379 PING || exit 1

# 4. WebSocket娴嬭瘯
echo "Testing WebSocket..."
wscat -c ws://localhost:18079 -x '{"type":"auth.init","token":"test"}'

# 5. API娴嬭瘯
echo "Testing API..."
curl -X POST http://localhost:18079/im/v3/api/messages \
  -H "Authorization: Bearer test" \
  -H "Content-Type: application/json" \
  -d '{"content":"test"}'

echo "鉁?All deployment checks passed"
```

---

## 2. 鐩戞帶鍛婅

### 2.1 鐩戞帶鎸囨爣浣撶郴

#### 鏍稿績鐩戞帶鎸囨爣

```yaml
# 搴旂敤鎸囨爣
application_metrics:
  - name: http_requests_total
    type: Counter
    description: HTTP璇锋眰鎬绘暟
    
  - name: http_request_duration_seconds
    type: Histogram
    description: HTTP璇锋眰寤惰繜
    buckets: [0.01, 0.05, 0.1, 0.2, 0.5, 1.0, 2.0, 5.0]
    
  - name: websocket_connections_active
    type: Gauge
    description: 娲昏穬WebSocket杩炴帴鏁?
    
  - name: messages_sent_total
    type: Counter
    description: 鍙戦€佹秷鎭€绘暟
    
  - name: message_delivery_duration_seconds
    type: Histogram
    description: 娑堟伅鎶曢€掑欢杩?
    
  - name: tenant_quota_usage_ratio
    type: Gauge
    description: 绉熸埛閰嶉浣跨敤鐜?

# 绯荤粺鎸囨爣
system_metrics:
  - name: cpu_usage_percent
    type: Gauge
    description: CPU浣跨敤鐜?
    
  - name: memory_usage_bytes
    type: Gauge
    description: 鍐呭瓨浣跨敤閲?
    
  - name: disk_usage_percent
    type: Gauge
    description: 纾佺洏浣跨敤鐜?
    
  - name: network_io_bytes
    type: Counter
    description: 缃戠粶IO娴侀噺

# 鏁版嵁搴撴寚鏍?
database_metrics:
  - name: postgres_connections_active
    type: Gauge
    description: PostgreSQL娲昏穬杩炴帴鏁?
    
  - name: postgres_query_duration_seconds
    type: Histogram
    description: PostgreSQL鏌ヨ寤惰繜
    
  - name: postgres_replication_lag_seconds
    type: Gauge
    description: PostgreSQL澶嶅埗寤惰繜

# Redis鎸囨爣
redis_metrics:
  - name: redis_connections_active
    type: Gauge
    description: Redis娲昏穬杩炴帴鏁?
    
  - name: redis_memory_usage_bytes
    type: Gauge
    description: Redis鍐呭瓨浣跨敤閲?
    
  - name: redis_cluster_state
    type: Gauge
    description: Redis闆嗙兢鐘舵€?(1=ok, 0=fail)
```

### 2.2 Prometheus閰嶇疆

```yaml
# 鏂囦欢: deployments/prometheus/prometheus.yml

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

### 2.3 鍛婅瑙勫垯

```yaml
# 鏂囦欢: deployments/prometheus/alerts/critical.yml

groups:
  - name: critical_alerts
    rules:
      # 搴旂敤鍛婅
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
          
      # 鏁版嵁搴撳憡璀?
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
          
      # Redis鍛婅
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
          
      # 绯荤粺鍛婅
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

### 2.4 Grafana浠〃鐩?

#### 绯荤粺姒傝浠〃鐩?

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

## 3. 鏁呴殰澶勭悊

### 3.1 鏁呴殰璇婃柇娴佺▼

```mermaid
graph TD
    A[鏁呴殰鍙戠幇] --> B[鍒濇璇婃柇]
    B --> C{鏁呴殰绫诲瀷}
    
    C -->|搴旂敤鏁呴殰| D[搴旂敤璇婃柇]
    C -->|鏁版嵁搴撴晠闅渱 E[鏁版嵁搴撹瘖鏂璢
    C -->|Redis鏁呴殰| F[Redis璇婃柇]
    C -->|缃戠粶鏁呴殰| G[缃戠粶璇婃柇]
    
    D --> D1[妫€鏌ユ棩蹇梋
    D --> D2[妫€鏌ユ寚鏍嘳
    D --> D3[妫€鏌ヨ繘绋媇
    
    E --> E1[妫€鏌ヨ繛鎺
    E --> E2[妫€鏌ユ煡璇
    E --> E3[妫€鏌ュ鍒禲
    
    F --> F1[妫€鏌ラ泦缇
    F --> F2[妫€鏌ヨ繛鎺
    F --> F3[妫€鏌ュ唴瀛榏
    
    G --> G1[妫€鏌ヨ繛閫氭€
    G --> G2[妫€鏌ラ槻鐏]
    G --> G3[妫€鏌ヨ礋杞絔
    
    D1 & D2 & D3 --> H[瀹氫綅闂]
    E1 & E2 & E3 --> H
    F1 & F2 & F3 --> H
    G1 & G2 & G3 --> H
    
    H --> I[鎵ц淇]
    I --> J[楠岃瘉鎭㈠]
    J --> K[璁板綍鎶ュ憡]
```

### 3.2 甯歌鏁呴殰澶勭悊

#### 搴旂敤鏁呴殰

**鏁呴殰1: 鏈嶅姟鏃犳硶鍚姩**

```bash
# 璇婃柇姝ラ
1. 妫€鏌ユ棩蹇?
   docker logs sdkwork-im-gateway
   
2. 妫€鏌ラ厤缃?
   scripts/check-security-config.sh
   
3. 妫€鏌ヤ緷璧?
   curl http://postgres:5432/healthz
   curl http://redis:6379/PING
   
# 甯歌鍘熷洜
- 閰嶇疆閿欒: 妫€鏌?env鏂囦欢
- 渚濊禆鏈嶅姟涓嶅彲鐢? 鍚姩渚濊禆鏈嶅姟
- 绔彛鍐茬獊: 妫€鏌ョ鍙ｅ崰鐢?
- 鏉冮檺闂: 妫€鏌ユ枃浠舵潈闄?

# 淇鏂规
- 淇閰嶇疆鏂囦欢
- 鍚姩渚濊禆鏈嶅姟
- 璋冩暣绔彛閰嶇疆
- 淇鏉冮檺闂
```

**鏁呴殰2: HTTP璇锋眰閿欒鐜囬珮**

```bash
# 璇婃柇姝ラ
1. 妫€鏌ラ敊璇被鍨?
   curl http://localhost:9090/api/v1/query?query=http_requests_total{status=~"5.."}
   
2. 妫€鏌ユ棩蹇?
   grep "ERROR" /var/log/sdkwork-im/gateway.log
   
3. 妫€鏌ユ暟鎹簱杩炴帴
   psql $SDKWORK_IM_DATABASE_URL -c "SELECT count(*) FROM pg_stat_activity"
   
# 甯歌鍘熷洜
- 鏁版嵁搴撹繛鎺ユ睜鑰楀敖: 澧炲姞杩炴帴鏁?
- 鏌ヨ瓒呮椂: 浼樺寲SQL鎴栧鍔犺秴鏃舵椂闂?
- Redis鏁呴殰: 妫€鏌edis鐘舵€?
- 鍐呭瓨涓嶈冻: 澧炲姞鍐呭瓨鎴栦紭鍖栧唴瀛樹娇鐢?

# 淇鏂规
- 璋冩暣鏁版嵁搴撹繛鎺ユ睜澶у皬
- 浼樺寲鎱㈡煡璇?
- 淇Redis闂
- 澧炲姞绯荤粺鍐呭瓨
```

**鏁呴殰3: WebSocket杩炴帴鎺夌嚎**

```bash
# 璇婃柇姝ラ
1. 妫€鏌ebSocket杩炴帴鏁?
   curl http://localhost:9090/api/v1/query?query=websocket_connections_active
   
2. 妫€鏌ユ帀绾垮師鍥?
   grep "WebSocket.*disconnect" /var/log/sdkwork-im/gateway.log
   
3. 妫€鏌ョ綉缁滅姸鎬?
   ping client_ip
   netstat -an | grep :18079
   
# 甯歌鍘熷洜
- 缃戠粶涓嶇ǔ瀹? 妫€鏌ョ綉缁滈厤缃?
- 瀹㈡埛绔秴鏃? 璋冩暣瓒呮椂閰嶇疆
- 鏈嶅姟鍣ㄩ噸鍚? 妫€鏌ユ湇鍔＄ǔ瀹氭€?
- 璐熻浇杩囬珮: 澧炲姞鏈嶅姟鍣ㄨ祫婧?

# 淇鏂规
- 浼樺寲缃戠粶閰嶇疆
- 璋冩暣WebSocket瓒呮椂璁剧疆
- 澧炲姞鏈嶅姟鍣ㄧǔ瀹氭€?
- 鎵╁鏈嶅姟鍣ㄨ祫婧?
```

#### 鏁版嵁搴撴晠闅?

**鏁呴殰4: PostgreSQL杩炴帴澶辫触**

```bash
# 璇婃柇姝ラ
1. 妫€鏌ostgreSQL鐘舵€?
   systemctl status postgresql
   docker ps | grep postgres
   
2. 妫€鏌ヨ繛鎺?
   psql -h postgres_host -p 5432 -U postgres -c "SELECT 1"
   
3. 妫€鏌ユ棩蹇?
   tail -f /var/log/postgresql/postgresql.log
   
# 甯歌鍘熷洜
- PostgreSQL鏈嶅姟鍋滄: 閲嶅惎鏈嶅姟
- 杩炴帴鏁拌秴闄? 澧炲姞max_connections
- 璁よ瘉澶辫触: 妫€鏌ョ敤鎴峰悕瀵嗙爜
- 缃戠粶闂: 妫€鏌ョ綉缁滆繛閫氭€?

# 淇鏂规
- 閲嶅惎PostgreSQL鏈嶅姟
- 璋冩暣max_connections閰嶇疆
- 淇璁よ瘉閰嶇疆
- 淇缃戠粶闂
```

**鏁呴殰5: PostgreSQL鎱㈡煡璇?*

```bash
# 璇婃柇姝ラ
1. 鏌ユ壘鎱㈡煡璇?
   psql $SDKWORK_IM_DATABASE_URL -c "
   SELECT query, calls, total_time/calls as avg_time
   FROM pg_stat_statements
   ORDER BY avg_time DESC
   LIMIT 10"
   
2. 妫€鏌ユ墽琛岃鍒?
   psql $SDKWORK_IM_DATABASE_URL -c "EXPLAIN ANALYZE <slow_query>"
   
3. 妫€鏌ョ储寮曚娇鐢?
   psql $SDKWORK_IM_DATABASE_URL -c "
   SELECT indexrelname, idx_scan, idx_tup_read
   FROM pg_stat_user_indexes
   WHERE idx_scan = 0"
   
# 甯歌鍘熷洜
- 缂哄皯绱㈠紩: 鍒涘缓鍚堥€傜殑绱㈠紩
- 鏌ヨ澶嶆潅: 绠€鍖栨煡璇㈡垨鍒嗚В鏌ヨ
- 鏁版嵁閲忓ぇ: 鍒嗗尯琛ㄦ垨娓呯悊鍘嗗彶鏁版嵁
- 閿佺珵浜? 浼樺寲浜嬪姟閫昏緫

# 淇鏂规
- 鍒涘缓蹇呰鐨勭储寮?
- 浼樺寲鏌ヨ璇彞
- 瀹炵幇鏁版嵁鍒嗗尯
- 浼樺寲浜嬪姟閫昏緫
```

#### Redis鏁呴殰

**鏁呴殰6: Redis Cluster鏁呴殰**

```bash
# 璇婃柇姝ラ
1. 妫€鏌ラ泦缇ょ姸鎬?
   redis-cli -c CLUSTER INFO
   redis-cli -c CLUSTER NODES
   
2. 妫€鏌ヨ妭鐐瑰仴搴?
   for port in {6379..6384}; do
     redis-cli -h localhost -p $port PING
   done
   
3. 妫€鏌ユЫ鍒嗛厤
   redis-cli -c CLUSTER SLOTS
   
# 甯歌鍘熷洜
- 鑺傜偣鏁呴殰: 閲嶅惎鎴栨浛鎹㈣妭鐐?
- 缃戠粶鍒嗗尯: 淇缃戠粶闂
- 妲芥湭瑕嗙洊: 閲嶆柊鍒嗛厤妲?
- 閰嶇疆涓嶄竴鑷? 鍚屾閰嶇疆

# 淇鏂规
- 閲嶅惎鏁呴殰鑺傜偣
- 淇缃戠粶鍒嗗尯
- 浣跨敤CLUSTER ADDSLOTS閲嶆柊鍒嗛厤
- 鍚屾闆嗙兢閰嶇疆
```

**鏁呴殰7: Redis鍐呭瓨涓嶈冻**

```bash
# 璇婃柇姝ラ
1. 妫€鏌ュ唴瀛樹娇鐢?
   redis-cli INFO memory
   
2. 鏌ユ壘澶ey
   redis-cli --bigkeys
   
3. 妫€鏌ヨ繃鏈熺瓥鐣?
   redis-cli CONFIG GET maxmemory-policy
   
# 甯歌鍘熷洜
- 鏁版嵁杩囧: 娓呯悊杩囨湡鏁版嵁
- 澶ey瀛樺湪: 鍒嗗壊澶ey
- 鍐呭瓨闄愬埗杩囦綆: 澧炲姞maxmemory
- 杩囨湡绛栫暐涓嶅綋: 璋冩暣maxmemory-policy

# 淇鏂规
- 娓呯悊杩囨湡鏁版嵁: redis-cli SCAN + DEL
- 鍒嗗壊澶ey
- 澧炲姞maxmemory閰嶇疆
- 璁剧疆鍚堥€傜殑maxmemory-policy (濡俛llkeys-lru)
```

### 3.3 鏁呴殰鍗囩骇绛栫暐

```yaml
# 鏁呴殰绛夌骇瀹氫箟
severity_levels:
  P0_critical:
    criteria:
      - 鏈嶅姟瀹屽叏涓嶅彲鐢?
      - 鏁版嵁涓㈠け椋庨櫓
      - 瀹夊叏婕忔礊
    response_time: 5鍒嗛挓
    escalation: 绔嬪嵆閫氱煡璐熻矗浜哄拰鍥㈤槦
    resolution_time: 30鍒嗛挓
    
  P1_high:
    criteria:
      - 鏍稿績鍔熻兘鍙楀奖鍝?
      - 鎬ц兘涓ラ噸涓嬮檷
      - 澶ч噺鐢ㄦ埛鍙楀奖鍝?
    response_time: 15鍒嗛挓
    escalation: 30鍒嗛挓鏈В鍐冲崌绾?
    resolution_time: 2灏忔椂
    
  P2_medium:
    criteria:
      - 閮ㄥ垎鍔熻兘鍙楀奖鍝?
      - 鎬ц兘杞诲井涓嬮檷
      - 灏戦噺鐢ㄦ埛鍙楀奖鍝?
    response_time: 30鍒嗛挓
    escalation: 1灏忔椂鏈В鍐冲崌绾?
    resolution_time: 4灏忔椂
    
  P3_low:
    criteria:
      - 闈炴牳蹇冨姛鑳介棶棰?
      - UI灏忛棶棰?
      - 鍗曚釜鐢ㄦ埛闂
    response_time: 2灏忔椂
    escalation: 4灏忔椂鏈В鍐冲崌绾?
    resolution_time: 24灏忔椂

# 鍗囩骇璺緞
escalation_path:
  P0: [鍊肩彮宸ョ▼甯?-> 鎶€鏈礋璐ｄ汉 -> CTO]
  P1: [鍊肩彮宸ョ▼甯?-> 鎶€鏈礋璐ｄ汉]
  P2: [鍊肩彮宸ョ▼甯?-> 灏忕粍闀縘
  P3: [鍊肩彮宸ョ▼甯圿
```

---

## 4. 瀹归噺瑙勫垝

### 4.1 鎬ц兘鍩哄噯

```yaml
# 鎬ц兘鍩哄噯鎸囨爣
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

# 璧勬簮娑堣€楀熀鍑?
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

### 4.2 鎵╁鎸囨爣

```yaml
# 鎵╁瑙﹀彂闃堝€?
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

# 鎵╁鏂规
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

### 4.3 瀹归噺瑙勫垝鍏紡

```python
# 瀹归噺瑙勫垝璁＄畻

def calculate_required_resources(users: int, messages_per_day: int):
    """璁＄畻鎵€闇€璧勬簮"""
    
    # 搴旂敤鏈嶅姟鍣?
    instances_needed = users / 1000  # 姣?000鐢ㄦ埛1涓疄渚?
    cpu_cores = instances_needed * 2  # 姣忓疄渚?鏍?
    memory_gb = instances_needed * 4  # 姣忓疄渚?GB
    
    # 鏁版嵁搴?
    db_connections = instances_needed * 20
    db_storage_gb = users * 1 + messages_per_day * 0.1  # 姣忕敤鎴?MB + 姣忔秷鎭?.1MB
    
    # Redis
    redis_memory_gb = users * 0.1  # 姣忕敤鎴?00KB
    redis_nodes = max(6, redis_memory_gb / 2)  # 姣忚妭鐐?GB
    
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

# 绀轰緥: 5000鐢ㄦ埛锛屾瘡澶?0涓囨秷鎭?
resources = calculate_required_resources(5000, 100000)
print(resources)
# {
#   'application': {'instances': 5, 'cpu_cores': 10, 'memory_gb': 20},
#   'database': {'connections': 100, 'storage_gb': 15},
#   'redis': {'memory_gb': 0.5, 'nodes': 6}
# }
```

---

## 5. 瀹夊叏杩愮淮

### 5.1 瀹夊叏閰嶇疆妫€鏌?

#### 姣忔棩妫€鏌ラ」

```bash
#!/bin/bash
# 鏂囦欢: scripts/daily-security-check.sh

echo "=== Daily Security Check ==="

# 1. JWT绛惧悕楠岃瘉
JWT_SIG=$(grep SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE .env | cut -d'=' -f2)
if [ "$JWT_SIG" != "true" ]; then
    echo "鉂?CRITICAL: JWT signature verification disabled"
fi

# 2. HTTPS寮哄埗
HTTPS=$(grep SDKWORK_IM_FORCE_HTTPS .env | cut -d'=' -f2)
if [ "$HTTPS" != "true" ]; then
    echo "鉂?CRITICAL: HTTPS not forced"
fi

# 3. 闃茬伀澧欒鍒?
iptables -L -n | grep -E "ACCEPT|DROP" > firewall_rules.txt
if ! grep -q "DROP.*18079" firewall_rules.txt; then
    echo "鈿狅笍  WARNING: Gateway port 18079 not properly restricted"
fi

# 4. SSL璇佷功鏈夋晥鏈?
cert_expiry=$(openssl s_client -connect localhost:443 -servername localhost 2>/dev/null | openssl x509 -noout -enddate | cut -d= -f2)
days_left=$(( ( $(date -d "$cert_expiry" +%s) - $(date +%s) ) / 86400 ))
if [ $days_left -lt 30 ]; then
    echo "鈿狅笍  WARNING: SSL certificate expires in $days_left days"
fi

# 5. 鏁版嵁搴撹闂帶鍒?
db_connections=$(psql $SDKWORK_IM_DATABASE_URL -c "SELECT count(*) FROM pg_stat_activity WHERE usename NOT IN ('postgres', 'replication')" -t)
if [ $db_connections -gt 100 ]; then
    echo "鈿狅笍  WARNING: High database connections: $db_connections"
fi

echo "鉁?Security check completed"
```

#### 瀹夊叏瀹¤鏃ュ織

```bash
# 妫€鏌ュ璁℃棩蹇楀畬鏁存€?
audit_log_count=$(grep -c "AUDIT" /var/log/sdkwork-im/audit.log)
if [ $audit_log_count -lt 100 ]; then
    echo "鈿狅笍  WARNING: Audit log entries too few"
fi

# 妫€鏌ュ紓甯哥櫥褰?
failed_logins=$(grep "LOGIN_FAILED" /var/log/sdkwork-im/audit.log | wc -l)
if [ $failed_logins -gt 100 ]; then
    echo "鉂?CRITICAL: High failed login attempts: $failed_logins"
fi
```

### 5.2 婕忔礊淇娴佺▼

```yaml
vulnerability_response:
  discovery:
    - 瀹夊叏鎵弿鍙戠幇婕忔礊
    - 鐢ㄦ埛鎶ュ憡婕忔礊
    - 绗笁鏂归€氭姤婕忔礊
    
  assessment:
    - 璇勪及婕忔礊涓ラ噸鎬?(CVSS璇勫垎)
    - 纭畾褰卞搷鑼冨洿
    - 鍒跺畾淇鏂规
    
  remediation:
    P0_critical:
      - 绔嬪嵆淇
      - 鍙戝竷琛ヤ竵
      - 閫氱煡鎵€鏈夌敤鎴?
      
    P1_high:
      - 7澶╁唴淇
      - 鍙戝竷琛ヤ竵
      - 閫氱煡鍙楀奖鍝嶇敤鎴?
      
    P2_medium:
      - 30澶╁唴淇
      - 瀹氭湡鍙戝竷琛ヤ竵
      
    P3_low:
      - 90澶╁唴淇
      - 璁″垝鎬т慨澶?
      
  verification:
    - 楠岃瘉淇鏁堟灉
    - 鍥炲綊娴嬭瘯
    - 瀹夊叏澶嶆煡
    
  communication:
    - 鍙戝竷瀹夊叏鍏憡
    - 鏇存柊鏂囨。
    - 鐢ㄦ埛閫氱煡
```

### 5.3 瀹夊叏浜嬩欢鍝嶅簲

```yaml
security_incident_response:
  detection:
    - 鐩戞帶鍛婅瑙﹀彂
    - 鐢ㄦ埛鎶ュ憡寮傚父
    - 鏃ュ織鍒嗘瀽鍙戠幇
    
  containment:
    immediate_actions:
      - 闅旂鍙楀奖鍝嶇郴缁?
      - 鏆傚仠鍙枒璐︽埛
      - 闃绘柇鏀诲嚮婧怚P
      - 鏀堕泦璇佹嵁
      
    team_activation:
      - 鍚姩瀹夊叏浜嬩欢鍥㈤槦
      - 閫氱煡绠＄悊灞?
      - 鍚姩搴旀€ュ搷搴旀祦绋?
      
  eradication:
    - 瀹氫綅鏀诲嚮婧愬ご
    - 娓呴櫎鎭舵剰浠ｇ爜
    - 淇瀹夊叏婕忔礊
    - 鍔犲浐绯荤粺閰嶇疆
    
  recovery:
    - 鎭㈠绯荤粺鏈嶅姟
    - 楠岃瘉鏁版嵁瀹屾暣鎬?
    - 鎭㈠鐢ㄦ埛璁块棶
    - 鐩戞帶鍚庣画寮傚父
    
  post_incident:
    - 缂栧啓浜嬩欢鎶ュ憡
    - 鍒嗘瀽鏍规湰鍘熷洜
    - 鍒跺畾鏀硅繘鎺柦
    - 鏇存柊搴旀€ラ妗?
```

---

## 6. 澶囦唤鎭㈠

### 6.1 澶囦唤绛栫暐

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

### 6.2 澶囦唤鎵ц鑴氭湰

```bash
#!/bin/bash
# 鏂囦欢: scripts/backup.sh

set -euo pipefail

S3_BUCKET="s3://backup-sdkwork-im"

# The only supported implementation is scripts/backup.sh. Do not copy this
# runbook block into a separate job or use a hand-written object-removal pipeline.
# Safe cleanup requires strict backup names, LastModified validation, a
# protected database recovery point, a deletion cap, and fail-closed errors.
# The recovery point is the newest valid database archive plus strict companion
# objects with the same timestamp; the newest config and Redis objects also remain.
# Scheduled backup command:
# ./scripts/backup.sh --target "${S3_BUCKET}" --retention-days 30 --delete-limit 100

# 1. 搴旂敤閰嶇疆澶囦唤
# Configuration archive and upload are owned by scripts/backup.sh.

# 2. 鏁版嵁搴撳叏閲忓浠?
# PostgreSQL archive and upload are owned by scripts/backup.sh.

# 3. Redis澶囦唤
# Redis snapshot and upload are owned by scripts/backup.sh.

# 4. 娓呯悊杩囨湡澶囦唤
# Preview cleanup before changing a retention policy. This does not suppress
# backup uploads when supplied to scripts/backup.sh; it only disables deletion.
./scripts/backup.sh --target "${S3_BUCKET}" --retention-days 30 --delete-limit 100 --dry-run

echo "鉁?Backup completed successfully"
```

### 6.3 鎭㈠娴佺▼

```bash
#!/bin/bash
# 鏂囦欢: scripts/restore.sh

BACKUP_DATE=$1  # 鏍煎紡: YYYYMMDD_HHMMSS

if [ -z "$BACKUP_DATE" ]; then
    echo "Usage: scripts/restore.sh YYYYMMDD_HHMMSS"
    exit 1
fi

S3_BUCKET="s3://backup-sdkwork-im"

echo "=== Starting Restore ==="

# 1. 鍋滄鏈嶅姟
echo "Stopping services..."
docker-compose down

# 2. 鎭㈠鏁版嵁搴?
echo "Restoring database..."
aws s3 cp ${S3_BUCKET}/db-full/db_${BACKUP_DATE}.dump /tmp/
pg_restore -d $SDKWORK_IM_DATABASE_URL -Fc /tmp/db_${BACKUP_DATE}.dump

# 3. 鎭㈠Redis
echo "Restoring Redis..."
aws s3 cp ${S3_BUCKET}/redis/redis_${BACKUP_DATE}.rdb /tmp/
docker-compose -f deployments/redis/redis-cluster.yml down
cp /tmp/redis_${BACKUP_DATE}.rdb /var/lib/redis/dump.rdb
docker-compose -f deployments/redis/redis-cluster.yml up -d

# 4. 鎭㈠閰嶇疆
echo "Restoring config..."
aws s3 cp ${S3_BUCKET}/config/config_${BACKUP_DATE}.tar.gz /tmp/
tar -xzf /tmp/config_${BACKUP_DATE}.tar.gz -C /

# 5. 鍚姩鏈嶅姟
echo "Starting services..."
docker-compose up -d

# 6. 楠岃瘉鎭㈠
echo "Verifying recovery..."
sleep 30
curl -f http://localhost:18079/healthz || exit 1
curl -f http://localhost:18079/readyz || exit 1

echo "鉁?Restore completed successfully"
```

### 6.4 鎭㈠楠岃瘉娓呭崟

```yaml
recovery_verification:
  application:
    - [ ] 鏈嶅姟鍋ュ悍妫€鏌ラ€氳繃 (/healthz, /readyz)
    - [ ] WebSocket杩炴帴娴嬭瘯鎴愬姛
    - [ ] API鍔熻兘娴嬭瘯閫氳繃
    - [ ] 閰嶇疆鏂囦欢姝ｇ‘鍔犺浇
    
  database:
    - [ ] 鏁版嵁搴撹繛鎺ユ甯?
    - [ ] 鏁版嵁瀹屾暣鎬ч獙璇?(checksum)
    - [ ] 鏌ヨ鍔熻兘姝ｅ父
    - [ ] 澶嶅埗鐘舵€佹甯?
    
  redis:
    - [ ] Redis Cluster鍋ュ悍
    - [ ] 鏁版嵁瀹屾暣鎬ч獙璇?
    - [ ] 浼氳瘽璺敱姝ｅ父
    - [ ] 搴忓垪鍒嗛厤姝ｅ父
    
  user_verification:
    - [ ] 鐢ㄦ埛鐧诲綍娴嬭瘯
    - [ ] 娑堟伅鍙戦€佹帴鏀舵祴璇?
    - [ ] 鏂囦欢涓婁紶涓嬭浇娴嬭瘯
    - [ ] 鎼滅储鍔熻兘娴嬭瘯
    
  monitoring:
    - [ ] 鐩戞帶绯荤粺鎭㈠
    - [ ] 鍛婅瑙勫垯鐢熸晥
    - [ ] 鏃ュ織鏀堕泦姝ｅ父
```

---

## 7. 鍗囩骇缁存姢

### 7.1 鐗堟湰鍗囩骇娴佺▼

```yaml
upgrade_process:
  preparation:
    - 閫氱煡鐢ㄦ埛鍗囩骇璁″垝
    - 澶囦唤褰撳墠绯荤粺
    - 鍑嗗鍥炴粴鏂规
    - 娴嬭瘯鍗囩骇娴佺▼
    
  execution:
    step1_pre_upgrade:
      - 鍋滄鏃х増鏈湇鍔?
      - 楠岃瘉澶囦唤瀹屾暣鎬?
      - 鍑嗗鏂扮増鏈暅鍍?
      
    step2_database_migration:
      - 鎵ц鏁版嵁搴撹縼绉昏剼鏈?
      - 楠岃瘉杩佺Щ缁撴灉
      - 璁板綍杩佺Щ鏃ュ織
      
    step3_config_update:
      - 鏇存柊閰嶇疆鏂囦欢
      - 楠岃瘉閰嶇疆鏈夋晥鎬?
      - 澶囦唤鏂伴厤缃?
      
    step4_service_startup:
      - 鍚姩鏂扮増鏈湇鍔?
      - 楠岃瘉鏈嶅姟鍋ュ悍
      - 鎵ц鍔熻兘娴嬭瘯
      
  verification:
    - API鍔熻兘楠岃瘉
    - WebSocket杩炴帴娴嬭瘯
    - 鎬ц兘鍩哄噯娴嬭瘯
    - 瀹夊叏閰嶇疆妫€鏌?
    
  rollback:
    trigger:
      - 鍔熻兘楠岃瘉澶辫触
      - 鎬ц兘涓嶈揪鏍?
      - 鐢ㄦ埛涓ラ噸鎶曡瘔
      
    steps:
      - 鍋滄鏂扮増鏈湇鍔?
      - 鎭㈠鏁版嵁搴撳浠?
      - 鎭㈠鏃х増鏈厤缃?
      - 鍚姩鏃х増鏈湇鍔?
      - 楠岃瘉鍥炴粴鎴愬姛
```

### 7.2 鏁版嵁搴撹縼绉绘寚鍗?

```bash
#!/bin/bash
# 鏂囦欢: scripts/migrate-database.sh

NEW_VERSION=$1

echo "=== Database Migration ==="

# 1. 妫€鏌ヨ縼绉昏剼鏈?
ls database/migrations/ | grep -E "^${NEW_VERSION}"

# 2. 澶囦唤鏁版嵁搴?
scripts/backup.sh

# 3. 鎵ц杩佺Щ
for migration in database/migrations/${NEW_VERSION}/*.sql; do
    echo "Executing $migration..."
    psql $SDKWORK_IM_DATABASE_URL -f $migration || {
        echo "鉂?Migration failed"
        scripts/restore.sh latest
        exit 1
    }
done

# 4. 楠岃瘉杩佺Щ
psql $SDKWORK_IM_DATABASE_URL -c "SELECT * FROM schema_migrations ORDER BY version"

echo "鉁?Migration completed successfully"
```

### 7.3 鏈嶅姟閲嶅惎娴佺▼

```bash
#!/bin/bash
# 鏂囦欢: scripts/restart-services.sh

echo "=== Restarting Services ==="

# 1. 浼橀泤鍋滄
echo "Gracefully stopping services..."
docker-compose stop --timeout 30

# 2. 绛夊緟杩炴帴鏂紑
echo "Waiting for connections to drain..."
sleep 30

# 3. 鍚姩鏈嶅姟
echo "Starting services..."
docker-compose up -d

# 4. 鍋ュ悍妫€鏌?
echo "Checking health..."
for i in {1..10}; do
    if curl -f http://localhost:18079/readyz; then
        echo "鉁?Services restarted successfully"
        exit 0
    fi
    echo "Waiting for services to be ready... ($i/10)"
    sleep 5
done

echo "鉂?Services failed to start"
exit 1
```

---

## 8. 妫€鏌ユ竻鍗?

### 8.1 閮ㄧ讲妫€鏌ユ竻鍗?

#### 瀹夊叏閰嶇疆妫€鏌?(CRITICAL)

```markdown
# 鐢熶骇閮ㄧ讲瀹夊叏閰嶇疆妫€鏌ユ竻鍗?

## 蹇呴』椤?(CRITICAL - 鏈€氳繃鎷掔粷閮ㄧ讲)

- [ ] **JWT绛惧悕楠岃瘉宸插惎鐢?*
  - 閰嶇疆椤? `SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE=true`
  - 楠岃瘉鍛戒护: `grep SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE .env`
  - 棰勬湡缁撴灉: `true`
  
- [ ] **IAM鏁版嵁搴撹繛鎺ュ凡閰嶇疆**
  - 閰嶇疆椤? `SDKWORK_IM_IAM_DATABASE_URL`宸茶缃?
  - 楠岃瘉: 鏁版嵁搴撹繛鎺ユ祴璇曟垚鍔?
  - 娴嬭瘯鍛戒护: `psql $SDKWORK_IM_IAM_DATABASE_URL -c "SELECT 1"`
  
- [ ] **寮€鍙戠幆澧僨allback宸茬鐢?*
  - 閰嶇疆椤? `SDKWORK_IM_RUNTIME_PROFILE=production`
  - 楠岃瘉: 鍚姩鏃ュ織鏄剧ず"Production mode enforced"
  - 妫€鏌ュ懡浠? `grep SDKWORK_IM_RUNTIME_PROFILE .env`
  
- [ ] **HTTPS寮哄埗鍚敤**
  - 閰嶇疆椤? `SDKWORK_IM_FORCE_HTTPS=true`
  - 楠岃瘉: HTTP璇锋眰鑷姩閲嶅畾鍚戝埌HTTPS
  - 娴嬭瘯: `curl -I http://localhost:18079` 搴旇繑鍥?01閲嶅畾鍚?

## 鎺ㄨ崘椤?(HIGH - 寤鸿鍚敤)

- [ ] Redis Cluster宸查儴缃?(鑷冲皯3涓?浠?
  - 楠岃瘉: `redis-cli -c CLUSTER INFO` 鏄剧ず cluster_state:ok
  - 鑺傜偣鏁? 鑷冲皯6涓妭鐐?
  
- [ ] 鏁版嵁搴撹繛鎺ユ睜宸蹭紭鍖?
  - 閰嶇疆椤? `SDKWORK_IM_DATABASE_MAX_CONNECTIONS >= 50`
  - 楠岃瘉: 杩炴帴姹犵洃鎺ф樉绀哄厖瓒宠繛鎺?
  
- [ ] 閫熺巼闄愬埗宸查厤缃?
  - Layer 1: Per-IP rate limiting宸插惎鐢?
  - Layer 2: Per-tenant rate limiting宸插惎鐢?
  - 楠岃瘉: 鐩戞帶鏄剧ず闄愭祦鎸囨爣
  
- [ ] 鐔旀柇鍣ㄥ凡鍚敤
  - 姣忎釜涓婃父鏈嶅姟鐙珛鐔旀柇鍣?
  - 楠岃瘉: 鐔旀柇鍣ㄩ厤缃纭?
  
- [ ] 瀹¤鏃ュ織宸插惎鐢?
  - 鎵€鏈夊畨鍏ㄦ晱鎰熸搷浣滆褰曞璁℃棩蹇?
  - 楠岃瘉: `/var/log/sdkwork-im/audit.log` 鍖呭惈瀹¤璁板綍

## 鎵ц姝ラ

1. 杩愯瀹夊叏閰嶇疆妫€鏌ヨ剼鏈?
   ```bash
   scripts/check-security-config.sh
   ```

2. 淇鎵€鏈夋鏌ュけ璐ラ」

3. 閲嶆柊杩愯妫€鏌ョ洿鍒板叏閮ㄩ€氳繃

4. 璁板綍妫€鏌ョ粨鏋滀綔涓洪儴缃茶瘉鎹?
```

#### 鍔熻兘楠岃瘉妫€鏌?

```markdown
# 鍔熻兘楠岃瘉妫€鏌ユ竻鍗?

## 鏍稿績鍔熻兘楠岃瘉

- [ ] **鐢ㄦ埛璁よ瘉**
  - 鐧诲綍鎴愬姛骞惰幏鍙杢oken
  - Token楠岃瘉閫氳繃
  - 鏃犳晥token鎷掔粷璁块棶
  
- [ ] **娑堟伅鍙戦€佹帴鏀?*
  - 鍙戦€佹秷鎭垚鍔?
  - 鎺ユ敹鏂瑰疄鏃舵敹鍒版秷鎭?
  - 娑堟伅鐘舵€佹纭樉绀?
  
- [ ] **WebSocket杩炴帴**
  - WebSocket杩炴帴寤虹珛鎴愬姛
  - auth.init璁よ瘉閫氳繃
  - 瀹炴椂娑堟伅鎺ㄩ€佹甯?
  
- [ ] **鏂囦欢涓婁紶涓嬭浇**
  - 鏂囦欢涓婁紶鎴愬姛
  - 鏂囦欢涓嬭浇鎴愬姛
  - 鏂囦欢绫诲瀷楠岃瘉姝ｇ‘
  
- [ ] **娑堟伅鎼滅储**
  - 鍏ㄦ枃鎼滅储杩斿洖姝ｇ‘缁撴灉
  - 鎼滅储鎬ц兘婊¤冻瑕佹眰
  
## 闆嗘垚楠岃瘉

- [ ] **澶氱鎴烽殧绂?*
  - 涓嶅悓绉熸埛鏁版嵁闅旂
  - 璺ㄧ鎴疯闂嫆缁?
  
- [ ] **鏉冮檺楠岃瘉**
  - 鏉冮檺妫€鏌ユ纭墽琛?
  - 鏃犳潈闄愭搷浣滄嫆缁?
  
- [ ] **瀹¤鏃ュ織**
  - 鍏抽敭鎿嶄綔璁板綍瀹¤鏃ュ織
  - 鏃ュ織鏍煎紡绗﹀悎瑙勮寖

## 鎬ц兘楠岃瘉

- [ ] **API寤惰繜**
  - P99寤惰繜 < 100ms
  - 骞冲潎寤惰繜 < 50ms
  
- [ ] **骞跺彂鎬ц兘**
  - 鏀寔1000骞跺彂鐢ㄦ埛
  - 6000娑堟伅/鍒嗛挓
  
- [ ] **WebSocket杩炴帴**
  - 鏀寔1000骞跺彂杩炴帴
  - 杩炴帴绋冲畾涓嶆帀绾?

## 鎵ц鑴氭湰

```bash
scripts/verify-deployment.sh
```
```

### 8.2 杩愮淮鏃ュ父妫€鏌ユ竻鍗?

```markdown
# 杩愮淮鏃ュ父妫€鏌ユ竻鍗?

## 姣忔棩妫€鏌?(鑷姩鍖栨墽琛?

- [ ] **绯荤粺鍋ュ悍妫€鏌?*
  - 鎵€鏈夋湇鍔″仴搴风姸鎬佹甯?
  - 鑷姩鎵ц: Prometheus鍋ュ悍妫€鏌?
  
- [ ] **璧勬簮浣跨敤鐩戞帶**
  - CPU浣跨敤鐜?< 80%
  - 鍐呭瓨浣跨敤鐜?< 80%
  - 纾佺洏浣跨敤鐜?< 85%
  - 鑷姩鎵ц: Prometheus鎸囨爣鐩戞帶
  
- [ ] **鏁版嵁搴撳仴搴?*
  - PostgreSQL杩炴帴鏁版甯?
  - 鏌ヨ寤惰繜姝ｅ父
  - 澶嶅埗寤惰繜 < 10s
  - 鑷姩鎵ц: PostgreSQL exporter鐩戞帶
  
- [ ] **Redis鍋ュ悍**
  - Redis Cluster鐘舵€佹甯?
  - 鍐呭瓨浣跨敤姝ｅ父
  - 杩炴帴鏁版甯?
  - 鑷姩鎵ц: Redis exporter鐩戞帶
  
- [ ] **澶囦唤楠岃瘉**
  - 姣忔棩澶囦唤鎵ц鎴愬姛
  - 澶囦唤鏂囦欢瀹屾暣鎬ч獙璇?
  - 鑷姩鎵ц: 澶囦唤鑴氭湰
  
## 姣忓懆妫€鏌?(浜哄伐鎵ц)

- [ ] **鏃ュ織瀹℃煡**
  - 妫€鏌ュ紓甯告棩蹇?
  - 鍒嗘瀽鍛婅瓒嬪娍
  - 璇嗗埆娼滃湪闂
  
- [ ] **瀹夊叏瀹¤**
  - 妫€鏌ュ畨鍏ㄩ厤缃?
  - 瀹℃煡瀹¤鏃ュ織
  - 楠岃瘉璁块棶鎺у埗
  
- [ ] **鎬ц兘鍒嗘瀽**
  - 鍒嗘瀽鎬ц兘瓒嬪娍
  - 璇嗗埆鎬ц兘鐡堕
  - 鍒跺畾浼樺寲璁″垝
  
- [ ] **瀹归噺璇勪及**
  - 璇勪及璧勬簮浣跨敤瓒嬪娍
  - 棰勬祴瀹归噺闇€姹?
  - 鍒跺畾鎵╁璁″垝
  
## 姣忔湀妫€鏌?(浜哄伐鎵ц)

- [ ] **鐏惧婕旂粌**
  - 鎵ц鐏惧婕旂粌鑴氭湰
  - 楠岃瘉鎭㈠娴佺▼
  - 璁板綍婕旂粌缁撴灉
  
- [ ] **瀹夊叏鎵弿**
  - 鎵ц瀹夊叏鎵弿宸ュ叿
  - 淇鍙戠幇婕忔礊
  - 璁板綍淇缁撴灉
  
- [ ] **鐗堟湰鏇存柊**
  - 妫€鏌ヤ緷璧栫増鏈?
  - 璇勪及鏇存柊椋庨櫓
  - 鍒跺畾鏇存柊璁″垝
  
- [ ] **鏂囨。鏇存柊**
  - 鏇存柊杩愮淮鏂囨。
  - 鏇存柊妫€鏌ユ竻鍗?
  - 鍒嗕韩杩愮淮缁忛獙
```

### 8.3 鏁呴殰澶勭悊妫€鏌ユ竻鍗?

```markdown
# 鏁呴殰澶勭悊妫€鏌ユ竻鍗?

## 鏁呴殰鍙戠幇

- [ ] **鍛婅瑙﹀彂**
  - 鍛婅绫诲瀷璇嗗埆
  - 褰卞搷鑼冨洿璇勪及
  - 涓ラ噸鎬у垽瀹?
  
- [ ] **鍒濇璇婃柇**
  - 妫€鏌ョ郴缁熺姸鎬?
  - 鏌ョ湅閿欒鏃ュ織
  - 鍒嗘瀽鐩戞帶鎸囨爣
  
## 鏁呴殰瀹氫綅

- [ ] **鏃ュ織鍒嗘瀽**
  - 鏌ユ壘鍏抽敭閿欒鏃ュ織
  - 鍒嗘瀽閿欒鍫嗘爤
  - 瀹氫綅閿欒婧?
  
- [ ] **鎸囨爣鍒嗘瀽**
  - 鍒嗘瀽寮傚父鎸囨爣
  - 瀵规瘮姝ｅ父鍩哄噯
  - 鎵惧嚭寮傚父妯″紡
  
- [ ] **渚濊禆妫€鏌?*
  - 妫€鏌ユ暟鎹簱鐘舵€?
  - 妫€鏌edis鐘舵€?
  - 妫€鏌ョ綉缁滆繛閫氭€?
  
## 鏁呴殰淇

- [ ] **淇鎵ц**
  - 鎵ц淇鏂规
  - 璁板綍淇姝ラ
  - 鐩戞帶淇鏁堟灉
  
- [ ] **楠岃瘉鎭㈠**
  - 鍔熻兘楠岃瘉
  - 鎬ц兘楠岃瘉
  - 鐢ㄦ埛楠岃瘉
  
## 鏁呴殰鎬荤粨

- [ ] **鎶ュ憡缂栧啓**
  - 鏁呴殰鎻忚堪
  - 褰卞搷璇勪及
  - 淇杩囩▼
  - 缁忛獙鏁欒
  
- [ ] **鏀硅繘鎺柦**
  - 鍒跺畾棰勯槻鎺柦
  - 浼樺寲鐩戞帶鍛婅
  - 鏇存柊搴旀€ラ妗?
```

---

**鏂囨。缁存姢**: 杩愮淮鍥㈤槦  
**鏇存柊棰戠巼**: 姣忓搴eview  
**涓嬫鏇存柊**: 2026-09-30
