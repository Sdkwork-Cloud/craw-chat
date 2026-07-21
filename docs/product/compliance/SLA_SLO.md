# Sdkwork IM — Service Level Agreements & Objectives

Status: active
Owner: SDKWork maintainers
Updated: 2026-07-21
Specs: OBSERVABILITY_SPEC.md, PRIVACY_SPEC.md, SECURITY_SPEC.md, PERFORMANCE_SPEC.md

## 1. Purpose

This document defines the Service Level Agreements (SLA) and Service Level Objectives (SLO)
for the Sdkwork IM platform. It covers availability targets, latency budgets, error rate
budgets, measurement methodology, error budget policy, and credit policy for SLA violations.

It supplements the operational summary in `CUSTOMER_OPERATIONS.md` and the monitoring
infrastructure described in `docs/operations/OPERATIONS_MANUAL.md`.

## 2. Service Tier Definitions

| Tier | Edition | Topology | HA | Target Availability | Support Response |
| --- | --- | --- | --- | --- | --- |
| Community | Community | Single-node standalone | No | Best-effort | Community only |
| Professional | Professional | Single-node cloud | Soft | 99.5% monthly | Business hours |
| Enterprise | Enterprise | Multi-node cluster | Yes | 99.9% monthly | 24/7, 1-hour P0 |

Topology profiles are defined in `etc/topology/` and `specs/topology.spec.json`.

## 3. Availability SLA

### 3.1 Service Surfaces

| Surface | Measurement | Professional | Enterprise |
| --- | --- | --- | --- |
| Application ingress (`/healthz`, `/readyz`) | HTTP 200 ratio | 99.5% | 99.9% |
| Realtime WebSocket plane | Successful handshake ratio | 99.5% | 99.9% |
| Conversation API (`/im/v3/api/*`) | HTTP 2xx + 4xx (excluding 5xx) ratio | 99.5% | 99.9% |
| Session-gateway RPC | gRPC OK ratio | 99.5% | 99.9% |
| Projection read API | HTTP 2xx + 4xx ratio | 99.5% | 99.9% |
| Media upload/download (SDKWork Drive) | HTTP 2xx ratio | 99.5% | 99.9% |

Push delivery is excluded from the current SLA. Device-token authority, provider workers, receipts,
retries, dead letters, and delivery metrics are not implemented; commercial release and any push
SLO remain blocked until direct production-like evidence exists.

### 3.2 Measurement Window

- **Monthly billing cycle**: calendar month, 00:00 UTC on the 1st through 23:59 UTC on the last day.
- **Sampling interval**: 15 seconds for HTTP health probes, 1 minute for synthetic API probes.
- **Exclusions**: planned maintenance windows announced at least 48 hours in advance, force
  majeure events, upstream cloud provider outages documented in the provider's status page, and
  client-side network failures outside Sdkwork-controlled infrastructure.

### 3.3 Measurement Methodology

Availability is measured as:

```
availability = (total_requests - failed_requests) / total_requests * 100
```

Where `failed_requests` are:
- HTTP 5xx responses from application services.
- WebSocket handshakes that fail before `auth.init` completes.
- gRPC responses with status codes `UNAVAILABLE`, `DEADLINE_EXCEEDED`, or `INTERNAL`.

Probe infrastructure: Prometheus scrape targets defined in
`deployments/observability/prometheus-rules.yaml` with alert rules in group
`sdkwork-im.availability`.

## 4. Latency SLO

### 4.1 API Latency Targets

| Operation | P50 | P95 | P99 | SLO Violation |
| --- | --- | --- | --- | --- |
| Send message (POST `/im/v3/api/messages`) | 50 ms | 150 ms | 300 ms | P99 > 500 ms for 5 min |
| Fetch inbox (GET `/im/v3/api/chat/inbox`) | 80 ms | 200 ms | 400 ms | P99 > 800 ms for 5 min |
| Fetch message history (GET `/im/v3/api/chat/conversations/{conversationId}/messages`) | 100 ms | 250 ms | 500 ms | P99 > 1000 ms for 5 min |
| Create conversation (POST `/im/v3/api/chat/conversations`) | 80 ms | 200 ms | 400 ms | P99 > 800 ms for 5 min |
| User authentication (POST `/im/v3/api/auth/*`) | 100 ms | 300 ms | 600 ms | P99 > 1000 ms for 5 min |
| File upload metadata (POST `/im/v3/api/media`) | 60 ms | 150 ms | 300 ms | P99 > 500 ms for 5 min |
| Search messages (GET `/im/v3/api/chat/messages/search`) | 200 ms | 500 ms | 1000 ms | P99 > 2000 ms for 5 min |

