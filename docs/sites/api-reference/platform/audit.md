# Platform Audit

This page keeps the platform audit documentation anchor stable for operators and release checks.
The canonical backend audit API page is [Backend Audit](../backend/audit.md).

## Chain Verification

`GET /backend/v3/api/audit/verify` verifies the visible audit hash chain.

Response schema: `AuditChainVerification`

Key fields:

- `chainHeadHash`: latest chain hash at verification time.
- `chainValid`: whether the tenant audit chain is currently valid.

