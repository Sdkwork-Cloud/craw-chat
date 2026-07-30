# RUNBOOK: Provider Outage Response

Status: active
Owner: `im-platform` and SDKWork infrastructure
Updated: 2026-07-24
Specs: `HEALTH_CHECK_SPEC.md`, `SECURITY_SPEC.md`, `DEPLOYMENT_SPEC.md`, `DOCUMENTATION_SPEC.md`

## Trigger

- `GET /readyz` returns `503` while `GET /healthz` remains `200`.
- PostgreSQL connectivity, pool-acquisition, or migration-authority checks fail.
- A configured Redis route store or realtime cluster bus cannot complete `PING`.
- Embedded Agents state, the IM Agent dispatch worker, or the realtime plane reports unavailable.

`/healthz` proves only that the HTTP process is alive. `/readyz` is the traffic-admission signal and
combines all dependencies required by the running topology. A `503` must be treated as unavailable;
operators must not route traffic based only on liveness.

## Prerequisites

- Operator access to the selected deployment environment and its secret manager.
- PostgreSQL and Redis provider consoles or approved command-line clients.
- Access to redacted gateway logs, metrics, and the deployment controller.
- The exact topology profile and release artifact currently deployed.

Never print connection URLs, credentials, provider payloads, or raw readiness errors into tickets,
shell history, dashboards, or customer-visible responses. Detailed dependency errors belong only in
the restricted server logs; the public readiness response remains generic.

## Procedure

### 1. Confirm the failing plane

```bash
curl --silent --show-error https://<application-ingress>/healthz
curl --silent --show-error https://<application-ingress>/readyz
```

Expected during a dependency outage:

- `/healthz`: `200 {"status":"ok"}`
- `/readyz`: `503` with the canonical client-safe dependency-unavailable detail

Correlate the transition with low-cardinality readiness and pool metrics. Read the restricted server
log for the failing check name; do not expose its raw detail to clients.

### 2. Verify providers through approved operator channels

Resolve credentials from the environment's secret manager without echoing them. Then run the
smallest read-only check:

```bash
psql "$SDKWORK_DATABASE_URL" -v ON_ERROR_STOP=1 -c "SELECT 1;"
redis-cli -u "$SDKWORK_IM_REDIS_URL" PING
```

For realtime profiles, test each configured route-store or cluster-bus endpoint through the same
approved secret-handling path. Do not substitute an in-memory store, change a cluster endpoint, or
unset a required dependency in a running production profile.

### 3. Assess impact

- PostgreSQL outage: normalized Conversation, Message, Member, ReadCursor, idempotency, journal,
  outbox, and dispatch state are unavailable. Writes and authoritative reads must fail closed.
- Redis outage in a topology that requires Redis: cluster routing and realtime coordination are
  unavailable. PostgreSQL remains the durable authority, but `/readyz` stays `503` until the selected
  topology is healthy.
- Embedded Agents outage: Agent execution dispatch is unavailable and gateway readiness fails when
  the dependency is required by the running profile. Human IM facts remain distinct from Agents
  Session and SessionItem facts.
- Worker or realtime-plane failure: stop traffic admission and restart the affected process only
  after its durable dependencies are verified.

There is no alternate query model, compatibility store, dual write, or silent local fallback.

### 4. Restore the provider and validate authority

1. Restore provider service or fail over using the provider's reviewed procedure.
2. Verify PostgreSQL recovery, replication, and connection-pool capacity.
3. Run `pnpm db:status`, `pnpm db:drift:check`, and `pnpm db:contract:check` from the exact release
   source against a disposable or approved operator target as required by the incident plan.
4. Verify Redis persistence/replication and `PING` for every endpoint required by the topology.
5. Restart or roll the affected gateway and workers using the deployment controller. Do not change
   dependency identities during the recovery restart.

### 5. Verify recovery

```bash
curl --fail-with-body https://<application-ingress>/healthz
curl --fail-with-body https://<application-ingress>/livez
curl --fail-with-body https://<application-ingress>/readyz
curl --fail-with-body https://<application-ingress>/metrics
```

Confirm that `/readyz` returns `200 {"status":"ready"}` only after IM database connectivity,
configured Redis connectivity, embedded Agents state, Agent dispatch worker health, and realtime
plane health all pass. Then execute one authorized Conversation read, one idempotent Message write,
and the profile-specific realtime/Agent smoke checks. Verify outbox and worker lag return to their
normal bounded ranges before restoring full traffic.

## Rollback And Safety

This procedure restores required providers; it does not authorize a topology downgrade. If recovery
validation fails, keep the instance out of service, roll back to the last verified release only when
its database contract is compatible, and follow
[`RUNBOOK-migration-rollback.md`](RUNBOOK-migration-rollback.md) when schema state changed.

## Escalation

- Primary: `im-platform` on-call
- Provider: SDKWork infrastructure on-call
- Security or suspected data exposure: SDKWork security incident owner
