# SDKWork IM Commercial Readiness Review

Status: superseded
Owner: SDKWork maintainers
Review: `REVIEW-2026-0710`
Review period: 2026-07-10 through 2026-07-21
Superseded: 2026-07-24
Specs: `DOCUMENTATION_SPEC.md`, `CODE_REVIEW_SPEC.md`, `QUALITY_GATE_SPEC.md`, `RELEASE_SPEC.md`

## Historical Outcome

This review did not approve SDKWork IM for commercial production. It recorded a point-in-time audit
of the implementation and local verification environment during the review period. Its detailed
implementation findings and locally observed database-state counts were inputs to subsequent work;
they are not current architecture, migration, readiness, or release authority.

## Supersession

The persistence and IM-to-Agents portions of this review were superseded by the following active
records and their implementation evidence:

- [Normalized IM authority requirement](../../product/requirements/REQ-2026-0722-normalized-im-authority.md)
- [Normalized IM authority decision](../../architecture/decisions/ADR-20260722-normalized-im-authority.md)
- [Normalized IM authority migration](../../migrations/MIGRATION-20260722-normalized-im-authority.md)
- [IM-to-Agents dispatch requirement](../../product/requirements/REQ-2026-0719-agents-dispatch.md)
- [Current Technical Architecture Canon](../../architecture/tech/TECH_ARCHITECTURE.md)

Current IM state is read from typed normalized PostgreSQL tables. The commit journal is immutable
audit/integration evidence and the outbox is a delivery mechanism. Neither is a second current-state
model. IM-visible Messages and Agents SessionItems remain separate domain facts. IM consumes the
canonical Agents Session facade and does not own Agent Session, Turn, or SessionItem storage.

Any earlier finding that assumes a retired service, alternate persisted query authority, duplicate
Message timeline, or a second Agent conversation/session model must not be used to guide new work.

## Current Release Posture

Commercial release posture is resolved only from current machine contracts and fresh evidence:

- `sdkwork.app.config.json` remains the application publication and artifact-policy authority.
- `database/contract/table-registry.json` and `database/contract/schema.yaml` define current IM
  persistence ownership and migration roots.
- the authored OpenAPI documents under `apis/` define IM-owned HTTP operations.
- `/healthz`, `/livez`, `/readyz`, and `/metrics` are the infrastructure probe surface.
- `/readyz` must combine IM database connectivity, configured Redis dependencies, embedded Agents
  state, Agent dispatch worker health, and realtime plane health for the running topology.
- release evidence must be produced for the exact signed artifact; fixture, document-only, or stale
  local evidence cannot satisfy checksum, SBOM, provenance, capacity, HA, recovery, or migration gates.

At supersession time the app remains `DRAFT`; this historical review grants no exception and no
release approval.

## Database State Evidence Rule

Migration pending state and schema drift are live environment facts. Historical numeric results are
intentionally not preserved as current blockers in this review. Operators and reviewers must run the
canonical commands against the selected target and attach redacted outputs to the new review:

```bash
pnpm db:status
pnpm db:drift:check
pnpm db:contract:check
pnpm db:validate
```

Checksum repair, migration-history edits, and production migration/rollback remain human-reviewed
operations; a green documentation or static-contract check is not a substitute for database evidence.

## Required Fresh Review

A commercial sign-off requires a new `REVIEW-*` record linked from `docs/INDEX.yaml`. That review
must evaluate the current release candidate rather than reopening this snapshot and must include:

1. Current requirement, ADR, migration, and architecture traceability.
2. API, SDK, database, security, tenant-isolation, dependency, and architecture gates.
3. Rust standalone gateway and PC production build/runtime evidence.
4. Live PostgreSQL transaction, idempotency, recovery, migration, and rollback rehearsal evidence.
5. Composite readiness failure tests proving `503` and client-safe redaction.
6. Staging capacity, soak, restart, node-loss, Redis interruption, and PostgreSQL failover evidence.
7. Signing, checksum, SBOM, provenance, artifact identity, rollout, and rollback evidence.
8. An explicit release-owner decision under `QUALITY_GATE_SPEC.md` and `RELEASE_SPEC.md`.

Until that record exists and every required gate passes, SDKWork IM remains blocked from commercial
production sign-off.
