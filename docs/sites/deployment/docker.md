# Docker

Docker deployment is optional for container validation. The authoritative development entry is
`pnpm dev` with topology profile `standalone.unified-process.development`.

The formal packaged install contract is [Server Lifecycle](/deployment/server-lifecycle) using
`deployments/templates/server.env.example` and `sdkwork-im-server`.

## Retired Compose Profiles

`local-minimal` and `local-default` Compose files are removed. Do not reference
`deployments/docker-compose/local-minimal.yml` or `deploy-local.*` scripts.

## Current Development Path

From the repository root:

```bash
pnpm install
pnpm dev
```

Default dev listeners:

| Plane | URL |
| --- | --- |
| Application ingress | `http://127.0.0.1:18079` |
| Platform API gateway | `http://127.0.0.1:3900` |

Health check:

```bash
curl http://127.0.0.1:18079/healthz
```

## Production Container Notes

Production container images should bind application ingress using topology v2 keys:

- `SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL`
- `SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL`
- `SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND`
- `SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL`

See [Profiles and Environment](/deployment/profiles-and-env) and
[Production Domain Binding](/deployment/production-domain-binding).

## What To Read Next

- [Deployment](/deployment/index)
- [Profiles and Environment](/deployment/profiles-and-env)
- [Quick Start](/getting-started/quick-start)
- [Server Lifecycle](/deployment/server-lifecycle)
