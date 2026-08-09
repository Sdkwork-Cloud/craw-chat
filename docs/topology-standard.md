# Sdkwork IM Runtime Topology

Human-facing summary for IM. Machine contract: `specs/topology.spec.json`.
Platform naming authority: `../sdkwork-specs/APP_RUNTIME_TOPOLOGY_SPEC.md`.

## Archetype

**realtime-application-platform** — application HTTP/WebSocket ingress + platform API gateway (embedded in standalone).

## Default Profile

**standalone.development**

Profile env files: `etc/topology/`

## Surfaces

| Surface id | Standalone | Cloud |
| --- | --- | --- |
| `application.public-ingress` | `sdkwork-api-im-standalone-gateway` | `sdkwork-api-im-standalone-gateway` / `sdkwork-im-server` |
| `platform.api-gateway` | embedded in standalone gateway | `platform.api-gateway` |
| `operations.control-ingress` | (optional) | (optional) |

## Env Keys (standalone development)

```bash
# Application plane — IM product APIs + embedded IAM
SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND=127.0.0.1:18089
SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=http://127.0.0.1:18089
SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=ws://127.0.0.1:18089
SDKWORK_IM_STANDALONE_GATEWAY_ENVIRONMENT=development

# Platform plane — collapsed onto application.public-ingress in standalone
SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=http://127.0.0.1:18089
SDKWORK_IM_PLATFORM_API_GATEWAY_AUTOSTART=true

# Client mirror (Vite)
VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=http://127.0.0.1:18089
VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=ws://127.0.0.1:18089
VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=http://127.0.0.1:18089
```

## Commands

```bash
pnpm dev           # standalone.development
pnpm gateway:run:standalone
pnpm build         # cloud.production
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
- "Standalone profiles embed IAM through **sdkwork-api-im-standalone-gateway**."
- "Foundation SDKs use **SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL** only."

See `topology-greenfield.md` for migration notes.
