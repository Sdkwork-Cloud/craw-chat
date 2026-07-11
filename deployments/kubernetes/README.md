# Kubernetes Deployment Artifacts

## Purpose

Reference manifests for `cloud.production` and `cloud.staging`
(`SDKWORK_IM_DEPLOYMENT_PROFILE=cloud`). These files are
non-secret templates aligned with `configs/topology/` profiles and
`deployments/templates/server.env.example`.

## Layout

- `cloud/namespace.yaml` 鈥?application namespace
- `cloud/ingress.yaml` 鈥?public HTTP/WebSocket ingress for `im-gateway`
- `cloud/pod-disruption-budgets.yaml` 鈥?HA disruption budgets
- `cloud/horizontal-pod-autoscalers.yaml` 鈥?CPU-based autoscaling for realtime/comms
- `cloud/im-gateway/` 鈥?public ingress gateway with dependency-aware `/readyz`
- `cloud/session-gateway/` 鈥?realtime plane service
- `cloud/conversation-service/` 鈥?conversation runtime service
- `cloud/governance-service/` 鈥?control-plane service
- `cloud/notification-service/` 鈥?push/in-app notification service
- `cloud/projection-service/` 鈥?timeline and inbox projection service
- `cloud/media-service/` 鈥?media reference service
- `cloud/streaming-service/` 鈥?stream lifecycle service

## Prerequisites

- PostgreSQL and Redis reachable from the cluster (managed services or in-cluster operators)
- Secrets mounted for database, Redis, JWT, app-context signature, and FCM service account material
- Platform API gateway (`SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL`) deployed separately
- Container images built from cloud-service binaries or `deployments/docker/sdkwork-im-server.Dockerfile`

## Verification

```bash
kubectl apply --dry-run=client -f deployments/kubernetes/cloud/
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
