# S05 Loop42 当前检查点 - 2026-04-11

- Step: `S05`
- 状态: `not_closed / local_closure`
- 本轮已兑现:
  - `services/control-plane-api/src/lib.rs` 为 runtime-dir social durable truth 新增 `state/social-transaction-marker.json`
  - social 写路径现在收敛为 `append social-commit-journal -> write social-transaction-marker -> save social-state -> clear marker`
  - `snapshot save` 失败时，marker 会作为 pending snapshot repair 边界保留下来
  - runtime startup 的 journal replay、same-event retry 的 best-effort repair、HTTP `repair-derived-snapshot` 与 standalone `repair-social-runtime-dir` CLI 在修复成功后都会清除 marker
  - fresh targeted regression：`cargo test -p control-plane-api --offline --test social_friend_request_test test_control_plane_social_file_runtime_leaves_pending_tx_marker_after_snapshot_failure_and_clears_it_after_restart_replay -- --nocapture` = `passed`
  - full package regression：`cargo test -p control-plane-api --offline --tests -- --nocapture` = `61 passed`
  - operator wrapper regression：`cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test -- --nocapture` = `64 passed`
- 当前仍缺:
  - `S05 step_closure`
  - 更强 `staged / manifest` 级 `atomic multi-file tx` 证明
- 下一主刀:
  - 判断 `repair-marker based atomic multi-file tx minimal boundary` 是否已足够支撑 `S05 step_closure`
  - 若不足，则继续收敛 `staged / manifest` 级事务边界；若足够，则转做 `95/97` 口径下的 step exit evidence
