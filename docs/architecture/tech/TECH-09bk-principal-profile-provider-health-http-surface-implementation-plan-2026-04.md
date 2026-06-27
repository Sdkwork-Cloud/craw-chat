> Migrated from `docs/架构/09BK-principal-profile-provider-health-http-surface-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# User-Module Provider Health HTTP Surface Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Expose principal-profile provider health over HTTP and freeze both healthy and unavailable contracts.

**Architecture:** Reuse the provider-health route pattern already used by RTC, media, and IoT. Keep route availability separate from provider availability by returning `ProviderHealthSnapshot.status`.

**Tech Stack:** Rust, Axum, Tokio, sdkwork-im-server integration tests.

---

### Task 1: Freeze the missing route

**Files:**
- Create: `services/sdkwork-im-cloud-gateway/tests/principal_profile_provider_http_test.rs`

- [ ] **Step 1: Write the failing tests**
- [ ] **Step 2: Run `cargo test -p sdkwork-im-cloud-gateway --offline --test principal_profile_provider_http_test -- --nocapture`**
Expected: FAIL with `404 != 200`

### Task 2: Add the HTTP surface

**Files:**
- Modify: `services/sdkwork-im-cloud-gateway/src/node.rs`
- Modify: `services/sdkwork-im-cloud-gateway/src/node/build.rs`
- Modify: `services/sdkwork-im-cloud-gateway/src/node/principal_profile.rs`

- [ ] **Step 1: Add `AppState` principal-profile health accessor**
- [ ] **Step 2: Add `principal_profile` health handler**
- [ ] **Step 3: Register `/backend/v3/api/principal_profile/provider_health`**
- [ ] **Step 4: Add minimal health details for local/external providers**

### Task 3: Remove env-test leakage

**Files:**
- Modify: `services/sdkwork-im-cloud-gateway/tests/principal_profile_provider_http_test.rs`
- Modify: `services/sdkwork-im-cloud-gateway/tests/principal_profile_provider_runtime_selection_test.rs`

- [ ] **Step 1: Add a per-binary async mutex**
- [ ] **Step 2: Guard env-mutating tests**
- [ ] **Step 3: Re-run focused tests**

### Task 4: Verify and backwrite

**Files:**
- Modify: `docs/review/README.md`
- Modify: `docs/step/README.md`
- Modify: `docs/架构/README.md`

- [ ] **Step 1: Run `cargo fmt --all --check`**
- [ ] **Step 2: Run `cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture`**
- [ ] **Step 3: Update review/design/step indexes**

