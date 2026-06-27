> Migrated from `docs/review/step-12-cp12-2-sdk目录README与facade边界收口-质量审计与复盘-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 12 / CP12-2 SDK 目录 README facade 边界收口 质量审计与复盘- 2026-04-08

## 审计范围
- `README.md`
- `sdks/README.md`
- `sdks/sdkwork-im-sdk/README.md`
- `sdks/sdkwork-control-plane-sdk/README.md`
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md`
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/README.md`
- `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/README.md`
- `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/README.md`
- `tools/chat-cli/tests/chat_cli_contract_test.rs`

## 审计结论
- 本轮未发现阻`CP12-2` 关闭的剩余缺陷
- 当前交付的价值不是“多写了几份 README”，而是`sdks/` 目录从无约束占位路径升级为受测试保护workspace 边界：
- app-facing admin-facing facade 的职责已经在仓库入口、总览 README、语言子目录三个层级上保持一致性

## 正向结果
- `sdks/README.md` 已成为两SDK 的统一入口，不再需要靠目录名猜测职责
- `sdkwork-im-sdk` `sdkwork-control-plane-sdk` 的边界已经分离：
  - 前者承载app-facing conversation / message / realtime facade
  - 后者承载`control-plane` / `protocol governance` / `compatibility matrix`
- `TypeScript` `Flutter` 目录现在是稳定入口，而不是后续容易被忽略的空文件夹
- `README.md` 已把 SDK 总览提升到仓库公开入口，后续不会再出现“文档存在但不可发现”的回归

## 仍需关注的风
- 当前 README 只冻结边界，不代表多语言 SDK 代码生成、发布与示例链已完成
- `CP12-2` 只解SDK facade 的目录与职责问题，不代表 `compatibility matrix` 已形成文/ 测试 / control-plane 三类证据闭环
- 子目标README 目前仍是占位入口，后续一旦落真实实现，必须继续遵守当app/admin 边界，不能把 control-plane 与聊facade 再次混在一起

## 验证证据
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test -- --nocapture`
- `cargo fmt --all --check`

## 复盘结论
- 本轮最重要的决策是先把 SDK 目录边界冻结成可测试资产，再继续推进 `CP12-3`
- 这样后续compatibility matrix control-plane 映射不会建立在模糊消费者边界上
- 目前 `CP12-2` 的结论可信：
  - workspace 入口已建
  - SDK facade 已分
  - 多语言子目录已具备最小可信的职责占位

