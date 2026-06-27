# IM SDK Multilanguage Expansion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Upgrade `sdks/sdkwork-im-sdk` into a production-grade three-language app SDK workspace for TypeScript, Flutter, and Rust, with a strict generated/manual ownership boundary and a professional Rust composed SDK layered above generated HTTP transport.

**Architecture:** Keep `generated/server-openapi` as the only generator-owned output in every language workspace. Extend the root IM SDK wrappers so they generate, normalize, verify, and assemble `typescript`, `flutter`, and `rust`, then add a manual-owned Rust `composed` crate that mirrors the existing TypeScript and Flutter consumer-facing `SdkworkImClient + modules + builders` pattern without duplicating DTOs or HTTP transport.

**Tech Stack:** OpenAPI 3.0.3, Node.js SDK generator, PowerShell, POSIX shell, Rust/Cargo, TypeScript, existing IM SDK verification scripts

---

### Task 1: Lock the root Rust support contract with failing automation checks

**Files:**
- Modify: `sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs`
- Modify: `sdks/sdkwork-im-sdk/bin/verify-powershell-wrapper-args.mjs`
- Modify: `sdks/sdkwork-im-sdk/README.md`
- Test: `sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs`
- Test: `sdks/sdkwork-im-sdk/bin/verify-powershell-wrapper-args.mjs`

- [ ] **Step 1: Write the failing test**

Extend the workspace automation checks so they require Rust to be treated as a first-class language. The assertions should fail until the documented default language set, wrapper examples, and workspace layout mention `rust`.

```js
expectReadmeToContain('- Rust: [sdkwork-im-sdk-rust]');
expectGenerateExamplesToContain('-Languages typescript,flutter,rust');
expectWrapperParserToAccept('rust');
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs
node sdks/sdkwork-im-sdk/bin/verify-powershell-wrapper-args.mjs
```

Expected: FAIL because the current workspace automation and documented wrapper examples only recognize `typescript` and `flutter`.

- [ ] **Step 3: Write minimal implementation**

Update the root README and wrapper-argument validation so the workspace contract explicitly includes `rust` as a supported language target.

- [ ] **Step 4: Run test to verify it passes**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs
node sdks/sdkwork-im-sdk/bin/verify-powershell-wrapper-args.mjs
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs sdks/sdkwork-im-sdk/bin/verify-powershell-wrapper-args.mjs sdks/sdkwork-im-sdk/README.md
git commit -m "test(sdk): lock rust workspace contract"
```

### Task 2: Scaffold the Rust workspace boundary and forwarding scripts

**Files:**
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/README.md`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/bin/sdk-gen.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/bin/sdk-gen.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/bin/sdk-verify.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/bin/sdk-verify.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/bin/sdk-assemble.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/bin/sdk-assemble.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/Cargo.toml`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/README.md`
- Test: `sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs`

- [ ] **Step 1: Write the failing test**

Strengthen `verify-sdk-automation.mjs` so it requires the Rust workspace boundary to exist and to follow the same root-forwarding pattern as the TypeScript and Flutter workspaces.

```js
requirePath('sdkwork-im-sdk-rust/README.md');
requirePath('sdkwork-im-sdk-rust/bin/sdk-gen.ps1');
requirePath('sdkwork-im-sdk-rust/composed/Cargo.toml');
```

- [ ] **Step 2: Run test to verify it fails**

Run: `node sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs`

Expected: FAIL because the Rust workspace does not exist yet.

- [ ] **Step 3: Write minimal implementation**

Create the Rust workspace skeleton and use the existing TypeScript and Flutter workspace scripts as the forwarding-script template. The Rust workspace README should document:

- `generated/server-openapi` is generator-owned
- `composed` is manual-owned
- root wrappers remain the canonical generate and verify entrypoints

- [ ] **Step 4: Run test to verify it passes**

