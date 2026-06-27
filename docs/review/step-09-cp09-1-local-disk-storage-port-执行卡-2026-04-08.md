# Step 09 / CP09-1 local-disk storage port 执行卡 - 2026-04-08

## 当前上下文
- 当前波次：`Wave C`
- 当前 step：`Step 09`
- 当前子任务：`CP09-1`
- 前置状态：
  - `Step 08` 已于 `2026-04-08` 闭环完成
  - `Wave C / 93` 仍阻塞于 `Step 09`
  - `local-memory` 已具备 `MetadataStore` 与 `TimelineProjectionStore` 的内存实现
  - `local-disk` 在本轮开始前仍缺对应 file adapter，导致“统一存储抽象 + 本地适配器对齐”存在明显断口

## 本轮为什么做这个子任务
- `docs/step/09-存储投影与可观测治理.md` 把 `CP09-1` 明确成“存储抽象与适配边界已统一”。
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md` 已冻结 `MetadataStore`、投影模型与“先抽象、后替换底层”的实施原则。
- 如果 `local-memory` 和 `local-disk` 对平台契约的覆盖面不对齐，`managed standalone.split-services.development` 就无法被视为真实存储适配基线，`CP09-1` 只能停留在“有 trait、无完整 adapter”的伪完成状态。

## 本轮实际完成

### 1. 为 `local-disk` 补齐 `MetadataStore`
- `adapters/local-disk/src/metadata.rs`
  - 新增 `FileMetadataStore`
  - 新增 `validate_metadata_store_file(...)`
  - 采用与现有 file store 一致的 JSON 原子写入模式
- `FileMetadataStore` 现已支持：
  - `put_snapshot(...)`
  - 跨 reopen 保留最新 snapshot
  - 文件形状校验

### 2. 为 `local-disk` 补齐 `TimelineProjectionStore`
- `adapters/local-disk/src/projection.rs`
  - 新增 `FileTimelineProjectionStore`
  - 新增 `validate_timeline_projection_store_file(...)`
  - 按 `conversation_id -> message_seq -> payload` 进行稳定 upsert
- 这让 `local-disk` 拥有与 `local-memory` 对等的 timeline projection 持久化端口，而不是只剩 runtime state 类 store。

### 3. 保持 adapter 模块边界不回退
- `adapters/local-disk/src/lib.rs`
  - 只负责模块导出，不回退到把实现重新堆回 `lib.rs`
- `adapters/local-disk/tests/lib_structure_test.rs`
  - 新增 metadata / projection surface 结构门禁

## 改动范围
- 代码：
  - `adapters/local-disk/src/lib.rs`
  - `adapters/local-disk/src/metadata.rs`
  - `adapters/local-disk/src/projection.rs`
- 测试：
  - `adapters/local-disk/tests/storage_port_test.rs`
  - `adapters/local-disk/tests/lib_structure_test.rs`

## TDD 证据

### Red
- `cargo test -p im-adapters-local-disk --test storage_port_test --offline`
  - 失败点与预期一致：
    - `FileMetadataStore`
    - `FileTimelineProjectionStore`
    - `validate_metadata_store_file(...)`
    - `validate_timeline_projection_store_file(...)`
  - 上述符号在 `local-disk` root 中均不存在，说明当前缺口确实是“适配器未落地”，而不是测试误判。

### Green
- `cargo test -p im-adapters-local-disk --test storage_port_test --offline`
- `cargo test -p im-adapters-local-disk --test lib_structure_test --offline`

## 回归验证
- `cargo test -p im-adapters-local-disk --offline`
- `cargo fmt -p im-adapters-local-disk -- --check`

## 结论
- 这是 `Wave C / Step 09 / CP09-1` 的第一个真实代码增量。
- `local-disk` 已补齐 `MetadataStore` 与 `TimelineProjectionStore`，`local-memory / local-disk` 在当前已冻结平台契约上的对齐度明显提升。
- 但 `CP09-1` 仍不能整体判定通过，因为 `Step 09` 的“统一存储抽象”还没有扩展到 projection rebuild、backup/restore 实际接线，以及更完整的 storage port 族闭环。

## 当前还差什么
- `projection-service` 仍没有真实 projection rebuild/export/import 语义
- `Step 09 / CP09-2` 的 rebuild / recovery 证据仍未落地
- `Step 09 / CP09-3` 的 metrics / tracing / logging 尚未按 plane 收口
- `Step 09 / CP09-4` 的 backup / restore / repair / archive 仍未形成 step-wide 审计闭环

## 下一轮继续做什么
1. 优先把 `projection-service` 的 snapshot / rebuild 能力变成真实可验证路径
2. 让 `managed runtime-dir` 的恢复链路能消费 projection rebuild，而不是只恢复 runtime state
3. 再进入 `CP09-3 / CP09-4` 的观测与恢复整合，而不是提前宣称 `Step 09` 通过
