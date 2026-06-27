> Migrated from `docs/topology-greenfield.md` on 2026-06-24.
> Owner: SDKWork maintainers

Target deployment system for Sdkwork IM. No compatibility bridges — delete retired items instead of aliasing them.

| Document | Role |
| --- | --- |
| `../../sdkwork-specs/APP_RUNTIME_TOPOLOGY_NAMING.md` | **Naming authority** — env keys, profile ids, spoken phrases |
| `../../sdkwork-specs/APP_RUNTIME_TOPOLOGY_SPEC.md` | Platform connectivity standard (v2) |
| `../../sdkwork-specs/APP_RUNTIME_TOPOLOGY_ARCHETYPES.md` | Archetype `realtime-application-platform` |
| `../specs/topology.spec.json` | Machine contract |

## 1. Communication Cheat Sheet

Use these exact terms in standups, PRs, and runbooks:

| English (canonical) | 中文 | Meaning |
| --- | --- | --- |
| standalone | 独立部署 | Customer VPC, on-prem, developer laptop, private appliance |
| cloud | 云部署 | SDKWork SaaS or split cloud services |
| split-services | 拆分服务 | Product ingress + internal upstreams (default dev) |
| unified-process | 单进程 | Smoke/CI — routes in one process |
| application plane | 应用平面 | IM HTTP + WebSocket |
| platform plane | 平台平面 | IAM, Drive, Agent via api-gateway |
| application.public-ingress | 应用公网入口 | `sdkwork-im-server` |
| platform.api-gateway | 平台 API 网关 | `sdkwork-api-cloud-gateway` |

Default dev profile spoken form: **standalone unified-process development**.

## 2. Target Architecture

```text
PC / Web Client
  |  IAM, Drive, Agent, AIoT REST
  +-------------------------------> platform.api-gateway (sdkwork-api-cloud-gateway)
  |
  |  /im/v3/api/* HTTP
  |  /im/v3/api/realtime/ws
  +-------------------------------> application.public-ingress (sdkwork-im-server)

Operator (optional)
  +-------------------------------> operations.control-ingress
```

Client URL authority (one key per surface):

| Surface | Server env | Client env |
| --- | --- | --- |
| Application HTTP | `SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL` | `VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL` |
| Application WebSocket | `SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL` | `VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL` |
| Platform gateway | `SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` | `VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` |

## 3. Service Layout

| serviceLayout | Meaning | When |
| --- | --- | --- |
| `split-services` | Ingress proxies to internal upstream services | Default dev and all production |
| `unified-process` | Routes in-process behind ingress binary | CI smoke only |

`unified-process` does **not** include the platform plane. IAM flows still require `platform.api-gateway`.

## 4. Profile Matrix

| Profile id | hosting | serviceLayout | environment |
| --- | --- | --- | --- |
| `standalone.unified-process.development` | standalone | unified-process | development |
| `standalone.split-services.development` | standalone | split-services | development |
| `standalone.unified-process.production` | standalone | unified-process | production |
| `cloud.split-services.production` | cloud | split-services | production |

Target commands:

```bash
pnpm dev              # standalone.unified-process.development
pnpm dev:browser:postgres:unified-process:standalone      # standalone.unified-process.development (smoke)
pnpm build            # cloud.split-services.production
```

CLI flags (orchestrator):

```bash
node scripts/im-dev.mjs --deployment-profile standalone --service-layout split-services
```

## 5. Delete List (No Aliases)

### Binaries (completed)

- `local-minimal-node` — removed; use `sdkwork-im-server` via `pnpm dev` / `pnpm dev:browser`

### Profile / runtime names (completed)

- `local-minimal`, `local-default` — removed; use topology profile ids under `configs/topology/`
- v1 profile ids: `standalone.*`, `cloud.*`, `*.embedded.*`, `*.distributed.*`

### Env keys

See `topology.spec.json` → `retired.envKeys`. Notable retirements:

- `SDKWORK_IM_SERVER_*` — plane-specific `APPLICATION_PUBLIC_*` / `PLATFORM_API_GATEWAY_*`
- `SDKWORK_IM_PRODUCT_*`, `SDKWORK_IM_FOUNDATION_*` — v1 draft names, never ship
- `VITE_SDKWORK_IM_APP_API_BASE_URL` — replaced by surface-specific client keys

### Scripts (completed)

- `scripts/lib/im-pc-dev.mjs` — shared PC/server dev orchestration library
- `scripts/im-dev.mjs` — topology-aware PC dev entry (`pnpm dev`, `pnpm dev`)
- `scripts/im-server-dev.mjs` — server-only dev stack (`pnpm dev:server`)
- Deleted: `scripts/dev/run-sdkwork-im-pc-dev.mjs`, `scripts/dev/start-sdkwork-im-unified-web.mjs`

### Spec contradictions (completed)

- `component.spec.json` — platform SDK roots use `SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL`
- `im-app-api-sdk-integration.spec.md` §5.1 — rewritten to connectivity planes
- `docs/sites/deployment/production-domain-binding.md` — v2 surface keys

## 6. Port Authority

Ports exist only in:

1. `configs/topology/<profile-id>.env`
2. `specs/topology.spec.json` → `internalUpstreams.split-services.*.defaultBind`

Remove Rust `default_split_upstreams()` hardcodes; load from spec or profile.

Suggested development binds (until profile files land):

| Surface / upstream | Bind env | Suggested bind |
| --- | --- | --- |
| application.public-ingress | `SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND` | `127.0.0.1:18079` |
| platform.api-gateway | `SDKWORK_API_CLOUD_GATEWAY_BIND` | `127.0.0.1:3900` |
| internal session-gateway | `SDKWORK_IM_INTERNAL_SESSION_GATEWAY_BIND` | `127.0.0.1:18080` |

## 7. Cloud Public URLs (Pattern A)

IM application public host: **`im.sdkwork.com`** (not `chat.sdkwork.com` — that host is reserved for LLM conversational apps).

| Surface | URL |
| --- | --- |
| Application HTTP | `https://im.sdkwork.com` |
| Application WebSocket | `wss://im.sdkwork.com` + path `/im/v3/api/realtime/ws` |
| Platform gateway | `https://api.sdkwork.com` |

No alternate realtime host unless declared as a second surface in the profile.

## 8. Verification

```bash
pnpm test:topology-baggage
pnpm test:runtime-standard
pnpm test:workflow-commercial-gates
node ../sdkwork-app-topology/scripts/sdkwork-topology.mjs validate --root . --spec specs/topology.spec.json
```

- `test:topology-baggage` scans adapters, artifacts, archive docs, `docs/architecture`, active docs, configs, and code for retired topology vocabulary
- Contract tests load fixture profile env; dev default ingress is `18079` via `configs/topology/*.env`
- Governance: platform SDKs must not be constructed with application HTTP URL; RTC SDK stays in sibling `../sdkwork-rtc`

## 9. Documentation Retirement (completed)

- `README.md`, `AGENTS.md`, `docs/README.md`, `deployments/README.md`, `specs/README.md`
- `apps/sdkwork-im-pc/README.md`, `sdks/README.md`, `docs/部署/README.md`
- Retired AI Studio / Gemini scaffold from PC `vite.config.ts`, `local-api.ts`, `server.ts`, `.env.example`
- `docs/sites/architecture/overview.md`, `runtime-topology.md`, `module-map.md`, `index.md`
- `docs/sites/getting-started/quick-start.md`, `deployment/docker.md`, `deployment/local-binary.md`
- Archive index disclaimers: `docs/架构/README.md`, `docs/step/README.md`, `docs/review/README.md`
- ADR index: `docs/architecture/decisions/README.md`
- SDK quick-start base URLs → `http://127.0.0.1:18079`
- Retired: `run-local-minimal.*`, `smoke-local-minimal.ps1`, `bin/install-local.*`, `bin/start-local.*`, `bin/deploy-local.*`, `bin/status-local.*`, `bin/stop-local.*`, `bin/restart-local.*`
- Kept: `bin/chat-cli-local.*` (direct-binary CLI wrappers; not a topology profile)

