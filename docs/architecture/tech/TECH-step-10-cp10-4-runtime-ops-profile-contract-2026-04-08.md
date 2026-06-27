> Migrated from `docs/review/step-10-cp10-4-runtime-ops-profile-contract-执行卡-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 10 / CP10-4 runtime ops profile contract 执行卡 - 2026-04-08

## 当前上下文
- 当前波次：`Wave D`
- 当前 step：`Step 10`
- 当前子任务：`CP10-4`
- 前置状态：
  - `CP10-1` 已冻结统一命令面
  - `CP10-2` 已冻结 `standalone.split-services.development` / `standalone.split-services.development` profile 与配置模板合同
  - `CP10-3` 已完成 Docker/public smoke 的 signed bearer 闭环
  - 当前真实缺口收敛到 runtime ops：
    - `inspect / repair / list / archive / prune / preview / restore` 脚本仍锁死 `.runtime/standalone.split-services.development/config/standalone.split-services.development.env`
    - PowerShell 运行时脚本没有 `-ProfileName`
    - Bash 运行时脚本没有统一 `--profile` 运维合同

## 本轮为什么做这个增量
- `docs/step/10-部署脚本与多环境发布治理.md` 明确要求 `CP10-4` 证明“恢复、修复、检查脚本已经纳入标准运维闭环”。
- `CP10-2` 已经把 `standalone.split-services.development` 冻结为真实 profile 名称，如果 runtime ops 仍只认 `standalone.split-services.development`，则 profile 体系仍然是半成品。
- 因此本轮最优动作是补齐 runtime ops profile-aware 合同，而不是引入新的 runtime 拓扑。

## 本轮实际完成

### 1. runtime ops 已具备统一 profile 选择合同
- PowerShell：
  - `inspect-runtime-local.ps1`
  - `repair-runtime-local.ps1`
  - `list-runtime-backups-local.ps1`
  - `archive-runtime-backup-local.ps1`
  - `prune-runtime-archives-local.ps1`
  - `preview-runtime-restore-local.ps1`
  - `restore-runtime-local.ps1`
  - 统一新增 `-ProfileName <standalone.split-services.development|standalone.split-services.development>`
- Bash：
  - `inspect-runtime-local.sh`
  - `repair-runtime-local.sh`
  - `list-runtime-backups-local.sh`
  - `archive-runtime-backup-local.sh`
  - `prune-runtime-archives-local.sh`
  - `preview-runtime-restore-local.sh`
  - `restore-runtime-local.sh`
  - 统一新增 `--profile <standalone.split-services.development|standalone.split-services.development>`
- CMD：
  - 继续通过 `_cmd-forward-powershell.cmd` 把 `--profile` 归一化为 `-ProfileName`

### 2. `standalone.split-services.development` 运维配置解析已进入显式合同
- 新增共享 helper：
  - `bin/_runtime-profile-common.ps1`
  - `bin/_runtime-profile-common.sh`
- 统一解析顺序：
  1. 显式 `RuntimeDir`
  2. `standalone.split-services.development` 时优先读取 `.runtime/standalone.split-services.development/config/standalone.split-services.development.env`
  3. 若未落地，则回退 `.runtime/standalone.split-services.development/config/standalone.split-services.development.env`
  4. 默认运行目录仍回退到 `.runtime/standalone.split-services.development`
- 该回退与 `docs/部署/多环境Profile与配置模板.md` 当前冻结的“`standalone.split-services.development` 仍复用 `standalone.split-services.development` 运行合同”一致

### 3. PowerShell runtime ops 保持单文件可执行回归模型
- 共享 helper 存在时优先复用
- 若脚本被复制到临时工作区且 helper 不在旁边，PowerShell 脚本会回退到内嵌 profile 解析实现
- 这保持了现有 `deployment_profile_test.rs` 的 temp-workspace 回归模型，不把脚本变成必须成套复制的脆弱合同

### 4. 文档与回归门禁已追平
- `docs/部署/快速启动脚本.md`
  - 新增 runtime ops profile 选择说明
  - 明确 `standalone.split-services.development` 的 config-first / runtime-contract fallback 口径
- `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
  - `test_runtime_operation_scripts_expose_profile_selection_contract`
  - `test_inspect_runtime_local_ps1_uses_local_default_profile_config_when_requested`
  - `test_inspect_runtime_local_cmd_supports_profile_switch`
  - `test_repair_runtime_local_sh_uses_local_default_profile_config_when_requested`

## TDD / Red-Green 证据

### Red
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_runtime_operation_scripts_expose_profile_selection_contract -- --exact`
  - 初始失败：`inspect-runtime-local.ps1 must expose a profile selector`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_inspect_runtime_local_ps1_uses_local_default_profile_config_when_requested -- --exact`
  - 初始失败：实际仍解析到 `runtime-from-local-minimal`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_inspect_runtime_local_cmd_supports_profile_switch -- --exact`
  - 初始失败：CMD 转发后的 PowerShell 仍落到 `.runtime/standalone.split-services.development`

### Green
- 补齐 profile selector、config 解析顺序、PowerShell 单文件 fallback 与文档合同后，上述 red 用例全部保持通过
- `deployment_profile_test` 全量 fresh 通过，证明本轮修复没有破坏既有 deployment/runtime scripts 合同

## Fresh 验证
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/inspect-runtime-local.ps1 -Help`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/repair-runtime-local.ps1 -Help`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/list-runtime-backups-local.ps1 -Help`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/archive-runtime-backup-local.ps1 -Help`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/prune-runtime-archives-local.ps1 -Help`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/preview-runtime-restore-local.ps1 -Help`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/restore-runtime-local.ps1 -Help`
- `cmd /c bin\\inspect-runtime-local.cmd --help`

## 当前判断
- `CP10-4`：闭环
- 已兑现：
  - runtime ops 已纳入统一 profile-aware 运维合同
  - `standalone.split-services.development` 与 `standalone.split-services.development` 的运维入口不再分叉
  - PowerShell / CMD / Bash 的 runtime ops 语义已对齐到同一组参数与 fallback 规则
- 当前仍未兑现：
  - `Step 10` 的整步审计与架构回写
  - `Step 11` 的压测 / 故障演练 / 灾备验证
  - `Wave D` 总验收

## 下一轮继续做什么
1. 提升到 `Step 10` 整步审计
2. 完成 `Step 10` 的 review 汇总与架构回写
3. 自动进入 `Wave D / Step 11`

