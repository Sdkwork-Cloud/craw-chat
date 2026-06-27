> Migrated from `docs/sites/deployment/index.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Deployment

This section covers deployment and operational entry points implemented in the repository today.

## Deployment Modes

| Mode | Entry points | Best use |
| --- | --- | --- |
| Development stack | `pnpm dev`, `pnpm dev:browser`, `pnpm dev:desktop`, `pnpm dev:server` | Local development, PC integration, smoke |
| Packaged server | `bin/install-server.*`, `bin/start-server.*`, `bin/verify-server.*` | Production-style installs and service management |
| Standalone control plane | `cargo run -p governance-service --offline` | Governance API development |

## How To Choose A Path

### For development and debugging

Use [Quick Start](/getting-started/quick-start) and [Local Binary](/deployment/local-binary):

- `pnpm dev` loads the PostgreSQL standalone development profile
- application ingress defaults to `http://127.0.0.1:18079`

### For production-style installs

Use [Server Lifecycle](/deployment/server-lifecycle):

- `sdkwork-im-server` single-port contract
- PostgreSQL-backed config roots and service-manager wrappers

## Profile Authority

Topology profile ids live in `specs/topology.spec.json`. Retired legacy profile names must not be used.

## What To Read Next

- [Local Binary](/deployment/local-binary)
- [Docker](/deployment/docker)
- [Server Lifecycle](/deployment/server-lifecycle)
- [Profiles and Environment](/deployment/profiles-and-env)
- [Runtime Operations](/deployment/runtime-operations)

