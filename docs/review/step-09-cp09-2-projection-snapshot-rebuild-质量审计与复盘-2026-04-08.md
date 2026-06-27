# Step 09 / CP09-2 projection snapshot rebuild 质量审计与复盘- 2026-04-08

## 审计范围
- `crates/sdkwork-im-contract-core/src/lib.rs`
- `crates/sdkwork-im-contract-message/src/lib.rs`
- `adapters/local-memory/src/lib.rs`
- `adapters/local-disk/src/metadata.rs`
- `adapters/local-disk/src/projection.rs`
- `services/projection-service/src/model.rs`
- `services/projection-service/src/projection.rs`
- `services/projection-service/src/snapshot.rs`
- `services/projection-service/tests/projection_snapshot_test.rs`
- `crates/im-platform-contracts/tests/contract_split_smoke_test.rs`
- `crates/im-platform-contracts/tests/contracts_smoke_test.rs`

## 审计结论
- 本轮未发现阻塞当前增量交付的剩余缺陷
- 改动符合 `CP09-2` 的真实目标：`projection-service` 开始具备基于统一 storage port snapshot 导出与恢复能力
- 改动没有回退 `Step 02` 已建立的结构边界：
  - `projection-service/src/lib.rs` 仍保持主干装
  - 新语义单独落`projection-service/src/snapshot.rs`

## 正向结果
- `MetadataStore` `TimelineProjectionStore` 不再只是“写接口”
- `projection-service` 现在可以
  - 持久conversation summary snapshot
  - 持久timeline projection entries
  - 在新service 实例中从共享 store 恢复这两类视
- 恢复路径已明tenant 作用域：
  - 相同 `conversation_id` 在不tenant 下不会因为共享底store 而互相覆

## 本轮发现并修正的问题
- `CP09-1` 虽然已经补齐 file adapter，但底层 contract 仍缺少读取语义，导致 store 无法restore 路径消费
- `projection-service` 之前只能在进程内持有投影状态，缺少显式 export/import/rebuild seam
- `TimelineViewEntry / ConversationSummaryView` 之前只有 `Serialize`，无法作为恢复输入反序列化

## 剩余风险
- 当前 restore 只覆`summary + timeline`
  - `inbox`
  - `member snapshot`
  - `read cursor`
  - `conversation catalog`
  仍未进入统一 snapshot rebuild 语义
- 当前 restore 仍是 `projection-service` 级能力，不等`managed runtime-dir` 已真实消费该路径
- 当前没有 plane-level metrics / tracing 去观snapshot persist/restore 的耗时、失败率lag

## 验证证据
- `cargo test -p projection-service --test projection_snapshot_test --offline`
- `cargo test -p projection-service --offline`
- `cargo test -p im-platform-contracts --offline`
- `cargo test -p im-adapters-local-memory --offline`
- `cargo test -p im-adapters-local-disk --offline`
- `cargo fmt --all --check`

## 复盘结论
- 本轮最正确的决策，是没有去新建更大`storage-core` 或新一projection adapter，而是先把 `CP09-1` 已冻结的 storage port 真正变成可读回、可恢复盘rebuild seam
- 这让 `Step 09` 从“已adapter 雏形”推进到了“已有第一条可验证 projection snapshot/rebuild 路径”
- `CP09-2` 仍只是第一段落地，不应误判为整`Step 09` 已通过
