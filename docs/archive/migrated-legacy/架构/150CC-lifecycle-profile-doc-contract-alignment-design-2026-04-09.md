# Lifecycle Profile Doc Contract Alignment Design

## Decision

- 顶层 operator 文档必须显式公开 lifecycle 全链的 profile 入口，而不能只公开 runtime ops 或 deploy 侧入口。

## Contract

- README 与快速启动文档都必须展示：
  - `install/init/start/restart/stop` 的 `standalone.development` 示例
  - `--profile <standalone.development|standalone.development>` / `-ProfileName <standalone.development|standalone.development>`
  - `.runtime/standalone.development/config/standalone.development.env`
  - `standalone.development` 当前仍复用 `.runtime/standalone.development` runtime-dir

## Boundary

- 这是文档合同设计，不改变 runtime selection 实现。
- 若未来 `standalone.development` 拥有独立 topology，继续扩充同一入口，不新增别名文档。
