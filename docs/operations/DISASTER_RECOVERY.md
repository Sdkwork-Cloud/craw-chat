# Sdkwork IM — Disaster Recovery Plan

Status: active
Owner: SDKWork maintainers
Updated: 2026-07-03
Specs: OBSERVABILITY_SPEC.md, SECURITY_SPEC.md, PRIVACY_SPEC.md, DEPLOYMENT_SPEC.md, REGION_SPEC.md, CONFIG_SPEC.md

## 1. Purpose

This document is the comprehensive Disaster Recovery (DR) plan for the Sdkwork IM platform. It
defines the strategies, recovery scenarios, failover procedures, replication architecture, testing
cadence, and communication protocols required to restore service after a partial or total outage.

It goes beyond the routine backup and restore procedures in
`docs/operations/OPERATIONS_MANUAL.md` section 6 and the recovery objectives in
`docs/product/compliance/SLA_SLO.md` section 7. Where those documents define *what* is backed up
and *how fast* recovery must be, this document defines *how a regional or systemic failure is
detected, contained, and recovered* in production.

### 1.1 Scope

| In Scope | Out of Scope |
| --- | --- |
| Service-level, component-level, and region-level failures | Single-user support tickets |
| PostgreSQL, Redis, object storage, and microservice recovery | Application bug fixes (see OPERATIONS_MANUAL §3) |
| Multi-region failover (Phase 2 active-passive) | Active-active write coordination (Phase 3 roadmap only) |
| Data corruption point-in-time recovery | Capacity planning (see OPERATIONS_MANUAL §4) |
| DR drills, communication, and post-incident review | Security incident response (see OPERATIONS_MANUAL §5.3) |

### 1.2 Audience

- **On-call engineers**: runbook execution, failover operations.
- **SRE / platform team**: replication health, DR drill ownership.
- **Engineering leadership**: failover decision authority, customer communication.
- **Customer success / status page owner**: external communication.

## 2. Current Architecture Limitations

Sdkwork IM is a multi-tenant real-time messaging platform built in Rust. It uses CQRS with Event
Sourcing, where `im_commit_journal` (PostgreSQL) is the append-only durable event log and
projections are rebuilt from the journal. Redis holds session state, presence, and sequence caches.
The platform runs as 15 microservices on Kubernetes.

### 2.1 Single-Region Topology

Today the platform operates in **single-region mode** per deployment topology in `configs/topology/`.
All write traffic for a tenant is served by one PostgreSQL primary and one Redis cluster in the
same region.

| Component | Current Topology | DR Implication |
| --- | --- | --- |
| PostgreSQL primary | Single writer per region | Loss of primary = write outage until promoted replica is live |
| PostgreSQL replicas | 1–2 read replicas, same region | No cross-region standby; region loss = RPO gap |
| Redis cluster | Single cluster, 6 nodes, same region | Cache only; rebuilt on cold start, but session resumption is lost |
| Object storage (SDKWork Drive) | Region-local with optional cross-region replication | Media availability depends on replication being enabled |
| Kubernetes cluster | Single regional cluster | Pod auto-restart only; no cross-cluster failover |
| DNS / ingress | Single-region ingress | Manual DNS switch required for regional failover |

### 2.2 Hard Limitations (Phase 1)

1. **Single-writer PostgreSQL**: `im_commit_journal` appends go to one primary. There is no
   multi-master write path today.
2. **No cross-region replication**: PostgreSQL streaming replication is intra-region only. There
   is no logical publication to a standby region.
3. **No automated regional failover**: DNS, ingress, and traffic shifting are manual.
4. **Event journal is regional**: the CQRS journal has no dual-write or CDC pipeline to a second
   region, so a region loss can lose unreplicated events up to the last archived WAL segment.
5. **Redis is not durable**: session and presence state is not replicated across regions; a
   regional failover forces all clients to reconnect and re-authenticate.

These limitations are the reason the Enterprise RPO of 5 minutes (see §5) is only achievable once
Phase 2 cross-region replication ships. Until then, region-level RPO is bounded by the WAL
archival interval (best case 1 hour for Professional tier).

## 3. DR Strategy Overview

### 3.1 Phased Roadmap

| Phase | Timeline | Topology | Strategy | RTO / RPO |
| --- | --- | --- | --- | --- |
| Phase 1 (current) | Now | Single-region | Backup + PITR; intra-region replica failover | 30 min / up to 1 hr (region loss) |
| Phase 2 | 2026 Q4 | Active-passive, two regions | Async logical replication to DR region; manual failover | 30 min / 5 min |
| Phase 3 | 2027+ (roadmap) | Active-active, multi-region | Dual-write + conflict-free journals; automated failover | Target < 5 min / < 1 min |

