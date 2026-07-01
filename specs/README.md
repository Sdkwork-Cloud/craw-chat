# Sdkwork IM Component Specs

This directory is the local standards index for `sdkwork-im`.

Root SDKWork standards remain authoritative. Local component specs can narrow or document this component, but they must not contradict [the root standards](../sdkwork-specs/README.md).

## Component

| Field | Value |
| --- | --- |
| Name | `sdkwork-im` |
| Type | `app` |
| Root | `sdkwork-im` |
| Domain | `communication` |
| Capability | `chat` |
| Languages | `javascript, rust` |
| Status | `ACTIVE` |

## Contract Manifest

- [component.spec.json](./component.spec.json) is the machine-readable component contract.
- Shared foundation API composition targets `sdkwork-api-cloud-gateway` through
  `SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` and `VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL`
  for split-service and cloud deployments. In `standalone.unified-process.*`, the sibling
  `sdkwork-im-standalone-gateway` collapses platform ingress on one bind and mounts Drive,
  Knowledgebase, Commerce, Mail, and Notary dependency APIs in-process via Cargo-linked route crates.
- Application HTTP/WebSocket traffic uses `SDKWORK_IM_APPLICATION_PUBLIC_*` and
  `VITE_SDKWORK_IM_APPLICATION_PUBLIC_*`. `services/sdkwork-im-cloud-gateway` and
  `crates/sdkwork-im-cloud-gateway-config` keep product-owned IM routing only; split-service
  platform API routing is owned by the shared gateway boundary.
- Local PC development starts through `scripts/im-dev.mjs` (`pnpm dev`), which loads topology
  profiles from `configs/topology/` and starts `sdkwork-im-standalone-gateway` only. It does
  not spawn a separate `sdkwork-api-cloud-gateway` process in the default unified-process profile.
- `crates/sdkwork-im-cloud-gateway-config` omits HTTP upstream targets for standalone-embedded
  dependency APIs in unified-process mode. Direct module URLs remain explicit split-deployment
  overrides through `SDKWORK_IM_*_APP_API_UPSTREAM` keys documented in `component.spec.json`.
- Consumers should integrate through public exports, runtime entrypoints, SDK clients, or adapters declared in the manifest.
- Generated SDK language outputs are represented at their SDK family root instead of duplicating local specs in generated folders.

## Platform Framework Alignment

| Framework | Status | Integration point |
| --- | --- | --- |
| `sdkwork-web-framework` | **Integrated** | Gateway (`services/sdkwork-im-cloud-gateway`) and upstream HTTP services wrap routers through `crates/sdkwork-im-web-bootstrap` (`WebFrameworkLayer`, `ImAppContextInjector`, IAM resolver). OpenAPI authorities materialize `x-sdkwork-request-context` / `x-sdkwork-api-surface`. Verified by `pnpm test:web-framework-standard`. |
| `sdkwork-database` | **Integrated** | `Cargo.toml` workspace deps (`sdkwork-database-config`, `sdkwork-database-sqlx`); pool bootstrap in `crates/sdkwork-im-database-pool`; postgres adapters consume unified pool config. Verified by `pnpm test:database-framework-standard`. |
| `sdkwork-utils` | **Integrated** | `Cargo.toml` workspace dep (`sdkwork-utils-rust`); PC core and H5 core consume `@sdkwork/utils`; Flutter mobile consumes `sdkwork_common_flutter` through generated IM SDK HTTP stack. Crypto/encoding helpers must not duplicate `sha2` or ad-hoc base64url in shared runtime paths. Verified by `pnpm test:utils-standard` and `pnpm test:h5-utils-standard`. |
| `sdkwork-drive` | **Integrated** | File upload/download delegated to sibling `sdkwork-drive` at `/app/v3/api/drive/*`. PC/H5/Flutter chat media uploads share canonical attribution (`im_conversation`, `scene=im`, `source=chat_message`) per `specs/im-app-api-sdk-integration.spec.md`. PC uses `@sdkwork/drive-app-sdk` through `@sdkwork/im-pc-core`; H5 through `sdkwork-im-h5-core`; Flutter through IM-composed `drive_app_sdk_client.dart`. Verified by PC/H5 drive integration contract tests, `pnpm test:chat-drive-upload-attribution-standard`, and `services/media-service/tests/provider_integration_test.rs`. |

