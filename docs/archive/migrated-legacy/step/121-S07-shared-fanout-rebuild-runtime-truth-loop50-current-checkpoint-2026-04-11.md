## Loop Status
- date: `2026-04-11`
- loop_id: `50`
- current_wave: `Wave-2`
- current_mode: `实施批次`
- current_steps: `S07`
- closure_level: `local_closure`

## Truth Checked
- code: `services/projection-service/src/access.rs` `services/projection-service/src/lib.rs` `services/notification-service/src/lib.rs`
- tests: `services/projection-service/tests/timeline_projection_test.rs` `services/projection-service/tests/lib_structure_test.rs` `services/notification-service/tests/notification_pipeline_test.rs` `services/notification-service/tests/lib_structure_test.rs`
- review_docs: `docs/review/S07-执行卡-2026-04-10.md`
- architecture_docs: `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`
- release_docs: `docs/release/CHANGELOG.md`
- git_status: `dirty workspace; Loop50 only adds shared linked notification fanout / rebuild truth and its backwrite`

## Batch Plan
- serial_path: `Loop50 execution-card freeze -> red shared fanout tests -> minimal projection recipient seam -> package regression -> backwrite`
- parallel_windows: `verification only`
- blocked_items: `none`

## Gap Triage
- p0: `shared fanout / rebuild`
- p1: `shared_channel_policy auto-projection / durable sync`
- p2: `full im_thread_subscription durable model / per-user unread-notification level`
- chosen_main_gap: `shared fanout / rebuild`

## Actions This Loop
- actual_changes:
  - `projection-service` 新增 `message_posted_notification_principal_ids_from_auth_context(...)`，把 message-posted recipient resolution 收敛为 projection owner seam
  - 该 seam 在调用者仍需是 active member 的前提下，返回 `joined active members + shared-history-visible linked members`
  - `notification-service::request_message_posted_notifications(...)` 现在通过该 seam 解析 message-posted recipients，actor 仍在 fanout 阶段被排除
  - 因为 projection 继续完整持有 `ConversationMember`（包括 `state = linked` 与 shared/external 锚点 attributes），事件重放 / rebuild 后仍能恢复相同通知目标集合
  - 本轮同时把结构测试回写到新 seam，并把 `projection-service/src/lib.rs` 保持回 Step-02 redline 以内
- changed_files:
  - `services/projection-service/src/access.rs`
  - `services/projection-service/src/lib.rs`
  - `services/projection-service/tests/timeline_projection_test.rs`
  - `services/projection-service/tests/lib_structure_test.rs`
  - `services/notification-service/src/lib.rs`
  - `services/notification-service/tests/notification_pipeline_test.rs`
  - `services/notification-service/tests/lib_structure_test.rs`
  - `docs/review/S07-执行卡-2026-04-10.md`
  - `docs/step/121-S07-shared-fanout-rebuild-runtime-truth-loop50-current-checkpoint-2026-04-11.md`
  - `docs/review/S07-Loop50补充-2026-04-11.md`
  - `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`
  - `docs/架构/152CJ-Loop50补充-2026-04-11.md`
  - `docs/release/CHANGELOG.md`
  - `docs/release/2026-04-11-v0.0.50-loop-50.md`
- implemented_now: `S07 shared fanout / rebuild runtime truth`
- deferred_now: `shared_channel_policy auto-projection / durable sync` `full im_thread_subscription durable model / per-user unread-notification level` `archive / restore / retention owner integration`

## Verification
- commands:
  - `cargo test -p notification-service --offline --test notification_pipeline_test test_request_message_posted_notifications_includes_shared_linked_recipients_from_projection -- --nocapture`
  - `cargo test -p projection-service --offline --test timeline_projection_test test_message_posted_notification_principal_ids_from_auth_context_includes_shared_linked_members -- --nocapture`
  - `cargo fmt -p notification-service -p projection-service`
  - `cargo test -p projection-service --offline --tests -- --nocapture`
  - `cargo test -p notification-service --offline --tests -- --nocapture`
- results:
  - red: `test_request_message_posted_notifications_includes_shared_linked_recipients_from_projection` 初始失败，断言为 `left: 1` / `right: 2`
  - red: `test_message_posted_notification_principal_ids_from_auth_context_includes_shared_linked_members` 初始失败为缺少 `message_posted_notification_principal_ids_from_auth_context`
  - green: targeted shared fanout / projection seam 测试通过
  - `projection-service --tests = 47 passed`
  - `notification-service --tests = 19 passed`
- unverified_risks:
  - `shared_channel_policy` durable truth 仍未自动投影为 runtime linked members
  - 尚未实现 per-user unread / notification level / follow-unfollow 配置
  - 尚未引入独立 `im_thread_subscription` durable model

## Backwrite
- step_backwrite: `docs/step/121-S07-shared-fanout-rebuild-runtime-truth-loop50-current-checkpoint-2026-04-11.md`
- review_backwrite: `docs/review/S07-执行卡-2026-04-10.md` `docs/review/S07-Loop50补充-2026-04-11.md`
- architecture_backwrite: `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md` `docs/架构/152CJ-Loop50补充-2026-04-11.md`
- release_backwrite: `docs/release/CHANGELOG.md` `docs/release/2026-04-11-v0.0.50-loop-50.md`

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
- next_upgrade_focus: `S07 shared_channel_policy auto-projection / durable sync`

## Next Loop Input
- next_mode: `实施批次`
- next_steps: `S07`
- next_goal: `补齐 shared_channel_policy auto-projection / durable sync`
- next_files_to_check: `services/control-plane-api/src/lib.rs` `services/conversation-runtime/src/runtime/http.rs` `services/conversation-runtime/tests/http_smoke_test.rs` `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`
- next_verification_focus: `control-plane durable truth -> runtime linked member materialization` `shared policy replay alignment` `fresh package evidence`

## Stop Decision
- continue_or_stop: `continue`
- reason: `shared fanout / rebuild` 已进入最小 runtime truth，但 `S07` 仍被 `shared_channel_policy auto-projection / durable sync` 与更强 thread subscription durable model 阻塞
