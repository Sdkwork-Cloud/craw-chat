# Local Binary

Local development no longer uses `bin/install-local.*` or `local-minimal-node`. The verified binary
workflow is topology v2 orchestration through Node scripts and the `sdkwork-im-server` ingress
binary.

## Development Commands

| Command | Profile | Purpose |
| --- | --- | --- |
| `pnpm dev` | `standalone.unified-process.development` | Default PostgreSQL standalone browser dev stack |
| `pnpm dev:browser` | `standalone.unified-process.development` | Browser dev stack |
| `pnpm dev:desktop` | `standalone.unified-process.development` | Desktop dev stack |
| `pnpm dev:server` | `standalone.split-services.development` | Explicit server-only split-services dev stack |

Configuration authority:

- `specs/topology.spec.json`
- `configs/topology/*.env`

## Application Ingress Binary

Build and test the ingress directly:

```bash
cargo build -p sdkwork-im-cloud-gateway
cargo test -p sdkwork-im-cloud-gateway --tests
```

The packaged binary name is `sdkwork-im-server`.

## Default Dev Bind Address

Application ingress listens on `127.0.0.1:18079` in the default split-services development profile.

```bash
curl http://127.0.0.1:18079/healthz
```

## Packaged Server Install

For production-style install, service management, PostgreSQL-backed storage, and release bundles,
use [Server Lifecycle](/deployment/server-lifecycle) with `bin/start-server.*` and
`deployments/templates/server.env.example`.

## Retired Scripts

The following are removed and must not be referenced in new work:

- `bin/install-local.*`, `bin/start-local.*`, `bin/deploy-local.*`
- `services/local-minimal-node`
- `deployments/docker-compose/local-minimal.yml`

## What To Read Next

- [Quick Start](/getting-started/quick-start)
- [Runtime Topology](/architecture/runtime-topology)
- [Profiles and Environment](/deployment/profiles-and-env)
