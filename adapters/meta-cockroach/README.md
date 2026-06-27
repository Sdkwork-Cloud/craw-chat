# meta-cockroach

`meta-cockroach` 是生产默认 `MetadataStore` 适配器目录保留位。

目标能力：

- transaction
- unique constraint
- optimistic concurrency
- secondary index
- tenant scope

后续实现要求：

- 不改变租户与实体主键语义
- 支持统一 conformance test
- 支持 SaaS 与私有化 profile 的连接与迁移策略

当前状态：

- 目录标准已冻结
- 具体实现待接入 CockroachDB schema 与迁移脚本

