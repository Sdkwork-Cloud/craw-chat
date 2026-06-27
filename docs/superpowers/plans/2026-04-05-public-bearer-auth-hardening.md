# Public Bearer Auth Hardening Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Close the public-route auth hardening wave by requiring signed public bearer tokens everywhere, updating all affected tests, and documenting the deploy/runtime contract.

**Architecture:** Public HTTP entrypoints now use a strict verifier that only accepts HS256 bearer tokens signed with `SDKWORK_IM_PUBLIC_BEARER_HS256_SECRET`. Test suites must stop using `alg=none` fixtures, must provision the public secret explicitly, and must serialize env-mutation access inside each test binary because bearer verification now depends on process environment.

**Tech Stack:** Rust, Axum, Tokio, serde_json, HMAC-SHA256 bearer verification.

---

### Task 1: Capture the regression surface

**Files:**
- Modify: `services/*/tests/public_auth_test.rs`
- Test: `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`

- [ ] **Step 1: Run one representative public auth suite before edits**

Run: `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`
Expected: FAIL because unsigned bearer now returns `401` instead of reaching permission checks.

- [ ] **Step 2: Enumerate all public auth suites still using unsigned fixtures**

Run: `rg -n "DEMO_BEARER|OWNER_BEARER|Bearer ey" services -g "*public_auth_test.rs"`
Expected: list every suite still using `alg=none` bearer constants.

### Task 2: Migrate public auth suites to signed bearer helpers

**Files:**
- Modify: `services/audit-service/tests/public_auth_test.rs`
- Modify: `services/automation-service/tests/public_auth_test.rs`
- Modify: `services/control-plane-api/tests/public_auth_test.rs`
- Modify: `services/conversation-runtime/tests/public_auth_test.rs`
- Modify: `services/media-service/tests/public_auth_test.rs`
- Modify: `services/notification-service/tests/public_auth_test.rs`
- Modify: `services/ops-service/tests/public_auth_test.rs`
- Modify: `services/projection-service/tests/public_auth_test.rs`
- Modify: `services/im-call-runtime/tests/public_auth_test.rs`
- Modify: `services/session-gateway/tests/public_auth_test.rs`
- Modify: `services/streaming-service/tests/public_auth_test.rs`
- Modify: `services/local-minimal-node/tests/public_auth_e2e_test.rs`

- [ ] **Step 1: Add a shared per-file helper pattern**

Add:
- `TEST_PUBLIC_SECRET`
- `public_auth_guard()`
- `configure_public_bearer_secret()`
- `bearer(claims)`

- [ ] **Step 2: Replace all unsigned bearer constants with signed helper calls**

Expected: every request that intends to exercise authorization logic uses an HS256 token signed with the configured test secret.

- [ ] **Step 3: Re-run representative suites after each batch**

Run:
- `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`
- `cargo test -p automation-service --offline --test public_auth_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test public_auth_e2e_test -- --nocapture`

Expected: PASS.

### Task 3: Document the public bearer contract

**Files:**
- Add: `docs/架构/48-公网上行Bearer必须进行签名校验标准-2026-04-05.md`

- [ ] **Step 1: Record the runtime contract**

Document:
- public routes reject `alg=none`
- secret source is `SDKWORK_IM_PUBLIC_BEARER_HS256_SECRET`
- trusted identity headers remain internal-only
- local deployment scripts must provision the secret

### Task 4: Verification

**Files:**
- Modify: none
- Test: workspace commands

- [ ] **Step 1: Format workspace**

Run: `cargo fmt --all`
Expected: exit 0.

- [ ] **Step 2: Run targeted suites**

Run:
- `cargo test -p im-auth-context --offline -- --nocapture`
- `cargo test -p session-gateway --offline --test public_auth_test -- --nocapture`
- `cargo test -p media-service --offline --test public_auth_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test public_auth_e2e_test -- --nocapture`

Expected: PASS.

- [ ] **Step 3: Run broad offline verification**

Run: `cargo test --workspace --offline`
Expected: exit 0 or a narrowed, concrete remaining failure list for the next fix wave.
