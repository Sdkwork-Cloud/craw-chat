# IM SDK Multilanguage Expansion Design

## Goal

Extend `sdks/sdkwork-im-sdk` from a TypeScript and Flutter HTTP SDK workspace into a professional three-language app-facing SDK family covering TypeScript, Flutter, and Rust, with a strict split between generator-owned transport output and manual-owned composed SDK layers.

This design extends `docs/superpowers/specs/2026-04-09-im-sdk-generation-design.md` rather than replacing its contract-source decisions.

## Scope

This round covers:

- TypeScript generated transport SDK and composed product SDK
- Flutter generated transport SDK and composed product SDK
- Rust generated transport SDK and composed product SDK
- root generation, verification, assembly, and documentation workflows
- professional consumer-facing module surfaces for sessions, presence, realtime HTTP coordination, devices, inbox, conversations, messages, media, streams, and RTC

This round does not cover:

- admin control-plane SDKs
- ops, audit, or diagnostics routes
- websocket realtime adapter implementation
- local persistence, offline cache, or sync engine behavior beyond the current HTTP contract

## Design Principles

- Preserve the authority OpenAPI 3.x document as the only contract source.
- Keep all generator-owned output inside `generated/server-openapi`.
- Keep all manual-owned business-facing SDK code outside generated output.
- Do not hand-edit generated files to fix product concerns; fix generator inputs, generator templates, or root normalization.
- Keep the composed SDK thin but professional: semantic client entrypoint, business modules, auth helpers, and common builders.
- Keep cross-language consumer experience aligned where practical: same client naming, same module names, same builder semantics, and comparable README quality.

## Decisions

- Workspace family: `sdks/sdkwork-im-sdk` becomes a three-language app-facing SDK workspace for `typescript`, `flutter`, and `rust`.
- Contract source: continue using `openapi/sdkwork-im-im.openapi.yaml` as authority and derived sdkgen inputs under `openapi/`.
- Auth model: continue enforcing the public bearer-token model. Generated public auth surfaces must not drift toward trusted internal headers.
- Realtime boundary: generated SDKs continue to cover HTTP coordination only. WebSocket transport remains documented but not implemented as a handwritten adapter in this round.
- TypeScript and Flutter architecture: keep the existing layered shape of `generated/server-openapi + composed`.
- Rust architecture: add a new workspace following the same generated/manual split, using a generated Rust HTTP crate plus a thin manual composed crate.
- Assembly model: `.sdkwork-assembly.json` becomes the machine-readable catalog for all three language workspaces and their generated and composed packages.

## Workspace Architecture

The root workspace continues to own:

- authority and derived OpenAPI inputs
- root wrapper scripts for generation and verification
- language selection and unified version resolution
- post-generation normalization for public auth semantics
- workspace assembly metadata
- release-facing README and boundary documentation

Per-language workspaces own only language-specific packaging, composed SDK code, and thin forwarding scripts:

```text
sdkwork-im-sdk/
  openapi/
  bin/
  sdkwork-im-sdk-typescript/
  sdkwork-im-sdk-flutter/
  sdkwork-im-sdk-rust/
```

Each language workspace follows the same top-level pattern:

- `generated/server-openapi`
  Generator-owned transport package or crate
- `composed`
  Manual-owned consumer-facing SDK package or crate
- `bin/`
  Thin forwarding scripts back to root workspace wrappers
- `README.md`
  Manual-owned language documentation

## Language Ownership Model

### TypeScript

Keep the current mature layout:

- generated package: `sdkwork-im-sdk-typescript/generated/server-openapi`
- composed package: `sdkwork-im-sdk-typescript/composed`
- public package names:
  - generated: `@sdkwork-internal/im-sdk-generated`
  - composed: `@sdkwork/im-sdk`

The composed package remains the preferred consumer surface and continues to expose:

- `ImSdkClient`
- business modules
- message, stream, and RTC builders

### Flutter

Keep the current mature layout:

- generated package: `sdkwork-im-sdk-flutter/generated/server-openapi`
- composed package: `sdkwork-im-sdk-flutter/composed`
- public package names:
  - generated: `backend_sdk`
  - composed: `im_sdk`

The composed package remains the preferred consumer surface and continues to expose:

- `SdkworkImClient`
- business modules
- message, stream, and RTC builders

### Rust

Add a new language workspace:

```text
sdkwork-im-sdk-rust/
  README.md
  bin/
    sdk-assemble.ps1
    sdk-assemble.sh
    sdk-gen.ps1
    sdk-gen.sh
    sdk-verify.ps1
    sdk-verify.sh
  generated/
    server-openapi/
      Cargo.toml
      README.md
      sdkwork-sdk.json
      custom/
      .sdkwork/
      src/
  composed/
    Cargo.toml
    README.md
    src/
      lib.rs
      client.rs
      context.rs
      error.rs
      builders.rs
      types.rs
      session_module.rs
      presence_module.rs
      realtime_module.rs
      device_module.rs
      inbox_module.rs
      conversations_module.rs
      messages_module.rs
      media_module.rs
      streams_module.rs
      rtc_module.rs
    tests/
      sdkwork_im_client_test.rs
```

Rust follows the same policy boundary:

- `generated/server-openapi` is generator-owned and must not be hand-edited
- `composed` is manual-owned and provides the app-facing Rust SDK surface

## Rust Composed SDK Shape

The Rust composed crate should be intentionally thin. It should not duplicate generated models or re-implement HTTP transport.

