# Step 09 / CP09-1 local-disk storage port 质量审计与复盘 - 2026-04-08

## 审计范围
- `adapters/local-disk/src/lib.rs`
- `adapters/local-disk/src/metadata.rs`
- `adapters/local-disk/src/projection.rs`
- `adapters/local-disk/tests/storage_port_test.rs`
- `adapters/local-disk/tests/lib_structure_test.rs`

## 审计结论
- 本轮未发现阻塞当前增量交付的剩余缺陷。
- `local-disk` 对平台存储契约的覆盖面已从“运行态 store 为主”扩展到“metadata + projection”。
- 本轮没有回退 `Step 02` 已建立的模块边界，`lib.rs` 仍保持薄 facade。

## 正向结果
- `MetadataStore` 与 `TimelineProjectionStore` 现在都具备：
  - file-backed 最小实现
  - reopen 持久化行为
  - 非法文件形状校验
- `local-memory` 与 `local-disk` 现在对这两个平台端口保持语义对齐，有利于后续把 projection rebuild 与 backup/restore 从“只依赖内存语义”推进到“可替换底层”。
- `cargo test -p im-adapters-local-disk --offline` 与 `cargo fmt -p im-adapters-local-disk -- --check` 均通过，说明这轮增量没有破坏已有 file store。

## 本轮发现并修正的问题
- `local-disk` 之前缺失 `MetadataStore`，导致 metadata snapshot 无法复用统一 file adapter 模式。
- `local-disk` 之前缺失 `TimelineProjectionStore`，导致 timeline projection 端口只有内存基线，没有本地持久化对等物。
- `adapters/local-disk/src/lib.rs` 之前没有结构门禁覆盖 metadata / projection surface，无法防止后续实现回流到 `lib.rs`。

## 剩余风险
- `CP09-1` 还不是整项闭环：
  - projection rebuild 尚未形成可执行恢复链路
  - 更完整的 storage port 族仍未全部对齐
- `Step 09` 的关键闭环项依然缺失：
  - projection rebuild
  - plane-level metrics / tracing / logging
  - backup / restore / repair / archive 的整步审计
- 当前 `FileTimelineProjectionStore` 只解决最小 upsert 语义，还没有接入 `projection-service` 的真实 rebuild 流程。

## 验证证据
- `cargo test -p im-adapters-local-disk --test storage_port_test --offline`
- `cargo test -p im-adapters-local-disk --test lib_structure_test --offline`
- `cargo test -p im-adapters-local-disk --offline`
- `cargo fmt -p im-adapters-local-disk -- --check`

## 复盘结论
- 本轮最有效的决策，是不凭空新建一层 `storage-core` crate，而是先把当前仓库已经冻结的存储契约补到真实 adapter 上。
- 这属于 `CP09-1` 的必要前置条件闭合，不等于 `Step 09` 整体通过。
- 下一轮应继续沿“projection rebuild / recovery 可执行”推进；如果转去扩更多 store 名称或空接口，只会制造新的伪完成。
