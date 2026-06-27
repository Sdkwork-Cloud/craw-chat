## Loop Status
- date: `2026-04-11`
- loop_id: `49`
- current_wave: `Wave-2`
- current_mode: `实施批次`
- current_steps: `S07`
- closure_level: `local_closure`

## Truth Checked
- code: `services/conversation-runtime/src/runtime/creation.rs` `services/notification-service/src/lib.rs`
- tests: `services/conversation-runtime/tests/conversation_flow_test.rs` `services/notification-service/tests/notification_pipeline_test.rs`
- review_docs: `docs/review/S07-执行卡-2026-04-10.md`
- architecture_docs: `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`
- release_docs: `docs/release/CHANGELOG.md`
- git_status: `dirty workspace; Loop49 only adds thread root-author auto-subscription runtime truth and its backwrite`

## Batch Plan
- serial_path: `Loop49 execution-card freeze -> red thread root-author subscription test -> minimal thread membership propagation -> package regression -> backwrite`
- parallel_windows: `verification only`
- blocked_items: `none`

## Gap Triage
- p0: `thread subscription / notification runtime truth`
- p1: `shared fanout / rebuild`
- p2: `full im_thread_subscription durable model / per-user unread-notification level`
- chosen_main_gap: `thread subscription / notification runtime truth`

## Actions This Loop
- actual_changes:
  - thread create path 现在会在 creator owner 之外，把当前仍是 parent active member 的 root message author 一并 materialize 为 thread active member
  - auto-subscribed root author 会保留 `parentConversationId / rootMessageId / threadRole = root_author`，并初始化默认 read cursor
  - recovery replay 后，这条 membership truth 仍可恢复，root author 仍能在 thread 中发消息
  - 现有 `notification-service` message-posted fanout 无需新增 route，即可继续消费 projection 中的 thread active principal 集合
  - 本轮不引入独立 `im_thread_subscription` durable model，也不实现 per-user notification level
- changed_files:
  - `services/conversation-runtime/src/runtime/creation.rs`
  - `services/conversation-runtime/tests/conversation_flow_test.rs`
  - `docs/review/S07-执行卡-2026-04-10.md`
  - `docs/step/120-S07-thread-subscription-notification-runtime-truth-loop49-current-checkpoint-2026-04-11.md`
  - `docs/review/S07-Loop49补充-2026-04-11.md`
  - `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`
  - `docs/架构/152CJ-Loop49补充-2026-04-11.md`
  - `docs/release/CHANGELOG.md`
  - `docs/release/2026-04-11-v0.0.49-loop-49.md`
- implemented_now: `S07 thread root-author subscription / notification runtime truth`
- deferred_now: `shared fanout / rebuild` `full im_thread_subscription durable model / per-user unread-notification level` `archive / restore / retention owner integration`

## Verification
- commands:
  - `cargo test -p conversation-runtime --offline --test conversation_flow_test test_create_thread_conversation_auto_subscribes_root_message_author_for_notification_truth -- --nocapture`
  - `cargo fmt -p conversation-runtime -p im-domain-core`
  - `cargo test -p im-domain-core --offline --tests -- --nocapture`
  - `cargo test -p conversation-runtime --offline --tests -- --nocapture`
  - `cargo test -p notification-service --offline --tests -- --nocapture`
- results:
  - red: `test_create_thread_conversation_auto_subscribes_root_message_author_for_notification_truth` 初始失败，断言为 `left: 1` / `right: 2`
  - green: targeted thread subscription 测试通过，覆盖 auto-subscribe、default read cursor 与 replay 后 root author reply
  - `im-domain-core --tests = 38 passed`
  - `conversation-runtime --tests = 126 passed`
  - `notification-service --tests = 18 passed`
- unverified_risks:
  - 尚未引入独立 `im_thread_subscription` durable model
  - 尚未实现 thread per-user unread / notification level / follow-unfollow 配置
  - `shared_channel_policy` 自动 fanout / rebuild 仍未进入当前 runtime

## Backwrite
- step_backwrite: `docs/step/120-S07-thread-subscription-notification-runtime-truth-loop49-current-checkpoint-2026-04-11.md`
- review_backwrite: `docs/review/S07-执行卡-2026-04-10.md` `docs/review/S07-Loop49补充-2026-04-11.md`
- architecture_backwrite: `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md` `docs/架构/152CJ-Loop49补充-2026-04-11.md`
- release_backwrite: `docs/release/CHANGELOG.md` `docs/release/2026-04-11-v0.0.49-loop-49.md`

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
- next_upgrade_focus: `S07 shared fanout / rebuild`

## Next Loop Input
- next_mode: `实施批次`
- next_steps: `S07`
- next_goal: `补齐 shared fanout / rebuild`
- next_files_to_check: `services/notification-service/src/lib.rs` `services/projection-service/src/lib.rs` `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`
- next_verification_focus: `shared target resolution` `rebuild alignment` `fresh package evidence`

## Stop Decision
- continue_or_stop: `continue`
- reason: `thread subscription / notification runtime truth` 已进入最小 runtime truth，但 `S07` 仍被 `shared fanout / rebuild` 与更强 thread subscription durable model 阻塞
