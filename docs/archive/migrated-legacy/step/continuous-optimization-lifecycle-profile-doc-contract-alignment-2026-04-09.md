# Continuous Optimization: Lifecycle Profile Doc Contract Alignment

## Goal

- 让 README 和快速启动文档与 lifecycle 脚本的 profile 能力完全对齐。

## Scope

- 修改 `README.md`、`docs/部署/快速启动脚本.md`。
- 扩展 `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`。

## Implementation

- 先写失败测试冻结 `standalone.split-services.development` lifecycle 示例与兼容边界说明。
- 回写三端 `install/init/start/restart/stop` 的 profile 示例。
- 补 lifecycle 参数表与 `.runtime/standalone.split-services.development/config/standalone.split-services.development.env` 说明。
- 回跑部署文档契约测试、格式检查和包级回归。

## Expected State

- operator 从 README 或快速启动文档进入，都能看到同一套 profile 合同。
- `standalone.split-services.development` 示例不再只停留在 `status/deploy/runtime ops`。

## Boundary

- 本轮是文档追平，不改变脚本运行行为。
- `standalone.split-services.development` 继续复用 `.runtime/standalone.split-services.development` 运行目录合同。
