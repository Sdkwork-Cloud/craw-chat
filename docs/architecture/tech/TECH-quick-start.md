> Migrated from `docs/sites/getting-started/quick-start.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Quick Start

This is the shortest verified path to a working local Sdkwork IM development stack.

## Prerequisites

- Rust toolchain
- Node.js 22 + pnpm 10
- Sibling checkout: `sdkwork-api-cloud-gateway` (platform plane)

## 1. Install Dependencies

```bash
pnpm install
```

## 2. Start the Development Stack

```bash
pnpm dev
```

This starts the PostgreSQL standalone development profile. The current topology
adapter maps that standard public profile to the checked-in
`configs/topology/standalone.unified-process.development.env` file.

Default listeners:

| Surface | URL |
| --- | --- |
| IM application ingress | `http://127.0.0.1:18079` |
| Platform API gateway | `http://127.0.0.1:3900` |
| PC renderer (when started) | `http://127.0.0.1:4176` |

Explicit full browser profile:

```bash
pnpm dev:browser:postgres:unified-process:standalone
```

Server only (no PC renderer):

```bash
pnpm dev:server
```

## 3. Verify Health

```bash
curl http://127.0.0.1:18079/healthz
curl http://127.0.0.1:18079/readyz
```

## 4. Auth Expectations

Public and smoke requests use SDKWork dual-token headers:

- `Authorization: Bearer <auth-token>`
- `Access-Token: <access-token>`

Tenant, user, session, device, actor, and permission context comes from those token claims. Do not
send client-controlled identity projection headers.

## 5. First SDK Integration

For local app-SDK integration against the development profile, use
`baseUrl = http://127.0.0.1:18079`.

The public app surface is documented in [App API Overview](/api-reference/app-api).

## 6. Packaged Server Install

If you need the production-style single-port server contract instead of the development orchestrator,
use [Server Lifecycle](/deployment/server-lifecycle).

## What To Read Next

- [Deployment](/deployment/index)
- [Runtime Topology](/architecture/runtime-topology)
- [API Reference](/api-reference/index)
- [SDK Overview](/sdk/index)

