# Runbooks

Operational runbooks for Sdkwork IM production deployments. Each runbook follows the `RUNBOOK-*.md` naming convention and covers a single operational procedure.

## Index

| Runbook | Scope |
| --- | --- |
| [RUNBOOK-token-key-rotation.md](RUNBOOK-token-key-rotation.md) | JWT signing key and IAM tenant key rotation |
| [RUNBOOK-tenant-isolation-verification.md](RUNBOOK-tenant-isolation-verification.md) | Cross-tenant data isolation audit |
| [RUNBOOK-migration-rollback.md](RUNBOOK-migration-rollback.md) | Database migration rollback procedure |
| [RUNBOOK-provider-outage.md](RUNBOOK-provider-outage.md) | Postgres/Redis provider outage response |
| [RUNBOOK-audit-log-investigation.md](RUNBOOK-audit-log-investigation.md) | Audit ledger investigation and export |

## Convention

Each runbook MUST include:

1. Trigger conditions (when to execute)
2. Prerequisites (access, tools, env vars)
3. Step-by-step procedure with verification commands
4. Rollback / safety net
5. Escalation contacts
