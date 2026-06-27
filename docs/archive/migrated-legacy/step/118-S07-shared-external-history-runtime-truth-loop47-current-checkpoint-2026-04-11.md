## Loop Status
- date: `2026-04-11`
- loop_id: `47`
- current_wave: `Wave-2`
- current_mode: `实施批次`
- current_steps: `S07`
- closure_level: `local_closure`

## Truth Checked
- code: `crates/im-domain-core/src/conversation.rs` `services/conversation-runtime/src/runtime/membership.rs` `services/conversation-runtime/src/runtime/policy.rs` `services/conversation-runtime/src/runtime/http.rs`
- tests: `crates/im-domain-core/tests/conversation_domain_builder_test.rs` `services/conversation-runtime/tests/http_smoke_test.rs`
- review_docs: `docs/review/S07-执行卡-2026-04-10.md`
- architecture_docs: `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`
- release_docs: `docs/release/CHANGELOG.md`
- git_status: `dirty workspace; Loop47 only adds shared-external history runtime truth and its backwrite`

## Batch Plan
- serial_path: `Loop47 execution-card freeze -> red shared tests -> minimal domain/runtime implementation -> package regression -> backwrite`
- parallel_windows: `verification only`
- blocked_items: `none`

## Gap Triage
- p0: `shared-external history runtime truth`
- p1: `retention enforcement`
- p2: `thread subscription / notification`
- chosen_main_gap: `shared-external history runtime truth`

## Actions This Loop
- actual_changes:
  - `ConversationPolicy::normalize` 正式接受 `history_visibility = shared`
  - HTTP `add-member` surface 允许显式提交 shared/external 锚点属性
  - 带完整 shared/external 锚点的新成员会落为 `MembershipState::Linked`
  - `Linked` 成员可读 shared-history，但不被视为 generic active member，不能走常规写路径
  - 普通 outsider 继续被拒绝；本轮不接 control-plane 自动投影
- changed_files:
  - `crates/im-domain-core/src/conversation.rs`
  - `crates/im-domain-core/tests/conversation_domain_builder_test.rs`
  - `services/conversation-runtime/src/runtime/http.rs`
  - `services/conversation-runtime/src/runtime/membership.rs`
  - `services/conversation-runtime/src/runtime/policy.rs`
  - `services/conversation-runtime/tests/http_smoke_test.rs`
  - `docs/review/S07-执行卡-2026-04-10.md`
  - `docs/step/118-S07-shared-external-history-runtime-truth-loop47-current-checkpoint-2026-04-11.md`
  - `docs/review/S07-Loop47补充-2026-04-11.md`
  - `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`
  - `docs/架构/152CJ-Loop47补充-2026-04-11.md`
  - `docs/release/CHANGELOG.md`
  - `docs/release/2026-04-11-v0.0.47-loop-47.md`
- implemented_now: `S07 shared-external history runtime truth`
- deferred_now: `retention enforcement` `thread subscription / notification` `shared fanout / rebuild` `shared_channel_policy -> runtime auto-projection`

## Verification
- commands:
  - `cargo test -p im-domain-core --offline test_conversation_policy_normalize_accepts_shared_history_visibility -- --nocapture`
  - `cargo test -p conversation-runtime --offline --test http_smoke_test test_shared_history_visibility_allows_external_linked_history_reads_but_not_writes_over_http -- --nocapture`
  - `cargo fmt -p conversation-runtime -p im-domain-core`
  - `cargo test -p im-domain-core --offline --tests -- --nocapture`
  - `cargo test -p conversation-runtime --offline --tests -- --nocapture`
- results:
  - red: `test_conversation_policy_normalize_accepts_shared_history_visibility` 初始失败，错误为 `unsupported conversation history visibility until invitation/shared-channel truth is implemented: shared`
  - red: `test_shared_history_visibility_allows_external_linked_history_reads_but_not_writes_over_http` 初始失败，shared conversation 创建返回 `400`
  - green: shared-history domain 归一化测试通过
  - green: external-linked shared-history HTTP 读历史/拒绝写入回归通过
  - `im-domain-core --tests = 38 passed`
  - `conversation-runtime --tests = 124 passed`
- unverified_risks:
  - `shared_channel_policy` durable truth 仍未自动投影到 `conversation-runtime`
  - `shared` fanout / rebuild / retention 仍未进入 runtime truth

## Backwrite
- step_backwrite: `docs/step/118-S07-shared-external-history-runtime-truth-loop47-current-checkpoint-2026-04-11.md`
- review_backwrite: `docs/review/S07-执行卡-2026-04-10.md` `docs/review/S07-Loop47补充-2026-04-11.md`
- architecture_backwrite: `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md` `docs/架构/152CJ-Loop47补充-2026-04-11.md`
- release_backwrite: `docs/release/CHANGELOG.md` `docs/release/2026-04-11-v0.0.47-loop-47.md`

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
- implementation_completeness: `98`
- test_closure: `99`
- operability_release_readiness: `96`
- commercial_readiness: `93`
- lowest_score_item: `commercial_readiness`
- next_upgrade_focus: `S07 retention enforcement runtime truth`

## Next Loop Input
- next_mode: `实施批次`
- next_steps: `S07`
- next_goal: `补齐 retention enforcement runtime truth`
- next_files_to_check: `services/conversation-runtime/src/runtime/policy.rs` `services/conversation-runtime/src/runtime/http.rs` `docs/review/S07-*` `docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md`
- next_verification_focus: `retention owner/runtime boundary` `shared linked member replay stability` `fresh package evidence`

## Stop Decision
- continue_or_stop: `continue`
- reason: `shared-external history` 已进入最小 runtime truth，但 `S07` 仍被 `retention enforcement / thread subscription / shared fanout-rebuild` 阻塞
