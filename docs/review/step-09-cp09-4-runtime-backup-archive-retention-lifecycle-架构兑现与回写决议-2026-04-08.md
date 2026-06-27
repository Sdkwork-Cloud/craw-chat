# Step 09 / CP09-4 runtime backup archive retention lifecycle 架构兑现与回写决议 - 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
- `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md`

## 本轮已兑现能力力力力力
- `09`
  - `CP09-4` 现在不再只有 archive path，而是具备 archive lifecycle metadata、retention policy、legal hold 与 prune 动作
- `138`
  - archived snapshot 现在既保留 restore path，又具备 hold-aware retention 清理语义
- `141`
  - `storageClass / retentionPolicy / retentionDays / archivedAt / restoreStatus / legalHold` 已进入真实 archive metadata
  - legal hold archive 已有“不得清理”的真实执行语义

## 本轮未兑现能力力力力力
- tenant-aware plan policy
- 后台异步 lifecycle worker
- object storage hot / warm / archive bucket 分层
- 更完整 retry / alerting / observability

## 是否偏离架构
- 无偏离。
- 当前属于“实现更具体”：
  - 文档要求 archive lifecycle 必须兼顾恢复、合规与成本
  - 当前代码先在 `Local Minimal` runtime-dir snapshot 上落下最小可执行基线

## 回写决议
- `docs/架构/09-实施计划.md` 追加 `As-Built 96`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md` 追加 `As-Built 11`
- `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md` 追加 `As-Built 2`

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
  - `bin/prune-runtime-archives-local.ps1`
  - `bin/prune-runtime-archives-local.sh`
  - `bin/prune-runtime-archives-local.cmd`
  - `bin/retired-lifecycle-status.ps1`
  - `bin/retired-lifecycle-status.sh`
- 验证：
  - `cargo fmt --all --check`
  - `cargo test -p sdkwork-im-cloud-gateway --offline`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin/archive-runtime-backup-local.ps1 -Help`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin/prune-runtime-archives-local.ps1 -Help`

## 当前判断
- `CP09-4`：通过
- `Step 09`：具备进入整步 `91 / 95 / 97` 的最后前提
- `Wave C / 93`：仍需等待 `Step 09` 整步审计结论
