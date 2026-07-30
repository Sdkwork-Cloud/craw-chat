# Sdkwork IM IM App API And SDK Integration Standard

- Version: 1.0
- Scope: Sdkwork IM PC, `sdkwork-im-app-sdk`, `sdkwork-im-backend-sdk`, `@sdkwork/im-sdk`, appbase IAM integration, local PostgreSQL development, and release dependency sourcing
- Related root standards: `../../../specs/API_SPEC.md`, `../../../specs/SDK_SPEC.md`, `../../../specs/IAM_SPEC.md`, `../../../specs/DATABASE_SPEC.md`, `../../../specs/CONFIG_SPEC.md`, `../../../specs/DEPLOYMENT_SPEC.md`, `../../../specs/TEST_SPEC.md`

This local standard narrows the root SDKWork standards for the Sdkwork IM application. It is authoritative for the Sdkwork IM app workspace when root examples mention retired generic Spring app/backend SDK packages or authorities.

## 1. API Surface Ownership

| Surface | Prefix | Sdkwork IM owner | SDK family | Purpose |
| --- | --- | --- | --- | --- |
| IM Open API | `/im/v3/api` | `im-open-api` | `@sdkwork/im-sdk` | Standard open instant-messaging runtime API for conversations, messages, live/chat/game rooms, contacts, realtime, streams, and RTC signaling integration. |
| IM App API | `/app/v3/api` | `im-app-api` | `sdkwork-im-app-sdk` | Current Sdkwork IM app backend API implementation surface for app/client business flows, appbase IAM integration, bootstrap, and product-facing app capabilities. |
| IM Backend API | `/backend/v3/api` | `im-backend-api` | `sdkwork-im-backend-sdk` | Operator, admin, governance, audit, and control-plane capabilities for Sdkwork IM. |

Rules:

- `im-open-api` is the standard open IM API and must not expose login, register, logout, refresh, password reset, verification-code, or OAuth session creation endpoints.
- `im-app-api` is the current app/backend implementation surface for Sdkwork IM app clients. It may expose app login and appbase IAM operations through the injected appbase IAM module.
- `im-backend-api` owns backend/admin management. It must not expose user login/session creation endpoints.
- Frontend code must call `im-open-api` through `@sdkwork/im-sdk`, `im-app-api` through `sdkwork-im-app-sdk`, and backend/admin APIs through `sdkwork-im-backend-sdk`.
- Missing backend or IM capabilities must be closed by adding or updating the OpenAPI contract and generator flow. Frontend services must not add raw `/im/v3/*`, `/app/v3/*`, or `/backend/v3/*` HTTP fallbacks.

## 2. Product SDK Ownership

Sdkwork IM uses product-scoped SDKs, not generic Spring SDK packages.

| Concern | Required package | Required primary client | Compatibility alias |
| --- | --- | --- | --- |
| App API | `@sdkwork-internal/im-app-api-generated` from `sdks/sdkwork-im-app-sdk` | `SdkworkImAppClient` | `SdkworkAppClient` |
| Backend API | `@sdkwork-internal/im-backend-api-generated` from `sdks/sdkwork-im-backend-sdk` | `SdkworkImBackendClient` | `SdkworkBackendClient` |
| IM API | `@sdkwork/im-sdk` from `sdks/sdkwork-im-sdk` | `ImSdkClient` | none |

Rules:

- Sdkwork IM app code must not import retired generic Spring app/backend SDK packages or authorities.
- Generated client classes must be product-scoped so multiple app SDKs can be installed without class-name collisions.
- Compatibility aliases may exist only inside the generated SDK package for migration. Sdkwork IM integration code must use the product-scoped client names.
- App SDK and backend SDK generated outputs must remain reproducible from their OpenAPI inputs and generator configuration. Generated files must not be the long-term source of hand-written behavior.

## 3. IAM Login And IM Session Continuity

Sdkwork IM PC uses appbase IAM UI and runtime, but the concrete client injected into IAM is `SdkworkImAppClient`.

