# RUNBOOK: Provider Outage Response

## Trigger

- Postgres connection errors in session-gateway logs
- Redis connection failures in realtime cluster bus
- Service health check degradation

## Prerequisites

- Operator access to deployment environment
- Postgres/Redis connection credentials
- Backup and recovery tools

## Procedure

### 1. Confirm outage scope

```bash
# Check Postgres connectivity
psql "$SDKWORK_IM_DATABASE_URL" -c "SELECT 1;"

# Check Redis connectivity
redis-cli -h "$SDKWORK_IM_REDIS_HOST" -p "$SDKWORK_IM_REDIS_PORT" ping
```

### 2. Assess service impact

- session-gateway: Realtime delivery disrupted if Redis is down
- projection-service: Reads degraded if Postgres is down; falls back to in-memory hot path
- audit-service: In-memory ledger continues; durability at risk if Postgres is down

### 3. Enable degraded mode (if applicable)

```bash
# For Redis outage, session-gateway continues with single-node mode
# Ensure SDKWORK_IM_REALTIME_CLUSTER_BUS_URL is unset to avoid retry storms
```

### 4. Restore provider

```bash
# Restart Postgres/Redis instance
# Verify connectivity before reconnecting services
```

### 5. Verify recovery

```bash
# Check service health
curl https://<gateway>/health
curl https://<gateway>/ready

# Verify projection snapshots are restored
curl https://<gateway>/im/v3/api/health/projection
```

## Rollback

No rollback needed — procedure is restorative.

## Escalation

Contact: SDKWork infrastructure team
