# RUNBOOK: Tenant Isolation Verification

## Trigger

- Scheduled quarterly isolation audit
- After multi-tenant schema changes
- After new shared-channel configuration

## Prerequisites

- Read access to Postgres (`SDKWORK_IM_DATABASE_URL`)
- Operator access to session-gateway health endpoints

## Procedure

### 1. Verify schema isolation

```bash
# Confirm each tenant has an isolated schema
psql "$SDKWORK_IM_DATABASE_URL" -c "
  SELECT schema_name FROM information_schema.schemata
  WHERE schema_name LIKE 'sdkwork_im_%';
"
```

### 2. Verify query scoping

```bash
# Confirm projection-service queries include tenant_id filter
psql "$SDKWORK_IM_DATABASE_URL" -c "
  EXPLAIN ANALYZE SELECT * FROM conversation_messages
  WHERE tenant_id = '<test-tenant-id>';
"
```

### 3. Verify dual-token enforcement

```bash
# Confirm session-gateway rejects requests without tenant_id
curl -s -o /dev/null -w "%{http_code}" \
  https://<gateway>/im/v3/api/chat/inbox \
  -H "Authorization: Bearer <token-without-tenant>"
# Expected: 401 or 403
```

### 4. Verify shared-channel isolation

```bash
# Confirm shared-channel sync only delivers to authorized members
# Check the sync pending pool for cross-tenant leakage
psql "$SDKWORK_IM_DATABASE_URL" -c "
  SELECT tenant_id, target_tenant_id, status
  FROM shared_channel_sync_pending
  WHERE created_at > NOW() - INTERVAL '1 hour';
"
```

## Rollback

No rollback needed — this is a read-only verification procedure.

## Escalation

Contact: SDKWork security team
