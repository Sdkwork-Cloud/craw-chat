> Migrated from `docs/架构/150CC-lifecycle-profile-doc-contract-alignment-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Lifecycle Profile Doc Contract Alignment Design

## Decision

- 顶层 operator 文档必须显式公开 lifecycle 全链的 profile 入口，而不能只公开 runtime ops 或 deploy 侧入口。

## Contract

- README 与快速启动文档都必须展示：
  - `install/init/start/restart/stop` 的 `standalone.split-services.development` 示例
  - `--profile <standalone.split-services.development|standalone.split-services.development>` / `-ProfileName <standalone.split-services.development|standalone.split-services.development>`
  - `etc/topology/standalone.development.env` 为当前配置权威
  - 进程状态和一次性生成配置位于源码树外的私有 OS/CI 临时目录

## Boundary

- 这是文档合同设计，不改变 runtime selection 实现。
- 若未来 `standalone.split-services.development` 拥有独立 topology，继续扩充同一入口，不新增别名文档。
