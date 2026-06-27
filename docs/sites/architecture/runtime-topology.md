# Runtime Topology

Sdkwork IM uses topology v2 connectivity planes. See `specs/topology.spec.json` and
`docs/topology-greenfield.md`.

## Development default

```text
PC / Web Client
  ├─ IAM, Drive, Notary, Agent REST
  +-------------------------------> platform.api-gateway (sdkwork-api-cloud-gateway :3900)
  |
  ├─ /im/v3/api/* HTTP
  ├─ /im/v3/api/realtime/ws
  +-------------------------------> application.public-ingress (sdkwork-im-server :18079)
```

Commands:

- `pnpm dev` — `standalone.unified-process.development`
- `pnpm dev:browser` — browser development target
- `pnpm dev:desktop` — desktop development target
- `pnpm dev:server` — server-only dev stack

## Production SaaS

| Surface | Host |
| --- | --- |
| IM application | `im.sdkwork.com` |
| Platform gateway | `api.sdkwork.com` |

## Split internal upstreams

When `serviceLayout=split-services`, `sdkwork-im-server` proxies to internal services declared in
`specs/topology.spec.json` → `internalUpstreams.split-services`.

## Retired

`local-minimal-node`, `local-minimal`, and `local-default` profiles are removed. Do not use
`.runtime/local-*` trees or `deployments/templates/local-*.env.example`.