Run: `node sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs`

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust
git commit -m "feat(sdk): scaffold rust workspace boundary"
```

### Task 3: Extend root generation, auth normalization, and assembly for Rust

**Files:**
- Modify: `sdks/sdkwork-im-sdk/bin/generate-sdk.ps1`
- Modify: `sdks/sdkwork-im-sdk/bin/generate-sdk.sh`
- Modify: `sdks/sdkwork-im-sdk/bin/normalize-generated-auth-surface.mjs`
- Modify: `sdks/sdkwork-im-sdk/bin/assemble-sdk.mjs`
- Modify: `sdks/sdkwork-im-sdk/bin/verify-auth-surface-alignment.mjs`
- Test: `sdks/sdkwork-im-sdk/bin/generate-sdk.ps1`
- Test: `sdks/sdkwork-im-sdk/bin/generate-sdk.sh`
- Test: `sdks/sdkwork-im-sdk/bin/assemble-sdk.mjs`
- Test: `sdks/sdkwork-im-sdk/bin/verify-auth-surface-alignment.mjs`

- [ ] **Step 1: Write the failing test**

Add Rust-specific expectations before implementation:

- root generation wrappers must accept `rust`
- the assembly file must emit a Rust language entry with generated and composed package metadata
- Rust generated auth surface must expose bearer-token auth only

Use concrete Rust auth assertions similar to:

```js
assertIncludes(generatedClientSource, 'pub fn set_auth_token');
assertNotIncludes(generatedClientSource, 'pub fn set_api_key');
assertNotIncludes(generatedClientSource, 'pub fn set_access_token');
assertNotIncludes(generatedReadmeSource, 'set_api_key("your-api-key")');
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-auth-surface-alignment.mjs --language rust
powershell -ExecutionPolicy Bypass -File sdks/sdkwork-im-sdk/bin/generate-sdk.ps1 -Languages rust
node sdks/sdkwork-im-sdk/bin/assemble-sdk.mjs --language rust
```

Expected: FAIL because the wrappers, auth normalization, and assembly logic do not yet support Rust.

- [ ] **Step 3: Write minimal implementation**

Add Rust to the root generation map and keep the generated Cargo package name stable:

```powershell
rust = @{
  OutputDir = Join-Path $WorkspaceDir "sdkwork-im-sdk-rust\generated\server-openapi"
  PackageName = "sdkwork-im-sdk-generated"
  Input = $PreparedInput
}
```

Normalize the generated Rust public surface after each generation run so the final generated package:

- keeps `set_auth_token`
- drops `set_api_key` and `set_access_token`
- documents bearer-only setup in `README.md`

Extend `assemble-sdk.mjs` so Rust assembly metadata records:

- generated manifest: `sdkwork-im-sdk-rust/generated/server-openapi/Cargo.toml`
- composed manifest: `sdkwork-im-sdk-rust/composed/Cargo.toml`
- entrypoint: `src/lib.rs`

- [ ] **Step 4: Run test to verify it passes**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-auth-surface-alignment.mjs --language rust
powershell -ExecutionPolicy Bypass -File sdks/sdkwork-im-sdk/bin/generate-sdk.ps1 -Languages rust
node sdks/sdkwork-im-sdk/bin/assemble-sdk.mjs --language rust
```

Expected: PASS, and `.sdkwork-assembly.json` now includes a Rust language entry.

- [ ] **Step 5: Commit**

```bash
git add sdks/sdkwork-im-sdk/bin/generate-sdk.ps1 sdks/sdkwork-im-sdk/bin/generate-sdk.sh sdks/sdkwork-im-sdk/bin/normalize-generated-auth-surface.mjs sdks/sdkwork-im-sdk/bin/assemble-sdk.mjs sdks/sdkwork-im-sdk/bin/verify-auth-surface-alignment.mjs sdks/sdkwork-im-sdk/.sdkwork-assembly.json
git commit -m "feat(sdk): add rust generation and assembly flow"
```