### 3.2 Phase 2 Strategy — Active-Passive

- **Primary region**: serves all read and write traffic. PostgreSQL primary, Redis cluster,
  full microservice fleet (15 services).
- **DR region**: hot standby. PostgreSQL promoted-as-replica (logical subscriber), independent
  Redis cluster, full microservice fleet running at minimum replica count. No client traffic.
- **Replication**: asynchronous PostgreSQL logical replication (publication/subscription) for
  `im_commit_journal` and projection tables. Eventual consistency for DR region.
- **Failover**: manual decision, DNS switch, promote DR PostgreSQL, shift ingress.
- **Fallback**: planned switch-back after primary region is repaired and re-synced.

### 3.3 Phase 3 Roadmap — Active-Active

Phase 3 is research-only at this point. The intended direction is region-affinity with
conflict-free replicated journals (CRDT-style sequence assignment or region-prefixed event IDs)
so that both regions can accept writes for their own tenants and reconcile asynchronously. This
is out of scope for the current plan and will be defined in a separate ADR before implementation.

## 4. Recovery Scenarios

Each scenario has a detection signal, a contained recovery path, and a defined owner.

### 4.1 Service-Level Failure (Single Pod Crash)

A single microservice pod crashes or becomes unready. The other 14 services and the data plane
are unaffected.

| Attribute | Value |
| --- | --- |
| Detection | Kubernetes liveness/readiness probe failure; Prometheus `HighErrorRate` or pod restart alert |
| Recovery mechanism | Kubernetes auto-restart (`Deployment` with `restartPolicy: Always`); HPA scales healthy pods |
| Owner | On-call engineer (verify only) |
| Typical recovery time | 30–90 seconds |
| Data loss | None |

```bash
# Verify pod recovery
kubectl -n sdkwork-im get pods -l app.kubernetes.io/name=<service>
kubectl -n sdkwork-im rollout status deployment/<service>

# If restart loops, inspect
kubectl -n sdkwork-im describe pod <pod-name>
kubectl -n sdkwork-im logs <pod-name> --previous
```

If a pod fails to recover after 3 restarts, escalate to §4.2 (component failure) — the root cause
is likely a dependency (PostgreSQL or Redis), not the pod itself.

### 4.2 Component Failure (PostgreSQL Primary Down)

The PostgreSQL primary becomes unreachable or fails write queries. This affects every
microservice that appends to `im_commit_journal` or reads projections.

| Attribute | Value |
| --- | --- |
| Detection | `PostgreSQLDown` alert (`pg_up == 0` for 1 min); write errors on conversation/message APIs |
| Recovery mechanism | Promote a synchronous streaming replica to primary; reconfigure services via `SDKWORK_IM_DATABASE_URL` |
| Owner | SRE on-call + on-call engineer |
| Typical recovery time | 5–15 minutes (Enterprise RTO 30 min) |
| Data loss | Zero if a synchronous replica was promoted; otherwise up to last WAL flush |

Recovery path:

1. Confirm primary is down (not a network blip): `psql $SDKWORK_IM_DATABASE_URL -c "SELECT 1"`.
2. Pick the most up-to-date replica: `SELECT application_name, write_lag, flush_lag, replay_lag
   FROM pg_stat_replication`.
3. Promote the replica using the managed PostgreSQL operator (`kubectl` for CloudNativePG) or
   `pg_ctl promote` for self-managed.
4. Update `SDKWORK_IM_DATABASE_URL` and roll the microservice fleet:
   `kubectl -n sdkwork-im rollout restart deployment`.
5. Verify health: `/healthz`, `/readyz`, and a test message send.

### 4.3 Region Failure (Entire Region Unavailable)

The primary region is lost: Kubernetes control plane, PostgreSQL, Redis, and ingress are all
unreachable. This is the scenario the Phase 2 DR region exists for.

| Attribute | Value |
| --- | --- |
| Detection | Multi-service outage; region health probe failure; cloud provider status page |
| Recovery mechanism | Promote DR region PostgreSQL, switch DNS to DR ingress, scale DR fleet to full capacity |
| Owner | Engineering leadership (decision) + SRE (execution) |
| Typical recovery time | 30 minutes (Enterprise RTO), 2 hours (Professional) |
| Data loss | Up to RPO: 5 min (Enterprise, Phase 2) or up to 1 hr (Phase 1, WAL archival gap) |

