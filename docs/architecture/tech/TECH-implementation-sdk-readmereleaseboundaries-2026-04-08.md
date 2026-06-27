> Migrated from `docs/架构/09W-实施计划-sdk叶子README发布边界收敛-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09W - 实施计划 - SDK 叶子 README 发布边界收敛

## 目标

让四个叶子 SDK README 与 bundle 级 `sdk-release-catalog.json` 共享同一份发布状态真源，避免发布边界只停留在 workspace 总入口和 release bundle，而不下沉到具体语言入口。

## 最小实施面

1. 先在 `tools/chat-cli/tests/chat_cli_contract_test.rs` 写红测
2. 更新四个叶子 README：
   - app TypeScript
   - app Flutter
   - admin TypeScript
   - admin Flutter
3. 每个 README 至少补齐：
   - `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
   - `template_only_pending_generation`
   - `not_published`
4. 回归叶子 README contract test
5. 回写 `docs/step`、`docs/架构`、`docs/review`

## 约束

- 不伪造真实 SDK 版本号
- 不伪造真实 SDK 生成结果
- 不伪造真实发包状态
- README 只回链 bundle 真源，不在叶子目录私自维护第二套发布状态

## 放行标准

- 四个叶子 README 全部公开 release catalog 真源路径
- 四个叶子 README 全部公开模板态与未发布状态
- fresh `fmt --check` 与 `chat_cli_contract_test` 保持通过