Required flow:

```text
AuthGate
  -> SdkworkIamAuthRoutes
  -> getSdkworkChatIamRuntime()
  -> createAppAuthService(() => getAppSdkClientWithSession(readAppSdkSessionTokens()))
  -> createIamAppSdkAdapter(SdkworkImAppClient)
  -> sdkwork-iam-app-sdk auth.sessions / registrations / verificationCodes / oauth.deviceAuthorizations
  -> persist authToken + accessToken + refreshToken + context + sessionId + user
  -> reset and recreate @sdkwork/im-sdk with the same token manager for its declared dual-token branch
```

Rules:

- Default login methods are password-only.
- Verification-code login is disabled by default.
- Registration verification for email and phone is required unless a documented deployment setting changes it.
- QR scan login is enabled through the appbase OAuth device authorization resource: `oauth.deviceAuthorizations.create`, `oauth.deviceAuthorizations.retrieve`, `oauth.deviceAuthorizations.scans.create`, and `oauth.deviceAuthorizations.passwordCompletions.create`.
- The legacy appbase QR auth resource `openPlatform.qrAuth` and `/app/v3/api/open_platform/qr_auth/*` paths are retired and must not be consumed, proxied, regenerated, or documented as current capabilities.
- The current canonical appbase auth package exposes `qrLoginEnabled`; Sdkwork IM must not pass unsupported runtime-config fields such as `qrLoginType` until the canonical appbase type includes them.
- A successful login must persist `authToken`, `accessToken`, optional `refreshToken`, `context`, `sessionId`, and normalized user data.
- `@sdkwork/im-sdk` is a protected `api-key-or-dual-token` open-api client. API-key mode sends only `X-API-Key`. The PC authenticated runtime selects the dual-token branch, joins the global TokenManager closure, and sends both `Authorization` and `Access-Token`. The two branches are mutually exclusive.
- Current `tenantId`, `organizationId`, `userId`, and `sessionId` are server-resolved from the validated API-key record or matching dual-token claims. PC business services must not pass them as SDK method parameters, query fields, request bodies, or custom headers.
- After login, chat and RTC code must be able to call IM SDK methods without a second login or manually assembled auth headers.
- Drive, Notary, Agent, and other foundation SDK integrations must follow the same rule: only business fields belong in service-layer SDK calls. Tenant and organization scope belong to dual-token validation and backend `WebRequestContext` resolution.
- AIoT app SDK calls use token-scoped generated methods such as `client.iot.devicesList()` and `client.iot.devicesCommandsCreate(...)`; IM PC services must not pass `tenantId`, `organizationId`, or `X-Sdkwork-*` scope headers.
- Notary composed Drive helpers must not pass `tenantId` into Drive download/delete calls.

## 4. Database Sharing

Sdkwork IM uses the standard Sdkwork IM database policy for appbase IAM and IM data.

Rules:

- Local development and production default to PostgreSQL via the process-level `SDKWORK_DATABASE_*` profile. 桌面端本地用户数据使用浏览器本地存储(IndexedDB / localStorage),不依赖 SQL 数据库文件。
- Runtime startup rejects application- and module-prefixed database keys; it does not bridge aliases.
- Sdkwork IM must not create duplicate IAM tables or alternate login schemas when appbase IAM already owns those tables.
- Schema or migration changes require explicit approval before implementation.

## 5. Local Development And Release Dependency Sourcing

Local development and release use different source materialization strategies but both compile from source.

Rules:

