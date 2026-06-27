> Migrated from `docs/superpowers/plans/2026-04-15-craw-chat-typescript-sdk-next-gen-refactor.md` on 2026-06-24.
> Owner: SDKWork maintainers

# IM TypeScript SDK Next-Generation Refactor Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the transitional TypeScript SDK surface with the approved next-generation SDK contract built around synchronous client construction, `auth/messages/live/sync/rtc` domains, context-first receive APIs, and documentation that matches the real package behavior.

**Architecture:** Keep the generated OpenAPI transport under `src/generated/**`, but move the public experience into handwritten semantic modules. The root client becomes a thin orchestrator over explicit domains, while live push and durable catch-up become separate public concepts. The tests drive the new public contract first so refactors do not leak transport-oriented naming back into the package.

**Tech Stack:** TypeScript, Node.js ESM, handwritten semantic SDK modules, generated OpenAPI transport, Vite/tsc build, VitePress markdown docs, Node smoke tests.

---

### Task 1: Lock The New Public Contract With Failing Tests

**Files:**
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/test/sdkwork-im-client.test.mjs`
- Assemble to: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/test/sdkwork-im-client.test.mjs`

- [ ] **Step 1: Write failing tests for the new root construction and domain layout**

Add tests that expect:
- `new ImSdkClient({ baseUrl, authToken })` to work synchronously
- `sdk.auth` to exist and absorb portal-auth operations
- `sdk.messages.createText(...)` and `sdk.messages.send(...)` to be the primary path
- `sdk.connect(...)` to exist as the main live entrypoint
- `sdk.sync.catchUp(...)` and `sdk.sync.ack(...)` to exist as the durable entrypoints

- [ ] **Step 2: Run the focused smoke file and confirm the failures are real contract gaps**

Run: `npm --prefix sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript run smoke`

Expected: FAIL because the new public APIs do not exist yet.

- [ ] **Step 3: Write failing tests for context-first live receive**

Add tests that expect:
- `live.onMessage((ctx) => ...)`
- `live.onConversationMessage(id, (ctx) => ...)`
- `ctx.message`, `ctx.sequence`, `ctx.source`, `ctx.rawEvent`
- optional `ctx.ack()` for catch-up driven events where manual ack applies

- [ ] **Step 4: Re-run the focused smoke file**

Run: `npm --prefix sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript run smoke`

Expected: FAIL on missing `connect()` / `live` / context-first receive behavior.

- [ ] **Step 5: Write failing tests for the expanded standardized message families**

Add tests that expect typed creation and decode coverage for:
- `ai_text`
- `ai_image_generation`
- `ai_video_generation`
- `agent_state`
- `agent_handoff`
- `tool_result`
- `workflow_event`

- [ ] **Step 6: Re-run the focused smoke file**

Run: `npm --prefix sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript run smoke`

Expected: FAIL on missing message families and builder coverage.

### Task 2: Refactor Root Construction And Auth Domain

**Files:**
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src/sdk.ts`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src/sdk-context.ts`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src/types.ts`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src/auth-module.ts`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src/index.ts`

- [ ] **Step 1: Implement synchronous top-level construction**

Support `new ImSdkClient({ baseUrl, apiBaseUrl, websocketBaseUrl, authToken, tokenProvider, backendClient, webSocketFactory })` directly, without requiring `static create(...)` for the standard path.

- [ ] **Step 2: Move public auth operations under `sdk.auth`**

Expose semantic auth APIs:
- `login(...)`
- `useToken(...)`
- `clearToken()`
- `me()`

Allow auth internals to keep using generated `auth` and `portal` groups as needed.

- [ ] **Step 3: Narrow the root exports**

Remove internal-first exports from the public barrel where possible:
- `sdk-context`
- `receiver`
- `websocket-receiver`
- transport-oriented compatibility aliases as primary guidance

- [ ] **Step 4: Run the smoke test**

Run: `npm --prefix sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript run smoke`

Expected: root construction and auth tests PASS; remaining failures are in messaging/live/sync areas.

### Task 3: Move Messaging Into The Messages Domain

**Files:**
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src/messages-module.ts`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src/builders.ts`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src/message-codec.ts`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src/message-standards.ts`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src/types.ts`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src/sdk.ts`

- [ ] **Step 1: Implement `sdk.messages.createXxx()` and `sdk.messages.send()`**

Move the outbound builder/send surface under `messages`, keeping the root client thin.

- [ ] **Step 2: Add the missing message families**

Implement builders, decoded models, and summary rules for:
- `ai_text`
- `ai_image_generation`
- `ai_video_generation`
- `agent_state`
- `agent_handoff`
- `tool_result`
- `workflow_event`

- [ ] **Step 3: Keep media send ergonomics first-class**

Preserve:
- upload-first validation
- `sdk.messages.uploadAndSend(...)`
- standardized `mediaAssetId` handling

- [ ] **Step 4: Run the smoke test**

Run: `npm --prefix sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript run smoke`

Expected: messaging tests PASS; remaining failures are in live/sync orchestration.

### Task 4: Replace Transport-Leaning Receive APIs With `live` And `sync`

**Files:**
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src/live-module.ts`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src/sync-module.ts`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src/websocket-receiver.ts`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src/receiver.ts`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src/realtime-module.ts`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src/sdk.ts`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src/types.ts`

