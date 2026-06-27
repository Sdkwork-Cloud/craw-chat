# Step 10 / CP10-4 runtime ops profile contract 质量审计与复盘 - 2026-04-08

## 审计范围
- `bin/_runtime-profile-common.ps1`
- `bin/_runtime-profile-common.sh`
- `bin/inspect-runtime-local.*`
- `bin/repair-runtime-local.*`
- `bin/list-runtime-backups-local.*`
- `bin/archive-runtime-backup-local.*`
- `bin/prune-runtime-archives-local.*`
- `bin/preview-runtime-restore-local.*`
- `bin/restore-runtime-local.*`
- `docs/部署/快速启动脚本.md`
- `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`

## 审计结论
- 本轮未发现阻塞 `CP10-4` 关闭的剩余缺陷。
- 关键问题已经从“脚本有没有恢复命令”转成“同一 profile 体系下，运维脚本是否真的遵守统一语义”。
- 当前增量已经把这个缺口修正为可测试、可文档化、可跨平台复用的 operator contract。

## 正向结果
- runtime ops 的 profile 选择已不再只存在于 `retired-lifecycle-deploy`，而是进入 inspect / repair / backup / restore 全链路。
- `standalone.split-services.development` 没有被误实现成新的独立 runtime 拓扑，而是按既有架构约束：
  - 优先读 `standalone.split-services.development` config
  - 继续复用 `standalone.split-services.development` runtime contract
- PowerShell temp-workspace 回归模型已被保留，没有因为共享 helper 引入新的脆弱耦合。
- `deployment_profile_test` 现在同时覆盖：
  - 脚本文本合同
  - PowerShell profile 解析行为
  - CMD profile 转发行为
  - Bash profile 解析行为

## 仍需关注的风险
- `standalone.split-services.development` 当前仍不是独立运行拓扑；这不是 bug，而是当前冻结架构的明确选择。
- Git Bash 在当前宿主环境中执行 standalone `--help` 会命中 `couldn't create signal pipe, Win32 error 5`，因此 shell 帮助命令不能作为唯一放行证据。
- 但这不影响 `CP10-4` 结论，因为更强的 `deployment_profile_test` 已 fresh 执行 Bash runtime ops 行为回归。

## 验证证据
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/inspect-runtime-local.ps1 -Help`
- `cmd /c bin\\inspect-runtime-local.cmd --help`

## 复盘结论
- 本轮最关键的决策是没有把 `standalone.split-services.development` 扩成新的 runtime 目录体系，而是只把 runtime ops 做成 profile-aware。
- 这样做的收益是：
  - 与 `CP10-2` 已冻结的 profile/template 合同保持一致
  - 避免把 `CP10-4` 扩散成新的部署拓扑重构
  - 直接收口“脚本语义与 profile 命名不一致”的真实断层