## Client App Composition

| Surface | App root | Core package | Composition verification |
| --- | --- | --- | --- |
| PC | `apps/sdkwork-im-pc` | `@sdkwork/im-pc-core` | `pnpm test:sdkwork-im-pc-architecture-standard` |
| H5 | `apps/sdkwork-im-h5` | `@sdkwork/im-h5-core` | `pnpm test:sdkwork-im-h5-architecture-standard` |
| Flutter mobile | `apps/sdkwork-im-flutter-mobile` | `sdkwork_im_flutter_mobile_core` | `pnpm test:sdkwork-im-flutter-mobile-architecture-standard`, `pnpm test:flutter-drive-standard` |

PC/H5 app roots stay thin: sibling SDK paths register once in repository-root `pnpm-workspace.yaml`; feature packages declare import closure in local `package.json`; cross-repository SDK access flows through each surface core package per `APP_COMPOSITION_SPEC.md`.

| `sdkwork-discovery` | **Phase 2 Deferred** | Phase 1 RPC hosts are complete: three `*-rpc-bin` services (`session-gateway-rpc-bin`/50051, `sdkwork-comms-conversation-rpc-bin`/50052, `sdkwork-comms-conversation-internal-rpc-bin`/50053) ship through `sdkwork-rpc-framework` with optional `SDKWORK_IM_DISCOVERY_ENDPOINT` registration. The `sdkwork-discovery` product control plane itself is not integrated and remains Phase 2. Phased adoption plan: [ADR-20260619-im-rpc-discovery-integration-deferred](../docs/architecture/decisions/ADR-20260619-im-rpc-discovery-integration-deferred.md). Verified by `pnpm test:rpc-contract`, `cargo test -p sdkwork-im-rpc-service-rust`, `pnpm test:sdkwork-im-session-gateway-rpc-bin`, and `pnpm test:session-gateway-rpc-bin-rust`. |

Sibling checkout and release refs are declared in `sdkwork.workflow.json` (`sdkwork-web-framework`, `sdkwork-database`, `sdkwork-utils`, `sdkwork-drive`, `sdkwork-iam`, `sdkwork-rpc-framework`).

## Canonical Specs

| Spec | Applies Because |
| --- | --- |
| [APP_MANIFEST_SPEC.md](../sdkwork-specs/APP_MANIFEST_SPEC.md) | sdkwork.app.config.json application registration rules. |
| [APPLICATION_SPEC.md](../sdkwork-specs/APPLICATION_SPEC.md) | Application shell and module composition. |
| [COMPONENT_SPEC.md](../sdkwork-specs/COMPONENT_SPEC.md) | Local component specs directory and manifest rules. |
| [CONFIG_SPEC.md](../sdkwork-specs/CONFIG_SPEC.md) | Runtime configuration, environment, SDK bootstrap, and feature flag rules. |
| [DATABASE_SPEC.md](../sdkwork-specs/DATABASE_SPEC.md) | Database table naming, table profiles, schema registry, and prefix governance. |
| [WEB_FRAMEWORK_SPEC.md](../sdkwork-specs/WEB_FRAMEWORK_SPEC.md) | Mandatory `sdkwork-web-framework` integration for HTTP gateway and API runtimes. |
| [WEB_BACKEND_SPEC.md](../sdkwork-specs/WEB_BACKEND_SPEC.md) | HTTP handler/service/repository layering after the web framework boundary. |
| [DEPENDENCY_MANAGEMENT_SPEC.md](../sdkwork-specs/DEPENDENCY_MANAGEMENT_SPEC.md) | Native workspace dependency declarations, sibling SDKWork source paths, and Git-backed release dependency refs. |
| [DEPLOYMENT_SPEC.md](../sdkwork-specs/DEPLOYMENT_SPEC.md) | SaaS/private/local runtime parity and deployment rules. |
| [DOCUMENTATION_SPEC.md](../sdkwork-specs/DOCUMENTATION_SPEC.md) | Module README, examples, ADR, changelog, and runbook rules. |
| [DOMAIN_SPEC.md](../sdkwork-specs/DOMAIN_SPEC.md) | Canonical domain ownership and naming. |
| [FRONTEND_SPEC.md](../sdkwork-specs/FRONTEND_SPEC.md) | UI, service, SDK, accessibility, and frontend runtime rules. |
| [GOVERNANCE_SPEC.md](../sdkwork-specs/GOVERNANCE_SPEC.md) | Standard ownership, exception, compatibility, and migration rules. |
| [I18N_SPEC.md](../sdkwork-specs/I18N_SPEC.md) | User-facing language, locale, message catalog, and fallback rules. |
| [MODULE_SPEC.md](../sdkwork-specs/MODULE_SPEC.md) | Reusable package contract and dependency direction. |
| [README.md](../sdkwork-specs/README.md) | SDKWork root standards entrypoint. |
| [SDK_SPEC.md](../sdkwork-specs/SDK_SPEC.md) | SDK generation and SDK integration rules. |
| [TEST_SPEC.md](../sdkwork-specs/TEST_SPEC.md) | Contract, frontend, SDK, security, parity, and documentation verification rules. |