Recovery path: see §7 Failover Procedure.

### 4.4 Data Corruption (Logical or Accidental)

A bad migration, buggy write path, or operator error corrupts rows in `im_commit_journal` or a
projection. The cluster is up, but the data is wrong.

| Attribute | Value |
| --- | --- |
| Detection | Audit log anomaly, projection checksum mismatch, user reports of missing/wrong messages |
| Recovery mechanism | Point-in-time recovery (PITR) from WAL archives; journal replay to rebuild projections |
| Owner | SRE + on-call engineer |
| Typical recovery time | 30–90 minutes depending on database size |
| Data loss | Transactions between PITR target and now must be re-applied manually or lost |

Recovery path:

```bash
# 1. Stop write traffic (cordon the primary region ingress)
kubectl -n sdkwork-im scale deployment im-gateway --replicas=0

# 2. Restore to a target recovery time from WAL archives
#    (uses the base backup + archived WAL in s3://backup-sdkwork-im/db-wal/)
scripts/restore-pitr.sh --target-time "2026-07-03 09:15:00+00"

# 3. Replay journal into projections to rebuild derived state
scripts/rebuild-projections.sh --from <journal-cursor>

# 4. Verify with checksums, then restore write traffic
kubectl -n sdkwork-im scale deployment im-gateway --replicas=3
```

## 5. RTO/RPO Targets

RTO and RPO targets are owned by `docs/product/compliance/SLA_SLO.md` section 7.1 and are
reproduced here for operational reference. This document does not redefine them.

### 5.1 Targets per Tier

| Tier | RTO | RPO | Mechanism |
| --- | --- | --- | --- |
| Community | Best-effort | Best-effort | Backup restore only |
| Professional | 2 hours | 1 hour (WAL archival) | Intra-region replica + PITR |
| Enterprise | 30 minutes | 5 minutes (synchronous replication) | Phase 2: cross-region logical replication |

### 5.2 Targets per Scenario

| Scenario | Tier | Target RTO | Target RPO | Phase 1 achievable? | Phase 2 achievable? |
| --- | --- | --- | --- | --- | --- |
| Single pod crash (§4.1) | All | 90 seconds | 0 | Yes | Yes |
| PostgreSQL primary down (§4.2) | Enterprise | 15 minutes | 0 (sync replica) | Yes | Yes |
| PostgreSQL primary down (§4.2) | Professional | 30 minutes | Up to last WAL | Yes | Yes |
| Region failure (§4.3) | Enterprise | 30 minutes | 5 minutes | No (RPO up to 1 hr) | Yes |
| Region failure (§4.3) | Professional | 2 hours | 1 hour | No (best-effort) | Partial |
| Data corruption (§4.4) | All | 60 minutes | Up to PITR target | Yes | Yes |

### 5.3 Error Budget Interaction

Per `SLA_SLO.md` §6, a regional failover that exceeds the RTO consumes the monthly error budget.
If the budget is already exhausted, only security and reliability fixes may deploy during the
recovery window. DR failover itself is never blocked by the error budget policy — availability
takes precedence over the deployment freeze.

## 6. Multi-Region Architecture (Phase 2)

### 6.1 Regional Roles

| Region | Role | PostgreSQL | Redis | Microservices | Client traffic |
| --- | --- | --- | --- | --- | --- |
| Primary | Read-write | Primary (publisher) | Independent cluster (primary) | Full fleet, production replicas | 100% |
| DR | Hot standby | Logical subscriber (promotable) | Independent cluster (standby) | Full fleet, minimum replicas | 0% until failover |

### 6.2 PostgreSQL Logical Replication

Cross-region replication uses PostgreSQL logical replication (publication/subscription), not
physical streaming, so the DR region can run a different major version during upgrades and can
replicate only the tables that matter for DR.

```sql
-- On the primary region (publisher)
CREATE PUBLICATION sdkwork_im_dr_pub FOR TABLE
  im_commit_journal,
  im_outbox_events,
  conversations, messages, members, read_cursors,
  tenants, tenant_quotas
  WITH (publish = 'insert, update, delete');

-- On the DR region (subscriber)
CREATE SUBSCRIPTION sdkwork_im_dr_sub
  CONNECTION 'host=primary-region-pg.ssl.postgres.sql port=5432 dbname=im user=replicator'
  PUBLICATION sdkwork_im_dr_pub
  WITH (copy_data = true, create_slot = true, enabled = true);
```

