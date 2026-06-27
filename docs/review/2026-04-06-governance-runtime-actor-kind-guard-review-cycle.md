# 2026-04-06 Governance Runtime Actor Kind Guard Review Cycle

## 1. Finding

### 1.1 High: `conversation-runtime` governance writes accepted actor-kind mismatch at the mutation boundary

- Root cause:
  - governance write commands only carried actor identity ids:
    - `invited_by`
    - `removed_by`
    - `principal_id`
    - `transferred_by`
    - `changed_by`
  - unlike message mutation and agent handoff flows, governance write entry points did not accept a caller-supplied actor kind.
  - runtime therefore resolved the active member by id and continued the mutation without validating whether ingress auth claimed the same actor kind as durable member truth.
  - previous wave fixed only local side-effect truth alignment; it did not stop the mutation itself.
- Impact:
  - an ingress request could authenticate as:
    - actor id = `1`
    - actor kind = `agent`
  - if the active member truth was:
    - principal id = `1`
    - principal kind = `user`
  - runtime governance writes still succeeded for:
    - member add
    - member remove
    - member leave
    - owner transfer
    - member role change
  - this left the write boundary weaker than the durable domain model and allowed spoofed actor-kind claims to pass authorization.

## 2. Reproduction

Two regression tests were added before the fix:

- `services/conversation-runtime/tests/http_smoke_test.rs`
  - `test_group_member_governance_over_http_rejects_actor_kind_mismatch`
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - `test_group_member_governance_rejects_bearer_actor_kind_mismatch`

Red verification proved the defect:

- `conversation-runtime` HTTP governance write returned:
  - actual status = `200`
  - expected status = `403`
- `sdkwork-im-server` governance write with forged bearer `actor_kind` returned:
  - actual status = `200`
  - expected status = `403`

## 3. Fix Design

The write boundary must reject actor-kind mismatch before governance permission logic mutates conversation state.

The chosen design is:

1. add explicit runtime governance entry points that accept actor kind:
   - `add_member_with_actor_kind(...)`
   - `remove_member_with_actor_kind(...)`
   - `leave_conversation_with_actor_kind(...)`
   - `transfer_conversation_owner_with_actor_kind(...)`
   - `change_conversation_member_role_with_actor_kind(...)`
2. at runtime mutation start:
   - resolve active actor member by id
   - compare supplied actor kind against resolved member `principal_kind`
   - reject on mismatch with `PermissionDenied`
3. route all untrusted ingresses through these actor-kind-aware entry points:
   - `conversation-runtime` HTTP handlers
   - `sdkwork-im-server` governance handlers
4. keep legacy id-only runtime methods as compatibility wrappers for trusted/internal callers:
   - they self-normalize actor kind from runtime member truth
   - they do not weaken external ingress boundaries because public adapters no longer use them

This preserves minimal patch scope while moving the authorization truth to the correct boundary.

## 4. Implementation

- `services/conversation-runtime/src/lib.rs`
  - added actor-kind-aware governance methods for:
    - add member
    - remove member
    - leave conversation
    - transfer owner
    - change member role
  - each method now calls:
    - `resolve_active_member(...)`
    - `ensure_actor_kind_matches_member(...)`
    - then existing governance permission checks
  - updated HTTP governance handlers to pass `auth.actor_kind`
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
  - updated governance handlers to call runtime actor-kind-aware methods with raw ingress `auth.actor_kind`
  - existing side-effect normalization remains in place, but unauthorized mismatched writes are now rejected before mutation
- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - added:
    - `test_governance_writes_reject_actor_kind_mismatch`
- `services/conversation-runtime/tests/http_smoke_test.rs`
  - added:
    - `test_group_member_governance_over_http_rejects_actor_kind_mismatch`
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - added:
    - `test_group_member_governance_rejects_bearer_actor_kind_mismatch`

## 5. Verification

### Red

- `cargo test -p conversation-runtime --offline test_group_member_governance_over_http_rejects_actor_kind_mismatch -- --exact`
  - failed with status `200` instead of `403`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_group_member_governance_rejects_bearer_actor_kind_mismatch -- --exact`
  - failed with status `200` instead of `403`

### Green

- `cargo test -p conversation-runtime --offline test_governance_writes_reject_actor_kind_mismatch -- --exact`
- `cargo test -p conversation-runtime --offline test_group_member_governance_over_http_rejects_actor_kind_mismatch -- --exact`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_group_member_governance_rejects_bearer_actor_kind_mismatch -- --exact`

## 6. Remaining Risks

- other conversation-bound write paths may still rely on id-only runtime entry points and need the same audit:
  - read cursor writes
  - non-message stream mutations
  - future workflow or bot-governed roster operations
- trusted/internal callers can still use compatibility wrappers, so new integrations must be held to the actor-kind-aware API standard explicitly.
- this wave hardens governance writes only; broader ingress contract review is still needed for every state-mutating runtime surface.

## 7. Next Wave

1. audit remaining state-mutating runtime entry points for missing actor-kind or actor-structure constraints.
2. review whether read-cursor and other conversation-bound writes should adopt the same explicit actor-kind ingress contract.
3. continue consolidating ingress adapters so public and private profiles always call the same constrained runtime boundary.
