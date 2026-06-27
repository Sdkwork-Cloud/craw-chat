# Step 05 / CP05-4 message-posted projection recipient owner 质量审计 - 2026-04-07

## 1. 审计结论

- 本轮增量通过。
- `notification-service` 已开始统一拥有 message-posted recipient authority。
- `sdkwork-im-server` 不再把 `recipient_ids` 作为 edge 输入透传给 notification owner。

## 2. 证据

- 结构证据
  - `test_notification_runtime_exposes_message_posted_notification_owner_seam`
  - `test_local_minimal_node_effects_do_not_thread_message_posted_recipient_ids`
  - `cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test --offline --target-dir target-cp054k-reg-local-structure-full`
- 行为证据
  - `test_request_message_posted_notifications_resolves_current_active_recipients_from_projection_auth_context`
  - `test_local_minimal_profile_fanouts_message_notifications_to_other_active_members_only`
- 回归证据
  - `cargo test -p notification-service --offline --target-dir target-cp054k-reg-notification-full`
  - `rustfmt --edition 2024 --check services/notification-service/src/lib.rs services/notification-service/tests/lib_structure_test.rs services/notification-service/tests/notification_pipeline_test.rs services/sdkwork-im-cloud-gateway/src/node/build.rs services/sdkwork-im-cloud-gateway/src/node/effects.rs services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 3. 剩余问题

- 该增量只解决 `CP05-4` 中一条 projection -> notification 剩余连接点，不能据此结束 `CP05-4`。
- 从当前代码搜索看，`projection / sync` 的剩余闭环已经进一步收敛到 multi-client-route sync final closure。
- 可见的下一处高概率 blocker 是 `services/session-gateway/src/lib.rs` 仍在 session/presence 路径本地组装 `registered_devices` 与 `latest_sync_seq`，尚未共享 projection-owned session sync state seam。
- 因此 `Step 05 / 91 / 95 / 97 / Wave B / 93` 仍未闭环。
