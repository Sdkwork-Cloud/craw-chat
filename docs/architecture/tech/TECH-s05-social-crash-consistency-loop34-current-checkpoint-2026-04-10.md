> Migrated from `docs/step/105-S05-social-crash-consistency-loop34-current-checkpoint-2026-04-10.md` on 2026-06-24.
> Owner: SDKWork maintainers

# S05 Loop34 当前检查点 - 2026-04-10

- Step: `S05`
- 状态: `not_closed / local_closure`
- 本轮已兑现:
  - runtime-dir startup 改为 `journal authority`：只要 `social-commit-journal.json` 存在，就从默认空状态 replay 并回写 `social-state.json`
  - social 写入顺序改为 `append journal -> save snapshot`
  - `snapshot ahead of journal` 幻影场景已被 TDD 锁定并保持通过：
    - `test_control_plane_social_file_runtime_discards_friend_request_snapshot_ahead_of_journal`
    - `test_control_plane_social_file_runtime_discards_direct_chat_snapshot_ahead_of_journal`
  - `snapshot missing` replay 回归继续为绿：
    - `test_control_plane_social_file_runtime_replays_friend_request_when_snapshot_is_missing`
    - `test_control_plane_social_file_runtime_replays_direct_chat_pair_guard_when_snapshot_is_missing`
- 当前仍缺:
  - `atomic tx boundary`
  - `journal append success / snapshot save fail` 的故障注入与 repair 证据
  - operator 级 replay / roll-forward / repair 闭环
  - `S05 step_closure`
- 下一主刀:
  - 为 snapshot save fail 注入可测故障点
  - 收敛 unavailable / retry / restart-repair 语义
  - 明确 repo / tx / repair 的最终闭环边界


