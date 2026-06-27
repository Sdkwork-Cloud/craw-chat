# Lifecycle Profile Doc Contract Alignment Implementation Plan

## Goal

- 关闭 lifecycle profile 已实现但公开 operator 文档未追平的合同缺口。

## Steps

- 写失败测试冻结 README 与快速启动文档中的 `standalone.split-services.development` lifecycle 示例。
- 回写 `install/init/start/restart/stop` 的 PowerShell、Bash、CMD 示例。
- 补生命周期参数表与 `standalone.split-services.development` config/runtime 边界说明。
- 回跑部署文档契约测试、格式检查和包级回归。

## Boundary

- 不修改脚本实现。
- 不把 `standalone.split-services.development` 描述成独立 runtime topology。
