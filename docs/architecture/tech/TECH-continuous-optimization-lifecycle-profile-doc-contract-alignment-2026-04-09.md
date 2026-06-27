> Migrated from `docs/review/continuous-optimization-lifecycle-profile-doc-contract-alignment-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Lifecycle Profile Doc Contract Alignment

## Context

- lifecycle 脚本已支持 `standalone.split-services.development / standalone.split-services.development` profile。
- `README.md` 与 `docs/部署/快速启动脚本.md` 仍主要展示旧的 `standalone.split-services.development` 示例。

## Confirmed Bug

- operator 入口文档没有对称公开 `install/init/start/restart/stop` 的 `standalone.split-services.development` 用法。
- 文档参数表也没有完整列出 lifecycle 全链的 `--profile` / `-ProfileName`。

## Root Cause

- 上一轮先完成了脚本实现与测试，公开文档没有同步补齐。
- 文档只覆盖了 `status/deploy/runtime ops` 的 profile 示例，遗漏 lifecycle 主链。

## Fix

- 在 `README.md` 补 `install/init/start/status/restart/stop` 的 `standalone.split-services.development` 示例。
- 在 `docs/部署/快速启动脚本.md` 补 lifecycle 全链 profile 参数、三端示例、运行产物与兼容边界说明。
- 新增回归测试 `test_quick_start_doc_surfaces_local_default_profile_examples_across_lifecycle_commands`。

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_quick_start_doc_surfaces_local_default_profile_examples_across_lifecycle_commands -- --exact --nocapture
```

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test -- --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Result

- 顶层 README 与快速启动文档已对齐脚本真实能力。
- `standalone.split-services.development` config 入口与当前共享 runtime-dir 的边界都已公开。

## Boundary

- 本轮只修正文档合同，不引入新的 runtime topology。
- 原生 Bash 执行态仍保持“在可用 Bash 环境补实机证明”的既有边界。

