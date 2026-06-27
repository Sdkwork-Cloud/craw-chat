# Step 09 / CP09-4 runtime backup archive retention lifecycle 质量审计与复盘 - 2026-04-08

## 审计范围
- `services/sdkwork-im-cloud-gateway/src/node/runtime_dir.rs`
- `services/sdkwork-im-cloud-gateway/src/node.rs`
- `services/sdkwork-im-cloud-gateway/src/main.rs`
- `services/sdkwork-im-cloud-gateway/tests/runtime_dir_backup_catalog_test.rs`
- `bin/archive-runtime-backup-local.ps1`
- `bin/archive-runtime-backup-local.sh`
- `bin/prune-runtime-archives-local.ps1`
- `bin/prune-runtime-archives-local.sh`
- `bin/prune-runtime-archives-local.cmd`
- `bin/retired-lifecycle-status.ps1`
- `bin/retired-lifecycle-status.sh`
- `bin/_cmd-forward-powershell.cmd`

## 审计结论
- 本轮未发现阻塞 `CP09-4` 交付的剩余缺陷。
- archive lifecycle 现在已经从“只有 archive path”推进到“有 metadata / policy / hold / prune 的最小治理基线”。
- `CP09-4` 已满足 `Step 09` 所要求的 `backup / restore / repair / archive` 代码与脚本支撑。

## 正向结果
- archived snapshot 现在有结构化 lifecycle metadata，而不是只靠目录名前缀表达语义。
- prune 行为受 retention 与 legal hold 共同约束，不会把归档路径退化成“随便删目录”。
- archive 后仍保持 restore path；因此 lifecycle 治理没有破坏恢复合同。
- CLI、PowerShell、CMD 导航都已覆盖 archive lifecycle 动作。

## 剩余风险
- 当前 lifecycle task 仍是手动触发，不是后台 worker。
- 当前仍未覆盖 tenant-aware plan policy、object storage bucket 分层与更完整 retry/alerting。
- 当前 Windows 沙箱无法执行 `bash ... --help`，报错为 `Bash/Service/CreateInstance/E_ACCESSDENIED`；这属于环境限制，不构成 `CP09-4` 的阻塞缺陷。

## 验证证据
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test runtime_dir_backup_catalog_test`
- `cargo test -p sdkwork-im-cloud-gateway --offline`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/archive-runtime-backup-local.ps1 -Help`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/prune-runtime-archives-local.ps1 -Help`

## 复盘结论
- `CP09-4` 能闭环的关键，不是扩出更大的归档系统，而是把已有 runtime-dir seam 做到“可归档、可保留、可冻结、可清理、仍可恢复”。
- 这条路径已经足以支撑 `Step 09` 的整步审计。
