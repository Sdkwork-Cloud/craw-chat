> Migrated from `docs/review/step-09-cp09-4-runtime-backup-archive-path-架构兑现与回写决议-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 09 / CP09-4 runtime backup archive path 架构兑现与回写决议 - 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`
- `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md`

## 本轮已兑现能力力力力力
- `09`
  - `Wave C / Step 09 / CP09-4` 已开始出现第一段真实落地：
    - runtime-dir backup snapshot 已有正式 archive 代码与脚本入口
    - archive 后仍保留清晰 restore path
- `138`
  - 灾备恢复链路现在不只包含 repair / restore，也开始具备“先归档 snapshot，再按需恢复”的最小本地路径
  - archived snapshot 仍可直接作为 restore preview / restore 输入，避免 archive 与 recovery 断开
- `141`
  - 本文第一次获得真实 as-built 证据：
    - runtime backup snapshot 可手动 archive
    - archive 动作可审计
    - catalog 可区分 `active / archived`
    - archived snapshot 仍可恢复

## 本轮未兑现能力力力力力
- `141`
  - retention policy 仍未建立
  - `legal hold` 仍未建立
  - `archived_at / restore_status / storage class` 等更完整生命周期元数据仍未建立
  - archive prune / delete / retry / observability 仍未建立
- `138`
  - tenant 级恢复、跨 cell / region 灾备演练仍未落地
- `132`
  - 本轮没有新增统一存储抽象层面的能力，只复核既有 runtime-dir file-backed seam
- `140`
  - 本轮没有新增新的 observability 能力，只复核现有 recovery / diagnostics 面
- `Step 09`
  - `CP09-4` 仍未通过
  - `Step 09` 仍未闭环

## 是否偏离架构
- 无偏离。
- 本轮实现属于“实现更具体”：
  - `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md` 提出了 archive / restore path 的原则
  - 当前代码把这件事先在 `Local Minimal` profile 的 runtime-dir snapshot 上 concretize 为一条最小路径
- 具体 as-built 语义是：
  - 先 archive 现有 managed backup snapshot
  - 归档后仍沿用同一 restore 合同恢复
  - 不额外引入新格式或新恢复旁路

## 回写决议
- `docs/架构/09-实施计划.md` 追加 `As-Built 95`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md` 追加 `As-Built 10`
- `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md` 追加 `As-Built 1`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
  - 本轮仅复核，不追加回写
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`
  - 本轮仅复核，不追加回写

## 证据
- 代码：
  - `services/sdkwork-im-cloud-gateway/src/node/runtime_dir.rs`
  - `services/sdkwork-im-cloud-gateway/src/node.rs`
  - `services/sdkwork-im-cloud-gateway/src/main.rs`
- 测试：
  - `services/sdkwork-im-cloud-gateway/tests/runtime_dir_backup_catalog_test.rs`
- 脚本：
  - `bin/archive-runtime-backup-local.ps1`
  - `bin/archive-runtime-backup-local.sh`
  - `bin/archive-runtime-backup-local.cmd`
  - `bin/retired-lifecycle-status.ps1`
  - `bin/retired-lifecycle-status.sh`
- 验证：
  - `cargo fmt --all --check`
  - `cargo test -p sdkwork-im-cloud-gateway --offline`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin/archive-runtime-backup-local.ps1 -Help`

## 当前判断
- 这是 `CP09-4` 的第一个真实代码增量。
- `CP09-4`：未通过。
- `Step 09`：未闭环。
- `Wave C / 93`：继续阻塞于 `Step 09 / CP09-4`。