The crate exposes:

- `SdkworkImClient`
- business modules for sessions, presence, realtime HTTP, devices, inbox, conversations, messages, media, streams, and RTC
- convenience builders for common text-message, stream-frame, and RTC-signal flows
- auth-token update helpers and ergonomic client-construction helpers

Recommended responsibilities:

- `client.rs`
  Defines `SdkworkImClient` and wires all module entrypoints around the generated client.
- `context.rs`
  Shared access to generated client state, config, and auth updates.
- `types.rs`
  Thin aliases, options structs, and generated-type re-exports needed for a stable consumer surface.
- `builders.rs`
  High-frequency builder helpers for common request payloads.
- `*_module.rs`
  Semantic business wrappers that call generated APIs without introducing new DTO forks.
- `error.rs`
  Thin composed-layer error mapping and ergonomic result aliases where useful.

Rust consumer ergonomics should mirror the existing TypeScript and Flutter patterns as closely as the language allows.

## Root Generation Flow

The root wrapper scripts remain the only supported generation entrypoints.

Required behavior:

1. Read the authority OpenAPI document.
2. Refresh the derived sdkgen inputs.
3. Resolve one unified SDK version.
4. Generate TypeScript, Flutter, and Rust `generated/server-openapi` outputs.
5. Apply language-appropriate normalization for public auth semantics and packaging boundaries.
6. Run per-language verification.
7. Refresh `.sdkwork-assembly.json`.

Rust becomes a first-class language in:

- `bin/generate-sdk.ps1`
- `bin/generate-sdk.sh`
- `bin/verify-sdk.mjs`
- `bin/assemble-sdk.mjs`

## Assembly Contract

`.sdkwork-assembly.json` must describe all generated and composed packages for the workspace.

For each language, record:

- workspace name
- generated package path
- generated manifest path
- generated package identity and entrypoints
- composed package identity and entrypoints when present

Rust should use the same assembly shape already used for TypeScript and Flutter so automation can reason about the workspace uniformly.

## Public SDK Contract

Across all three languages, the preferred consumer entrypoint is the composed SDK rather than the generated HTTP layer.

The composed SDK should provide:

- a clear `SdkworkImClient` entrypoint
- stable module names
- semantic helper methods for common message and realtime workflows
- simple bearer-token configuration
- README examples focused on real app integration rather than raw endpoint mirroring

The generated layer remains available for lower-level HTTP use, debugging, or extension work, but it is not the primary app-facing product surface.

## Verification Strategy

Completion requires verification beyond "generation succeeded".

### TypeScript

Preserve the existing verification chain:

- generated package build and artifact verification
- public API boundary checks
- dist cleanup checks
- determinism regression checks
- Windows concurrency regression checks
- composed package type-check, build, and smoke usage

### Flutter

Preserve the existing verification chain:

- generated-model regression checks
- bearer-auth surface alignment checks
- composed parity checks
- public API boundary checks
- package metadata checks
- optional native Dart analysis when explicitly requested

### Rust

Add a new root verification entrypoint, for example `bin/verify-rust-workspace.mjs`, that proves:

- generated crate metadata exists and is aligned with root assembly data
- generated crate passes `cargo check`
- composed crate passes `cargo test`
- composed crate only depends on supported generated public exports
- public `SdkworkImClient`, module exports, and builder exports compile and are exercised by smoke tests

## Testing Strategy

Manual-owned code follows TDD.

- Write Rust composed tests first.
- Watch them fail for the expected reason.
- Implement the minimal code to pass.
- Re-run tests and keep the workspace green.

Generated output itself is not hand-edited under TDD. Instead:

- generator regressions are covered in `sdkwork-sdk-generator`
- workspace integration is covered by generation and verification scripts

## Implementation Order

1. Upgrade `sdkwork-sdk-generator` where Rust generated output needs better public exports, metadata, or README behavior for composed consumption.
2. Add Rust-related generator tests for those changes.
3. Extend root generation, verification, and assembly scripts to treat Rust as a supported language.
4. Add the Rust workspace skeleton and forwarding scripts.
5. Implement the Rust composed crate with TDD.
6. Update root and language READMEs to document the three-language family and ownership rules.
7. Run workspace verification and confirm generation, verification, and assembly are repeatable.

## Risks And Controls

- Risk: Rust generated output is not cleanly consumable by a composed crate.
  Control: fix generator templates and tests first; do not hand-edit generated files.

- Risk: auth semantics drift away from the Sdkwork IM bearer-only public model.
  Control: keep auth normalization in the root wrapper flow and verify public surfaces explicitly.

- Risk: TypeScript, Flutter, and Rust diverge into unrelated SDK styles.
  Control: align on shared client naming, module naming, builder semantics, and README structure.

- Risk: work scope expands into websocket adapter or offline sync features.
  Control: keep realtime limited to HTTP coordination plus transport documentation in this round.

- Risk: manual SDK layers fork generated DTOs or transport behavior.
  Control: keep composed SDKs thin and reuse generated models and APIs directly.

## Success Criteria

The round is complete only when all of the following are true:

- the workspace supports `typescript`, `flutter`, and `rust`
- each language has a clean generated/manual ownership split
- the preferred public surface in each language is a composed `SdkworkImClient`
- Rust reaches the same professional quality bar as TypeScript and Flutter for entrypoint design, documentation, and verification
- generation is repeatable and does not rely on hand-editing generated output
- root documentation, language documentation, and assembly metadata all reflect the three-language family accurately
