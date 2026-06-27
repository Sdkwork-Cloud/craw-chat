# Step 09 / CP09-4 runtime backup archive path 执行卡 - 2026-04-08

## 当前上下文
- 当前波次：`Wave C`
- 当前 step：`Step 09`
- 当前子任务：`CP09-4`
- 前置状态：
  - `CP09-3` 已在本轮补齐 `rebuild duration` 后判定通过
  - `Step 09` 当前唯一剩余检查点就是：
    - `CP09-4`：备份、恢复、修复、归档策略已有代码和脚本支撑
  - 当前仓库真实已有：
    - runtime-dir inspection
    - repair
    - restore preview
    - restore
    - backup catalog
    - 对应 `bin/*.ps1 / *.sh / *.cmd`
  - 但 archive / retention 仍基本空白：
    - 没有任何真实 archive 入口
    - 没有 archive report
    - 现有 backup catalog 无法区分 active / archived snapshot

## 本轮为什么做这个子任务
- 相比直接做完整 retention engine、legal hold 或对象存储分层，最小且真实的下一步，是先把现有 runtime-dir backup snapshot 变成“可归档、可审计、归档后仍可恢复”的路径。
- 这条路径完全复用已有的 runtime-dir 资产：
  - 现有 backup snapshot 目录
  - 现有 restore preview / restore 合同
  - 现有 backup catalog
- 因此本轮最优决策是：
  - 先做 in-place runtime backup archive path
  - 保持 archived snapshot 仍可作为 `--backup-dir` 被 preview / restore 消费
  - 不在本轮提前扩成 retention 调度器或多层对象存储治理

## 本轮实际完成

### 1. `sdkwork-im-server` 新增 runtime backup archive owner seam
- `services/sdkwork-im-cloud-gateway/src/node/runtime_dir.rs`
  - 新增 `RuntimeDirArchiveView`
  - 新增 `archive_runtime_backup(...)`
  - 新增 `format_runtime_dir_archive(...)`
  - archive 动作会把选定 backup snapshot 重命名到：
    - `runtime-dir/backups/archived-<backup-name>`
  - 并在 archived snapshot 内写入：
    - `archive-report.json`
- 这意味着 archive 现在不再只是人工移动目录，而是有正式代码 owner 与审计产物

### 2. archive 后仍保留清晰 restore path
- archive 并不改写 snapshot 格式，也不复制成另一种归档结构
- archived snapshot 仍保留原始：
  - `state/`
  - restore / repair report
  - managed runtime state snapshot
- 因此 archived snapshot 现在仍可直接作为：
  - `preview-runtime-restore --backup-dir <archived-path>`
  - `restore-runtime-dir --backup-dir <archived-path>`
  的输入

### 3. backup catalog 现在可区分 active / archived snapshot
- `services/sdkwork-im-cloud-gateway/src/node/runtime_dir.rs`
  - `RuntimeDirBackupCatalogItemView` 新增：
    - `lifecycleStage`
  - `list_runtime_backups(...)` 现在会把 catalog item 明确标记为：
    - `active`
    - `archived`
  - 同时保留原始 `operation`：
    - `restore`
    - `repair`
- 这保证：
  - archived snapshot 不会再被误看成普通 active backup
  - 但它依然保留原来的 restore/repair 来源语义

### 4. CLI 与脚本已经完整接通 archive 入口
- `services/sdkwork-im-cloud-gateway/src/main.rs`
  - 新增：
    - `archive-runtime-backup --backup-dir <path> [--runtime-dir <path>] [--json]`
- 新增脚本：
  - `bin/archive-runtime-backup-local.ps1`
  - `bin/archive-runtime-backup-local.sh`
  - `bin/archive-runtime-backup-local.cmd`
- 更新脚本导航：
  - `bin/retired-lifecycle-status.ps1`
  - `bin/retired-lifecycle-status.sh`
- 这意味着 runtime-dir 的 `inspect / repair / list / archive / preview / restore` 现在已形成连续操作面

### 5. 自动化测试证明 archive 后仍可 preview restore
- `services/sdkwork-im-cloud-gateway/tests/runtime_dir_backup_catalog_test.rs`
  - 现已新增 archive 场景回归：
    - archive 会把 active snapshot 移到 archived 路径
    - catalog 会把该 item 标记为 `archived`
    - `preview_restore_runtime_dir(...)` 仍可消费 archived snapshot
- 这条测试直接证明：
  - 归档没有切断恢复路径
  - archive 不是“挪走就算完”，而是真正与 restore 合同联通

## 改动范围
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
- 文档：
  - 本执行卡
  - 本轮质量审计与复盘
  - 本轮架构兑现与回写决议
  - `docs/架构/09-实施计划.md`
  - `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
  - `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md`

## TDD 证据

### Red
- 先写测试，再验证缺口：
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test runtime_dir_backup_catalog_test`
- 红测失败点与预期一致：
  - `sdkwork-im-server` 还没有 `archive_runtime_backup(...)`
  - `RuntimeDirBackupCatalogItemView` 还没有 `lifecycle_stage`

### Green
- 定向测试现已通过：
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test runtime_dir_backup_catalog_test`

## 回归验证
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/archive-runtime-backup-local.ps1 -Help`

## 结论
- 这是 `Wave C / Step 09 / CP09-4` 的第一个真实代码增量。
- `Step 09` 现在不再只有 backup / restore / repair，而是第一次有了真实 archive 代码和脚本路径。
- 但 `CP09-4` 仍不能整体判定通过，因为当前仍缺：
  - retention policy
  - legal hold
  - archive 生命周期治理
  - 更完整的 archive catalog / prune / restore drill 审计

## 下一轮继续做什么
1. 继续留在 `CP09-4`，优先补 archive / retention 的下一段真实治理增量。
2. 评估是否在现有 runtime-dir toolchain 上补：
  - archive catalog 细化
  - retention metadata
  - archive prune / restore drill 审计
3. 在这些能力完成前，不允许把 `CP09-4`、`Step 09` 或 `Wave C / 93` 判定为通过。
