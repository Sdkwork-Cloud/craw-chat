> Migrated from `docs/架构/09T-实施计划-close-error-registry细粒度恢复词汇收敛-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09T - 实施计划 - close / error registry 细粒度恢复词汇收敛

## 目标

把 `Step 12` 已有的恢复策略，从“恢复方向说明”收敛到“明确的恢复输入与错误词汇”，让 CLI、SDK facade 文档和 operator 索引页共享同一套细粒度词表。

## 最小实施面

1. 先在 `tools/chat-cli/tests/chat_cli_contract_test.rs` 写红测
2. 统一冻结以下词汇：
   - `4001`
   - `session.disconnect`
   - `reconnect_required`
   - `pull-only`
   - `events.pull`
3. 更新：
   - `docs/部署/CLI聊天验证与兼容矩阵.md`
   - `sdks/sdkwork-im-sdk/README.md`
   - `sdks/sdkwork-control-plane-sdk/README.md`
   - `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
4. 回归同一条契约测试
5. 回写 `docs/step`、`docs/架构`、`docs/review`

## 约束

- 不修改 runtime 行为
- 不额外宣称多语言 SDK 真实生成链已完成
- 不引入新的恢复语义，只公开已经在现有测试与文档里被证明的词汇

## 放行标准

- 细粒度恢复词汇契约测试完成红绿闭环
- CLI / SDK / operator 单一索引页口径一致
- 下一轮新增 consumer 时不会再自行发明恢复错误词

