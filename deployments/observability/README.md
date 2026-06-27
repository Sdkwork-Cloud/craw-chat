# Sdkwork IM Observability Runbook

Status: active  
Owner: SDKWork maintainers  
Updated: 2026-06-24  
Specs: OBSERVABILITY_SPEC.md, DEPLOYMENT_SPEC.md

## Purpose

Operational guidance for monitoring Sdkwork IM split-services deployments in Kubernetes or standalone
systemd hosts. Alert rules live in [prometheus-rules.yaml](./prometheus-rules.yaml).  
OpenTelemetry collector reference manifest: [otel-collector.yaml](./otel-collector.yaml).  
Grafana operations dashboard: [grafana-dashboard.json](./grafana-dashboard.json) — import via Grafana UI or provisioning; covers service readiness, readyz probe success, HTTP 5xx ratio, per-service error rate, and firing alerts.

## Metrics and Logs

| Signal | Source | Notes |
| --- | --- | --- |
| Process health | `/healthz` | Liveness; returns `ok` when process is up |
| Dependency readiness | `/readyz` | Readiness; fails when DB/Redis/IAM dependencies unavailable |
| HTTP request metrics | `http_requests_total` | Namespace label `sdkwork-im` when Prometheus scrape is configured |
| IM domain counters | `im_*` metrics | Session gateway, social sync, retention purge expose Prometheus text at `/metrics` where enabled |
| Structured logs | stdout/stderr | JSON/text via `init_im_service_tracing_from_env()`; target `sdkwork.im` for domain events |

Configure OpenTelemetry export with `OTEL_EXPORTER_OTLP_ENDPOINT` (see `.env.postgres.example`).

## Alert Response

### SdkworkImGatewayNotReady / SdkworkImSessionGatewayNotReady / SdkworkImConversationServiceNotReady / SdkworkImProjectionServiceNotReady / SdkworkImMediaServiceNotReady / SdkworkImStreamingServiceNotReady

1. `kubectl -n sdkwork-im get pods` — identify CrashLoopBackOff or Pending pods.
2. `kubectl -n sdkwork-im logs deploy/<service> --tail=200` — check DB URL, Redis, JWT secret mount errors.
3. Verify secrets: database password file, Redis password, FCM credentials path.
4. Roll back deployment if a bad image tag was applied: `kubectl rollout undo deploy/<service> -n sdkwork-im`.

### SdkworkImGatewayReadyzFailures

1. Confirm PostgreSQL and Redis reachable from pod network.
2. Check IAM database pool env for session-gateway and gateway embed paths.
3. Scale up if CPU throttling: review HPA in `horizontal-pod-autoscalers.yaml`.

### SdkworkImHighHttp5xxRate

1. Identify failing service via ingress access logs or per-service `/metrics`.
2. Check conversation-service and session-gateway for overload (503 + retry hints).
3. Review recent deploy or migration: `pnpm db:drift:check` against staging before prod changes.

## Verification Commands

```bash
kubectl apply --dry-run=client -f deployments/kubernetes/cloud-split-services/
kubectl apply --dry-run=client -f deployments/observability/otel-collector.yaml
pnpm run test:commercial-deployment-contract
pnpm run test:step11-ha-dr-drill
curl -sf http://127.0.0.1:18079/healthz
curl -sf http://127.0.0.1:18079/readyz
```

## Related

- [../kubernetes/README.md](../kubernetes/README.md)
- [../../docs/部署/性能与灾备演练场景.md](../../docs/部署/性能与灾备演练场景.md)
