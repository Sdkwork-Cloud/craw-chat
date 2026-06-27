> Migrated from `docs/superpowers/specs/2026-04-09-craw-chat-sdk-generation-design.md` on 2026-06-24.
> Owner: SDKWork maintainers

# IM SDK Generation Design

## Goal

Build professional app-facing SDK workspaces for `sdkwork-im` TypeScript and Flutter by introducing an OpenAPI 3.x authority contract, a generator-compatible derived spec, root regeneration wrappers, and language workspaces that keep generated output isolated from manual-owned files.

## Decisions

- Contract source: use a checked-in OpenAPI 3.x snapshot in `sdks/sdkwork-im-sdk/openapi/`.
- Canonical runtime surface: public app-facing routes from `services/local-minimal-node/src/node/build.rs`.
- SDK scope: include app-facing IM, session, realtime HTTP, conversation, message, media, stream, and RTC endpoints.
- Excluded scope: admin, ops, audit, IoT, provider health, and other control-plane-only endpoints.
- Auth model: use bearer auth for public app access; do not model trusted internal headers as the public contract.
- Workspace shape: mirror the mature `apps/openchat/sdkwork-im-sdk` pattern with root wrappers and per-language layered workspaces.
- Generator boundary: only `generated/server-openapi` is generator-owned. Manual docs and wrapper scripts stay outside generated output.
- Realtime websocket treatment: document `/im/v3/api/realtime/ws` in the authority contract and workspace docs, but do not promise a handwritten websocket adapter in this round.

## Architecture

The workspace root `sdks/sdkwork-im-sdk` will own the OpenAPI source, regeneration wrappers, and release-facing documentation. Each language workspace will receive generator output under `generated/server-openapi`, preserving a clean boundary for future handwritten adapters or composed SDK layers. The initial round focuses on correct HTTP SDK generation and professional regeneration ergonomics rather than speculative custom realtime clients.

## Files And Responsibilities

- `sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml`
  Authority OpenAPI 3.x contract for the app-facing sdkwork-im surface.
- `sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.sdkgen.yaml`
  Generator-compatible derived spec consumed by `sdkwork-sdk-generator`.
- `sdks/sdkwork-im-sdk/openapi/README.md`
  Documents authority-vs-derived ownership and offline regeneration rules.
- `sdks/sdkwork-im-sdk/bin/generate-sdk.ps1`
  Root Windows regeneration entrypoint for TypeScript and Flutter.
- `sdks/sdkwork-im-sdk/bin/generate-sdk.sh`
  Root POSIX regeneration entrypoint for TypeScript and Flutter.
- `sdks/sdkwork-im-sdk/bin/prepare-openapi-source.mjs`
  Normalizes the checked-in authority spec into the derived sdkgen input.
- `sdks/sdkwork-im-sdk/bin/assemble-sdk.mjs`
  Refreshes workspace metadata after generation.
- `sdks/sdkwork-im-sdk/README.md`
  Workspace-level documentation for scope, boundaries, regeneration, and packaging.
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md`
  TypeScript workspace documentation and generation entrypoints.
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/bin/*`
  Thin forwarding scripts to root regeneration and assembly commands.
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/README.md`
  Flutter workspace documentation and generation entrypoints.
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/bin/*`
  Thin forwarding scripts to root regeneration and assembly commands.
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/generated/server-openapi/*`
  Generator-owned TypeScript SDK output.
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/generated/server-openapi/*`
  Generator-owned Flutter SDK output.

## Contract Strategy

- Model the full route list from `build.rs` for the chosen app surface.
- Prefer request and response shapes from the concrete Rust domain and service structs already used by handlers.
- Use semantically clear operation IDs and tags so generated clients read like an SDK instead of raw endpoint mirrors.
- Normalize common errors into a shared `ErrorResponse` schema.
- Preserve websocket endpoint visibility in the authority contract, with a note that the generated SDK owns only HTTP capability in this round.

## Verification Strategy

- Regenerate both TypeScript and Flutter SDKs through `sdkwork-sdk-generator` from the derived spec.
- Verify the generated directories are present and stable under `generated/server-openapi`.
- Run language-appropriate validation where feasible:
  - TypeScript: package metadata inspection and build or type-check if available in generated output.
  - Flutter: `dart analyze` or `flutter analyze` against the generated package if the local toolchain is available.
- Re-run generation after wrapper stabilization to confirm idempotent output.

## Risks And Controls

- Risk: hand-written OpenAPI drift from runtime behavior.
  Control: derive paths and primary schemas from actual route handlers, runtime structs, and e2e examples.
- Risk: generated output gets hand-edited later.
  Control: keep output isolated under `generated/server-openapi` and document ownership clearly.
- Risk: TypeScript and Flutter packages diverge in versioning or contract.
  Control: resolve one SDK version at the root wrapper and pass `--fixed-sdk-version` to both generation runs.
- Risk: websocket expectations exceed generated SDK capability.
  Control: explicitly document websocket support boundaries in workspace docs and do not ship a fake adapter.

