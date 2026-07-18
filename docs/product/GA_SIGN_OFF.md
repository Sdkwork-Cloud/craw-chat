# Sdkwork IM — General Availability (GA) Readiness Sign-Off

Status: active
Owner: SDKWork maintainers
Updated: 2026-07-05
Specs: API_SPEC.md, SECURITY_SPEC.md, PRIVACY_SPEC.md, IAM_SPEC.md, OBSERVABILITY_SPEC.md, DEPLOYMENT_SPEC.md, RELEASE_SPEC.md

## 1. Purpose

This document defines the General Availability (GA) readiness criteria for the Sdkwork IM
multi-tenant real-time messaging platform and tracks sign-off status across functional,
security, reliability, performance, operability, and commercial dimensions.

It is the authoritative gating artifact for promoting a platform phase from
"development / pre-release" to "generally available." No phase is declared GA until every
required checklist item for that phase is signed off by the designated owner with linked
evidence.

This document supplements:

- `docs/product/prd/PRD.md` — product scope and requirements.
- `docs/product/compliance/SLA_SLO.md` — availability, latency, and error budget targets.
- `docs/product/compliance/COMPLIANCE_FRAMEWORK.md` — regulatory and data protection controls.
- `docs/product/pricing/BILLING_METERING.md` — commercial metering and billing architecture.
- `docs/operations/OPERATIONS_MANUAL.md` — runbooks, deployment, and incident response.

### 1.1 Status Legend

| Symbol | Meaning |
| --- | --- |
| ✅ | Complete and verified with evidence. |
| 🟡 | In progress; not yet ready for sign-off. |
| ❌ | Not started; on roadmap for a later phase. |

## 2. GA Readiness Definition

"General Availability" for Sdkwork IM means that the platform meets all of the following
conditions for the target phase:

1. **Functional completeness**: All in-scope core IM features for the phase are implemented,
   tested, and documented.
2. **Production hardening**: P0 and P1 defects are resolved; P2 hardening items that affect
   the in-scope surfaces are complete.
3. **Security baseline**: Authentication, authorization, tenant isolation, transport
   encryption, and at-rest encryption are enforced in production configuration (not just
   development defaults).
4. **Reliability commitment**: SLA/SLO targets are published, monitoring and alerting are
   wired to those targets, and error budgets are operational.
5. **Operability**: Deployment, rollback, runbooks, and on-call procedures exist and have
   been exercised at least once in a staging or production-equivalent environment.
6. **Commercial readiness**: Billing, metering, support tiering, and customer-facing
   documentation are in place for the editions being launched.
7. **Compliance posture**: Regulatory alignment (GDPR, PIPL, CCPA/CPRA, LGPD) is documented
   and evidence collection is operational for the data classes in scope.
8. **Sign-off**: Each readiness category owner has signed off in section 9 with linked
   evidence.

A phase that does not meet all eight conditions is not GA and must not be advertised,
contracted, or invoiced as such.

## 3. Readiness Categories

### 3.1 Functional Completeness

Core IM features are implemented, covered by automated tests, and verified against the
product requirements in `docs/product/prd/PRD.md`.

| Capability | Phase 1 | Phase 2 | Phase 3 |
| --- | --- | --- | --- |
| Send / receive text messages | ✅ | ✅ | ✅ |
| Message edit and recall | ✅ | ✅ | ✅ |
| Conversation CRUD | ✅ | ✅ | ✅ |
| Realtime WebSocket (CCP) delivery | ✅ | ✅ | ✅ |
| Multi-tenant isolation | ✅ | ✅ | ✅ |
| Read receipts and typing indicators | ✅ | ✅ | ✅ |
| File and media sharing (SDKWork Drive) | ✅ | ✅ | ✅ |
| Full-text search (Postgres FTI) | ✅ | ✅ | ✅ |
| Enterprise management features | ❌ | ✅ | ✅ |
| E2EE for direct chats | ❌ | ✅ | ✅ |
| E2EE for group chats and media | ❌ | ❌ | ✅ |

### 3.2 Security & Compliance

Security controls are enforced in production configuration, not merely available as
defaults. Compliance posture is documented and evidence is collectible on demand.

