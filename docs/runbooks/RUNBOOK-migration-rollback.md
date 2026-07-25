# RUNBOOK: Database Migration Rollback

Status: active
Owner: `im-platform` and SDKWork database operations
Updated: 2026-07-24
Specs: `DATABASE_SPEC.md`, `MIGRATION_SPEC.md`, `RELEASE_SPEC.md`, `DOCUMENTATION_SPEC.md`

## Trigger

- An IM migration fails before completion.
- Post-deploy verification finds contract, checksum, tenant-isolation, or normalized-state damage.
- The release owner invokes the rollback decision recorded by the migration plan.

Rollback is a reviewed data operation. This runbook does not authorize an unplanned down migration,
manual migration-history edits, checksum repair, table deletion, or production restore.

## Prerequisites

- Human approval from the release owner and database owner.
- The exact release artifact, migration record, and database contract used by the deployment.
- An encrypted, integrity-verified pre-migration backup and a successful restore rehearsal.
- Admin access obtained from the approved secret manager.
- Recorded per-tenant row counts, constraints, indexes, migration history, and checksums.

Canonical authority:

- `database/contract/schema.yaml`
- `database/contract/table-registry.json`
- the migration roots registered by those contracts
- the migration-specific record under `docs/migrations/`

Do not infer current migration state from a historical review document or a previously captured
pending/drift count. Re-run the canonical tools against the target environment.

## Procedure

### 1. Freeze traffic and durable writers

1. Remove the affected standalone gateway or cloud service instances from traffic.
2. Stop the IM Agent dispatch worker, retention scheduler, realtime relays, and every separately
   deployed IM worker that can write the affected tables.
3. Confirm `GET /readyz` is no longer used to admit traffic and that active write rate is zero.
4. Record the incident time, release identity, topology profile, and last successful migration step.

Do not stop only a route surface. In standalone topology the application gateway owns the listener
and embeds the relevant IM runtimes.

### 2. Re-evaluate migration state

Run from the exact release source with the approved target configuration:

```bash
pnpm db:status
pnpm db:drift
pnpm db:contract:check
pnpm db:postgres:plan
```

Capture redacted outputs as incident evidence. Do not run `db:postgres:repair`, edit migration
checksums, or mark migrations successful merely to make status green.

### 3. Select the reviewed rollback path

Use only the path approved by the migration record:

- **Pre-commit failure:** allow the database framework transaction to roll back, then verify no
  partial schema or data changes remain.
- **Paired reversible migration:** execute the reviewed paired down migration through the approved
  database operator procedure. The repository has no generic `pnpm db:postgres:rollback` command;
  never invent one or run ad hoc SQL.
- **Irreversible or post-cutover migration:** restore the encrypted pre-migration backup into a new
  database target, validate it, and switch traffic only after release-owner approval.
- **Forward repair:** use only when the migration plan explicitly chooses a new immutable forward
  migration and the release owner rejects restore/down migration.

For the normalized IM authority cutover, follow
[`MIGRATION-20260722-normalized-im-authority.md`](../migrations/MIGRATION-20260722-normalized-im-authority.md).
After retired duplicate state has been removed, backup restore is the only rollback path unless a
new reviewed migration says otherwise.

### 4. Validate restored state

Validate in an isolated target before reconnecting application traffic:

```bash
pnpm db:status
pnpm db:drift:check
pnpm db:contract:check
pnpm db:validate
```

Also verify:

- scoped row counts and checksums match the approved rollback checkpoint;
- `im_conversations`, `im_conversation_members`, `im_conversation_messages`, and
  `im_conversation_read_cursors` preserve tenant and organization isolation;
- message sequence, membership episode, read cursor, idempotency, journal, and outbox invariants
  pass;
- no compatibility table, shadow store, dual write, or alternate current-state authority is active;
- the application release selected for restart is compatible with the restored schema version.

### 5. Restart and verify service

1. Start one canary instance with workers disabled or paused where the deployment controller permits.
2. Confirm startup database lifecycle and embedded dependency bootstrap complete.
3. Enable required workers, then require the composite readiness check to pass.
4. Roll out remaining instances and restore traffic gradually.

```bash
curl --fail-with-body https://<application-ingress>/healthz
curl --fail-with-body https://<application-ingress>/livez
curl --fail-with-body https://<application-ingress>/readyz
```

Run authorized Conversation read, Message write/idempotency, membership, read-cursor, realtime, and
Agent dispatch smoke tests. Monitor error rate, pool saturation, outbox lag, worker leases, and
sequence conflicts throughout the observation window.

## Rollback Failure

If validation fails, keep traffic and workers disabled. Do not retry destructive steps against the
same target. Preserve evidence, provision a fresh restore target, and escalate to the database and
release owners for a new reviewed recovery decision.

## Escalation

- Database owner: SDKWork database operations
- Application owner: `im-platform`
- Release decision: SDKWork release owner
- Suspected tenant or data exposure: SDKWork security incident owner
