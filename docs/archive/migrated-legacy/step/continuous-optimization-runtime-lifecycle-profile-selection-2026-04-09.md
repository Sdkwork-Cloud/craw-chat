# Continuous Optimization: Runtime Lifecycle Profile Selection

## Goal

- 让 `init/install/start/stop/restart` 与现有 runtime ops 一样，稳定支持 `standalone.split-services.development / standalone.split-services.development` profile。

## Scope

- 修改 `bin/init-config-local.*`、`bin/retired-lifecycle-install.*`、`bin/retired-lifecycle-start.*`、`bin/retired-lifecycle-stop.*`、`bin/retired-lifecycle-restart.*`。
- 扩展 `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`。

## Implementation

- 先写失败测试，冻结 `standalone.split-services.development` config 入口与 restart 参数透传合同。
- 复用 `_runtime-profile-common.*` 解析 profile config 链与 runtime-dir。
- 对 `standalone.split-services.development` 保持“独立 config 文件 + 共享 standalone.split-services.development runtime-dir”的当前兼容策略。
- 复验 `deployment_profile_test`、格式检查与 `sdkwork-im-server` 包级离线测试。

## Expected State

- `init/install/start/stop/restart` 与 `status/inspect/repair` 使用同一 profile 语义。
- `retired-lifecycle-restart.*` 不再丢失 profile。
- CMD 帮助与 PowerShell 帮助面公开相同 profile 合同。

## Boundary

- 这轮不引入 `standalone.split-services.development` 独立拓扑。
- 原生 Bash 真实运行态证明仍需在有可用 Bash 的环境补齐。