| Control | Phase 1 | Phase 2 | Phase 3 |
| --- | --- | --- | --- |
| JWT signature verification enforced | ✅ | ✅ | ✅ |
| TLS 1.2+ for HTTP and WebSocket | ✅ | ✅ | ✅ |
| mTLS for internal RPC | ✅ | ✅ | ✅ |
| At-rest encryption (PostgreSQL / disk) | ✅ | ✅ | ✅ |
| Field-level encryption for credentials | ✅ | ✅ | ✅ |
| Multi-tenant isolation (IAM, schema, projection) | ✅ | ✅ | ✅ |
| Audit logging with hash chain | ✅ | ✅ | ✅ |
| Audit PostgreSQL persistence | ❌ | ✅ | ✅ |
| HTML sanitization (XSS defense) | ✅ | ✅ | ✅ |
| Dev-mode guards (no dev defaults in prod) | ✅ | ✅ | ✅ |
| Defense-in-depth input validation | ✅ | ✅ | ✅ |
| GDPR / PIPL / CCPA / LGPD alignment | ✅ | ✅ | ✅ |
| SOC 2 Type II certification | ❌ | ❌ | 🟡 |
| ISO 27001 certification | ❌ | ❌ | 🟡 |

### 3.3 Reliability & HA

Reliability targets are defined in `docs/product/compliance/SLA_SLO.md`. HA capabilities
scale with phase.

| Capability | Phase 1 | Phase 2 | Phase 3 |
| --- | --- | --- | --- |
| SLA / SLO published | ✅ | ✅ | ✅ |
| Error budgets operational | ✅ | ✅ | ✅ |
| Prometheus + Grafana monitoring | ✅ | ✅ | ✅ |
| Alertmanager + PagerDuty wiring | ✅ | ✅ | ✅ |
| Single-node standalone / cloud soft-HA | ✅ | ✅ | ✅ |
| Multi-region active-passive DR | ❌ | ✅ | ✅ |
| Multi-node cluster HA | ❌ | ❌ | ✅ |
| Active-active multi-region | ❌ | ❌ | ✅ |

### 3.4 Performance & Scale

Capacity baselines are defined in `docs/product/compliance/SLA_SLO.md` section 11.

| Target | Phase 1 | Phase 2 | Phase 3 |
| --- | --- | --- | --- |
| Single-instance baseline (1k WS, 6k msg/min) | ✅ | ✅ | ✅ |
| API P99 latency within SLO | ✅ | ✅ | ✅ |
| WebSocket delivery P99 within SLO | ✅ | ✅ | ✅ |
| 5-node cluster baseline (5k users) | ❌ | 🟡 | ✅ |
| 20-node cluster baseline (20k users) | ❌ | ❌ | ✅ |
| Rate limiting and quota enforcement | 🟡 | 🟡 | ✅ |

Phase 1 client surfaces use server-side pagination (`list*Page`, bounded `forEachCursorPage` sync) per `PAGINATION_SPEC.md`; platform-wide rate limiting remains Phase 2.

### 3.5 Operability

Operability covers deployment, rollback, runbooks, and on-call procedures.

| Capability | Phase 1 | Phase 2 | Phase 3 |
| --- | --- | --- | --- |
| K8s deployment with versioned image tags | ✅ | ✅ | ✅ |
| K8s ConfigMaps for non-secret config | ✅ | ✅ | ✅ |
| Operations manual | ✅ | ✅ | ✅ |
| Runbooks (audit, isolation, token rotation, provider outage, migration rollback) | ✅ | ✅ | ✅ |
| Structured JSON logging | ✅ | ✅ | ✅ |
| OpenTelemetry tracing | ✅ | ✅ | ✅ |
| Backup and restore procedures exercised | ✅ | ✅ | ✅ |
| Incident management and post-incident review process | ✅ | ✅ | ✅ |

### 3.6 Commercial Readiness

Commercial readiness covers billing, metering, support, and customer-facing documentation.

| Capability | Phase 1 | Phase 2 | Phase 3 |
| --- | --- | --- | --- |
| Billing & metering architecture | ✅ | ✅ | ✅ |
| Metering pipeline (durable PostgreSQL store) | ✅ | ✅ | ✅ |
| Quota enforcement (concurrent users, storage, API) | 🟡 | 🟡 | ✅ |
| Overage billing | ✅ | ✅ | ✅ |
| Tenant lifecycle (trial, convert, suspend, cancel) | ✅ | ✅ | ✅ |
| SLA credit policy | ✅ | ✅ | ✅ |
| Support tier definitions (Community / Professional / Enterprise) | ✅ | ✅ | ✅ |
| SDK documentation site | ✅ | ✅ | ✅ |
| API reference documentation | ✅ | ✅ | ✅ |

## 4. Sign-off Checklist

