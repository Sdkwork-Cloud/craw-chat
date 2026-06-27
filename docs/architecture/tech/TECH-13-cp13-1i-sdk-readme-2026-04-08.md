> Migrated from `docs/step/13-CP13-1I-sdk叶子README版本决议来源收敛-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 13-CP13-1I - SDK 叶子 README 版本决议来源收敛

## 本轮目的

`versionDecisionSourcePath = null` bundle release catalog 和总入README 继续下沉到四个语言叶子 README，避免消费者进入具体语言目录后又看不到“版本决议来源尚未分配”的真实边界：

## 发现的问

- `sdk-release-catalog.json`、`sdks/README.md`、`artifacts/releases/README.md` 已公开
  - `versionDecisionSourcePath = null`
- 但四个叶README 还只表达
  - `plannedVersion = null`
  - `versionStatus = version_unassigned_pending_freeze`
- 结果是：导航链到具体语言目录时，版本决议来源占位再次断层

## 本轮决策

- 继续使用 `tools/chat-cli/tests/chat_cli_contract_test.rs` contract gate
- 新增 `test_continuous_optimization_sdk_leaf_readmes_freeze_version_decision_source_boundary`
- 最小要求：
  - 四个叶子 README 都必须公开
    - `versionDecisionSourcePath`
    - `null`

## 实施结果

- 更新
  - `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md`
  - `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/README.md`
  - `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/README.md`
  - `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/README.md`
- 现在叶子 README 也显式对齐：
  - `plannedVersion = null`
  - `versionStatus = version_unassigned_pending_freeze`
  - `versionDecisionSourcePath = null`

## 验证

- 红灯
  - `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test test_continuous_optimization_sdk_leaf_readmes_freeze_version_decision_source_boundary -- --nocapture`
  - 失败点：`app TypeScript README must contain version decision source boundary text versionDecisionSourcePath`
- 绿灯
  - `cargo fmt --all --check`
  - `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test test_continuous_optimization_sdk_leaf_readmes_freeze_version_decision_source_boundary -- --nocapture`
  - `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`

## 下一轮建

- SDK release 链当前剩余的更小真实缺口已经落到容器 README
  - `versionDecisionSourcePath = null` 还没有补齐app/admin 容器入口