## Public Exports

- Public exports are not declared in the package manifest.

## SDK Clients

- No generated SDK client class is declared at this component boundary.

## Local Extension Specs

- [im-app-api-sdk-integration.spec.md](./im-app-api-sdk-integration.spec.md) defines Sdkwork IM's IM API, IM app API, IM backend API, product SDK ownership, IAM login integration, shared database, local source-link development, and git-backed release dependency rules.
- [../docs/architecture/decisions/ADR-20260617-comms-service-naming-boundaries.md](../docs/architecture/decisions/ADR-20260617-comms-service-naming-boundaries.md) records canonical communication service ids, social/space ownership, and deprecated contact/interaction HTTP surfaces.
- [../docs/architecture/decisions/ADR-20260619-im-rpc-discovery-integration-deferred.md](../docs/architecture/decisions/ADR-20260619-im-rpc-discovery-integration-deferred.md) records Phase 1 RPC host completion (three `*-rpc-bin` services) and deferred Phase 2 `sdkwork-discovery` product integration.
- [database-prefix-registry.json](./database-prefix-registry.json) registers `im` as the controlled prefix for instant-messaging tables in the `im` app.
- [database-table-registry.json](./database-table-registry.json) lists the checked-in IM table contracts, table profiles, write owners, and migration source.
- [database-table-naming-standard.md](../docs/部署/database-table-naming-standard.md) documents the local naming policy: IM tables use `im_`; non-IM tables keep their own business prefix or approved legacy name.

## PC Client Packages

The PC client app lives under `apps/sdkwork-im-pc` and is composed of capability
packages following the SDKWork PC architecture segment. Canonical package naming:

- Console surface: `sdkwork-im-console-*` (normalized PC target `sdkwork-im-pc-console-*`).
- Admin surface: `sdkwork-im-admin-*` (normalized PC target `sdkwork-im-pc-admin-*`).
- PC-native capabilities: `sdkwork-im-pc-*`.

Historical `sdkwork-clawchat-*` package names were retired by the
`sdkwork-clawchat ? sdkwork-im` rebrand and must not be reintroduced.

## Verification

- `cargo test --workspace`
- `pnpm test:sdkwork-workspace-structure-standard`
- `node scripts/sdkwork-workspace-structure-standard.test.mjs`
- `pnpm test:web-framework-standard`
- `pnpm test:database-framework-standard`
- `pnpm test:utils-standard`
- `pnpm test:h5-utils-standard`
- `pnpm test:h5-drive-app-sdk-integration`
- `pnpm test:flutter-drive-standard`
- `pnpm check:api-response-envelope`
- `pnpm test:topology-baggage`
- `pnpm test:runtime-standard`
- `pnpm test:rtc-signaling-boundary`
- `pnpm test:rpc-contract`
- `pnpm check:dependency-management`
- `pnpm test:database-naming-standard`
- `pnpm test:component-spec-consistency`
- `pnpm test:runtime-id-standard`
- `pnpm test:deprecated-service-boundary`
- `pnpm test:apis-authority-standard`
- `pnpm test:deployment-docs-encoding`
- `pnpm test:governed-docs-encoding`
- `pnpm test:review-step-docs-encoding`
- `pnpm test:release-docs-encoding`
- `pnpm test:architecture-docs-encoding`
- `pnpm test:docs-strip-damage`
- `node ../sdkwork-app-topology/scripts/sdkwork-topology.mjs validate --root . --spec specs/topology.spec.json`
