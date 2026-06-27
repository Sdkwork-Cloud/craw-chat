## Loop Status
- date: `2026-04-11`
- loop_id: `51`
- current_wave: `Wave-2`
- current_mode: `实施批次`
- current_steps: `S07`
- closure_level: `local_closure`

## Truth Checked
- code: `services/conversation-runtime/src/runtime.rs` `services/conversation-runtime/src/runtime/membership.rs` `services/conversation-runtime/src/runtime/policy.rs` `services/conversation-runtime/src/runtime/http.rs`
- tests: `services/conversation-runtime/tests/conversation_flow_test.rs` `services/conversation-runtime/tests/http_smoke_test.rs` `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
- review_docs: `docs/review/S07-执行卡-2026-04-10.md`
- architecture_docs: `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`
- release_docs: `docs/release/CHANGELOG.md`
- git_status: `dirty workspace; Loop51 only adds shared-channel durable sync seam / runtime linked-member materialization truth and its backwrite`

## Batch Plan
- serial_path: `Loop51 execution-card freeze -> red shared-channel sync http test -> minimal system-owned runtime sync seam -> replay/structure/package regression -> backwrite`
- parallel_windows: `verification only`
- blocked_items: `none`

## Gap Triage
- p0: `shared_channel_policy durable sync seam`
- p1: `control-plane -> conversation-runtime automatic sync trigger / auto-projection`
- p2: `full im_thread_subscription durable model / per-user unread-notification level`
- chosen_main_gap: `shared_channel_policy durable sync seam`

## Actions This Loop
- actual_changes:
  - `conversation-runtime` 新增 `SyncSharedChannelLinkedMemberCommand` 与 `sync_shared_channel_linked_member_*` runtime seam，把由 `shared_channel_policy + external_member_link` 派生的最小 payload materialize 为 runtime `state = linked` member
  - 该 seam 明确要求 `system` actor，且只允许落到 `history_visibility = shared` 的 conversation 上；若当前 principal 已被 materialize 为不兼容成员 truth，则显式返回 conflict
  - sync path 会把 linked member 固定落为 `role = guest`，持久化 `sharedChannelPolicyId / externalConnectionId / externalMemberId` 三元锚点，并初始化默认 read cursor
  - `conversation-runtime` 新增 `POST /im/v3/api/chat/conversations/shared_channel_links/sync` operator surface；HTTP path 通过 runtime auth-context entrypoint 收敛 authority capture，不在 service edge 手工穿透 requester kind
  - 新增 replay 级回归，确认 linked member truth 经 `conversation.member_joined` commit 后可被恢复，并在恢复后继续读取 shared history
- changed_files:
  - `services/conversation-runtime/src/runtime.rs`
  - `services/conversation-runtime/src/runtime/membership.rs`
  - `services/conversation-runtime/src/runtime/policy.rs`
  - `services/conversation-runtime/src/runtime/http.rs`
  - `services/conversation-runtime/tests/conversation_flow_test.rs`
  - `services/conversation-runtime/tests/http_smoke_test.rs`
  - `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
  - `docs/review/S07-执行卡-2026-04-10.md`
  - `docs/step/122-S07-shared-channel-policy-durable-sync-loop51-current-checkpoint-2026-04-11.md`
  - `docs/review/S07-Loop51补充-2026-04-11.md`
  - `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`
  - `docs/架构/152CJ-Loop51补充-2026-04-11.md`
  - `docs/release/CHANGELOG.md`
  - `docs/release/2026-04-11-v0.0.51-loop-51.md`
- implemented_now: `S07 shared_channel_policy durable sync seam / runtime linked-member materialization`
- deferred_now: `control-plane -> conversation-runtime automatic sync trigger / auto-projection` `full im_thread_subscription durable model / per-user unread-notification level` `archive / restore / retention owner integration`

## Verification
- commands:
  - `cargo test -p conversation-runtime --offline --test http_smoke_test test_sync_shared_channel_linked_member_over_http_materializes_linked_history_reader -- --nocapture`
  - `cargo test -p conversation-runtime --offline --test conversation_flow_test test_sync_shared_channel_linked_member_materializes_runtime_truth_and_survives_recovery_replay -- --nocapture`
  - `cargo fmt -p conversation-runtime -p im-domain-core`
  - `cargo test -p conversation-runtime --offline --tests -- --nocapture`
- results:
  - red: `test_sync_shared_channel_linked_member_over_http_materializes_linked_history_reader` 初始失败断言为 `left: 404` / `right: 200`
  - green: targeted shared-channel sync HTTP 回归通过
  - green: targeted runtime replay 回归通过
  - `conversation-runtime --tests = 128 passed`
- unverified_risks:
  - control-plane 当前仍不会在 `shared_channel_policy / external_member_link` durable truth 提交后自动触发这条 sync seam
  - `external_member_link` durable truth 仍不携带 principal kind；当前 sync contract 仍要求调用方桥接 `localActorKind`
  - 尚未实现 per-user unread / notification level / follow-unfollow 配置

## Backwrite
- step_backwrite: `docs/step/122-S07-shared-channel-policy-durable-sync-loop51-current-checkpoint-2026-04-11.md`
- review_backwrite: `docs/review/S07-执行卡-2026-04-10.md` `docs/review/S07-Loop51补充-2026-04-11.md`
- architecture_backwrite: `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md` `docs/架构/152CJ-Loop51补充-2026-04-11.md`
- release_backwrite: `docs/release/CHANGELOG.md` `docs/release/2026-04-11-v0.0.51-loop-51.md`

## Step Exit Check
- q1: `no`
- q2: `yes`
- q3: `yes`
- q4: `yes`
- q5: `yes`
- q6: `yes`
- exit_result: `S07 = not_closed / local_closure`

## Scoreboard
- architecture_alignment: `99`
- ddd_boundary_integrity: `99`
- implementation_completeness: `99`
- test_closure: `99`
- operability_release_readiness: `97`
- commercial_readiness: `95`
- lowest_score_item: `commercial_readiness`
- next_upgrade_focus: `control-plane -> conversation-runtime automatic sync trigger / auto-projection`

## Next Loop Input
- next_mode: `实施批次`
- next_steps: `S07`
- next_goal: `把 control-plane shared durable truth 自动触发到 conversation-runtime sync seam`
- next_files_to_check: `services/control-plane-api/src/lib.rs` `services/control-plane-api/tests/social_external_collaboration_test.rs` `services/conversation-runtime/src/runtime/http.rs` `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`
- next_verification_focus: `shared_channel_policy/external_member_link durable truth -> runtime sync trigger` `localActorKind bridge ownership` `fresh package evidence`

## Stop Decision
- continue_or_stop: `continue`
- reason: `shared_channel_policy durable sync seam` 已进入最小 runtime truth，但 `S07` 仍被 `control-plane -> conversation-runtime automatic sync trigger / auto-projection` 与更强 thread subscription durable model 阻塞
