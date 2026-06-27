# timeline-scylla

`timeline-scylla` 是生产默认 `TimelineProjectionStore` 适配器目录保留位。

目标能力：

- append-friendly write
- conversation range query
- pagination by seq
- idempotent upsert
- rebuild from journal

后续实现要求：

- 保持时间线与会话摘要视图语义稳定
- 支持统一 conformance test
- 支持冷热分层、重建与分区治理

当前状态：

- 目录标准已冻结
- 具体实现待接入 ScyllaDB 表设计与重建流程

