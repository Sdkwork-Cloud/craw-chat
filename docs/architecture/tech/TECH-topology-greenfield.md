> Migrated from `docs/topology-greenfield.md` on 2026-06-24.
> Owner: SDKWork maintainers

Target deployment system for Sdkwork IM. No compatibility bridges: delete retired items instead of aliasing them.

| Document | Role |
| --- | --- |
| `../../../../sdkwork-specs/APP_RUNTIME_TOPOLOGY_SPEC.md` | Platform connectivity and topology authority |
| `../../../../sdkwork-specs/APP_RUNTIME_TOPOLOGY_ARCHETYPES.md` | Archetype `realtime-application-platform` |
| `../../../specs/topology.spec.json` | Machine contract |

## 1. Communication Cheat Sheet

Use these exact public terms in standups, PRs, and runbooks:

| English | Meaning |
| --- | --- |
| `standalone` | Self-contained deployment profile for local development, desktop, on-prem, private appliance, or single-unit server/container delivery |
| `cloud` | Managed service deployment profile for SaaS, customer VPC, Kubernetes, or equivalent orchestration |
| `environment` | Lifecycle tier: `development`, `staging`, or `production` for current IM topology profiles |
| `application.public-ingress` | IM HTTP and WebSocket application ingress |
| `platform.api-gateway` | SDKWork platform APIs such as IAM and Drive |

Default dev profile spoken form: **standalone development** (`standalone.development`).

Process layout terms such as internal upstreams or in-process route mounting are implementation details. They must not appear in profile ids, public pnpm scripts, SDK bootstrap config, or runbook commands.

## 2. Target Architecture

```text
PC / Web Client
  |  IAM, Drive, Agent, AIoT REST
  +-------------------------------> platform.api-gateway
  |
  |  /im/v3/api/* HTTP
  |  /im/v3/api/realtime/ws
  +-------------------------------> application.public-ingress

Operator (optional)
  +-------------------------------> operations.control-ingress
```

Client URL authority:

| Surface | Server env | Client env |
| --- | --- | --- |
| Application HTTP | `SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL` | `VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL` |
| Application WebSocket | `SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL` | `VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL` |
| Platform gateway | `SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` | `VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` |

## 3. Profile Matrix

Topology profile ids are exactly `<deploymentProfile>.<environment>`.

| Profile id | deploymentProfile | environment | Primary use |
| --- | --- | --- | --- |
| `standalone.development` | `standalone` | `development` | Default local dev and CI smoke |
| `standalone.staging` | `standalone` | `staging` | Standalone pre-production rehearsal |
| `standalone.production` | `standalone` | `production` | Standalone server/container/desktop release |
| `cloud.development` | `cloud` | `development` | Cloud integration development |
| `cloud.staging` | `cloud` | `staging` | Cloud pre-production rehearsal |
| `cloud.production` | `cloud` | `production` | Cloud production release |

Target commands:

```bash
pnpm dev                              # standalone.development
pnpm dev:browser:postgres:standalone  # standalone.development
pnpm build                            # cloud.production
```

CLI flags:

```bash
node scripts/im-dev.mjs --deployment-profile standalone --environment development
```

`--service-layout` is retired. `scripts/im-dev.mjs` must reject it instead of normalizing it.

## 4. Delete List (No Aliases)

### Profile / Runtime Names

- `local-minimal`, `local-default`
- any three-segment profile id that encodes process layout
- `*.embedded.*`
- `*.distributed.*`

### Env Keys

See `topology.spec.json` -> `retired.envKeys`. Notable retirements:

- `SDKWORK_IM_SERVER_*`
- `SDKWORK_IM_PRODUCT_*`
- `SDKWORK_IM_FOUNDATION_*`
- `VITE_SDKWORK_IM_APP_API_BASE_URL`

### Scripts

- `scripts/dev/run-sdkwork-im-pc-dev.mjs`
- `scripts/dev/start-sdkwork-im-unified-web.mjs`

Current entries:

- `scripts/lib/im-pc-dev.mjs`: shared PC/server dev orchestration library
- `scripts/im-dev.mjs`: topology-aware PC dev entry (`pnpm dev`, `pnpm dev:browser`, `pnpm dev:desktop`)
- `scripts/im-server-dev.mjs`: server-only dev stack (`pnpm dev:server`)

## 5. Port Authority

Ports exist only in:

1. `etc/topology/<profile-id>.env`
2. `specs/topology.spec.json` -> `internalUpstreams.*.defaultBind`

Development binds:

| Surface / upstream | Bind env | Default bind |
| --- | --- | --- |
| application.public-ingress | `SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND` | `127.0.0.1:18079` |
| platform.api-gateway | `SDKWORK_API_CLOUD_GATEWAY_BIND` | `127.0.0.1:3900` |
| internal session-gateway | `SDKWORK_IM_INTERNAL_SESSION_GATEWAY_BIND` | `127.0.0.1:18080` |

## 6. Cloud Public URLs

IM application public host: **`im.sdkwork.com`**. `chat.sdkwork.com` is reserved for LLM conversational apps.

| Surface | URL |
| --- | --- |
| Application HTTP | `https://im.sdkwork.com` |
| Application WebSocket | `wss://im.sdkwork.com` + path `/im/v3/api/realtime/ws` |
| Platform gateway | `https://api.sdkwork.com` |

No alternate realtime host exists unless declared as a second surface in the topology spec.

## 7. Verification

```bash
pnpm test:topology-baggage
pnpm test:runtime-standard
pnpm test:workflow-commercial-gates
node ../sdkwork-app-topology/scripts/sdkwork-topology.mjs validate --root . --spec specs/topology.spec.json
```

Contract tests load fixture profile env. Default dev ingress is `http://127.0.0.1:18079` through `etc/topology/standalone.development.env`.
