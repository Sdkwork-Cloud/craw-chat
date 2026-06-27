# RUNBOOK: Audit Log Investigation

## Trigger

- Security incident requiring audit trail review
- Compliance audit request
- Suspicious activity investigation

## Prerequisites

- Operator access to audit-service endpoints
- `SDKWORK_IM_DATABASE_URL` set (if Postgres-backed audit is configured)
- Appropriate authorization for audit data access

## Procedure

### 1. Export audit bundle

```bash
# Export audit records for a specific time range and tenant
curl -X GET "https://<gateway>/backend/v3/api/audit/export?tenantId=<tenant-id>&from=<start>&to=<end>" \
  -H "Authorization: Bearer <admin-token>" \
  -H "Access-Token: <access-token>" \
  -o audit-bundle.json
```

### 2. Verify chain integrity

```bash
# Verify the hash chain is unbroken
curl -X GET "https://<gateway>/backend/v3/api/audit/verify?tenantId=<tenant-id>" \
  -H "Authorization: Bearer <admin-token>" \
  -H "Access-Token: <access-token>"
```

### 3. Query specific records

```bash
# List audit records by aggregate type and action
curl -X GET "https://<gateway>/backend/v3/api/audit/records?tenantId=<tenant-id>&aggregateType=<type>&action=<action>&limit=100" \
  -H "Authorization: Bearer <admin-token>" \
  -H "Access-Token: <access-token>"
```

### 4. Analyze audit data

```bash
# Parse the exported bundle for specific events
jq '.records[] | select(.action == "message.delete")' audit-bundle.json
```

### 5. Document findings

Record the investigation results, including:
- Time range investigated
- Aggregate types and actions reviewed
- Chain verification status
- Anomalies detected

## Rollback

No rollback needed — this is a read-only investigation procedure.

## Escalation

Contact: SDKWork security team
