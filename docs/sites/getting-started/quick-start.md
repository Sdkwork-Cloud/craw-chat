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

## 2. Configure Local PostgreSQL (recommended)

```bash
cp .env.postgres.example .env.postgres
```

Edit database credentials if needed. Redis is enabled by default for realtime checkpoints and event windows.
To enable Redis-backed route store or cluster bus, uncomment the optional realtime variables in
`.env.postgres` and set `SDKWORK_IM_REALTIME_NODE_ID` plus `SDKWORK_IM_REALTIME_CLUSTER_BUS_SECRET`.
When those URLs are set in development, `pnpm dev` and `gateway:run:standalone` inject safe local
defaults for the node id and cluster bus secret if they are missing.

## 3. Start the Development Stack

```bash
pnpm dev
```

This starts the PostgreSQL standalone development profile. The current topology
adapter maps that standard public profile to the checked-in
`configs/topology/standalone.unified-process.development.env` file.

Default listeners (standalone unified profile — IAM and platform APIs collapse onto application ingress):

| Surface | URL |
| --- | --- |
| Application ingress (IM + embedded IAM) | `http://127.0.0.1:18079` |
| Platform API gateway (collapsed) | `http://127.0.0.1:18079` |
| PC renderer (when started) | `http://127.0.0.1:4176` |

Explicit full browser profile:

```bash
pnpm dev:browser:postgres:unified-process:standalone
```

Server only (no PC renderer):

```bash
pnpm dev:server
```

## 4. Verify Health

After startup, the standalone gateway prints `Listening on http://127.0.0.1:18079` when the HTTP
server is ready (the `Gateway Surface Groups` block is a startup summary, not a hang).

```bash
curl http://127.0.0.1:18079/healthz
curl http://127.0.0.1:18079/readyz
```

## 5. Auth Expectations

Public and smoke requests use SDKWork dual-token headers:

- `Authorization: Bearer <auth-token>`
- `Access-Token: <access-token>`

Anonymous login, registration, and QR auth use credential-entry routes on the same application
ingress (`18079`). Those routes accept `Access-Token: <bootstrap JWT>` only; do not send
`Authorization` on credential-entry calls. Dev orchestration injects private `SDKWORK_ACCESS_TOKEN`
before the PC renderer starts. See `specs/SDKWORK_APPBASE_IAM_INTEGRATION_SPEC.md`.

Tenant, user, session, device, actor, and permission context comes from those token claims. Do not
send client-controlled identity projection headers.

## 6. First SDK Integration

For local app-SDK integration against the development profile, use
`baseUrl = http://127.0.0.1:18079`.

The public app surface is documented in [App API Overview](/api-reference/app-api).

## 7. Packaged Server Install

If you need the production-style single-port server contract instead of the development orchestrator,
use [Server Lifecycle](/deployment/server-lifecycle).

## What To Read Next

- [Deployment](/deployment/index)
- [Runtime Topology](/architecture/runtime-topology)
- [API Reference](/api-reference/index)
- [SDK Overview](/sdk/index)