- [ ] **Step 1: Add `sdk.connect(...)` and return a live runtime object**

The live runtime should own WebSocket connection behavior and subscription sync for the standard app path.

- [ ] **Step 2: Convert semantic handlers to context-first shape**

Expose:
- `live.onMessage((ctx) => {})`
- `live.onData((ctx) => {})`
- `live.onSignal((ctx) => {})`
- `live.onConversationMessage(id, (ctx) => {})`

The context must include semantic payload, sequence, source, raw event, and ack handle where relevant.

- [ ] **Step 3: Separate durable replay under `sdk.sync`**

Expose:
- `catchUp(...)`
- `ack(...)`
- `replayConversation(...)` if the existing backend surface can support it cleanly, otherwise add a typed placeholder or clearly scoped method boundary that does not lie about availability

- [ ] **Step 4: Keep internal adapters but stop teaching `receiver.pull()` as the public model**

Existing low-level receiver code can remain as implementation detail if it makes the refactor cheaper and safer.

- [ ] **Step 5: Run the smoke test**

Run: `npm --prefix sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript run smoke`

Expected: live and sync tests PASS.

### Task 5: Align The Published Docs With The Real SDK

**Files:**
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md`
- Modify: `docs/sites/sdk/typescript-sdk.md`
- Modify: `docs/sites/sdk/app-sdk.md`
- Modify: `docs/sites/sdk/index.md`
- Modify: `docs/sites/api-reference/app/messages.md`
- Modify: `docs/sites/api-reference/app/session-and-realtime.md`
- Modify: any closely related SDK reference pages needed for consistency

- [ ] **Step 1: Rewrite the quick start around synchronous construction**

Show:
- `new ImSdkClient({...})`
- split `apiBaseUrl` / `websocketBaseUrl`
- auth through `sdk.auth`

- [ ] **Step 2: Rewrite messaging examples around `sdk.messages.createXxx()`**

Cover:
- text
- media
- custom
- AI-generation families
- agent/workflow families

- [ ] **Step 3: Rewrite realtime examples around `sdk.connect()` and `sdk.sync`**

Clarify the difference between:
- live push
- durable catch-up
- RTC signaling in live flows

- [ ] **Step 4: Run the docs verification scripts that are already part of this workspace**

Run the relevant local verification commands from `docs/sites/scripts` and SDK verification scripts as available.

### Task 6: Verify The Workspace And Re-Audit The Package

**Files:**
- No new source files required

- [ ] **Step 1: Assemble the single package**

Run: `node sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/bin/assemble-single-package.mjs`

- [ ] **Step 2: Run root package typecheck**

Run: `npm --prefix sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript run typecheck`

- [ ] **Step 3: Run root package smoke tests**

Run: `npm --prefix sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript run smoke`

- [ ] **Step 4: Run SDK workspace verification**

Run: `node sdks/sdkwork-im-sdk/bin/verify-typescript-workspace.mjs`

- [ ] **Step 5: Re-audit remaining public-surface gaps**

Confirm the earlier review issues are resolved:
- no async-only primary construction
- no root-level message-first API leakage
- no public transport-first receive standard
- auth naming normalized
- missing message families filled
- docs consistent with implementation

