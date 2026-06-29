# Sdkwork IM PC

SDKWork PC application root for the IM browser renderer and Tauri desktop shell.

Topology v2 authority: [../../docs/topology-greenfield.md](../../docs/topology-greenfield.md) · [../../specs/topology.spec.json](../../specs/topology.spec.json)

## Prerequisites

- Node.js 22, pnpm 10
- Rust toolchain (server + desktop host)
- Sibling checkouts: `../sdkwork-api-cloud-gateway`, `../sdkwork-rtc` (RTC media), plus shared SDK sources linked from `pnpm-workspace.yaml`

## Development (recommended)

Start the full stack from the repository root so topology profiles, platform gateway, and application ingress stay aligned:

```bash
cd ../..
pnpm install
pnpm dev              # browser renderer + sdkwork-im-server + platform gateway
pnpm dev:desktop      # Tauri desktop shell
pnpm dev:server          # server only
```

Default surfaces when using `pnpm dev` (standalone unified topology — IAM and IM share application ingress):

| Surface | URL |
| --- | --- |
| PC renderer | `http://127.0.0.1:4176` |
| Application ingress (IM + embedded IAM) | `http://127.0.0.1:18079` |
| Platform API gateway (collapsed) | `http://127.0.0.1:18079` |

Database profiles:

```bash
pnpm dev:browser:postgres     # PostgreSQL via .env.postgres
pnpm dev:browser:sqlite       # local SQLite user data
```

## App-root commands

Run from `apps/sdkwork-im-pc` when working on renderer packages only (server must already be running or started separately):

```bash
pnpm install
pnpm dev               # Vite renderer only
pnpm build
pnpm lint
pnpm test:domain-app-sdk-auth-runtime
pnpm test:notary-app-sdk-integration
pnpm test:drive-app-sdk-integration
pnpm test:knowledgebase-app-sdk-integration
pnpm test:qr-scan-standard
```

Desktop host package: `packages/sdkwork-im-pc-desktop`.

## Layout

```text
apps/sdkwork-im-pc/
├─ src/                 # thin bootstrap, providers, route assembly
├─ packages/            # sdkwork-im-pc-* feature and shell packages
├─ specs/               # PC application contract
├─ scripts/             # app-local contract tests
└─ sdkwork.app.config.json
```

## SDK integration

- IM HTTP/WebSocket: generated `@sdkwork/im-sdk` and app/backend SDK families under repository `sdks/`
- Platform IAM/Drive/Knowledgebase/Agent: sibling app SDK families via `VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL`
  - Knowledgebase: `@sdkwork/knowledgebase-app-sdk` through composed `createKnowledgebaseAppClient` (not raw generated transport)
- RTC media: `@sdkwork/rtc-sdk` from sibling `../sdkwork-rtc` (not checked into this repository's `sdks/`)

Do not add raw HTTP wrappers or manual auth headers in feature packages; bootstrap owns SDK construction.

### IAM login and QR auth (credential entry)

Anonymous login, registration, and QR auth use `@sdkwork/iam-credential-entry` through `createSdkworkAppbasePcAuthRuntime`:

- Runtime app id: `sdkwork-im-pc` (must match dev bootstrap JWT and standalone gateway IAM provisioning).
- Dev orchestration injects private `SDKWORK_ACCESS_TOKEN` before renderer startup; Vite defines `process.env.SDKWORK_ACCESS_TOKEN` only (never `VITE_*`).
- Credential-entry SDK calls send `Access-Token: <bootstrap JWT>`; `Authorization` is forbidden on those routes.
- The IM TokenManager holds bootstrap access tokens in memory only until a full dual-token session is committed.

Standalone gateway startup provisions tenant application `sdkwork-im-pc` for tenant `100001` before serving credential-entry routes. See `specs/SDKWORK_APPBASE_IAM_INTEGRATION_SPEC.md`.

When debugging QR login in DevTools, filter network requests to **`18079`** (application ingress), not `4176` (Vite renderer).

## Verification

From repository root:

```bash
pnpm test:sdkwork-im-pc-dev-command
pnpm test:workflow-commercial-gates
```

From this directory:

```bash
pnpm build
pnpm lint
```

Playwright production + authenticated chat e2e (via repository root gate):

```bash
cd ../..
node scripts/dev/sdkwork-im-pc-playwright-e2e.test.mjs
```

See [e2e/README.md](./e2e/README.md) for port `4173`, fixture boundaries, and optional staging workflow.

## Related docs

- [../../README.md](../../README.md)
- [../../docs/部署/README.md](../../docs/部署/README.md)
- [AGENTS.md](./AGENTS.md)