Replication lag is monitored via `pg_stat_subscription` and exported to Prometheus. The SLO for
replication lag is P95 < 5 seconds; if lag exceeds 60 seconds, a `DRReplicationLagHigh` alert
fires (see OPERATIONS_MANUAL §2.3).

### 6.3 Redis — Independent Clusters

Redis is treated as a cache, not a durable store. Each region runs its own independent Redis
cluster; there is no cross-region Redis replication.

| Data type | Region failure behavior |
| --- | --- |
| Session tokens | Lost on failover; clients must re-authenticate via IAM |
| Presence state | Lost on failover; clients re-publish presence on reconnect |
| Sequence caches | Rebuilt from `im_commit_journal` cursor on startup |
| Rate-limit counters | Reset; brief burst tolerance allowed |

This is an explicit trade-off: the cost of cross-region Redis replication (latency, split-brain
risk) is not justified for non-durable data. The impact is bounded — clients reconnect within
seconds via the WebSocket reconnection protocol.

### 6.4 Object Storage — Cross-Region Replication via SDKWork Drive

Media attachments and file messages are stored in SDKWork Drive, which performs cross-region
replication at the storage layer. This is independent of the PostgreSQL replication path.

| Object class | Replication | RPO |
| --- | --- | --- |
| Media blobs (images, files) | Async cross-region replication | < 1 minute |
| Tenant avatars | Async cross-region replication | < 1 minute |
| Audit log bundles | Replicated on archival | < 1 hour |

Replication is configured in `configs/topology/cloud.split-services.production.env` via
`SDKWORK_IM_DRIVE_REPLICATION_TARGETS`. Tenants with data residency constraints (per
`COMPLIANCE_FRAMEWORK.md` §4) may opt out of cross-region replication; such tenants are not
eligible for cross-region DR and must rely on intra-region recovery only.

### 6.5 Microservice Fleet in DR Region

All 15 microservices are deployed to the DR region at all times, but at minimum replica counts to
control cost. On failover, HPA scales them to production capacity.

| Service | Primary replicas | DR standby replicas | On failover |
| --- | --- | --- | --- |
| im-gateway | 3+ | 1 | Scale to 3+ |
| session-gateway | 3+ | 1 | Scale to 3+ |
| conversation-service | 3+ | 1 | Scale to 3+ |
| projection-service | 3+ | 1 | Scale to 3+ |
| streaming-service | 3+ | 1 | Scale to 3+ |
| media-service | 2+ | 1 | Scale to 2+ |
| notification-service | 2+ | 1 | Scale to 2+ |
| audit-service | 2+ | 1 | Scale to 2+ |
| governance-service | 2+ | 1 | Scale to 2+ |
| automation-service | 2+ | 1 | Scale to 2+ |
| social-service | 2+ | 1 | Scale to 2+ |
| space-service | 2+ | 1 | Scale to 2+ |
| ops-service | 2+ | 1 | Scale to 2+ |

> **Note (2026-07):** `contact-service` and `interaction-service` are deprecated shims; social and projection surfaces run in `social-service` and `projection-service`. Do not scale retired process names in new deployments.

## 7. Failover Procedure

Regional failover is a five-phase procedure. Each phase has an owner, a verification gate, and a
rollback path.

### 7.1 Phase A — Detection

| Signal | Source | Threshold |
| --- | --- | --- |
| Multi-service 5xx surge | Prometheus `HighErrorRate` | > 0.1 errors/s for 5 min |
| Region health probe failure | Synthetic probe from DR region | 3 consecutive failures |
| PostgreSQL unreachable | `PostgreSQLDown` alert | `pg_up == 0` for 1 min |
| Cloud provider status | Provider status page | Confirmed regional incident |

Detection is automated; the alert pages the SRE on-call and the engineering leader on call.

### 7.2 Phase B — Decision (Manual)

Failover is **manual**, not automated. An automated failover risks split-brain if the primary
region is only partially degraded. The decision matrix:

| Condition | Action |
| --- | --- |
| Primary region fully unreachable, confirmed by provider status | Failover to DR |
| Primary region degraded but responding | Investigate first; failover only if RTO at risk |
| Network partition between regions | Wait for partition to heal; do not failover |
| Confirmed data corruption in primary | Failover to DR only if DR is not corrupted |

Decision authority: **engineering leadership on-call**, in consultation with SRE. The decision and
its rationale are recorded in the incident channel before execution.

