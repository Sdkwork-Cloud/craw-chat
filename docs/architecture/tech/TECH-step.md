> Migrated from `docs/prompts/反复执行Step指令.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 反复执行Step指令
1. 每轮先复核仓库真相，重新检查相关代码、测试、`docs/step`、`docs/review`、`docs/架构`、`docs/release` 与 `git status`。
2. 明确当前 `step / 波次 / 是否闭环`，判定属于实施批次、收口批次还是阻塞态。
3. 只选择一个当前依赖窗口内最关键的主缺口，能改代码就改代码，禁止只给建议。
4. 实现后至少完成一项最小验证，并记录命令、结果、风险与未验证项。
5. 任何行为、契约、模型、测试结论变化，都必须同步回写 `docs/step`、`docs/review`、`docs/架构`、`docs/release`。
6. Step 关闭前必须按 `95/97`、证据、回退条件完成出口检查；任一为 `no` 只能标记 `not_closed / local_closure / blocked`。
7. 每轮必须更新 changelog / release，写清 loop 编号、影响 step、验证证据、剩余风险。
8. 始终核对 `deployment_profile`、脚本入口、运行时状态与 operator surface 是否和文档一致。
9. 输出下一轮输入，至少包含主缺口、相关文件、验证重点与 `下一轮动作`。
10. 只有 `S00-S14` 全部闭环，或存在明确外部阻塞且写清影响与解除条件时，才允许停止。

