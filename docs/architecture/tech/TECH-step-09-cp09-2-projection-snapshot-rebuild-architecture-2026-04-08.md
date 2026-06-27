> Migrated from `docs/review/step-09-cp09-2-projection-snapshot-rebuild-架构兑现与回写决议-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 09 / CP09-2 projection snapshot rebuild 架构兑现与回写决议- 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/132-存储架构与自主演进路线设计2026-04-06.md`
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`
- `docs/架构/141-数据生命周期与归档成本治理设计2026-04-06.md`

## 本轮已兑现能力力力力力
- `09`
  - `Wave C / Step 09 / CP09-2` 已完成第一段真实落地：
    - `projection-service` 可以conversation summary + timeline snapshot 导出到统一 storage port
    - 新的 `projection-service` 实例可以从共storage port 恢复这两类视
- `132`
  - “先冻结抽象、再adapter service 消费这些抽象”的路线有了新的代码证据
  - `MetadataStore / TimelineProjectionStore` 不再只是落盘接口，而开始进projection rebuild 链路
  - snapshot restore 明确使用 tenant-scoped key，避免多租户 conversation id 冲突污染同一projection store

## 本轮未兑现能力力力力力
- `140`
  - snapshot persist / restore metrics / tracing / logging 本轮未触发
- `141`
  - snapshot export retention / archive / lifecycle policy 本轮未触发
- `Step 09`
  - `CP09-2` 仍未形成整步 rebuild / recovery 闭环
    - inbox / member / read-cursor / conversation catalog 仍未覆盖
    - `managed runtime-dir` 仍未消费 snapshot restore 路径

## 是否偏离架构
- 无偏离
- 本轮实现严格遵守`132` 的路线：
  - 先让统一 storage port 获得完整读写语义
  - 再让 projection service 消费这些端口
  - 没有跳过抽象直接把恢复逻辑绑死在某个具file adapter 或某runtime builder 

## 回写决议
- `docs/架构/09-实施计划.md` 追加 `As-Built 84`
- `docs/架构/132-存储架构与自主演进路线设计2026-04-06.md` 追加 `As-Built 2`
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`
  - 本轮仅复核，不追加回写，等待 plane-level observability 证据
- `docs/架构/141-数据生命周期与归档成本治理设计2026-04-06.md`
  - 本轮仅复核，不追加回写，等待 archive / retention / lifecycle 证据

## 证据
- 代码
  - `services/projection-service/src/snapshot.rs`
  - `services/projection-service/src/model.rs`
  - `crates/sdkwork-im-contract-core/src/lib.rs`
  - `crates/sdkwork-im-contract-message/src/lib.rs`
  - `adapters/local-memory/src/lib.rs`
  - `adapters/local-disk/src/metadata.rs`
  - `adapters/local-disk/src/projection.rs`
- 测试
  - `services/projection-service/tests/projection_snapshot_test.rs`
  - `crates/im-platform-contracts/tests/contract_split_smoke_test.rs`
  - `crates/im-platform-contracts/tests/contracts_smoke_test.rs`
- 验证
  - `cargo test -p projection-service --test projection_snapshot_test --offline`
  - `cargo test -p projection-service --offline`
  - `cargo test -p im-platform-contracts --offline`
  - `cargo test -p im-adapters-local-memory --offline`
  - `cargo test -p im-adapters-local-disk --offline`
  - `cargo fmt --all --check`

## 当前判断
- 这是 `CP09-2` 的真实增量，不是 `Step 09` 的整步通过
- `CP09-2`：继续推进中，尚不能整体判定通过
- `Step 09`：未闭环
- `Wave C / 93`：继续阻塞于 `Step 09`

