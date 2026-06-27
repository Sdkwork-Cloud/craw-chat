# journal-redpanda

`journal-redpanda` 是生产默认 `CommitJournal` 适配器目录保留位。

目标能力：

- ordered append
- durable ack
- replay
- checkpoint
- retention
- partition routing

后续实现要求：

- 不改变 `CommitEnvelope` 语义
- 支持统一 conformance test
- 支持 SaaS shared / dedicated 与私有化 profile 装配

当前状态：

- 目录标准已冻结
- 具体实现待接入 Redpanda 客户端与运维 profile

