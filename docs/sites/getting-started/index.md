# Getting Started

This section is for engineers, integrators, and operators who need to run Sdkwork IM with topology v5
defaults and minimal surprises.

## What You Get

- Topology v5 development orchestration through the shared `sdkwork-app` lifecycle facade.
- Standalone development uses one local `sdkwork-api-im-standalone-gateway`; cloud development uses the
  already deployed `platform.api-gateway` and starts only the selected local client.
- OpenAPI-style API documentation aligned to the implemented HTTP surface.
- Clear boundaries between IM standard APIs, app-business APIs, backend control/admin APIs, and SDK workspaces.

## Supported Runtime Modes

| Mode | Entry points | Best use |
| --- | --- | --- |
| Development stack | `pnpm dev`, `pnpm dev:browser`, `pnpm dev:desktop`, `pnpm dev:server` | Local development, PC integration, smoke |
| Cloud client development | `pnpm dev:cloud`, `pnpm dev:browser:cloud`, `pnpm dev:desktop:cloud` | Local client against deployed cloud development APIs |
| Packaged server | `bin/install-server.*`, `bin/start-server.*`, `bin/verify-server.*` | Production-style single-port installs |
| Standalone control plane | `cargo run -p governance-service --offline` | Governance API development |

## Prerequisites

- Rust toolchain with `cargo`
- Node.js 22 + pnpm 10
- Network access to `https://api-dev.sdkwork.com` for cloud development

## Runtime Profiles

Authority: `specs/topology.spec.json` and `etc/topology/*.env`.

| Profile id | Command | Application ingress |
| --- | --- | --- |
| `standalone.development` | `pnpm dev` / `pnpm dev:browser` / `pnpm dev:desktop` | `http://127.0.0.1:18079` |
| `cloud.development` | `pnpm dev:cloud` / `pnpm dev:browser:cloud` / `pnpm dev:desktop:cloud` | `https://api-dev.sdkwork.com` / `wss://api-dev.sdkwork.com` |

Cloud development starts no local gateway, API listener, PostgreSQL, Redis, migration, seed, or worker.

## Auth Boundary

Public clients authenticate through SDKWork dual-token headers:
`Authorization: Bearer <auth-token>` and `Access-Token: <access-token>`.

Control-plane routes require `control.read` or `control.write` permissions from AppContext projection.

## What To Read Next

- [Quick Start](/getting-started/quick-start)
- [Server Lifecycle](/deployment/server-lifecycle)
- [Architecture Overview](/architecture/overview)
- [API Reference](/api-reference/index)
- [SDK Overview](/sdk/index)
