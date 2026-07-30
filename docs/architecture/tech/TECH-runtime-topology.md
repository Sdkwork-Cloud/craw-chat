> Migrated from `docs/sites/architecture/runtime-topology.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Runtime Topology

Sdkwork IM uses topology v5 connectivity planes. See `specs/topology.spec.json` and
`docs/topology-greenfield.md`.

## Development default

```text
PC / Web Client
  ├─ IAM, Drive, Notary, Agent REST
  +-------------------------------> platform.api-gateway (platform.api-gateway :3900)
  |
  ├─ /im/v3/api/* HTTP
  ├─ /im/v3/api/realtime/ws
  +-------------------------------> application.public-ingress (sdkwork-im-server :18079)
```

Commands:

- `pnpm dev` — `standalone.development`
- `pnpm dev:browser` — browser development target
- `pnpm dev:desktop` — desktop development target
- `pnpm dev:server` — server-only dev stack

## Production SaaS

| Surface | Host |
| --- | --- |
| IM application | `im.sdkwork.com` |
| Platform gateway | `api.sdkwork.com` |

## Internal upstreams

Cloud and server-only profiles may proxy to internal services declared in
`specs/topology.spec.json` under `internalUpstreams`. Public profile ids do not encode process
layout.

## Retired

Pre-topology-v5 minimal-node/minimal/default profile ids are removed. Do not use legacy per-profile
source-tree runtime config directories or retired env templates under `deployments/templates/`.
