> Migrated from `docs/review/2026-04-06-conversation-bound-write-capability-actor-kind-review-cycle.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 2026-04-06 Conversation-Bound Write Capability Actor Kind Review Cycle

## 1. Finding

### 1.1 High: conversation-bound capability gate still trusted actor id without validating actor kind

- Root cause:
  - earlier hardening waves fixed actor-kind spoofing at concrete mutation APIs:
    - message write/edit/recall
    - agent handoff lifecycle
    - member governance writes
    - read-cursor writes
  - but the generic conversation-bound capability gate still remained id-only:
    - `conversation-runtime.ensure_conversation_bound_write_allowed(tenant_id, conversation_id, principal_id, capability)`
  - that gate resolved active membership by `principal_id` and then enforced only conversation-type capability rules.
  - it never checked whether ingress `actor_kind` matched the resolved member `principal_kind`.
  - `sdkwork-im-server` used that id-only gate for conversation-bound writes that delegate into other runtimes:
    - stream open / append / checkpoint / complete / abort
    - RTC create / invite / accept / reject / end / signal
  - after the id-only gate passed, downstream runtimes still consumed raw `auth.actor_kind` for sender/event stamping where applicable.
- Impact:
  - a caller could authenticate as:
    - actor id = `1`
    - actor kind = `agent`
  - if the actual active member was:
    - principal id = `1`
    - principal kind = `user`
  - the conversation-bound write capability gate still allowed the operation.
  - that meant stream and RTC writes could proceed under a forged actor-kind claim and stamp incorrect sender kind into downstream artifacts.

## 2. Reproduction

Regression tests were added first:

- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - `test_conversation_bound_write_capability_gate_rejects_actor_kind_mismatch`
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - `test_conversation_bound_stream_writes_reject_bearer_actor_kind_mismatch`
  - `test_conversation_bound_rtc_writes_reject_bearer_actor_kind_mismatch`

Red evidence:

- runtime unit layer failed to compile because the actor-kind-aware capability-gate API did not exist yet:
  - missing `ensure_conversation_bound_write_allowed_with_actor_kind(...)`
- root-cause inspection showed why the local profile was exposed:
  - `ensure_conversation_bound_write_access(...)` passed only tenant id, conversation id, actor id, and capability
  - stream/RTC handlers then delegated into downstream runtimes using raw `auth`

Green verification after the fix proved the gap was closed for both the generic gate and the local profile adapters.

## 3. Fix Design

Conversation-bound capability gates are themselves write authorization boundaries and must follow the same actor identity contract as concrete mutation APIs.

Chosen design:

1. add actor-kind-aware capability gate in `conversation-runtime`:
   - `ensure_conversation_bound_write_allowed_with_actor_kind(...)`
2. at the gate boundary:
   - resolve active member by actor id
   - call `ensure_actor_kind_matches_member(...)`
   - only then apply conversation-type capability rules
3. keep legacy id-only gate as a compatibility wrapper:
   - derive actor kind from runtime member truth
   - delegate into the actor-kind-aware gate
4. update `sdkwork-im-server` conversation-bound write gate helper to pass raw ingress actor kind into the runtime gate

## 4. Implementation

- `services/conversation-runtime/src/lib.rs`
  - added `ensure_conversation_bound_write_allowed_with_actor_kind(...)`
  - changed `ensure_conversation_bound_write_allowed(...)` into a compatibility wrapper
  - hardened the capability gate with `ensure_actor_kind_matches_member(...)`
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
  - updated `ensure_conversation_bound_write_access(...)` to call the actor-kind-aware runtime gate
- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - added runtime regression coverage for the generic capability gate
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - added local bearer actor-kind mismatch coverage for:
    - conversation-bound stream writes
    - conversation-bound RTC writes
  - verified rejected writes leave no persisted stream frames or RTC-generated conversation messages

## 5. Verification

### Red

- `cargo test -p conversation-runtime --offline test_conversation_bound_write_capability_gate_rejects_actor_kind_mismatch`
  - failed because `ensure_conversation_bound_write_allowed_with_actor_kind(...)` did not exist

### Green

- `cargo test -p conversation-runtime --offline test_conversation_bound_write_capability_gate_rejects_actor_kind_mismatch`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test access_control_e2e_test conversation_bound_`

Observed green results:

- runtime mismatch regression passed
- local profile stream mismatch regressions passed
- local profile RTC mismatch regressions passed
- no unexpected persisted stream frames after rejected forged append
- no unexpected conversation messages after rejected forged RTC signal

## 6. Remaining Risks

- `sdkwork-im-server` still passes raw `auth` into downstream streaming/RTC runtimes after the capability gate succeeds; this is acceptable only because the actor-kind-aware gate now rejects spoofed writes before mutation.
- standalone `streaming-service` and `im-call-runtime` do not own conversation membership logic; if future deployment profiles bind them directly to conversation authorization, the same actor-kind-aware gate must be inserted ahead of them.
- other generic conversation-bound delegation gates may still exist outside stream and RTC surfaces.

## 7. Next Wave

1. audit remaining delegated write surfaces such as notification, event, webhook, or workflow-triggered conversation writes for the same generic-gate weakness.
2. review whether any other id-only capability helpers remain reachable from public or semi-public adapters.
3. continue converging all conversation-bound write families on the same actor-kind-aware boundary contract.

