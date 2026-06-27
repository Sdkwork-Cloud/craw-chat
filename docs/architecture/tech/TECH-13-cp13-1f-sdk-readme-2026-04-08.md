> Migrated from `docs/step/13-CP13-1F-sdk容器README版本占位收敛-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 13-CP13-1F - SDK 容器 README 版本占位收敛

## 本轮目的

`plannedVersion / versionStatus` 版本占位 contract，从 bundle、workspace 总入口和叶子 README 继续补到 app/admin 两层容器 README，避免中间入口层级仍然看不到“版本尚未冻结”的真实边界：

## 发现的问

- `sdks/README.md` 已公开
  - `plannedVersion = null`
  - `versionStatus = version_unassigned_pending_freeze`
- 四个叶子 README 也已同步这组边界
- `sdkwork-im-sdk/README.md` `sdkwork-control-plane-sdk/README.md` 两个容器入口还没有显式表达版本占位状态
- 结果是：根入口和叶子入口一致了，中间层仍有断层

## 本轮决策

- 继续使用 `tools/chat-cli/tests/chat_cli_contract_test.rs` contract gate
- 新增 `test_continuous_optimization_sdk_container_readmes_freeze_version_placeholder_boundary`
- 最小要求：
  - 两个容器 README 都必须公开
    - `plannedVersion = null`
    - `versionStatus = version_unassigned_pending_freeze`

## 实施结果

- 更新
  - `sdks/sdkwork-im-sdk/README.md`
  - `sdks/sdkwork-control-plane-sdk/README.md`
- 现在 SDK release 入口链的三个层级都已显式对齐版本占位
  - `sdks/README.md`
  - `sdkwork-im-sdk* / sdkwork-control-plane-sdk*` 容器 README
  - 四个叶子 README

## 验证

- 红灯
  - `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_sdk_container_readmes_freeze_version_placeholder_boundary -- --exact --nocapture`
  - 失败点：`app SDK container README must contain version placeholder boundary text plannedVersion`
- 绿灯
  - `cargo fmt --all --check`
  - `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_sdk_container_readmes_freeze_version_placeholder_boundary -- --exact --nocapture`
  - `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`

## 下一轮建

- SDK release 链当前剩余的更小真实缺口，已经不再是入口层级的一致性，而是
  - 真实版本 freeze 决议来源
  - 真实生成输入
  - 真实发包归档