Each item below must be signed off by its owner with linked evidence before the
corresponding phase is declared GA. Phase 1 target GA date is 2026-07-31; Phase 2 targets
2026 Q4; Phase 3 targets 2027 Q1.

### 4.1 Phase 1 Sign-off Checklist (MVP commercial launch)

| # | Item | Status | Owner | Target | Evidence |
| --- | --- | --- | --- | --- | --- |
| 1 | Core messaging (send / receive / edit / recall) | ✅ | IM backend | 2026-07-31 | [PRD.md](prd/PRD.md), `services/conversation-runtime/` |
| 2 | Realtime WebSocket (CCP) | ✅ | IM backend | 2026-07-31 | `services/session-gateway/`, [OPERATIONS_MANUAL.md](../operations/OPERATIONS_MANUAL.md) |
| 3 | Multi-tenant isolation | ✅ | IAM / platform | 2026-07-31 | [RUNBOOK-tenant-isolation-verification.md](../runbooks/RUNBOOK-tenant-isolation-verification.md) |
| 4 | JWT authentication (signature verification enforced) | ✅ | IAM / platform | 2026-07-31 | `SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE=true`, [COMPLIANCE_FRAMEWORK.md](compliance/COMPLIANCE_FRAMEWORK.md) §8 |
| 5 | TLS encryption (in transit) | ✅ | Platform ops | 2026-07-31 | [SLA_SLO.md](compliance/SLA_SLO.md), [DATA_PROTECTION.md](compliance/DATA_PROTECTION.md) |
| 6 | At-rest encryption | ✅ | Platform ops | 2026-07-31 | [COMPLIANCE_FRAMEWORK.md](compliance/COMPLIANCE_FRAMEWORK.md) §3.2 |
| 7 | API response envelope (`SdkWorkApiResponse`) | ✅ | API platform | 2026-07-31 | `../sdkwork-specs/API_SPEC.md` §4.5, `apis/` |
| 8 | Error handling (`ProblemDetail`, 4xx/5xx) | ✅ | API platform | 2026-07-31 | `../sdkwork-specs/API_SPEC.md` §14–15 |
| 9 | Monitoring (Prometheus / Grafana) | ✅ | Platform ops | 2026-07-31 | `deployments/observability/`, [SLA_SLO.md](compliance/SLA_SLO.md) §9 |
| 10 | Operations manual | ✅ | Platform ops | 2026-07-31 | [OPERATIONS_MANUAL.md](../operations/OPERATIONS_MANUAL.md) |
| 11 | SLA / SLO documentation | ✅ | Platform ops | 2026-07-31 | [SLA_SLO.md](compliance/SLA_SLO.md) |
| 12 | Compliance framework | ✅ | Compliance | 2026-07-31 | [COMPLIANCE_FRAMEWORK.md](compliance/COMPLIANCE_FRAMEWORK.md) |
| 13 | Billing / metering architecture | ✅ | Commercial | 2026-07-31 | [BILLING_METERING.md](pricing/BILLING_METERING.md) |
| 14 | K8s deployment (versioned image tags, ConfigMaps) | ✅ | Platform ops | 2026-07-31 | `deployments/`, `etc/topology/` |
| 15 | Frontend error boundaries | ✅ | Frontend | 2026-07-31 | `apps/sdkwork-im-pc/` |
| 16 | Skeleton screens | ✅ | Frontend | 2026-07-31 | `apps/sdkwork-im-pc/` |
| 17 | E2EE | ❌ | Security | Phase 2 | Roadmap §6 |
| 18 | Multi-region DR | ❌ | Platform ops | Phase 2 | Roadmap §7 |
| 19 | SOC 2 certification | ❌ | Compliance | Roadmap | [COMPLIANCE_FRAMEWORK.md](compliance/COMPLIANCE_FRAMEWORK.md) §2.1 |

### 4.2 Phase 2 Sign-off Checklist (enterprise launch — 2026 Q4)