### 7.3 Phase C — Execution

```bash
# 1. Stop write traffic in the primary region (if still reachable)
kubectl --context=primary-context -n sdkwork-im scale deployment im-gateway --replicas=0

# 2. Wait for replication lag to drain (DR region subscriber)
psql $DR_DATABASE_URL -c "
  SELECT application_name, pid, received_lsn, latest_end_lsn,
         latest_end_lsn - received_lsn AS lag_bytes
  FROM pg_stat_subscription;"

# 3. Promote DR PostgreSQL (it is already a logical subscriber; cut over by
#    pointing services at the DR primary and disabling the subscription)
psql $DR_DATABASE_URL -c "ALTER SUBSCRIPTION sdkwork_im_dr_sub DISABLE;"

# 4. Scale the DR microservice fleet to production capacity
kubectl --context=dr-context -n sdkwork-im scale deployment im-gateway --replicas=3
# ... repeat for each service per §6.5

# 5. Switch DNS / ingress to the DR region
#    Update the global load balancer or DNS record (TTL <= 60s)
scripts/dr-switch-dns.sh --to dr-region

# 6. Update SDKWORK_IM_DATABASE_URL and SDKWORK_IM_REDIS_CLUSTER_NODES to DR endpoints
#    via the configmap rollout
kubectl --context=dr-context -n sdkwork-im rollout restart deployment
```

### 7.4 Phase D — Verification

Failover is not complete until every gate passes. A failed gate blocks customer traffic from being
declared restored.

| Gate | Verification | Pass criteria |
| --- | --- | --- |
| Health | `curl https://<dr-ingress>/healthz` and `/readyz` | HTTP 200 |
| Auth | Login + token validation | Success |
| Write path | Send a test message in a sandbox tenant | Message appended to `im_commit_journal` in DR |
| Realtime | WebSocket handshake + `auth.init` | Connection stable for 60s |
| Read path | Fetch inbox + timeline for sandbox tenant | Returns expected data |
| Replication lag | `pg_stat_subscription` lag (before disable) | < 5 MB backlog |
| Data consistency | Projection checksum vs journal cursor | Match |
| Monitoring | Prometheus scraping DR fleet | All targets up |

### 7.5 Phase E — Fallback (Planned Switch-Back)

Fallback is **planned and scheduled**, never rushed. The repaired primary region must be
re-synced and verified before traffic returns.

1. Repair the primary region's PostgreSQL and Kubernetes cluster.
2. Reverse the replication: primary region becomes the logical subscriber to DR.
3. Wait for lag to drain to zero.
4. Schedule a maintenance window (announced ≥ 48 hours in advance per `SLA_SLO.md` §3.2).
5. Repeat §7.3 in reverse: stop DR writes, promote primary, switch DNS, verify.
6. Confirm DR region is back to standby role.

Fallback is optional if the DR region is performing within SLO. Some incidents may justify
leaving traffic in the DR region until the next planned maintenance window.

## 8. Data Replication Architecture

### 8.1 Replication Topology

```
Primary Region                          DR Region (Standby)
─────────────────                       ───────────────────
                                         ┌──────────────────┐
┌──────────────────┐   logical rep      │ PostgreSQL       │
│ PostgreSQL       │ ─────────────────▶ │ (subscriber,     │
│ (primary,        │   publication/     │  promotable)     │
│  publisher)      │   subscription     └──────────────────┘
└──────────────────┘                            │
        │                                       │ projection
        │ same-tx write                         │ rebuild
        ▼                                       ▼
┌──────────────────┐                     ┌──────────────────┐
│ im_commit_journal│                     │ im_commit_journal│
│ im_outbox_events │                     │ im_outbox_events │
└──────────────────┘                     └──────────────────┘

┌──────────────────┐                     ┌──────────────────┐
│ Redis cluster    │   no replication    │ Redis cluster    │
│ (primary)        │   (cache, rebuild)  │ (standby, cold)  │
└──────────────────┘                     └──────────────────┘

┌──────────────────┐   cross-region      ┌──────────────────┐
│ SDKWork Drive    │ ◀────────────────▶ │ SDKWork Drive    │
│ (object storage) │   replication       │ (object storage) │
└──────────────────┘                     └──────────────────┘
```

### 8.2 PostgreSQL — Logical Replication

