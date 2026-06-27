# Sdkwork IM Unified API Ingress Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make `sdkwork-api-cloud-gateway` the only HTTP API ingress for Sdkwork IM while keeping `sdkwork-im-server` responsible only for static shell delivery and IM realtime/TCP transport.

**Architecture:** The migration is done in four phases. First fix the configuration and observability model so foundation gateway-backed services are not treated as ordinary product upstreams. Second make renderer HTTP API traffic explicitly target `sdkwork-api-cloud-gateway` while keeping realtime endpoints on `sdkwork-im-server`. Third strip generic HTTP ingress duties from `sdkwork-im-server`. Fourth tighten tests and startup output so the boundary cannot drift back.

**Tech Stack:** Rust 2024, Axum, Cargo workspace crates, Node/pnpm dev runners, TypeScript app bootstrap

---

## File Map

### Config and model ownership

- Modify: `crates/sdkwork-im-cloud-gateway-config/src/lib.rs`
  Purpose: separate direct product upstreams from shared foundation gateway-backed services in the configuration model.
- Modify: `crates/sdkwork-im-cloud-gateway-config/README.md`
  Purpose: document the new gateway-backed dependency model.

### Runtime observability and startup reporting

- Modify: `crates/sdkwork-im-cloud-gateway-observability/src/lib.rs`
  Purpose: format startup/runtime summaries with clear separation between direct upstreams and shared foundation gateway-backed services.
- Modify: `services/web-gateway/tests/openapi_index_test.rs`
  Purpose: lock the new startup/runtime summary semantics.

### Gateway routing and HTTP ingress

- Modify: `services/web-gateway/src/lib.rs`
  Purpose: ensure only API-related routes remain in the HTTP gateway responsibility set.
- Modify: `crates/sdkwork-api-product-runtime/src/lib.rs`
  Purpose: remove or reject generic HTTP API ingress responsibilities that should belong to `sdkwork-api-cloud-gateway`, while keeping static shell and realtime ownership.

### Product runtime and frontend bootstrap

- Modify: `scripts/dev/start-sdkwork-im-unified-web.mjs`
  Purpose: make local dev reflect the new ingress ownership while preserving static shell and realtime bootstrap.
- Modify: `scripts/dev/run-sdkwork-chat-pc-dev.mjs`
  Purpose: keep renderer API base URLs aligned with the gateway and realtime URLs aligned with `sdkwork-im-server`.
- Modify: `scripts/dev/sdkwork-chat-pc-dev-command.test.mjs`
  Purpose: verify dev topology ownership and environment propagation.
- Modify: `apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/appSdkClient.ts`
  Purpose: keep product app HTTP API calls on the gateway origin.
- Modify: `apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/imSdkClient.ts`
  Purpose: split HTTP API base URL from realtime websocket base URL so HTTP stays on gateway and realtime stays on `sdkwork-im-server`.

### Verification

- Modify: `scripts/dev/sdkwork-chat-pc-sdk-integration.test.mjs`
  Purpose: lock HTTP-vs-realtime base URL behavior.
- Run: `cargo test -p web-gateway --test openapi_index_test`
- Run: `node scripts/dev/sdkwork-chat-pc-dev-command.test.mjs`
- Run: `node scripts/dev/sdkwork-chat-pc-sdk-integration.test.mjs`

---

### Task 1: Reframe Gateway Config Ownership

**Files:**
- Modify: `crates/sdkwork-im-cloud-gateway-config/src/lib.rs`
- Modify: `crates/sdkwork-im-cloud-gateway-config/README.md`
- Test: `services/web-gateway/tests/openapi_index_test.rs`

- [ ] **Step 1: Write the failing config/summary test expectations**

Add assertions that gateway-backed foundation services are not treated as independent direct upstreams in startup summaries.

- [ ] **Step 2: Run the narrow test to verify the current model fails that expectation**

Run: `cargo test -p web-gateway --test openapi_index_test startup_summary_groups_foundation_gateway_backed_services_instead_of_listing_them_as_independent_upstreams -- --exact`
Expected: FAIL or reveal the old behavior if the config model is still flat.