| # | Item | Status | Owner | Target | Evidence |
| --- | --- | --- | --- | --- | --- |
| 1 | E2EE for direct chats | 🟡 | Security | 2026 Q4 | Roadmap, [DATA_PROTECTION.md](compliance/DATA_PROTECTION.md) |
| 2 | Multi-region active-passive DR | 🟡 | Platform ops | 2026 Q4 | `etc/topology/`, [SLA_SLO.md](compliance/SLA_SLO.md) §7 |
| 3 | Audit PostgreSQL persistence | 🟡 | Compliance | 2026 Q4 | [COMPLIANCE_FRAMEWORK.md](compliance/COMPLIANCE_FRAMEWORK.md) §6 |
| 4 | Enterprise management features | 🟡 | IM backend | 2026 Q4 | [PRD.md](prd/PRD.md), [roadmap/README.md](roadmap/README.md) |
| 5 | Message recall / edit tombstone propagation | ✅ | IM backend | 2026 Q4 | `services/conversation-runtime/` |
| 6 | @mention, reactions, pins, threads | 🟡 | IM backend | 2026 Q4 | [roadmap/README.md](roadmap/README.md) |

### 4.3 Phase 3 Sign-off Checklist (scale launch — 2027 Q1)

| # | Item | Status | Owner | Target | Evidence |
| --- | --- | --- | --- | --- | --- |
| 1 | Multi-node cluster HA | ❌ | Platform ops | 2027 Q1 | `etc/topology/`, [SLA_SLO.md](compliance/SLA_SLO.md) §11.2 |
| 2 | Active-active multi-region | ❌ | Platform ops | 2027 Q1 | `etc/topology/` |
| 3 | Rate limiting and quota enforcement | ❌ | API platform | 2027 Q1 | [BILLING_METERING.md](pricing/BILLING_METERING.md) §5, `../sdkwork-specs/SECURITY_SPEC.md` |
| 4 | E2EE for group chats and media | ❌ | Security | 2027 Q1 | [DATA_PROTECTION.md](compliance/DATA_PROTECTION.md) |
| 5 | SOC 2 Type II certification | 🟡 | Compliance | 2027 Q1 | [COMPLIANCE_FRAMEWORK.md](compliance/COMPLIANCE_FRAMEWORK.md) §2.1 |
| 6 | ISO 27001 certification | 🟡 | Compliance | 2027 Q1 | [COMPLIANCE_FRAMEWORK.md](compliance/COMPLIANCE_FRAMEWORK.md) §2.1 |

## 5. Phase 1 GA Criteria (MVP commercial launch)

Phase 1 is the current GA target. It defines the minimum commercial launch surface for
Sdkwork IM as a multi-tenant real-time messaging platform. As of 2026-07-03, all required
Phase 1 items are complete except the three explicitly deferred to later phases.

### 5.1 Completed Phase 1 Items

- [x] Core messaging (send / receive / edit / recall)
- [x] Realtime WebSocket (CCP)
- [x] Multi-tenant isolation
- [x] JWT authentication (signature verification enforced in production)
- [x] TLS encryption (in transit)
- [x] At-rest encryption
- [x] API response envelope (`SdkWorkApiResponse` with `code: 0`, `data`, `traceId`)
- [x] Error handling (`ProblemDetail` with numeric `code`, `traceId`, route-template `instance`, `operationId`, and `i18nKey`, HTTP 4xx/5xx)
- [x] Monitoring (Prometheus / Grafana with SLO dashboards)
- [x] Operations manual
- [x] SLA / SLO documentation
- [x] Compliance framework
- [x] Billing / metering architecture
- [x] K8s deployment (versioned image tags, ConfigMaps)
- [x] Frontend error boundaries
- [x] Skeleton screens

### 5.2 Completed Hardening (P0 / P1 / P2)

The following hardening work was completed prior to Phase 1 GA and is required for sign-off:

- **P0 critical bug fixes**: deadlocks, error mapping, fake-success response paths.
- **P1 important fixes**: error envelopes, Snowflake ID generation, session expiry, dev-mode
  guards preventing development defaults from leaking into production.
- **P2 improvements**: Rust 2024 safety lints, mutex poison recovery, defense-in-depth input
  validation, HTML sanitization for XSS defense, K8s versioned image tags, frontend error
  boundaries, skeleton screens, K8s ConfigMaps for non-secret configuration.
- **Documentation**: SLA/SLO, compliance framework, billing/metering architecture.

### 5.3 Phase 1 Deferred Items (Not Blocking Phase 1 GA)

The following items are explicitly out of scope for Phase 1 GA and are tracked under
Phase 2 / Phase 3:

- [ ] E2EE — Phase 2 roadmap.
- [ ] Multi-region disaster recovery — Phase 2 roadmap.
- [ ] SOC 2 certification — roadmap (Phase 3).

These items must be disclosed to Phase 1 customers as roadmap commitments, not as
generally available capabilities.

## 6. Phase 2 GA Criteria (enterprise launch — 2026 Q4)

