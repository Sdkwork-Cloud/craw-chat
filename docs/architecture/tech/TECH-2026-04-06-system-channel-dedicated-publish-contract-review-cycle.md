> Migrated from `docs/review/2026-04-06-system-channel-dedicated-publish-contract-review-cycle.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 2026-04-06 System Channel Dedicated Publish Contract Review Cycle

## 1. Finding

### 1.1 High: `system_channel` still depended on generic `POST /messages` for publisher writes

- Root cause:
  - earlier waves froze:
    - dedicated create for `system_channel`
    - publisher-only message post
    - publisher-only conversation-bound `stream/RTC` writes
  - but message publish itself still reused the generic conversation message route:
    - `POST /im/v3/api/chat/conversations/{id}/messages`
  - that left no dedicated publish contract for `system_channel`, so future scheduled/bulk/delegated publish work still had to extend a generic two-way conversation write surface.
- Impact:
  - `system_channel` publish semantics were still coupled to a generic message API intended for normal conversations.
  - the platform had no stable extension point for:
    - scheduled publish
    - batch publish
    - delegated/operator-controlled publish
  - message-route semantics remained inconsistent with the special-conversation direction already frozen for create and cross-capability writes.

## 2. Scope Freeze

This wave fixes only the immediate publish contract shape.

It does not:

- implement scheduled publish
- implement batch publish
- add delegation, moderation, or mute lifecycle
- change durable message storage away from unified `Message`

## 3. Design Decision

The minimal safe slice is:

- add a dedicated publish route for `system_channel`
- reject generic message post for `system_channel`
- keep the durable truth as normal `message.posted`

This gives the platform a stable contract boundary now, without prematurely designing the full orchestration layer.

The contract is:

- dedicated route:
  - `POST /im/v3/api/chat/conversations/{conversationId}/system-channel/publish`
- generic route:
  - `POST /im/v3/api/chat/conversations/{conversationId}/messages`
  - rejected for `system_channel`

## 4. Implementation

- `services/conversation-runtime/src/lib.rs`
  - added `PublishSystemChannelMessageCommand`
  - added `publish_system_channel_message(...)`
  - refactored message append logic into policy-aware internal flow
  - generic `post_message(...)` now rejects `system_channel` with:
    - `conversation_permission_denied`
    - message indicating dedicated publish is required
  - dedicated system-channel publish still enforces publisher-only access in runtime
  - runtime message write now also validates sender kind against the resolved member principal kind before appending
  - exposed route:
    - `POST /im/v3/api/chat/conversations/{conversation_id}/system-channel/publish`
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
  - exposed the same dedicated publish route
  - added local side-effect handling for dedicated publish:
    - notification fanout
    - audit anchor
    - realtime message event
  - generic `/messages` now inherits runtime rejection for `system_channel`
- full verification surfaced one stale unit-test fixture that used a mismatched `sender.kind`
  - updated that fixture to the real member principal kind
  - added an explicit regression test so sender-kind spoofing stays rejected at runtime

## 5. Tests Added

- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - `test_system_channel_requires_dedicated_publish_command_and_allows_only_publisher`
  - `test_post_message_rejects_sender_kind_mismatch_against_member_principal_kind`
- `services/conversation-runtime/tests/http_smoke_test.rs`
  - `test_system_channel_publisher_must_use_dedicated_publish_route_over_http`
  - `test_system_channel_dedicated_publish_over_http`
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - `test_system_channel_publisher_must_use_dedicated_publish_in_local_profile`
  - `test_system_channel_dedicated_publish_allows_only_publisher_in_local_profile`

These tests prove:

- generic `/messages` is no longer a valid publish contract for `system_channel`
- only the system publisher can use the dedicated publish route
- the dedicated route still produces a normal durable message result

## 6. Verification

### Red

- `cargo test -p conversation-runtime --offline test_system_channel_requires_dedicated_publish_command_and_allows_only_publisher -- --exact`
  - failed to compile because dedicated publish command/method did not exist
- `cargo test -p sdkwork-im-cloud-gateway --offline test_system_channel_publisher_must_use_dedicated_publish_in_local_profile -- --exact`
  - failed with `200 != 403`

### Green

- `cargo test -p conversation-runtime --offline test_system_channel_requires_dedicated_publish_command_and_allows_only_publisher -- --exact`
- `cargo test -p conversation-runtime --offline test_system_channel_publisher_must_use_dedicated_publish_route_over_http -- --exact`
- `cargo test -p conversation-runtime --offline test_system_channel_dedicated_publish_over_http -- --exact`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_system_channel_publisher_must_use_dedicated_publish_in_local_profile -- --exact`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_system_channel_dedicated_publish_allows_only_publisher_in_local_profile -- --exact`

## 7. Remaining Risks

- scheduled publish, batch publish, and delegated publish are still not implemented; this wave only freezes the contract boundary they must extend.
- `system_channel` still has no dedicated moderation or mute lifecycle.
- `agent_dialog` still lacks a comparable dedicated post-create lifecycle/governance contract.

## 8. Next Wave

1. Freeze the request model for scheduled publish and batch publish on top of the dedicated `system-channel/publish` contract.
2. Decide whether delegated publish must be represented as a separate actor model or as a system-owned orchestration command.
3. Continue `agent_dialog` lifecycle/governance hardening so special conversation types converge on the same standard.

