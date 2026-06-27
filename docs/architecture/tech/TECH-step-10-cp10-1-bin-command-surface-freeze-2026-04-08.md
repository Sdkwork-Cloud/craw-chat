> Migrated from `docs/review/step-10-cp10-1-bin-command-surface-freeze-执行卡-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 10 / CP10-1 bin command surface freeze 执行卡 - 2026-04-08

## 当前上下文
- 当前波次：`Wave D`
- 当前 step：`Step 10`
- 当前子任务：`CP10-1`
- 前置状态：
  - `Wave C / 93` 已完成
  - `Step 10` 已进入执行态，但此前还没有真实的 `CP10-1` 交付闭环
  - 仓库中虽然已经存在 `install/start/stop/restart/status/init` 与运行时维护脚本，但仍有两个未冻结缺口：
    - `retired-lifecycle-status.sh` 的 help 语义没有与 `retired-lifecycle-status.ps1` 对齐
    - `docs/部署/快速启动脚本.md` 仍停留在 install/deploy/start 的早期入口，未冻结完整本地生命周期和 runtime ops loop

## 本轮为什么做这个子任务
- `docs/step/10-部署脚本与多环境发布治理.md` 对 `CP10-1` 的要求是：
  - `bin/` 脚本体系按统一命名和统一语义治理
  - install / init / start / status / restart / stop 命令面先冻结，再继续 profile 和 smoke 闭环
- `docs/step/95-架构能力闭环验收标准.md` 对 `Step 10` 的要求是：
  - 运维人员不需要阅读源码，也能通过标准命令入口完成基本部署、状态检查与恢复动作
- 因此本轮最优决策不是扩展新 profile，而是先把跨平台命令语义和操作文档冻结成可回归契约。

## 本轮实际完成

### 1. `retired-lifecycle-status` 的跨平台 help 合同已冻结
- `bin/retired-lifecycle-status.sh`
  - help 首行语义已与 `bin/retired-lifecycle-status.ps1` 对齐
  - 现在明确声明：
    - pid / config / stdout / stderr / health
    - inspect / repair / list / archive / prune / preview / restore 后续动作

### 2. `CP10-1` 已新增可回归测试
- `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
  - 新增 `test_status_local_help_texts_share_runtime_ops_contract_across_platform_scripts`
  - 新增 `test_quick_start_doc_freezes_full_local_command_surface`
- 这两条测试把“脚本 help 语义统一”和“文档命令面冻结”变成了真实回归门禁，而不再只是人工约定。

### 3. 快启文档已升级为完整命令矩阵
- `docs/部署/快速启动脚本.md`
  - 现在显式冻结：
    - `retired-lifecycle-install`
    - `init-config-local`
    - `retired-lifecycle-start`
    - `retired-lifecycle-status`
    - `retired-lifecycle-restart`
    - `retired-lifecycle-stop`
    - `retired-lifecycle-deploy`
    - `open-chat-test`
    - `inspect-runtime-local`
    - `repair-runtime-local`
    - `list-runtime-backups-local`
    - `archive-runtime-backup-local`
    - `prune-runtime-archives-local`
    - `preview-runtime-restore-local`
    - `restore-runtime-local`
  - 现在还补齐了：
    - PowerShell / CMD / Bash 对应入口
    - 推荐执行顺序
    - 常用参数
    - `.runtime` 运行产物
    - runtime 运维闭环入口

## TDD 证据

### Red
- `cargo test -p sdkwork-im-cloud-gateway --offline test_status_local_help_texts_share_runtime_ops_contract_across_platform_scripts`
  - 失败点与预期一致：
    - `retired-lifecycle-status.sh` 尚未声明与 PowerShell 相同的 runtime ops contract
- `cargo test -p sdkwork-im-cloud-gateway --offline test_quick_start_doc_freezes_full_local_command_surface`
  - 失败点与预期一致：
    - `docs/部署/快速启动脚本.md` 缺少 `init-config-local` 等完整命令面

### Green
- `cargo test -p sdkwork-im-cloud-gateway --offline test_status_local_help_texts_share_runtime_ops_contract_across_platform_scripts`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_quick_start_doc_freezes_full_local_command_surface`

## 回归验证
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/retired-lifecycle-status.ps1 -Help`
- `cmd /c bin\\retired-lifecycle-status.cmd --help`

## 结论
- `CP10-1` 现在已经不再只是“脚本文件存在”，而是具备：
  - help 语义一致
  - 文档命令面冻结
  - 跨平台入口清单明确
  - 回归测试可执行
- `CP10-1`：通过。

## 下一轮继续做什么
1. 不停留在 `CP10-1`
2. 立刻转入 `CP10-2`，收敛多环境 profile 与配置模板的真实边界
3. `Step 10` 整步仍未闭环，后续还需要继续完成：
   - `CP10-2`
   - `CP10-3`
   - `CP10-4`