Phase 2 extends the platform to enterprise readiness with end-to-end confidentiality,
regional resilience, and durable audit persistence.

- [ ] E2EE for direct chats
- [ ] Multi-region active-passive DR
- [ ] Audit PostgreSQL persistence
- [ ] Enterprise management features

Phase 2 GA requires all Phase 1 GA items to remain signed off and all Phase 2 checklist
items in section 4.2 to be complete with evidence.

## 7. Phase 3 GA Criteria (scale launch — 2027 Q1)

Phase 3 extends the platform to scale and enterprise security certifications.

- [ ] Multi-node cluster HA
- [ ] Active-active multi-region
- [ ] Rate limiting and quota enforcement
- [ ] E2EE for group chats and media

Phase 3 GA requires all Phase 1 and Phase 2 GA items to remain signed off and all Phase 3
checklist items in section 4.3 to be complete with evidence.

## 8. Risk Assessment

| # | Risk | Likelihood | Impact | Mitigation | Owner |
| --- | --- | --- | --- | --- | --- |
| 1 | E2EE slips past Phase 2, blocking enterprise deals | Medium | High | Begin protocol design in Phase 1 Q3; prototype key management against KMS before Phase 2 GA | Security |
| 2 | Single-node topology cannot meet Enterprise 99.9% SLA | High | High | Phase 1 Enterprise tier is sold with explicit single-node caveat; Phase 3 multi-node HA is the SLA-enabling release | Platform ops |
| 3 | Audit log durability insufficient for regulated customers | Medium | High | Phase 1 uses hash-chained audit logs in PostgreSQL; Phase 2 adds dedicated audit PostgreSQL persistence and S3 object-lock evidence store | Compliance |
| 4 | Rate limiting absent exposes platform to abuse at scale | Medium | High | Phase 1 relies on per-IP gateway limits and tenant quota counters; Phase 3 adds full per-tenant quota enforcement with 429 responses | API platform |
| 5 | Multi-region DR not available for Phase 1 / Phase 2 active region | Medium | High | Phase 1 documents single-region RTO/RPO; Phase 2 delivers active-passive DR with WAL replication | Platform ops |
| 6 | SOC 2 / ISO 27001 not yet certified blocks regulated procurement | High | Medium | Phase 1 ships with documented controls alignment; certification engagement kicks off in Phase 2 with target completion in Phase 3 | Compliance |
| 7 | K8s deployment drift between environments | Low | Medium | Versioned image tags and ConfigMaps enforced; topology profiles in `etc/topology/` are the single source of truth | Platform ops |
| 8 | Frontend regressions on error surfaces | Low | Medium | Error boundaries and skeleton screens ship in Phase 1; regression coverage added to frontend CI | Frontend |
| 9 | JWT signing key compromise | Low | Critical | 90-day rotation runbook enforced; break-glass revocation procedure documented | IAM / platform |
| 10 | Tenant isolation bypass via projection layer | Low | Critical | Tenant isolation verification runbook exercised before each release; defense-in-depth validation at IAM, schema, and AppContext layers | IAM / platform |

## 9. Sign-off Process

### 9.1 Sign-off Roles

| Role | Responsibility | Phase 1 | Phase 2 | Phase 3 |
| --- | --- | --- | --- | --- |
| Engineering lead | Functional completeness and hardening sign-off | Required | Required | Required |
| Security lead | Security and compliance posture sign-off | Required | Required | Required |
| Platform ops lead | Reliability, HA, operability, deployment sign-off | Required | Required | Required |
| Commercial lead | Billing, metering, support, documentation sign-off | Required | Required | Required |
| Compliance lead | Regulatory alignment and evidence collection sign-off | Required | Required | Required |
| Product lead | Scope and customer-facing disclosure sign-off | Required | Required | Required |

### 9.2 Evidence Requirements

Each sign-off requires linked evidence from one of the following:

1. **Source artifact**: a service, crate, configuration, or deployment manifest under the
   repository (e.g., `services/session-gateway/`, `deployments/observability/`).
2. **Documentation artifact**: a document under `docs/` (e.g., `OPERATIONS_MANUAL.md`,
   `SLA_SLO.md`).
3. **Runbook artifact**: a runbook under `docs/runbooks/` that has been exercised at least
   once in staging or production-equivalent.
4. **Verification command output**: recorded output of a verification command such as
   `pnpm verify`, `cargo test --workspace`, or the API envelope checker
   (`node <sdkwork-specs>/tools/check-api-response-envelope.mjs --workspace <root>`).
