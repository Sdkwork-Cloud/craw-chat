# Runtime Lifecycle Profile Selection Implementation Plan

## Goal

- 关闭 lifecycle 脚本与 runtime ops 在 profile 选择上的合同分裂。

## Steps

- 用失败测试冻结 `standalone.split-services.development` init-config 与 restart 透传行为。
- 为 `init/install/start/stop/restart` 补 `-ProfileName` / `--profile`。
- 统一复用 `_runtime-profile-common.*` 解析 config 链和 runtime-dir。
- 保持 `standalone.split-services.development -> standalone.split-services.development runtime-dir` 的当前兼容回退。
- 回跑 `deployment_profile_test`、格式检查和 `sdkwork-im-server` 离线测试。

## Boundary

- 不在本轮引入新的 profile 或独立 standalone.split-services.development topology。
- 不把无 Bash 环境下的跳过测试包装成已完成的原生 Bash 证明。
