# 2026-04-06 Read Cursor Runtime Actor Kind Guard Review Cycle

## 1. Finding

### 1.1 High: `update_read_cursor(...)` still trusted actor id without validating actor kind

- Root cause:
  - previous waves hardened:
    - message mutations
    - agent handoff lifecycle
    - member governance writes
  - but read-cursor mutation still used an id-only runtime entry point:
    - `update_read_cursor(UpdateReadCursorCommand)`
  - inside runtime, the mutation resolved the active member by `principal_id` and proceeded directly.
  - unlike other hardened mutation paths, it never checked whether ingress auth `actor_kind` matched the resolved member `principal_kind`.
  - both public adapters still called the id-only runtime method:
    - `conversation-runtime` HTTP handler
    - `sdkwork-im-server` read-cursor handler
- Impact:
  - a caller could authenticate as:
    - actor id = `1`
    - actor kind = `agent`
  - if the actual active member was:
    - principal id = `1`
    - principal kind = `user`
  - the read cursor write still succeeded.
  - in local profile, that also meant audit was reachable after a spoofed actor-kind claim because rejection never happened at the runtime write boundary.

## 2. Reproduction

Three regression tests were added first:

- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - `test_read_cursor_rejects_actor_kind_mismatch_against_member_principal_kind`
- `services/conversation-runtime/tests/http_smoke_test.rs`
  - `test_read_cursor_over_http_rejects_actor_kind_mismatch`
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - `test_read_cursor_rejects_bearer_actor_kind_mismatch`

Red evidence:

- runtime unit layer failed to compile because actor-kind-aware API did not exist yet:
  - missing `update_read_cursor_with_actor_kind(...)`
- local profile e2e showed the real behavioral gap:
  - actual status = `200`
  - expected status = `403`

The HTTP-layer red was implied by the same id-only code path in `conversation-runtime` HTTP handlers and then confirmed green after the fix.

## 3. Fix Design

Read cursor updates are conversation-bound write mutations and must follow the same actor identity standard as message and governance writes.

Chosen design:

1. add actor-kind-aware runtime entry point:
   - `update_read_cursor_with_actor_kind(command, actor_kind)`
2. in runtime:
   - resolve active member
   - call `ensure_actor_kind_matches_member(...)`
   - only then mutate read cursor and append events
3. keep legacy id-only `update_read_cursor(...)` as a compatibility wrapper:
   - derive actor kind from runtime member truth
   - delegate into the actor-kind-aware method
4. update all untrusted adapters to pass raw ingress actor kind:
   - `conversation-runtime` HTTP route
   - `sdkwork-im-server` HTTP route

## 4. Implementation

- `services/conversation-runtime/src/lib.rs`
  - added `update_read_cursor_with_actor_kind(...)`
  - changed `update_read_cursor(...)` into a compatibility wrapper
  - hardened the runtime mutation with:
    - `resolve_active_member(...)`
    - `ensure_actor_kind_matches_member(...)`
  - updated HTTP read-cursor write handler to pass `auth.actor_kind`
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
  - updated read-cursor write handler to call `update_read_cursor_with_actor_kind(...)`
- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - added runtime-level mismatch regression test
- `services/conversation-runtime/tests/http_smoke_test.rs`
  - added HTTP mismatch regression test
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - added local bearer mismatch regression test

## 5. Verification

### Red

- `cargo test -p conversation-runtime --offline test_read_cursor_rejects_actor_kind_mismatch_against_member_principal_kind -- --exact`
  - failed because `update_read_cursor_with_actor_kind(...)` did not exist
- `cargo test -p sdkwork-im-cloud-gateway --offline test_read_cursor_rejects_bearer_actor_kind_mismatch -- --exact`
  - failed with status `200` instead of `403`

### Green

- `cargo test -p conversation-runtime --offline test_read_cursor_rejects_actor_kind_mismatch_against_member_principal_kind -- --exact`
- `cargo test -p conversation-runtime --offline test_read_cursor_over_http_rejects_actor_kind_mismatch -- --exact`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_read_cursor_rejects_bearer_actor_kind_mismatch -- --exact`

## 6. Remaining Risks

- other conversation-bound write APIs outside message, governance, and read cursor may still expose id-only runtime entry points.
- local profile audit surfaces still depend on raw auth context after successful writes; this is acceptable only because runtime now rejects mismatched actor kind before reaching those side effects.
- future adapters must be held to the actor-kind-aware runtime API instead of the compatibility wrapper.

## 7. Next Wave

1. continue auditing remaining conversation-bound writes, especially stream- and RTC-adjacent mutations, for missing actor-kind-aware runtime boundaries.
2. review whether any id-only compatibility wrappers remain reachable from untrusted adapters.
3. keep expanding the review set until every public mutation path follows the same actor identity contract.
