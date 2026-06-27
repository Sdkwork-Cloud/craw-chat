# Lifecycle Profile Doc Contract Alignment Design

## Decision

- 顶层 operator 文档必须显式公开 lifecycle 全链的 profile 入口，而不能只公开 runtime ops 或 deploy 侧入口。

## Contract

- README 与快速启动文档都必须展示：
  - `install/init/start/restart/stop` 的 `standalone.split-services.development` 示例
  - `--profile <standalone.split-services.development|standalone.split-services.development>` / `-ProfileName <standalone.split-services.development|standalone.split-services.development>`
  - `.runtime/standalone.split-services.development/config/standalone.split-services.development.env`
  - `standalone.split-services.development` 当前仍复用 `.runtime/standalone.split-services.development` runtime-dir

## Boundary

- 这是文档合同设计，不改变 runtime selection 实现。
- 若未来 `standalone.split-services.development` 拥有独立 topology，继续扩充同一入口，不新增别名文档。
