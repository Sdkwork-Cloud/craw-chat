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
pnpm test:voice-app-sdk-integration
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
├─ tsconfig.app.json    # lint/typecheck scope for IM-owned sources only
├─ types/stubs/         # lint-only module stubs for dynamically loaded sibling PC capabilities
└─ sdkwork.app.config.json
```

## Dependency composition (APP_COMPOSITION_SPEC)

PC follows the same native workspace model as H5:

| Layer | Authority | Declares |
| --- | --- | --- |
| Repository root | `pnpm-workspace.yaml` | sibling SDKWork source paths once |
| `@sdkwork/im-pc-core` | SDK registry + cross-repo facades | `@sdkwork/drive-app-sdk`, `@sdkwork/agents-app-sdk`, domain PC integrations |
| `@sdkwork/im-pc-shell` | capability module loaders | `@sdkwork/drive-pc-drive`, `@sdkwork/knowledgebase-pc-knowledge`, `@sdkwork/voice-pc-market`, `@sdkwork/voice-pc-speech`, local IM feature packages |
| Feature packages | UI modules | `@sdkwork/im-pc-core`, `@sdkwork/im-pc-commons`, UI/catalog deps |
| App root `@sdkwork/im-pc` | bootstrap + build/runtime | local IM packages, auth/UI base, required IM SDK inventory |

Rules enforced by `pnpm test:sdkwork-im-pc-architecture-standard`:

- Do not declare `"workspaces"` in app-root `package.json`.
- Do not hoist domain facade packages on the app root to compensate for empty member `dependencies`.
- Do not declare `pnpm.overrides` under the app root.
- Run `pnpm lint` through `tsconfig.app.json` so TypeScript does not typecheck sibling repositories via deep `paths` maps.
- `tsconfig.app.json` keeps narrow path aliases only for in-repo IM SDK sources (`@sdkwork/im-sdk`, generated IM app/backend transports, IAM/Drive/Voice/RTC entrypoints). Dynamically loaded sibling PC capability modules (`drive`, `knowledgebase`, `voice`, `course`, `notary`) and Node-only lint stubs such as `jsdom` resolve through `types/stubs/*.d.ts` so IM-owned code stays the typecheck boundary.
- Cross-capability runtime wiring lives in `@sdkwork/im-pc-core` integration modules and uses dynamic `import()` for sibling PC packages; Vite aliases in `vite.config.ts` may still source-link siblings for local HMR.

Regenerate missing member dependencies after import changes:

```bash
node ../../scripts/dev/fix-pc-package-import-closure.mjs
pnpm test:sdkwork-im-pc-architecture-standard
```

## SDK integration

- IM HTTP/WebSocket: generated `@sdkwork/im-sdk` and app/backend SDK families under repository `sdks/`
- Platform IAM/Drive/Knowledgebase/Voice/Agent: sibling app SDK families via `VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL`
  - Knowledgebase: `@sdkwork/knowledgebase-app-sdk` through composed `createKnowledgebaseAppClient` (not raw generated transport)
  - Voice: `@sdkwork/voice-app-sdk` through `voicePcIntegration` and embed packages `@sdkwork/voice-pc-market` / `@sdkwork/voice-pc-speech` (tabs `voice` / `voicegen`). Production market lists `audio_assets` via SDK; pilot preview uses `VITE_SDKWORK_VOICE_MARKET_PILOT`. Split-deploy requires `SDKWORK_IM_VOICE_APP_API_UPSTREAM`.
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
pnpm test:sdkwork-im-pc-architecture-standard
pnpm test:sdkwork-im-pc-dev-command
pnpm test:workflow-commercial-gates
node ../sdkwork-specs/tools/verify-repo.mjs --root ..
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
