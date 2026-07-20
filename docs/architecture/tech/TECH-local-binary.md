> Migrated from `docs/sites/deployment/local-binary.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Local Binary

Local development no longer uses retired local-install wrapper scripts or the removed minimal-node
service crate. The verified binary workflow is topology v4 orchestration through Node scripts and
the `sdkwork-im-server` ingress binary.

## Development Commands

| Command | Profile | Purpose |
| --- | --- | --- |
| `pnpm dev` | `standalone.development` | Default PostgreSQL standalone browser dev stack |
| `pnpm dev:browser` | `standalone.development` | Browser dev stack |
| `pnpm dev:desktop` | `standalone.development` | Desktop dev stack |
| `pnpm dev:server` | `standalone.development` | Server-only development stack |

Configuration authority:

- `specs/topology.spec.json`
- `etc/topology/*.env`

## Application Ingress Binary

Build and test the ingress directly:

```bash
cargo build -p sdkwork-api-im-standalone-gateway
cargo test -p sdkwork-api-im-standalone-gateway --tests
```

The packaged binary name is `sdkwork-im-server`.

## Default Dev Bind Address

Application ingress listens on `127.0.0.1:18079` in the default standalone development profile.

```bash
curl http://127.0.0.1:18079/healthz
```

If `/healthz` or `/app/v3/api/auth/sessions/current` hang while TCP connects succeed, the unified
`sdkwork-api-im-standalone-gateway` runtime is likely starved. Projection HTTP handlers run mutex-heavy
reads on dedicated blocking threads; restart stale gateway processes before debugging client auth.

```powershell
taskkill /F /IM sdkwork-api-im-standalone-gateway.exe
pnpm dev
```

## Packaged Server Install

For production-style install, service management, PostgreSQL-backed storage, and release bundles,
use [Server Lifecycle](/deployment/server-lifecycle) with `bin/start-server.*` and
`deployments/templates/server.env.example`.

## Retired Scripts

The following are removed and must not be referenced in new work:

- retired local install/start/deploy wrapper scripts under `bin/`
- the removed minimal-node service crate under `services/`
- retired compose templates under `deployments/docker-compose/`

## What To Read Next

- [Quick Start](/getting-started/quick-start)
- [Runtime Topology](/architecture/runtime-topology)
- [Profiles and Environment](/deployment/profiles-and-env)

