> Migrated from `docs/review/step-09-cp09-4-runtime-backup-archive-retention-lifecycle-执行卡-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 09 / CP09-4 runtime backup archive retention lifecycle 执行卡 - 2026-04-08

## 当前上下文
- 当前波次：`Wave C`
- 当前 step：`Step 09`
- 当前子任务：`CP09-4`
- 前置状态：
  - `CP09-3` 已通过
  - `CP09-4` 已在上一增量补齐 archive path
  - 当前唯一剩余缺口已经收敛为：
    - retention metadata
    - legal hold
    - archive prune
    - archive lifecycle evidence

## 本轮为什么做这个子任务
- `docs/step/09-存储投影与可观测治理.md` 对 `CP09-4` 的要求是：
  - 备份、恢复、修复、归档策略已有代码和脚本支撑
- 上一增量虽然补齐了 archive path，但仍不足以证明“归档策略”已经成立：
  - 没有 retention policy
  - 没有 legal hold
  - 没有 prune action
- 因此本轮最优决策不是扩对象存储，也不是新造后台 worker，而是继续沿现有 runtime-dir seam，把 archive lifecycle 补到最小闭环。

## 本轮实际完成

### 1. archive 现在会写出正式 lifecycle metadata
- `services/sdkwork-im-cloud-gateway/src/node/runtime_dir.rs`
  - 新增 `archive_runtime_backup_with_policy(...)`
  - archived snapshot 现在会写出 `archive-metadata.json`
  - `RuntimeDirArchiveView` 与 `RuntimeDirBackupCatalogItemView` 现已公开：
    - `storageClass`
    - `retentionPolicy`
    - `retentionDays`
    - `restoreStatus`
    - `legalHold`
    - `archivedAt`
- 这使 archive 不再只是“重命名一个目录”，而是开始具备生命周期语义。

### 2. retention / legal hold / prune 已有 owner seam
- `services/sdkwork-im-cloud-gateway/src/node/runtime_dir.rs`
  - 新增 `prune_archived_runtime_backups(...)`
  - prune 只处理 archived snapshot
  - 缺 metadata 的 archived snapshot 会被明确跳过
  - `legalHold = true` 的 archived snapshot 会被保留
  - retention 已到期的 archived snapshot 会被物理删除
- 这意味着 archive 现在第一次具备“何时允许清理、何时必须保留”的真实治理路径。

### 3. CLI 与脚本已完整接通 lifecycle 动作
- `services/sdkwork-im-cloud-gateway/src/main.rs`
  - `archive-runtime-backup` 现已支持：
    - `--retention-days <days>`
    - `--legal-hold`
  - 新增：
    - `prune-archived-runtime-backups [--runtime-dir <path>] [--json]`
- 新增脚本：
  - `bin/prune-runtime-archives-local.ps1`
  - `bin/prune-runtime-archives-local.sh`
  - `bin/prune-runtime-archives-local.cmd`
- 更新脚本：
  - `bin/archive-runtime-backup-local.ps1`
  - `bin/archive-runtime-backup-local.sh`
  - `bin/retired-lifecycle-status.ps1`
  - `bin/retired-lifecycle-status.sh`
  - `bin/_cmd-forward-powershell.cmd`

### 4. 自动化测试证明 archive lifecycle 行为正确
- `services/sdkwork-im-cloud-gateway/tests/runtime_dir_backup_catalog_test.rs`
  - 先写红测，再验证：
    - 新函数不存在
    - 新 catalog 字段不存在
  - 现已验证：
    - archive 会写出 metadata
    - catalog 会暴露 lifecycle fields
    - retention=0 且未 hold 的 archive 会被 prune
    - legal hold archive 会被保留

## TDD 证据

### Red
- `cargo test -p sdkwork-im-cloud-gateway --offline --test runtime_dir_backup_catalog_test`
- 红测失败点与预期一致：
  - 缺 `archive_runtime_backup_with_policy(...)`
  - 缺 `prune_archived_runtime_backups(...)`
  - 缺 archive lifecycle catalog fields

### Green
- `cargo test -p sdkwork-im-cloud-gateway --offline --test runtime_dir_backup_catalog_test`

## 回归验证
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/archive-runtime-backup-local.ps1 -Help`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/prune-runtime-archives-local.ps1 -Help`

## 结论
- 这是 `CP09-4` 的第二个真实代码增量。
- 当前仓库已经具备：
  - backup
  - restore preview
  - restore
  - repair
  - archive
  - retention metadata
  - legal hold
  - archive prune
  的连续工具链。
- `CP09-4`：通过。

## 下一轮继续做什么
1. 不再停留在 `CP09-4`
2. 立刻转入 `Step 09` 的整步 `91 / 95 / 97` 审计
3. 若整步通过，则继续执行 `Wave C / 93` 总验收

