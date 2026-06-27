# Continuous Optimization - detailed recovery registry baseline

## 结论

本轮补的是一个小但真实的公开契约缺口：`close / error registry` 已有基础恢复方向，但缺少在 CLI / SDK / operator 单一索引页中的细粒度统一词汇。

## 红绿证据

- 红灯：
  - `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_docs_freeze_detailed_recovery_registry_baseline -- --exact --nocapture`
  - 失败信息：`Step 12 CLI doc must contain detailed recovery baseline text 4001`
- 绿灯：
  - `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_docs_freeze_detailed_recovery_registry_baseline -- --exact --nocapture`

## 本轮改动

- 新增 `chat_cli_contract_test.rs` 细粒度恢复词汇契约
- 更新 CLI 文档
- 更新 app/admin SDK README
- 更新单一验证索引页

## 当前收益

- 恢复词汇不再只停留在“恢复方向”，而是进入统一的公开词表
- 新 consumer 若从索引页进入，不会漏掉 `4001 / reconnect_required / pull-only / events.pull`
- CLI / SDK / operator 文档对 `session.disconnect` 与 overload 的解释更一致

## 残余风险

- 当前仍是公开文档与契约测试收敛，不代表多语言 SDK 代码层已经暴露正式恢复 API
- Bash 路径仍缺少与 PowerShell 完全对称的 fresh E2E 证据

## 下一步

- 优先评估 Bash scripted validation 的最小可信对称证据
- 若环境继续受限，则推进多语言 SDK 生成/发布链的最小机器可读约束