### 4.2 Realtime Latency Targets

| Operation | P50 | P95 | P99 | SLO Violation |
| --- | --- | --- | --- | --- |
| WebSocket message delivery (send to receive) | 100 ms | 300 ms | 600 ms | P99 > 1000 ms for 5 min |
| Presence update propagation | 200 ms | 500 ms | 1000 ms | P99 > 2000 ms for 5 min |
| Typing indicator propagation | 100 ms | 300 ms | 600 ms | P99 > 1000 ms for 5 min |
| Realtime event acknowledgement | 50 ms | 150 ms | 300 ms | P99 > 500 ms for 5 min |

### 4.3 Infrastructure Latency Targets

| Component | Operation | P95 | P99 |
| --- | --- | --- | --- |
| PostgreSQL | Read query | 20 ms | 50 ms |
| PostgreSQL | Write query | 30 ms | 80 ms |
| PostgreSQL | Replication lag | 1 s | 5 s |
| Redis | GET/SET | 2 ms | 5 ms |
| Redis | Cluster failover | 3 s | 10 s |

## 5. Error Rate SLO

### 5.1 Error Budget

| Tier | Availability Target | Monthly Error Budget | Calculation |
| --- | --- | --- | --- |
| Professional | 99.5% | 3.6 hours / month | (100 - 99.5) / 100 * 720 hours |
| Enterprise | 99.9% | 43.2 minutes / month | (100 - 99.9) / 100 * 720 hours |

### 5.2 Error Rate Thresholds

| Metric | Warning | Critical | Page |
| --- | --- | --- | --- |
| HTTP 5xx ratio (per service) | > 1% for 2 min | > 5% for 1 min | Critical |
| gRPC error ratio (per service) | > 1% for 2 min | > 5% for 1 min | Critical |
| WebSocket disconnect rate | > 10/min for 2 min | > 50/min for 1 min | Critical |
| Message delivery failure rate | > 0.5% for 5 min | > 2% for 2 min | Critical |

Alert rules are defined in `deployments/observability/prometheus-rules.yaml`.

## 6. Error Budget Policy

### 6.1 Budget Consumption

- The error budget is consumed by any request that fails to meet the availability or latency SLO.
- When the error budget is exhausted, feature development freezes and all engineering effort
  shifts to reliability improvements until the budget recovers.
- A budget burn rate alert fires when 10% of the monthly budget is consumed within 1 hour
  (fast burn) or 50% within 12 hours (slow burn).

### 6.2 Budget Recovery

- The error budget resets at the start of each monthly billing cycle.
- Unused budget does not carry over to the next cycle.
- If the budget is exhausted for 3 consecutive months, a formal reliability review is triggered.

### 6.3 Change Management

- When the error budget is below 25% remaining, non-emergency deployments require approval from
  the on-call engineering lead.
- When the error budget is exhausted, only security fixes and reliability improvements may be
  deployed.

## 7. Recovery Objectives

### 7.1 RTO and RPO

| Tier | RTO (Recovery Time Objective) | RPO (Recovery Point Objective) |
| --- | --- | --- |
| Community | Best-effort | Best-effort |
| Professional | 2 hours | 1 hour (WAL archival) |
| Enterprise | 30 minutes | 5 minutes (synchronous replication) |

### 7.2 Backup Strategy

- **Database**: Daily full backup (pg_dump), hourly WAL archival, 30-day retention.
- **Redis**: Hourly RDB snapshot, 24-hour retention. Redis is a cache; no RPO guarantee.
- **Configuration**: Daily backup, 30-day retention.
- **Media files**: Cross-region replication via SDKWork Drive, indefinite retention.

Backup scripts and restore procedures are documented in
`docs/operations/OPERATIONS_MANUAL.md` section 6.

## 8. SLA Credit Policy

### 8.1 Credit Schedule

| Availability | Enterprise Credit | Professional Credit |
| --- | --- | --- |
| < 99.9% but >= 99.0% | 10% of monthly fee | 5% of monthly fee |
| < 99.0% but >= 95.0% | 25% of monthly fee | 10% of monthly fee |
| < 95.0% | 50% of monthly fee | 25% of monthly fee |

### 8.2 Claim Process