- Local `apps/sdkwork-im-pc/package.json` dependencies for SDKWork packages use relative `link:` specifiers.
- Vite and TypeScript aliases resolve generated IM app/backend SDKs, `@sdkwork/im-sdk`, `@sdkwork/drive-app-sdk`, appbase IAM packages, core PC React, and UI PC React to source entries, not prebuilt `dist`.
- Vite `optimizeDeps.exclude` must include linked SDKWork source packages so live source edits are not hidden by dependency pre-bundling.
- Local PC development exposes the IM application plane on `http://127.0.0.1:18079`. Foundation APIs use the single platform gateway root and are mounted there through selected Cargo assembly features; no per-module foundation upstream override is supported.
- The chat-pc `pnpm-workspace.yaml` must not register sibling `sdkwork-appbase`, `sdkwork-core`, or `sdkwork-ui` packages as workspace importers. They remain source-linked dependencies; otherwise pnpm install rewrites sibling `node_modules` and breaks isolated local builds.
- Release builds set `SDKWORK_SHARED_SDK_MODE=git`, run `sdk:shared:prepare`, materialize `sdkwork-im-app-sdk`, `sdkwork-im-backend-sdk`, `sdkwork-im-sdk`, `sdkwork-drive-app-sdk`, `sdkwork-appbase`, `sdkwork-core`, `sdkwork-ui`, and `sdkwork-clawrouter` from git-backed source checkouts, then build Sdkwork IM PC from those source links. Independent applications remain outside IM source preparation, dependency installation, runtime, and release boundaries.

## 5.1 Shared Gateway Foundation Composition

Sdkwork IM product APIs remain Sdkwork IM owned: IM open API stays under `/im/v3/api`, IM app API stays
under `/app/v3/api`, and IM backend API stays under `/backend/v3/api` with their own SDK families.
Foundation API integration depends on topology profile:

| Topology profile | Foundation API integration |
| --- | --- |
| `standalone.*` | The managed stack starts the IM application gateway and the platform assembly gateway. Foundation SDK roots resolve to the platform gateway; selected capabilities mount through Cargo-linked gateway assemblies. |
| `cloud.*` | Foundation APIs route through `platform.api-gateway`; per-module upstream and base URL overrides are retired. |

Rules:

- Production and private deployments should use `SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` as the
  platform SDK root and `SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL` /
  `SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL` as the IM application public surfaces.
- Browser/app SDK bootstrap uses `VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL`,
  `VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL`, and
  `VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL`; these are resolved by
  `apps/sdkwork-im-pc/scripts/sdkwork-im-iam-env.mjs` and topology profile env files.
- Local PC development starts through `scripts/im-dev.mjs`, which loads topology profiles and
  starts `sdkwork-api-im-standalone-gateway` for the default `standalone.development` profile. It must not spawn
  additional loopback HTTP servers for embedded dependency app APIs.
- In standalone profiles, `SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` selects the managed platform
  gateway bind. Cloud deployments select the same logical ingress with
  `SDKWORK_API_CLOUD_GATEWAY_BASE_URL`; neither profile accepts per-module foundation overrides.
- `crates/sdkwork-api-im-standalone-gateway`, `crates/sdkwork-api-im-standalone-gateway`, and internal services behind `application.public-ingress`
  remain product-owned IM routing, config, and local/private runtime layers. Foundation API
  aggregation is owned by `platform.api-gateway`, not by merging sibling route crates into
  `sdkwork-api-im-assembly`.
- Executable foundation API integration evidence is owned by sibling workspace metadata and
  `specs/component.spec.json` dependency surfaces. Add new foundation surfaces there only when an
  existing SDKWork spec or runtime contract proves the surface, prefix, SDK family, and gateway target.
- Product server code must not HTTP-proxy foundation APIs. Link sibling gateway assemblies through
  the platform gateway Cargo features per `APPLICATION_GATEWAY_SPEC.md` and
  `DEPENDENCY_MANAGEMENT_SPEC.md`.

## 6. IM Media Upload And Drive Attribution

Sdkwork IM owns IM conversation semantics, but SDKWork Drive owns all chat file upload, storage, Drive space, node, upload session, download grant, provider, and media lifecycle behavior.

Standard upload attribution for chat message attachments:

| Field | Value |
| --- | --- |
| Drive SDK package | `@sdkwork/drive-app-sdk` |
| Drive app API prefix | `/app/v3/api/drive` |
| `appId` | `chat` |
| `appResourceType` | `im_conversation` |
| `appResourceId` | backend conversation id |
| `scene` | `im` |
| `source` | `chat_message` |
| file upload profile | `attachment` for generic files; image, voice, and video use the Drive SDK media upload methods and Drive profile defaults |
| IM persisted reference | `driveUri`, `spaceId`, `nodeId`, and Drive-backed `MediaResource` |

Rules:

- Chat images, voice messages, videos, and files must be uploaded through `sdkwork-drive-app-sdk` uploader methods before the IM message is posted.
- UI-local `File`, `Blob`, `blob:` object URLs, `data:` URLs, recorder buffers, screenshot buffers, and picker previews are transient UI state only. They must not be stored in IM message bodies, render hints, `MediaResource.url`, `MediaResource.publicUrl`, or backend persistence.
- IM message content parts for uploaded media must use `kind = media`, a canonical Drive reference `drive://spaces/{spaceId}/nodes/{nodeId}`, and a Drive-backed `MediaResource` with `source = drive`.
- Chat services must not construct fake Drive URIs from conversation ids, local content, file names, timestamps, or browser preview URLs.
- Chat services must not call raw `/app/v3/api/drive/*`, `/drive/uploader/*`, `/upload_sessions/*`, provider presign URLs, S3, OSS, MinIO, or local object-storage endpoints directly.
- Backend IM validation must reject Drive-backed media snapshots that contain local preview delivery URLs, including nested posters, thumbnails, or variants.

## 7. Backend Capability Planning

When the front end needs a capability that is not present in the backend:

1. Classify the capability as `im-open-api`, `im-app-api`, or `im-backend-api`.
2. Define a resource-oriented path under `/im/v3/api`, `/app/v3/api`, or `/backend/v3/api`.
3. Define dotted operation IDs that generate nested SDK methods.
4. Define request/response schemas, pagination, idempotency, errors, auth requirements, and AppContext behavior.
5. Update the OpenAPI source and generator inputs.
6. Regenerate the owning SDK family.
7. Connect frontend services through the generated SDK or approved composed SDK layer.

No temporary frontend mock branch, raw HTTP client, manual `Authorization`, manual `Access-Token`, or local DTO fork is allowed as a substitute for the missing API contract.

Current local-only shell endpoints under `apps/sdkwork-im-pc/server.ts` and `local-api.ts` are limited to development support:

- `GET /api/config/modules`
- `POST /api/agent/doc`
- `POST /api/agent/icon`

These endpoints are not IM API endpoints and must not move under `/im/v3/api`. When promoted beyond local shell support, they must be classified under `im-app-api`, added to `/app/v3/api` OpenAPI, regenerated through `sdkwork-im-app-sdk`, and consumed through the product app SDK instead of frontend raw `fetch`.

## 8. Verification

Minimum verification for this standard:

- `node apps/sdkwork-im-pc/scripts/auth-appbase-ui-contract.test.mjs`
- `node scripts/dev/sdkwork-im-iam-application-bootstrap-standard.test.mjs`
- `node scripts/dev/sdkwork-im-bootstrap-access-token.test.mjs`
- `node apps/sdkwork-im-pc/scripts/sdk-runtime-token-manager-contract.test.mjs`
- `node scripts/dev/sdkwork-im-pc-sdk-integration.test.mjs`
- `node scripts/dev/sdkwork-im-pc-im-api-standard.test.mjs`
- `node scripts/dev/sdkwork-im-pc-dev-command.test.mjs`
- `node sdks/sdkwork-im-app-sdk/tests/app-sdk-auth-surface-contract.test.mjs`
- `node sdks/test/verify-im-v3-sdk-family-contract.test.mjs`
- `pnpm install --frozen-lockfile` from `apps/sdkwork-im-pc`
- `pnpm lint` from `apps/sdkwork-im-pc`
- `pnpm build` from `apps/sdkwork-im-pc`
