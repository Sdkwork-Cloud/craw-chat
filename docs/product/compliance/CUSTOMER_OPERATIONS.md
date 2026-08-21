# Sdkwork IM - Customer Operations Guide

Status: active  
Owner: SDKWork maintainers  
Updated: 2026-07-21
Specs: PRIVACY_SPEC.md, SECURITY_SPEC.md, OBSERVABILITY_SPEC.md

## 1. Service Levels

| Surface | Target availability | Recovery objective |
| --- | --- | --- |
| Application ingress (`/healthz`, `/readyz`) | 99.9% monthly | 30 minutes |
| Realtime WebSocket plane | 99.9% monthly | 30 minutes |
| Conversation API | 99.9% monthly | 30 minutes |

These are target service levels, not an active customer SLA. Direct staging/capacity evidence,
immutable release artifacts, and commercial sign-off are still blocked. Historical Step-11
artifacts are engineering inputs only. Push delivery is not offered because the device-token and
provider delivery plane is not implemented.

## 2. Deployment Profiles

| Profile | Use |
| --- | --- |
| `cloud.staging` | Pre-production validation |
| `cloud.production` | Customer-facing SaaS |
| `standalone.production` | Private/on-prem single-node |

Topology authority: `etc/topology/` and `specs/topology.spec.json`.

## 3. Incident Response

1. Confirm blast radius via `/readyz` on `im-gateway`, `session-gateway`, and `conversation-service`.
2. Inspect Prometheus alerts in group `sdkwork-im.availability`.
3. Drain affected realtime nodes through governance control-plane drain APIs when cluster routing is enabled.
4. Roll back to previous container image digest if a release regression is confirmed.
5. Record post-incident evidence in release notes and capacity evidence index when perf regressions are involved.

## 4. Data Handling Summary

- Tenant isolation enforced at IAM, database schema, and AppContext projection layers.
- Message retention governed by domain retention classes and purge schedulers.
- Customer export/delete requests follow `DATA_PROTECTION.md`.

## 5. Support Boundaries

- Notification request acceptance does not mean push dispatch or device receipt. Operators must
  not report `requested`/accepted records as delivered notifications.
- RTC media runtime is owned by sibling `sdkwork-rtc`; IM owns signaling only.
- Service discovery Phase 2 remains optional; static topology env vars are the supported fallback.