5. **External evidence**: third-party assessment, penetration test report, or certification
   audit result stored in the compliance evidence store.

### 9.3 Sign-off Recording

Sign-off is recorded by appending to the corresponding checklist table in section 4 with:

- The `✅` status symbol.
- The owner name or team.
- The target date (or actual completion date if earlier).
- A relative link to the evidence artifact.

A phase is declared GA only when every required item for that phase shows `✅` and all six
sign-off roles in section 9.1 have approved the promotion in the release record under
`docs/changelogs/`.

### 9.4 Withdrawal of GA Status

GA status for a phase may be withdrawn when:

- A P0 defect is discovered in an in-scope surface that cannot be mitigated within the SLO
  error budget.
- A security control is found to be bypassable in production configuration.
- A regulatory alignment claim is found to be inaccurate.

Withdrawal requires a post-incident review under `docs/engineering/reviews/` and re-sign-off
under section 9.3 before the phase is re-declared GA.

## 10. References

### 10.1 Product and Requirements

- [docs/product/prd/PRD.md](prd/PRD.md) — Product requirements.
- [docs/product/roadmap/README.md](roadmap/README.md) — Phase roadmap.
- [docs/product/pricing/README.md](pricing/README.md) — Commercial pricing model.

### 10.2 Compliance and Reliability

- [docs/product/compliance/SLA_SLO.md](compliance/SLA_SLO.md) — SLA / SLO targets.
- [docs/product/compliance/COMPLIANCE_FRAMEWORK.md](compliance/COMPLIANCE_FRAMEWORK.md) — Regulatory compliance framework.
- [docs/product/compliance/DATA_PROTECTION.md](compliance/DATA_PROTECTION.md) — Data protection summary.
- [docs/product/compliance/CUSTOMER_OPERATIONS.md](compliance/CUSTOMER_OPERATIONS.md) — Customer operations guide.
- [docs/product/pricing/BILLING_METERING.md](pricing/BILLING_METERING.md) — Billing and metering architecture.

### 10.3 Operations and Runbooks

- [docs/operations/OPERATIONS_MANUAL.md](../operations/OPERATIONS_MANUAL.md) — Operations manual.
- [docs/runbooks/RUNBOOK-audit-log-investigation.md](../runbooks/RUNBOOK-audit-log-investigation.md)
- [docs/runbooks/RUNBOOK-tenant-isolation-verification.md](../runbooks/RUNBOOK-tenant-isolation-verification.md)
- [docs/runbooks/RUNBOOK-token-key-rotation.md](../runbooks/RUNBOOK-token-key-rotation.md)
- [docs/runbooks/RUNBOOK-provider-outage.md](../runbooks/RUNBOOK-provider-outage.md)
- [docs/runbooks/RUNBOOK-migration-rollback.md](../runbooks/RUNBOOK-migration-rollback.md)

### 10.4 Architecture and Engineering

- [docs/architecture/TECH_ARCHITECTURE.md](../architecture/TECH_ARCHITECTURE.md) — Technical architecture.
- [docs/architecture/decisions/](../architecture/decisions/) — Architecture decision records.
- [docs/CODE_REVIEW_AND_BUG_FIXES.md](../CODE_REVIEW_AND_BUG_FIXES.md) — Code review and bug fix report.
- [docs/COMMUNICATION_FEATURE_AUDIT_REPORT.md](../COMMUNICATION_FEATURE_AUDIT_REPORT.md) — Communication feature audit.

### 10.5 Platform Specs

- `../sdkwork-specs/API_SPEC.md` — API response envelope and error codes.
- `../sdkwork-specs/SECURITY_SPEC.md` — Platform security standard.
- `../sdkwork-specs/PRIVACY_SPEC.md` — Platform privacy standard.
- `../sdkwork-specs/IAM_SPEC.md` — Identity and access management standard.
- `../sdkwork-specs/OBSERVABILITY_SPEC.md` — Observability standard.
- `../sdkwork-specs/DEPLOYMENT_SPEC.md` — Deployment standard.
- `../sdkwork-specs/RELEASE_SPEC.md` — Release standard.

### 10.6 Configuration and Deployment

- `etc/topology/` — Deployment topology profiles.
- `specs/topology.spec.json` — Topology machine contract.
- `deployments/observability/` — Prometheus, Grafana, and Alertmanager configuration.
- `sdkwork.app.config.json` — IM application identity and capability metadata.