- **Method**: publication/subscription (see §6.2).
- **What is replicated**: `im_commit_journal`, `im_outbox_events`, and all projection tables
  listed in the publication. Schema changes (DDL) are **not** replicated by logical replication;
  migrations must be applied to both regions in a coordinated rollout.
- **Consistency**: eventual. The DR region may lag by seconds. Reads from DR during steady state
  are not served to clients, so the lag is invisible.
- **Conflict resolution**: none. The DR region is read-only as a subscriber; conflicts cannot
  arise unless someone writes to it directly, which is forbidden by the runbook.

### 8.3 Event Journal — Dual-Write vs CDC

Two options were evaluated for the journal replication path:

| Option | Mechanism | Pros | Cons | Decision |
| --- | --- | --- | --- | --- |
| Dual-write | Application writes to both regions in the same transaction | Strong consistency possible | Adds write latency; partial failure risk | Rejected for Phase 2 |
| CDC pipeline | Debezium-style capture from WAL, stream to DR | Decouples app from replication; survives app outage | Operational complexity; extra component | Roadmap for Phase 3 |
| PostgreSQL logical replication | Native pub/sub | No extra components; well-supported | Schema-bound; DDL not replicated | **Selected for Phase 2** |

Phase 2 uses native PostgreSQL logical replication. The CDC pipeline is deferred to Phase 3
because active-active write coordination will require event-level semantics that logical
replication alone cannot provide.

### 8.4 Consistency Model

| Data | Steady-state consistency | After failover |
| --- | --- | --- |
| `im_commit_journal` | Eventual (seconds) | Last replicated event is the source of truth; unreplicated events lost (within RPO) |
| Projections | Eventual (rebuild from journal) | Rebuilt on promotion if stale |
| Redis session state | None (independent clusters) | Lost; clients re-authenticate |
| Object storage | Eventual (< 1 min) | Objects present if replication completed |
| Audit logs | Eventual (chain verifiable) | DR region continues the chain; verify with `RUNBOOK-audit-log-investigation.md` |

## 9. DR Testing and Drills

A DR plan that is never tested will fail when invoked. Drills are mandatory and scheduled.

### 9.1 Drill Cadence

| Drill | Frequency | Scope | Owner | Pass criteria |
| --- | --- | --- | --- | --- |
| Tabletop exercise | Monthly | Walkthrough of §7 failover with the on-call team | SRE lead | All steps understood; gaps logged |
| Partial failover | Quarterly | Fail over one non-critical service to DR | SRE on-call | Service recovers within RTO |
| Full failover test | Quarterly | Fail over the entire platform to DR in staging | SRE lead | §7.4 gates pass; RTO/RPO met |
| Full DR simulation | Annually | Fail over production to DR during a maintenance window | Engineering leadership | Production traffic served from DR for ≥ 1 hour |
| PITR restore test | Quarterly | Restore PostgreSQL to a target time in staging | SRE on-call | Restored data matches checksum |

### 9.2 Drill Records

Each drill produces a record stored in `docs/operations/dr-drills/` with:

- Date, participants, scenario.
- Steps executed and timings.
- Gates passed/failed.
- Action items with owners and due dates.
- Evidence (command output, Prometheus screenshots, gate checklists).

A drill that fails any §7.4 gate triggers a mandatory follow-up within 2 weeks.

### 9.3 Continuous Replication Health

Replication health is monitored continuously, not just during drills. The following alerts are
defined in `deployments/observability/prometheus-rules.yaml`:

```yaml
- alert: DRReplicationLagHigh
  expr: pg_replication_lag_seconds > 60
  for: 5m
  labels:
    severity: critical
  annotations:
    summary: "Cross-region DR replication lag is high"
    description: "Lag is {{ $value }}s; RPO at risk"

- alert: DRSubscriptionDown
  expr: pg_subscription_enabled == 0
  for: 5m
  labels:
    severity: critical
  annotations:
    summary: "DR PostgreSQL subscription is disabled"
    description: "DR region is not receiving replication; RPO unbounded"
```

## 10. Communication Plan

### 10.1 Internal Communication

| Audience | Channel | Trigger | Latency |
| --- | --- | --- | --- |
| On-call SRE + engineer | PagerDuty | Any P0 incident | Immediate |
| Engineering leadership | Incident Slack channel | P0 confirmed | < 5 min |
| All engineers | #incidents Slack | Regional failover initiated | < 10 min |
| Executive team | Email + Slack | Regional failover confirmed | < 15 min |
| Customer success | Slack #cs-alerts | Any customer-visible degradation | < 10 min |