1. Customer opens a support ticket within 30 days of the incident.
2. Sdkwork operations team verifies the downtime using Prometheus metrics and audit logs.
3. Credit is applied to the next billing cycle.
4. Credits are capped at 50% of the monthly fee per billing cycle.

### 8.3 Exclusions

- Downtime caused by customer misconfiguration or client-side issues.
- Downtime during planned maintenance windows announced at least 48 hours in advance.
- Force majeure events (natural disasters, government actions, upstream cloud provider outages).
- Downtime caused by the customer exceeding their allocated quota or rate limits.

## 9. Monitoring and Alerting

### 9.1 Monitoring Stack

- **Metrics**: Prometheus 2.40+ with 15-day retention.
- **Dashboards**: Grafana 9.0+ with pre-built SLO dashboards.
- **Alerting**: Alertmanager with PagerDuty integration for P0/P1, email for P2/P3.
- **Tracing**: OpenTelemetry Collector exporting to Jaeger or Tempo.
- **Logging**: Structured JSON logs shipped to Loki or ELK stack.

Configuration references:
- `deployments/observability/otel-collector.yaml`
- `deployments/observability/prometheus-rules.yaml`
- `deployments/observability/grafana-dashboard.json`

### 9.2 SLO Dashboard Panels

1. **Availability panel**: Real-time availability percentage per service surface.
2. **Latency panel**: P50/P95/P99 latency histograms per API endpoint.
3. **Error budget panel**: Remaining budget with burn rate indicator.
4. **Incident timeline**: Active and resolved incidents with MTTR tracking.

## 10. Incident Management

### 10.1 Severity Levels

| Severity | Criteria | Response Time | Escalation | Resolution Target |
| --- | --- | --- | --- | --- |
| P0 Critical | Service fully unavailable, data loss risk | 5 min (Enterprise) / 15 min (Professional) | CTO + on-call team | 30 min |
| P1 High | Core functionality impacted, severe degradation | 15 min (Enterprise) / 1 hour (Professional) | Tech lead | 2 hours |
| P2 Medium | Partial functionality impacted, minor degradation | 1 hour | Team lead | 4 hours |
| P3 Low | Non-critical issues, cosmetic defects | 4 hours | On-call engineer | 24 hours |

### 10.2 Post-Incident Review

Within 48 hours of a P0/P1 incident resolution:

1. **Timeline reconstruction**: Incident detection to resolution, with timestamps.
2. **Root cause analysis**: Technical root cause with contributing factors.
3. **Impact assessment**: User count, duration, data impact, revenue impact.
4. **Action items**: Preventive measures with owners and deadlines.
5. **Documentation**: Stored in `docs/engineering/reviews/` and linked from changelog.

## 11. Capacity and Performance Baselines

### 11.1 Single-Instance Baseline

| Metric | Value |
| --- | --- |
| Concurrent WebSocket connections | 1,000 |
| Messages per minute | 6,000 |
| API latency P99 | 100 ms |
| Database connections | 20 |

### 11.2 Cluster Baselines

| Cluster Size | Concurrent Users | Messages/min | API P99 |
| --- | --- | --- | --- |
| 5 nodes | 5,000 | 30,000 | 50 ms |
| 20 nodes | 20,000 | 120,000 | 30 ms |

Capacity planning formulas and scaling thresholds are documented in
`docs/operations/OPERATIONS_MANUAL.md` section 4.

## 12. Compliance Evidence

SLA compliance evidence is collected from:

- Prometheus time-series data (15-day retention for real-time analysis).
- Monthly availability reports generated from Prometheus query results.
- Audit logs stored in PostgreSQL with tamper-evident hash chains.
- Incident reports stored in `docs/engineering/reviews/`.

Customers on Enterprise tier may request a monthly SLA compliance report. The report includes:

1. Monthly availability percentage per service surface.
2. Incident summary with root cause and resolution time.
3. Error budget consumption summary.
4. Planned maintenance windows for the upcoming month.

## 13. References

- [CUSTOMER_OPERATIONS.md](CUSTOMER_OPERATIONS.md) — Operational summary and incident response.
- [DATA_PROTECTION.md](DATA_PROTECTION.md) — Data protection and privacy controls.
- [docs/operations/OPERATIONS_MANUAL.md](../../operations/OPERATIONS_MANUAL.md) — Full operations manual.
- `deployments/observability/` — Monitoring and alerting configuration.
- `specs/topology.spec.json` — Deployment topology machine contract.
