> Migrated from `docs/topology-standard.md` on 2026-06-24.
> Owner: SDKWork maintainers

Human-facing summary for IM. Machine contract: `../specs/topology.spec.json`.
Platform naming authority: `../../sdkwork-specs/APP_RUNTIME_TOPOLOGY_NAMING.md`.

## Archetype

**realtime-application-platform** — application HTTP/WebSocket ingress + platform API gateway (embedded in standalone).

## Default Profile

**standalone.unified-process.development**

Profile env files: `configs/topology/`

## Surfaces

| Surface id | Standalone unified-process | Cloud split-services |
| --- | --- | --- |
| `application.public-ingress` | `sdkwork-im-standalone-gateway` | `sdkwork-im-server` |
| `platform.api-gateway` | embedded in standalone gateway | `sdkwork-api-cloud-gateway` |
| `operations.control-ingress` | (optional) | (optional) |

## Env Keys (standalone unified-process development)

```bash
# Application plane — IM product APIs + embedded IAM
SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND=127.0.0.1:18079
SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=http://127.0.0.1:18079
SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=ws://127.0.0.1:18079
SDKWORK_IM_STANDALONE_GATEWAY_ENVIRONMENT=development

# Platform plane — collapsed onto application.public-ingress in standalone
SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=http://127.0.0.1:18079
SDKWORK_IM_PLATFORM_API_GATEWAY_AUTOSTART=true

# Client mirror (Vite)
VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=http://127.0.0.1:18079
VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=ws://127.0.0.1:18079
VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=http://127.0.0.1:18079
```

## Commands

```bash
pnpm dev           # standalone.unified-process.development
pnpm gateway:run:standalone
pnpm build         # cloud.split-services.production
```

## Cloud URLs (Pattern A)

Public application host for IM: **`im.sdkwork.com`**.

| Surface | Production URL |
| --- | --- |
| Application HTTP | `https://im.sdkwork.com` |
| Application WebSocket | `wss://im.sdkwork.com` (path `/im/v3/api/realtime/ws`) |
| Platform gateway | `https://api.sdkwork.com` |

## Phrases for reviews

- "WebSocket terminates on **application.public-ingress**, not platform.api-gateway."
- "Standalone profiles embed IAM through **sdkwork-im-standalone-gateway**."
- "Foundation SDKs use **SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL** only."

See `topology-greenfield.md` for migration notes.