The incident commander (SRE lead or designated backup) owns all internal communication. Status
updates are posted to #incidents every 15 minutes during an active incident, even if the update
is "no change".

### 10.2 Customer Communication

| Audience | Channel | Trigger | Content |
| --- | --- | --- | --- |
| Status page | status.sdkwork.com | Confirmed incident affecting any tier | Title, scope, severity, next update time |
| Tenant admins | Email | Enterprise tier incident | Impact, ETA, mitigation |
| All tenants | Status page | Regional failover | Acknowledgement + ongoing updates |
| Affected tenants | Email + in-app banner | Failover complete | Recovery confirmation, expected reconnect behavior |

Status page updates follow the cadence: first post within 10 minutes of confirmation, updates
every 30 minutes during the incident, a resolution post within 30 minutes of recovery, and a
post-mortem within 5 business days.

### 10.3 Post-Incident Review

Every regional failover, every RTO/RPO breach, and every drill failure triggers a post-incident
review (PIR) within 5 business days. The PIR is blameless and produces:

- Timeline of detection, decision, execution, verification.
- Root cause analysis.
- What went well / what went wrong.
- Action items with owners and due dates, tracked to closure.

PIRs are stored in `docs/operations/post-incidents/` and linked from the status page resolution
post for transparency with Enterprise customers.

## 11. Recovery Checklist

These checklists are the operational runbook for each scenario. They are designed to be executed
in order, top to bottom, without skipping steps.

### 11.1 Single Pod Crash (§4.1)

```markdown
# Runbook: Single Pod Crash

## Detect
- [ ] Identify failing service from Prometheus alert
- [ ] Confirm scope: only one service affected (else escalate to §4.2/§4.3)

## Recover
- [ ] Check pod status: `kubectl -n sdkwork-im get pods -l app.kubernetes.io/name=<svc>`
- [ ] Check restart count: `kubectl -n sdkwork-im describe pod <pod>`
- [ ] If CrashLoopBackOff: inspect logs `kubectl logs <pod> --previous`
- [ ] If OOMKilled: increase memory limit, redeploy
- [ ] Verify rollout: `kubectl -n sdkwork-im rollout status deployment/<svc>`

## Verify
- [ ] `/healthz` returns 200
- [ ] `/readyz` returns 200
- [ ] No new alerts for 10 minutes
- [ ] Log recovery in #incidents
```

### 11.2 PostgreSQL Primary Down (§4.2)

```markdown
# Runbook: PostgreSQL Primary Down

## Detect
- [ ] `PostgreSQLDown` alert fired (pg_up == 0 for 1 min)
- [ ] Confirm: `psql $SDKWORK_IM_DATABASE_URL -c "SELECT 1"` fails
- [ ] Check scope: is Redis also down? (if yes, suspect network/host — escalate to §4.3)

## Decide
- [ ] Identify most up-to-date replica: `SELECT * FROM pg_stat_replication`
- [ ] Confirm replica is healthy and accepting reads
- [ ] Notify incident commander

## Recover
- [ ] Promote replica (operator: `kubectl cnpg promote <cluster> <replica>`
      or self-managed: `pg_ctl promote -D <data-dir>`)
- [ ] Update `SDKWORK_IM_DATABASE_URL` in configmap
- [ ] Roll services: `kubectl -n sdkwork-im rollout restart deployment`
- [ ] Reconfigure former primary as a replica of the new primary (after repair)

## Verify
- [ ] `/healthz` and `/readyz` return 200
- [ ] Test message send in sandbox tenant succeeds
- [ ] `pg_stat_replication` shows new replica catching up
- [ ] No data loss: compare last journal LSN with former primary's archived WAL
- [ ] Log recovery, file PIR if RTO was at risk
```

### 11.3 Region Failure (§4.3)

