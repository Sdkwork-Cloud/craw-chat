# RUNBOOK: Token and Key Rotation

## Trigger

- Scheduled annual JWT signing key rotation
- Suspected key compromise
- IAM tenant signing key policy enforcement

## Prerequisites

- Operator access to IAM admin console
- `SDKWORK_IM_DATABASE_URL` set for the target environment
- Maintenance window (low-traffic period recommended)

## Procedure

### 1. Generate new signing key

```bash
# Generate a new HS256 secret for bootstrap signing
openssl rand -base64 48

# Or provision a new IAM tenant signing key via the admin API
```

### 2. Update environment configuration

```bash
# Set the new key ID and secret
export SDKWORK_IM_APP_CONTEXT_JWT_KEY_ID="<new-key-id>"
export SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET="<new-secret>"
```

### 3. Rolling restart session-gateway

```bash
# Restart one node at a time to maintain availability
# Verify each node accepts both old and new tokens during overlap
```

### 4. Verify token issuance

```bash
# Login via credential entry and verify the response contains the new kid
curl -X POST https://<gateway>/im/v3/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"test","password":"test"}'
```

### 5. Revoke old key

After all clients have refreshed tokens (typically 24h), revoke the old key via IAM admin.

## Rollback

If issues occur, restore the previous `SDKWORK_IM_APP_CONTEXT_JWT_KEY_ID` and `SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET` values and restart session-gateway.

## Escalation

Contact: SDKWork security team