- [ ] **Step 3: Introduce explicit foundation gateway-backed service classification**

Refactor `sdkwork-im-cloud-gateway-config` so appbase/drive/notary can be recognized as shared foundation gateway-backed dependency surfaces rather than product-local independent upstreams.

- [ ] **Step 4: Update the gateway-config README**

Document:
- direct product upstreams
- shared foundation gateway-backed services
- split override env keys as explicit exceptions only

- [ ] **Step 5: Re-run the narrow test**

Run: `cargo test -p web-gateway --test openapi_index_test startup_summary_groups_foundation_gateway_backed_services_instead_of_listing_them_as_independent_upstreams -- --exact`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add crates/sdkwork-im-cloud-gateway-config/src/lib.rs crates/sdkwork-im-cloud-gateway-config/README.md services/web-gateway/tests/openapi_index_test.rs
git commit -m "refactor: separate gateway-backed foundation service config"
```

### Task 2: Make Startup And Runtime Summary Match Real Ownership

**Files:**
- Modify: `crates/sdkwork-im-cloud-gateway-observability/src/lib.rs`
- Test: `services/web-gateway/tests/openapi_index_test.rs`

- [ ] **Step 1: Write/adjust failing assertions for startup summary wording**

Lock:
- `Shared Foundation Gateway`
- grouped service ids
- no repeated flat lines for appbase/drive/notary

- [ ] **Step 2: Run the test to verify current wording is wrong**

Run: `cargo test -p web-gateway --test openapi_index_test startup_summary_lists_gateway_openapi_endpoints -- --exact`
Expected: FAIL if wording/output still reflects flat upstreams.

- [ ] **Step 3: Implement the grouped startup summary**

Refactor `format_startup_summary(...)` to emit:
- direct product upstreams individually
- shared foundation gateway-backed services under one grouped section

- [ ] **Step 4: Run both openapi index tests**

Run:
`cargo test -p web-gateway --test openapi_index_test startup_summary_lists_gateway_openapi_endpoints -- --exact`
`cargo test -p web-gateway --test openapi_index_test startup_summary_groups_foundation_gateway_backed_services_instead_of_listing_them_as_independent_upstreams -- --exact`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/sdkwork-im-cloud-gateway-observability/src/lib.rs services/web-gateway/tests/openapi_index_test.rs
git commit -m "refactor: group gateway-backed foundation services in startup summary"
```

### Task 3: Split HTTP API Ingress From Realtime Ownership

**Files:**
- Modify: `crates/sdkwork-api-product-runtime/src/lib.rs`
- Modify: `services/web-gateway/src/lib.rs`
- Test: `services/web-gateway/tests/http_proxy_test.rs`
- Test: `services/web-gateway/tests/websocket_proxy_test.rs`

- [ ] **Step 1: Add failing assertions for the intended route split**

Lock the rule:
- `sdkwork-api-cloud-gateway` owns normal HTTP API ingress
- `sdkwork-im-server` keeps realtime transport only
- no extra generic HTTP proxy fallback from `sdkwork-im-server`

- [ ] **Step 2: Run the narrow gateway HTTP and realtime tests**

Run:
`cargo test -p web-gateway --test http_proxy_test gateway_derives_context_for_protected_routes_without_appbase_session_lookup -- --exact`
`cargo test -p web-gateway --test websocket_proxy_test gateway_derives_realtime_websocket_context_from_appbase_dual_tokens_not_client_headers -- --exact`
Expected: establish current baseline before changing behavior.

- [ ] **Step 3: Remove generic HTTP ingress duties from product runtime where applicable**

Change `sdkwork-api-product-runtime` so it no longer claims ownership of generic HTTP API ingress responsibilities that belong to `sdkwork-api-cloud-gateway`. Preserve:
- static shell/site routes
- realtime transport
- local desktop runtime routes that are not part of gateway-owned API ingress

- [ ] **Step 4: Keep gateway HTTP ownership explicit**

Ensure `services/web-gateway` remains the canonical place for:
- `/app/v3/api`
- `/backend/v3/api`
- normal `/im/v3/api` HTTP
- docs/OpenAPI endpoints

