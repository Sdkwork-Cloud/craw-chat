> Migrated from `docs/step/13-CP13-1C-sdk叶子README发布边界收敛-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 13-CP13-1C - SDK 叶子 README 发布边界收敛

## 本轮目的

bundle `sdk-release-catalog.json` 的发布状态，继续下沉到四个叶SDK README，避免消费方进入具体语言目录后再次回到“只看到职责说明、看不到当前发布边界”的状态

## 发现的问

- `sdk-release-catalog.json` schema 已经落地
- `sdks/README.md` 也已经公开bundle catalog
- 但四个叶SDK README 仍只冻结职责与约束，没有公开
  - `sdk-release-catalog.json`
  - `template_only_pending_generation`
  - `not_published`
- 结果是：workspace 总入口知道发布边界，具体语言入口却看不到当前 bundle 真源

## 本轮决策

- 继续使用 `tools/chat-cli/tests/chat_cli_contract_test.rs` 作为公开文档 contract gate
- 新增 `test_continuous_optimization_sdk_leaf_readmes_freeze_release_catalog_boundary`
- 最小要求：
  - 四个叶子 README 都必须回写`artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
  - 都必须明确当前状态：
    - `template_only_pending_generation`
    - `not_published`

## 实施结果

- 更新
  - `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md`
  - `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/README.md`
  - `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/README.md`
  - `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/README.md`
- 四个叶子 README 现已补齐
  - bundle catalog 真源路径
  - 当前模板态生成状态
  - 当前未发布状态

## 验证

- 红灯
  - `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_sdk_leaf_readmes_freeze_release_catalog_boundary -- --exact --nocapture`
  - 失败点：`app TypeScript README must contain release catalog boundary text sdk-release-catalog.json`
- 绿灯
  - `cargo fmt --all --check`
  - `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_sdk_leaf_readmes_freeze_release_catalog_boundary -- --exact --nocapture`
  - `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`

## 下一轮建

- 若继续推进真SDK 生成/版本冻结链，可把叶子 README 的边界说明继续升级为
  - 版本来源
  - 生成输入
  - 发布仓库坐标
- 在真实生成与发布链完成前，继续保持“README 引用 bundle 真源，不独自发明版本结论”的约束

