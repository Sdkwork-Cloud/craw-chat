> Migrated from `docs/step/117-S07-invited-history-visibility-loop46-current-checkpoint-2026-04-11.md` on 2026-06-24.
> Owner: SDKWork maintainers

## Loop Status
- date: `2026-04-11`
- loop_id: `46`
- current_wave: `Wave-2`
- current_mode: `实现批次`
- current_steps: `S07`
- closure_level: `local_closure`

## Truth Checked
- code: `crates/im-domain-core/src/conversation.rs` `services/conversation-runtime/src/runtime/membership.rs` `services/conversation-runtime/src/runtime/policy.rs`
- tests: `crates/im-domain-core/tests/conversation_domain_builder_test.rs` `services/conversation-runtime/tests/http_smoke_test.rs`
- review_docs: `docs/review/S07-执行卡-2026-04-10.md`
- architecture_docs: `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`
- release_docs: `docs/release/CHANGELOG.md`
- git_status: `dirty workspace; Loop46 only adds invited history visibility truth and its backwrite`

## Batch Plan
- serial_path: `Loop46 execution-card freeze -> red invited HTTP test -> minimal domain/runtime implementation -> package regression -> backwrite`
- parallel_windows: `verification only`
- blocked_items: `none`

## Gap Triage
- p0: `invited history visibility runtime truth`
- p1: `shared-external history`
- p2: `retention enforcement / thread subscription`
- chosen_main_gap: `invited history visibility runtime truth`

## Actions This Loop
- actual_changes:
  - `ConversationPolicy::normalize` 允许 `history_visibility = invited`
  - invited-history 会话的新增成员默认落为 `MembershipState::Invited`
  - `Invited` 成员可读 invited-history，但不再被当作 generic active member
  - runtime history read 对 `invited` 改为 `Joined | Invited` 可读，`shared` 继续显式拒绝
  - HTTP smoke test 拆分为 `invited` 绿测与 `shared` 拒绝回归
- changed_files:
  - `crates/im-domain-core/src/conversation.rs`
  - `crates/im-domain-core/tests/conversation_domain_builder_test.rs`
  - `services/conversation-runtime/src/runtime/membership.rs`
  - `services/conversation-runtime/src/runtime/policy.rs`
  - `services/conversation-runtime/tests/http_smoke_test.rs`
  - `docs/review/S07-执行卡-2026-04-10.md`
  - `docs/step/117-S07-invited-history-visibility-loop46-current-checkpoint-2026-04-11.md`
  - `docs/review/S07-Loop46补充-2026-04-11.md`
  - `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`
  - `docs/架构/152CJ-Loop46补充-2026-04-11.md`
  - `docs/release/CHANGELOG.md`
  - `docs/release/2026-04-11-v0.0.46-loop-46.md`
- implemented_now: `S07 invited history visibility runtime truth`
- deferred_now: `shared-external history` `retention enforcement` `thread subscription / notification`

## Verification
- commands:
  - `cargo test -p conversation-runtime --offline --test http_smoke_test test_invited_history_visibility_allows_invited_member_history_reads_before_join_over_http -- --nocapture`
  - `cargo test -p conversation-runtime --offline --test http_smoke_test test_create_conversation_rejects_shared_history_visibility_over_http -- --nocapture`
  - `cargo test -p im-domain-core --offline --test conversation_domain_builder_test -- --nocapture`
  - `cargo fmt -p conversation-runtime -p im-domain-core`
  - `cargo test -p im-domain-core --offline --tests -- --nocapture`
  - `cargo test -p conversation-runtime --offline --tests -- --nocapture`
- results:
  - red: `test_invited_history_visibility_allows_invited_member_history_reads_before_join_over_http` 初始失败，创建 invited-history 会话返回 `400`，证明缺口真实存在
  - green: invited-history HTTP 绿测通过
  - green: shared-history HTTP 拒绝回归通过
  - `conversation_domain_builder_test = 18 passed`
  - `im-domain-core --tests = 38 passed`
  - `conversation-runtime --tests = 124 passed`
- unverified_risks:
  - `shared-external history` 仍未进入 runtime truth
  - `retention enforcement` 仍未形成 owner/runtime 闭环

## Backwrite
- step_backwrite: `docs/step/117-S07-invited-history-visibility-loop46-current-checkpoint-2026-04-11.md`
- review_backwrite: `docs/review/S07-执行卡-2026-04-10.md` `docs/review/S07-Loop46补充-2026-04-11.md`
- architecture_backwrite: `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md` `docs/架构/152CJ-Loop46补充-2026-04-11.md`
- release_backwrite: `docs/release/CHANGELOG.md` `docs/release/2026-04-11-v0.0.46-loop-46.md`

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
- implementation_completeness: `97`
- test_closure: `99`
- operability_release_readiness: `96`
- commercial_readiness: `92`
- lowest_score_item: `commercial_readiness`
- next_upgrade_focus: `S07 shared-external history runtime truth`

## Next Loop Input
- next_mode: `实现批次`
- next_steps: `S07`
- next_goal: `补齐 shared-external history runtime truth`
- next_files_to_check: `crates/im-domain-core/src/conversation.rs` `services/conversation-runtime/src/runtime/policy.rs` `services/conversation-runtime/tests/http_smoke_test.rs` `docs/review/S07-*`
- next_verification_focus: `shared visibility create/read semantics` `external boundary ownership` `fresh package evidence`

## Stop Decision
- continue_or_stop: `continue`
- reason: `invited history visibility` 已闭合，但 `S07` 仍被 `shared-external history / retention enforcement` 阻塞

