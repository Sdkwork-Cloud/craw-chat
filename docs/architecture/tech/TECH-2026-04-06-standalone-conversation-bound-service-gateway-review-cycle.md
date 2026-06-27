> Migrated from `docs/review/2026-04-06-standalone-conversation-bound-service-gateway-review-cycle.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 2026-04-06 Standalone Conversation-Bound Service Gateway Review Cycle

## 1. Finding

### 1.1 High: standalone streaming and RTC services accepted conversation-bound requests without conversation authorization

- Root cause:
  - `sdkwork-im-server` already acted as the authorizing adapter for:
    - conversation-bound stream writes
    - conversation-bound RTC writes
  - however the standalone service HTTP apps still exposed the same conversation-bound request shapes directly:
    - `streaming-service`
      - `POST /im/v3/api/streams` with `scopeKind = conversation`
    - `im-call-runtime`
      - `POST /im/v3/api/calls/sessions` with `conversationId`
  - these standalone services do not own conversation membership state and therefore cannot authorize conversation-bound access by themselves.
  - before this fix, they still returned `200` and created state directly.
- Impact:
  - if either standalone service were exposed directly, callers could bypass the IM gateway and create conversation-bound stream or RTC state without conversation membership verification.
  - this violated the platform security boundary even though the local integrated profile had already added the proper pre-authorization gates.

## 2. Reproduction

Regression tests were added first:

- `services/streaming-service/tests/http_smoke_test.rs`
  - `test_standalone_streaming_service_rejects_conversation_scope_over_http`
- `services/im-call-runtime/tests/http_smoke_test.rs`
  - `test_standalone_rtc_service_rejects_conversation_binding_over_http`

Red evidence:

- `cargo test -p streaming-service --offline test_standalone_streaming_service_rejects_conversation_scope_over_http`
  - failed with actual status `200`
  - expected status `403`
- `cargo test -p im-call-runtime --offline test_standalone_rtc_service_rejects_conversation_binding_over_http`
  - failed with actual status `200`
  - expected status `403`

## 3. Fix Design

The bug is at the standalone service adapter boundary, not in the shared runtime:

- `sdkwork-im-server` legitimately reuses the same runtimes after conversation authorization succeeds
- if the runtime itself rejected conversation-bound sessions globally, the authorized integrated profile would break

Chosen design:

1. keep `StreamingRuntime` and `RtcRuntime` generic
2. harden only standalone HTTP handlers
3. reject conversation-bound access in standalone apps with:
   - HTTP `403`
   - code `conversation_gateway_required`
4. keep standalone generic stream/RTC use cases working by updating standalone tests to use non-conversation scopes / unbound RTC sessions

## 4. Implementation

- `services/streaming-service/src/lib.rs`
  - added standalone adapter guards for:
    - open by request scope
    - all subsequent stream session access by persisted session scope
  - standalone apps now reject conversation-bound stream access with `conversation_gateway_required`
- `services/im-call-runtime/src/lib.rs`
  - added standalone adapter guards for:
    - create by request `conversationId`
    - all subsequent RTC session access by persisted session binding
  - standalone apps now reject conversation-bound RTC access with `conversation_gateway_required`
- standalone service tests were realigned to the correct product boundary:
  - `streaming-service` tests now use generic non-conversation stream scope for positive standalone coverage
  - `im-call-runtime` tests now use unbound RTC sessions for positive standalone coverage

## 5. Verification

### Red

- `cargo test -p streaming-service --offline test_standalone_streaming_service_rejects_conversation_scope_over_http`
  - failed with `200` vs expected `403`
- `cargo test -p im-call-runtime --offline test_standalone_rtc_service_rejects_conversation_binding_over_http`
  - failed with `200` vs expected `403`

### Green

- `cargo test -p streaming-service --offline`
- `cargo test -p im-call-runtime --offline`

Observed green results:

- new rejection regressions passed
- standalone generic stream tests still passed
- standalone generic RTC tests still passed
- public bearer auth tests still passed for standalone generic usage

## 6. Remaining Risks

- other standalone infra services may still accept conversation-identifying payloads without an IM authorization gateway in front of them.
- likely next candidates include services where request payloads can carry `conversationId` or equivalent conversation binding indirectly.
- current fix does not alter internal/local integrated routing, which is correct, but future gateway splits must preserve this adapter distinction.

## 7. Next Wave

1. audit standalone `notification-service` and `automation-service` for indirect conversation-bound payload handling that may also require an authorizing gateway boundary.
2. review whether any other standalone HTTP apps expose conversation-bound routes that should be local-gateway-only.
3. continue pushing security rules upward to the adapter that actually owns authorization context, while keeping shared runtimes generic and reusable.

