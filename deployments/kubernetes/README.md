# Kubernetes Deployment Artifacts

## Purpose

Reference manifests for `cloud.split-services.production` and `cloud.split-services.staging`
(`SDKWORK_IM_DEPLOYMENT_PROFILE=cloud`, `SDKWORK_IM_SERVICE_LAYOUT=split-services`). These files are
non-secret templates aligned with `configs/topology/` profiles and
`deployments/templates/server.env.example`.

## Layout

- `cloud-split-services/namespace.yaml` — application namespace
- `cloud-split-services/ingress.yaml` — public HTTP/WebSocket ingress for `im-gateway`
- `cloud-split-services/pod-disruption-budgets.yaml` — HA disruption budgets
- `cloud-split-services/horizontal-pod-autoscalers.yaml` — CPU-based autoscaling for realtime/comms
- `cloud-split-services/im-gateway/` — public ingress gateway with dependency-aware `/readyz`
- `cloud-split-services/session-gateway/` — realtime plane service
- `cloud-split-services/conversation-service/` — conversation runtime service
- `cloud-split-services/governance-service/` — control-plane service
- `cloud-split-services/notification-service/` — push/in-app notification service
- `cloud-split-services/projection-service/` — timeline and inbox projection service
- `cloud-split-services/media-service/` — media reference service
- `cloud-split-services/streaming-service/` — stream lifecycle service

## Prerequisites

- PostgreSQL and Redis reachable from the cluster (managed services or in-cluster operators)
- Secrets mounted for database, Redis, JWT, app-context signature, and FCM service account material
- Platform API gateway (`SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL`) deployed separately
- Container images built from split-service binaries or `deployments/docker/sdkwork-im-server.Dockerfile`

## Verification

```bash
kubectl apply --dry-run=client -f deployments/kubernetes/cloud-split-services/
pnpm run test:commercial-deployment-contract
```

After apply, probe readiness:

```bash
kubectl -n sdkwork-im get pods
kubectl -n sdkwork-im port-forward svc/im-gateway 18079:18079
curl -sf http://127.0.0.1:18079/readyz
```

## Observability

Prometheus alert rules: `deployments/observability/prometheus-rules.yaml`  
Runbook: `deployments/observability/README.md`  
Compliance guides: `docs/product/compliance/`

## Related Specs

- `../../sdkwork-specs/DEPLOYMENT_SPEC.md`
- `../../sdkwork-specs/ENVIRONMENT_SPEC.md`
- `../../sdkwork-specs/OBSERVABILITY_SPEC.md`
- `../templates/server.env.example`