```markdown
# Runbook: Region Failure

## Detect
- [ ] Multiple P0 alerts across services in primary region
- [ ] Synthetic probe from DR region to primary fails 3x
- [ ] Cloud provider status page confirms regional incident
- [ ] Confirm DR region is healthy (it must be, to fail over)

## Decide (manual — engineering leadership)
- [ ] Page engineering leadership on-call
- [ ] Confirm primary region is truly unreachable (not a partition):
      - Probe from a third region or external monitor
      - Check cloud provider status
- [ ] If partition suspected: WAIT, do not failover (split-brain risk)
- [ ] If confirmed regional outage: authorise failover
- [ ] Record decision rationale in #incidents

## Execute (see §7.3 for commands)
- [ ] Stop write traffic in primary (if reachable): scale im-gateway to 0
- [ ] Check DR replication lag has drained: `pg_stat_subscription`
- [ ] Disable DR subscription: `ALTER SUBSCRIPTION sdkwork_im_dr_sub DISABLE;`
- [ ] Scale DR microservice fleet to production capacity (per §6.5)
- [ ] Switch DNS to DR ingress: `scripts/dr-switch-dns.sh --to dr-region`
- [ ] Rollout DR configmap with DR DB + Redis endpoints
- [ ] Notify customer success + status page owner

## Verify (per §7.4 gates — ALL must pass)
- [ ] Health gate: /healthz, /readyz return 200
- [ ] Auth gate: login + token validation succeeds
- [ ] Write gate: test message appended to im_commit_journal in DR
- [ ] Realtime gate: WebSocket handshake stable for 60s
- [ ] Read gate: inbox + timeline return expected data
- [ ] Consistency gate: projection checksum matches journal cursor
- [ ] Monitoring gate: Prometheus scraping all DR targets

## Communicate
- [ ] Status page: "Service restored in DR region"
- [ ] Tenant admins: email with reconnect guidance
- [ ] #incidents: recovery confirmed, PIR scheduled

## Fallback (planned, NOT immediate)
- [ ] Repair primary region
- [ ] Reverse replication (primary subscribes to DR)
- [ ] Wait for lag to drain
- [ ] Schedule maintenance window (≥48h notice)
- [ ] Repeat execution in reverse
```

### 11.4 Data Corruption (§4.4)

```markdown
# Runbook: Data Corruption

## Detect
- [ ] Audit log anomaly or projection checksum mismatch
- [ ] User reports of missing/wrong messages
- [ ] Identify corrupted table and time window

## Contain
- [ ] Stop write traffic: scale im-gateway to 0
- [ ] Preserve evidence: snapshot current DB and WAL before any restore
- [ ] Identify target recovery time (PITR target)

## Recover
- [ ] Restore from base backup + WAL to target time:
      `scripts/restore-pitr.sh --target-time "<timestamp>"`
- [ ] Replay journal into projections: `scripts/rebuild-projections.sh`
- [ ] Verify data integrity: checksums, row counts, sample queries

## Verify
- [ ] Corrupted rows are correct
- [ ] No unrelated data lost (compare row counts)
- [ ] Write path functional: test message send
- [ ] Read path functional: inbox + timeline

## Restore traffic
- [ ] Scale im-gateway back to production replicas
- [ ] Monitor for 30 minutes
- [ ] File PIR: root cause of corruption, prevention measures
```

## 12. References

- [OPERATIONS_MANUAL.md](OPERATIONS_MANUAL.md) — Operations manual; §3 fault handling, §6 backup
  and recovery, §6.4 recovery verification checklist.
- [SLA_SLO.md](../product/compliance/SLA_SLO.md) — Service level agreements; §7 recovery
  objectives (Enterprise RTO 30 min / RPO 5 min), §6 error budget policy, §8 SLA credit policy.
- [COMPLIANCE_FRAMEWORK.md](../product/compliance/COMPLIANCE_FRAMEWORK.md) — Regulatory
  compliance; §4 data residency (cross-region replication opt-in), §3 data classification and
  retention, §5 data subject rights (erasure implications for DR).
- [CUSTOMER_OPERATIONS.md](../product/compliance/CUSTOMER_OPERATIONS.md) — Customer-facing
  operations guide.
- [DATA_PROTECTION.md](../product/compliance/DATA_PROTECTION.md) — Data protection summary.
- [TECH_ARCHITECTURE.md](../architecture/tech/TECH_ARCHITECTURE.md) — Technical architecture;
  CQRS + Event Sourcing, `im_commit_journal`, microservice boundaries.
- `configs/topology/cloud.split-services.production.env` — Regional deployment topology.
- `deployments/observability/prometheus-rules.yaml` — Alert rules including DR replication lag.
- `deployments/kubernetes/cloud-split-services/` — Per-service deployment manifests (15 services).
- `docs/runbooks/RUNBOOK-audit-log-investigation.md` — Audit log continuity verification after
  failover.
- `docs/runbooks/RUNBOOK-provider-outage.md` — Provider outage response (upstream cloud
  provider incidents).

---

**Document maintenance**: SDKWork SRE team
**Review cadence**: quarterly, plus after every regional failover or drill
**Next scheduled review**: 2026-10-03
