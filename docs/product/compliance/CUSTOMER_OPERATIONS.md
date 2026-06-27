# Sdkwork IM — Customer Operations Guide

Status: active  
Owner: SDKWork maintainers  
Updated: 2026-06-24  
Specs: PRIVACY_SPEC.md, SECURITY_SPEC.md, OBSERVABILITY_SPEC.md

## 1. Service Levels

| Surface | Target availability | Recovery objective |
| --- | --- | --- |
| Application ingress (`/healthz`, `/readyz`) | 99.9% monthly | 30 minutes |
| Realtime WebSocket plane | 99.9% monthly | 30 minutes |
| Conversation API | 99.9% monthly | 30 minutes |
| Push delivery (FCM HTTP v1) | 99.5% monthly | 2 hours |

Evidence: Step-11 capacity artifacts under `artifacts/perf/step-11/capacity/` and Prometheus rules in `deployments/observability/prometheus-rules.yaml`.

## 2. Deployment Profiles

| Profile | Use |
| --- | --- |
| `cloud.split-services.staging` | Pre-production validation |
| `cloud.split-services.production` | Customer-facing SaaS |
| `standalone.unified-process.production` | Private/on-prem single-node |

Topology authority: `configs/topology/` and `specs/topology.spec.json`.

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

- RTC media runtime is owned by sibling `sdkwork-rtc`; IM owns signaling only.
- Service discovery Phase 2 remains optional; static topology env vars are the supported fallback.