- [ ] **Step 5: Re-run the narrow gateway tests**

Run:
`cargo test -p web-gateway --test http_proxy_test gateway_derives_context_for_protected_routes_without_appbase_session_lookup -- --exact`
`cargo test -p web-gateway --test websocket_proxy_test gateway_derives_realtime_websocket_context_from_appbase_dual_tokens_not_client_headers -- --exact`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add crates/sdkwork-api-product-runtime/src/lib.rs services/web-gateway/src/lib.rs services/web-gateway/tests/http_proxy_test.rs services/web-gateway/tests/websocket_proxy_test.rs
git commit -m "refactor: separate gateway HTTP ingress from sdkwork-im realtime transport"
```

### Task 4: Split Renderer HTTP And Realtime Base URLs

**Files:**
- Modify: `apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/appSdkClient.ts`
- Modify: `apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/imSdkClient.ts`
- Modify: `scripts/dev/run-sdkwork-chat-pc-dev.mjs`
- Modify: `scripts/dev/start-sdkwork-im-unified-web.mjs`
- Test: `scripts/dev/sdkwork-chat-pc-sdk-integration.test.mjs`
- Test: `scripts/dev/sdkwork-chat-pc-dev-command.test.mjs`

- [ ] **Step 1: Add failing assertions for HTTP/gateway and realtime/server URL ownership**

Lock:
- app/api HTTP -> `sdkwork-api-cloud-gateway`
- IM HTTP -> `sdkwork-api-cloud-gateway`
- IM realtime websocket -> `sdkwork-im-server`

- [ ] **Step 2: Run the Node dev/integration contract tests**

Run:
`node scripts/dev/sdkwork-chat-pc-sdk-integration.test.mjs`
`node scripts/dev/sdkwork-chat-pc-dev-command.test.mjs`
Expected: FAIL once new stricter assertions are added.

- [ ] **Step 3: Refactor dev env resolution and SDK URL wiring**

Update:
- `run-sdkwork-chat-pc-dev.mjs`
- `start-sdkwork-im-unified-web.mjs`
- `appSdkClient.ts`
- `imSdkClient.ts`

So that:
- HTTP API URLs point at the gateway-owned API ingress
- realtime websocket URLs point at the realtime transport owner

- [ ] **Step 4: Re-run the Node contract tests**

Run:
`node scripts/dev/sdkwork-chat-pc-sdk-integration.test.mjs`
`node scripts/dev/sdkwork-chat-pc-dev-command.test.mjs`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/appSdkClient.ts apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/imSdkClient.ts scripts/dev/run-sdkwork-chat-pc-dev.mjs scripts/dev/start-sdkwork-im-unified-web.mjs scripts/dev/sdkwork-chat-pc-sdk-integration.test.mjs scripts/dev/sdkwork-chat-pc-dev-command.test.mjs
git commit -m "refactor: route HTTP APIs through gateway and realtime through sdkwork-im transport"
```

### Task 5: Verification And Cleanup

**Files:**
- Review only; no new source files unless a failing test forces one

- [ ] **Step 1: Run focused Rust verification**

Run:
`cargo test -p web-gateway --test openapi_index_test`
`cargo test -p web-gateway --test http_proxy_test gateway_derives_context_for_protected_routes_without_appbase_session_lookup -- --exact`
`cargo test -p web-gateway --test websocket_proxy_test gateway_derives_realtime_websocket_context_from_appbase_dual_tokens_not_client_headers -- --exact`

- [ ] **Step 2: Run focused Node verification**

Run:
`node scripts/dev/sdkwork-chat-pc-dev-command.test.mjs`
`node scripts/dev/sdkwork-chat-pc-sdk-integration.test.mjs`

- [ ] **Step 3: Record any remaining technical debt explicitly**

Only note:
- static hosting remains product-owned by design
- realtime transport remains product-owned by design
- any remaining split override env keys are transitional only

- [ ] **Step 4: Commit final verification-only adjustments if needed**

```bash
git add .
git commit -m "test: lock unified api ingress and realtime ownership boundaries"
```