### Task 4: Add failing Rust composed smoke tests and core crate scaffolding

**Files:**
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/tests/sdkwork_im_client_test.rs`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src/lib.rs`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src/client.rs`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src/context.rs`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src/types.rs`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src/error.rs`
- Test: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/tests/sdkwork_im_client_test.rs`

- [ ] **Step 1: Write the failing test**

Create focused core-client tests that lock the minimal Rust composed crate contract:

- `ImClient::new(generated_client)` exists
- `ImClient::new_with_base_url(base_url)` exists
- `set_auth_token` is exposed on the composed client
- the composed crate re-exports the generated config and error types needed for app setup

Use a minimal compile-and-smoke harness:

```rust
use im_sdk::ImClient;
use sdkwork_im_sdk_generated::ImGeneratedClient;

#[test]
fn im_client_wraps_generated_client() -> Result<(), Box<dyn std::error::Error>> {
    let generated = ImGeneratedClient::new_with_base_url("http://127.0.0.1:18090")?;
    let sdk = ImClient::new(generated.clone());
    sdk.set_auth_token("token");
    let _ = sdk.generated_client();
    Ok(())
}

#[test]
fn sdkwork_im_client_can_be_built_from_base_url() -> Result<(), Box<dyn std::error::Error>> {
    let sdk = SdkworkImClient::new_with_base_url("http://127.0.0.1:18090")?;
    sdk.set_auth_token("token");
    Ok(())
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
cargo test --manifest-path sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/Cargo.toml sdkwork_im_client_wraps_generated_backend_client -- --exact
cargo test --manifest-path sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/Cargo.toml sdkwork_im_client_can_be_built_from_base_url -- --exact
```

Expected: FAIL because the composed Rust crate and its public client surface do not exist yet.

- [ ] **Step 3: Write minimal implementation**

Create the composed crate core:

- `Cargo.toml` package name: `im-sdk`
- lib crate name: `im_sdk`
- dependency on generated crate via `path = "../generated/server-openapi"`
- core exports in `src/lib.rs`
- `SdkworkImClient` with `new`, `new_with_base_url`, and `set_auth_token`
- shared context that owns `SdkworkBackendClient`

- [ ] **Step 4: Run test to verify it passes**

Run the three focused test commands from Step 2.

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/Cargo.toml sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/tests/sdkwork_im_client_test.rs
git commit -m "test(rust-sdk): scaffold composed client core"
```

### Task 5: Implement Rust business modules and convenience builders

**Files:**
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src/builders.rs`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src/session_module.rs`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src/presence_module.rs`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src/realtime_module.rs`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src/device_module.rs`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src/inbox_module.rs`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src/conversations_module.rs`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src/messages_module.rs`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src/media_module.rs`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src/streams_module.rs`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src/rtc_module.rs`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src/client.rs`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src/lib.rs`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/tests/sdkwork_im_client_test.rs`
- Test: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/tests/sdkwork_im_client_test.rs`

- [ ] **Step 1: Write the failing test**

Add the rest of the composed SDK contract to the smoke tests:

- `sdk.conversations().post_text(...)`
- `sdk.streams().append_text_frame(...)`
- `sdk.calls.sendSignal(...)`
- module accessors for `session`, `presence`, `realtime`, `devices`, `inbox`, `messages`, `media`

Builder expectations should be concrete:

```rust
let request = build_text_message("hello world", PostTextOptions {
    client_msg_id: Some("client-1".into()),
    summary: Some("Greeting".into()),
    render_hints: Some(hashmap([("tone", "friendly")])),
});
assert_eq!(request.text.as_deref(), Some("hello world"));

let frame = build_text_stream_frame(7, "partial chunk", TextFrameOptions {
    schema_ref: Some("urn:sdkwork-im:stream:text".into()),
    attributes: Some(hashmap([("role", "assistant")])),
});
assert_eq!(frame.frame_type, "text");
assert_eq!(frame.encoding, "text/plain; charset=utf-8");

let signal = build_json_rtc_signal("offer", &json!({"sdp":"v=0","type":"offer"}), JsonRtcSignalOptions {
    schema_ref: Some("urn:sdkwork-im:rtc:signal".into()),
    signaling_stream_id: Some("signal-stream-1".into()),
});
assert_eq!(signal.signal_type, "offer");
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
cargo test --manifest-path sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/Cargo.toml --test sdkwork_im_client_test
```

Expected: FAIL because the module accessors and builder helpers are still missing.

- [ ] **Step 3: Write minimal implementation**

Implement the composed modules as thin wrappers over the generated APIs. Keep them semantic, not magical:

- `conversations_module.rs`
  `post_text`, `create`, `list_messages`, `list_members`, `update_read_cursor`
- `streams_module.rs`
  `append_text_frame`, `open`, `list_frames`, `checkpoint`, `complete`, `abort`
- `rtc_module.rs`
  `post_json_signal`, `create_session`, `invite`, `accept`, `reject`, `end`
- `builders.rs`
  `build_text_message`, `build_text_stream_frame`, `build_json_rtc_signal`

The implementation should always reuse generated request structs:

```rust
let body = PostMessageRequest {
    client_msg_id: options.client_msg_id,
    summary: options.summary,
    text: Some(text.into()),
    parts: None,
    render_hints: options.render_hints,
};

self.context.backend_client().conversation()
    .post_conversation_message(conversation_id, &body)
    .await
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --manifest-path sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/Cargo.toml --test sdkwork_im_client_test`

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/tests/sdkwork_im_client_test.rs
git commit -m "feat(rust-sdk): add composed modules and builders"
```

### Task 6: Add Rust workspace verification and wire it into the root verify path

**Files:**
- Create: `sdks/sdkwork-im-sdk/bin/verify-rust-workspace.mjs`
- Create: `sdks/sdkwork-im-sdk/bin/verify-rust-package-metadata.mjs`
- Create: `sdks/sdkwork-im-sdk/bin/verify-rust-public-api-boundary.mjs`
- Modify: `sdks/sdkwork-im-sdk/bin/verify-sdk.mjs`
- Modify: `sdks/sdkwork-im-sdk/bin/verify-sdk.ps1`
- Modify: `sdks/sdkwork-im-sdk/bin/verify-sdk.sh`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/README.md`
- Test: `sdks/sdkwork-im-sdk/bin/verify-rust-workspace.mjs`
- Test: `sdks/sdkwork-im-sdk/bin/verify-sdk.mjs`

- [ ] **Step 1: Write the failing test**

Create the Rust workspace verification entrypoint and make the root verify wrapper call it when `--language rust` is requested.

The Rust workspace verifier should require:

- generated crate `Cargo.toml`, `README.md`, `sdkwork-sdk.json` exist
- generated crate passes `node ./bin/publish-core.mjs --language rust --project-dir . --action check`
- generated crate passes `node ./bin/publish-core.mjs --language rust --project-dir . --action build`
- composed crate passes `cargo test`
- composed public imports stay on the generated crate root import path

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-rust-workspace.mjs
node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language rust
```

Expected: FAIL because the Rust verification entrypoint and root Rust verify wiring do not exist yet.

- [ ] **Step 3: Write minimal implementation**

Implement `verify-rust-workspace.mjs` with the same orchestration style used by `verify-typescript-workspace.mjs` and `verify-flutter-workspace.mjs`:

```js
run('node', ['./bin/publish-core.mjs', '--language', 'rust', '--project-dir', '.', '--action', 'check'], { cwd: generatedDir });
run('node', ['./bin/publish-core.mjs', '--language', 'rust', '--project-dir', '.', '--action', 'build'], { cwd: generatedDir });
run('cargo', ['test', '--manifest-path', composedCargoToml], { cwd: workspaceRoot });
run('node', [path.join(scriptDir, 'verify-rust-package-metadata.mjs')], { cwd: workspaceRoot });
run('node', [path.join(scriptDir, 'verify-rust-public-api-boundary.mjs')], { cwd: workspaceRoot });
```

- [ ] **Step 4: Run test to verify it passes**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-rust-workspace.mjs
node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language rust
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add sdks/sdkwork-im-sdk/bin/verify-rust-workspace.mjs sdks/sdkwork-im-sdk/bin/verify-rust-package-metadata.mjs sdks/sdkwork-im-sdk/bin/verify-rust-public-api-boundary.mjs sdks/sdkwork-im-sdk/bin/verify-sdk.mjs sdks/sdkwork-im-sdk/bin/verify-sdk.ps1 sdks/sdkwork-im-sdk/bin/verify-sdk.sh sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/README.md
git commit -m "feat(sdk): add rust workspace verification"
```

### Task 7: Align workspace documentation and release metadata across all three languages

**Files:**
- Modify: `sdks/sdkwork-im-sdk/README.md`
- Modify: `sdks/sdkwork-im-sdk/.sdkwork-assembly.json`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/README.md`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/README.md`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/README.md`
- Test: `sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs`

- [ ] **Step 1: Write the failing test**

Extend `verify-sdk-automation.mjs` so it requires every language README to describe the same generated/manual ownership rule and for the root README to list all three language workspaces and package names.

- [ ] **Step 2: Run test to verify it fails**

Run: `node sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs`

Expected: FAIL because the README set is not yet fully aligned around the three-language family.

- [ ] **Step 3: Write minimal implementation**

Update the documentation set so it consistently states:

- TypeScript generated package: `@sdkwork-internal/im-sdk-generated`
- TypeScript composed package: `@sdkwork/im-sdk`
- Flutter generated package: `im_sdk_generated`
- Flutter composed package: `im_sdk_composed`
- Rust generated package: `sdkwork-im-sdk-generated`
- Rust composed package: `im-sdk`
- websocket transport remains documented, not implemented

- [ ] **Step 4: Run test to verify it passes**

Run: `node sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs`

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add sdks/sdkwork-im-sdk/README.md sdks/sdkwork-im-sdk/.sdkwork-assembly.json sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/README.md sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/README.md sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/README.md
git commit -m "docs(sdk): align trilanguage workspace documentation"
```

### Task 8: Run full generation and verification for the commercial-grade SDK workspace

**Files:**
- Modify: `docs/superpowers/specs/2026-04-14-im-sdk-multilanguage-expansion-design.md`
- Modify: `docs/superpowers/plans/2026-04-14-im-sdk-multilanguage-expansion.md`

- [ ] **Step 1: Run focused Rust verification**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-auth-surface-alignment.mjs --language rust
node sdks/sdkwork-im-sdk/bin/verify-rust-workspace.mjs
cargo test --manifest-path sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/Cargo.toml
```

Expected: PASS

- [ ] **Step 2: Run full workspace generation**

Run:

```powershell
powershell -ExecutionPolicy Bypass -File sdks/sdkwork-im-sdk/bin/generate-sdk.ps1 -Languages typescript,flutter,rust
```

Expected: PASS and regenerate all three language workspaces without hand-editing generated files.

- [ ] **Step 3: Run full workspace verification**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language typescript --language flutter --language rust
```

Expected: PASS

- [ ] **Step 4: Run direct language-specific spot checks**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-typescript-workspace.mjs
node sdks/sdkwork-im-sdk/bin/verify-flutter-workspace.mjs
node sdks/sdkwork-im-sdk/bin/verify-rust-workspace.mjs
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add docs/superpowers/specs/2026-04-14-im-sdk-multilanguage-expansion-design.md docs/superpowers/plans/2026-04-14-im-sdk-multilanguage-expansion.md
git commit -m "docs(sdk): capture trilanguage verification baseline"
```
