> Migrated from `docs/review/step-09-cp09-4-runtime-backup-archive-path-质量审计与复盘-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 09 / CP09-4 runtime backup archive path 质量审计与复盘 - 2026-04-08

## 审计范围
- `services/sdkwork-im-cloud-gateway/src/node/runtime_dir.rs`
- `services/sdkwork-im-cloud-gateway/src/node.rs`
- `services/sdkwork-im-cloud-gateway/src/main.rs`
- `services/sdkwork-im-cloud-gateway/tests/runtime_dir_backup_catalog_test.rs`
- `bin/archive-runtime-backup-local.ps1`
- `bin/archive-runtime-backup-local.sh`
- `bin/archive-runtime-backup-local.cmd`
- `bin/retired-lifecycle-status.ps1`
- `bin/retired-lifecycle-status.sh`

## 审计结论
- 本轮未发现阻塞当前增量交付的剩余缺陷。
- 当前 archive 路径符合 `CP09-4` 的最小正确方向：
  - 复用既有 runtime-dir backup snapshot 合同
  - archive 后仍保留 restore path
  - 不引入新的 snapshot 格式或旁路存储层

## 正向结果
- runtime-dir 运维链路现在首次具备：
  - inspection
  - repair
  - list
  - archive
  - preview restore
  - restore
  的连续操作面。
- archived snapshot 不再是“人工移动一个目录”，而是：
  - 有正式命令
  - 有脚本入口
  - 有 archive report
  - 有 catalog lifecycle 标记
- `preview_restore_runtime_dir(...)` 仍能消费 archived snapshot，这一点直接把 archive 和 recovery 语义连了起来。

## 本轮关键判断
- 本轮最正确的设计，不是新造一套 archive 格式，而是保持 archived snapshot 与 active backup 使用同一 restore 合同。
- 这使 archive 成为对现有恢复链路的延伸，而不是新的旁路系统。
- `operation` 与 `lifecycleStage` 分开建模，也是必要的：
  - `operation` 回答该 snapshot 原本来自 restore 还是 repair
  - `lifecycleStage` 回答该 snapshot 当前是 active 还是 archived

## 残余风险
- 当前 archive 仍是手动触发，不是 retention policy 驱动的后台任务。
- 当前没有 legal hold、tenant policy、storage class、archived_at / restore_status 等更完整生命周期字段。
- 当前只覆盖 local runtime-dir backup snapshot，不覆盖更高阶的 object storage archive 或 tenant 级归档治理。
- 当前没有 archive prune / delete path，因此还谈不上完整 lifecycle 闭环。

## 验证证据
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/archive-runtime-backup-local.ps1 -Help`

## 复盘结论
- `CP09-4` 现在终于开始有真实代码增量，而不再只停留在“已有 repair / restore，所以 archive 以后再说”。
- 这仍只是第一段 archive 路径，不足以让 `CP09-4` 通过。
- 下一轮最应该继续补的是 retention / lifecycle，而不是重新回到 `CP09-3` 堆更多观测字段。

