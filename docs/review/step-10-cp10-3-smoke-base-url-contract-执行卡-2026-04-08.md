# Step 10 / CP10-3 smoke base-url contract 执行卡 - 2026-04-08

## 当前上下文
- 当前波次：`Wave D`
- 当前 step：`Step 10`
- 当前子任务：`CP10-3`
- 当前增量：`repeatable smoke base-url contract`
- 前置状态：
  - `CP10-1` 已冻结统一命令面
  - `CP10-2` 已冻结多环境 profile/template 边界并通过 fresh verification
  - 但 `CP10-3` 在本轮之前仍有一个真实缺口：
    - `tools/smoke/local_stack_smoke.*` 已具备 `BaseUrl` / `--base-url`
    - 统一 `retired-lifecycle-deploy` 入口却没有把这个能力暴露给 operator
    - 因而“可重复执行的 smoke”仍停留在底层脚本级，而不是标准交付入口

## 本轮为什么做这个增量
- `docs/step/10-部署脚本与多环境发布治理.md` 对 `CP10-3` 的要求是：
  - 部署后健康检查与 smoke 验证可重复执行
  - 任何平台脚本都必须通过同一 smoke 口径验证
- `docs/step/95-架构能力闭环验收标准.md` 对 `Step 10` 的伪完成警告是：
  - 只有脚本文件，没有真实 smoke 验证
- 因此本轮最优动作不是再造新 smoke 脚本，而是把现有 smoke 的 `base-url` 能力上抬到统一 `retired-lifecycle-deploy` 入口，形成跨平台 operator contract。

## 本轮实际完成

### 1. `retired-lifecycle-deploy` 已公开可重复 smoke 的目标地址合同
- `pnpm dev`
  - 新增 `-SmokeBaseUrl <url>`
  - `-Help` 现已公开该参数
- `pnpm dev`
  - 新增 `--smoke-base-url <url>`
  - help 文案同步公开该参数
- `bin/_cmd-forward-powershell.cmd`
  - 已把 `/smokeBaseUrl`、`/smokebaseurl`、`--smoke-base-url`、`--smokeBaseUrl`
    收口为 `-SmokeBaseUrl`

### 2. Docker bootstrap 已能把 smoke 目标显式转发到真实 smoke 脚本
- `deployments/scripts/bootstrap-local.ps1`
  - 新增 `-SmokeBaseUrl`
  - 未显式传入时仍走默认 smoke 行为
  - 显式传入时会把目标地址转发给 `tools/smoke/local_stack_smoke.ps1 -BaseUrl <url>`

### 3. 文档已追平新的 smoke 入口
- `docs/部署/快速启动脚本.md`
  - 已在 `retired-lifecycle-deploy` 条目、常用参数和命令示例中公开：
    - `-SmokeBaseUrl <url>`
    - `--smoke-base-url <url>`
- `README.md`
  - 根文档 Docker 入口说明已升级为统一 `retired-lifecycle-deploy.*` 的 smoke-base-url 示例

### 4. 回归门禁已补齐
- `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
  - 新增：
    - `test_deploy_local_scripts_expose_repeatable_smoke_base_url_contract`
    - `test_deploy_local_ps1_forwards_smoke_base_url_to_bootstrap_script`
    - `test_deploy_local_cmd_normalizes_smoke_base_url_switch`
  - 已扩展 `test_quick_start_doc_freezes_full_local_command_surface`
    - 把 `--smoke-base-url <url>` / `-SmokeBaseUrl <url>` 纳入快启文档合同

## TDD 证据

### Red
- `cargo test -p sdkwork-im-cloud-gateway --offline test_deploy_local_scripts_expose_repeatable_smoke_base_url_contract -- --exact`
  - 初始失败，证明 `retired-lifecycle-deploy.ps1` 尚未公开 smoke base-url override

### Green
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test`
  - 现已通过，说明脚本、文档与参数兼容层合同一致

## Fresh 验证
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test`
- `powershell -NoProfile -ExecutionPolicy Bypass -File pnpm dev -Help`
- `cmd /c bin\\retired-lifecycle-deploy.cmd --help`

## 当前判断
- 这是 `CP10-3` 的第一个真实代码增量，不是整个 `CP10-3` 的最终闭环。
- 已经兑现：
  - 统一入口可重复指定 smoke 目标
  - PowerShell / CMD / Bash 的 smoke 参数口径一致
  - 快启与 README 已不再落后于脚本实现
- 仍未兑现：
  - 更完整的健康检查 / smoke 演练证据
  - 真实容器链路的 repeatable smoke 交付闭环
  - `CP10-3` 整体通过判断

## 下一轮继续做什么
1. 继续停留在 `CP10-3`
2. 在新的 `SmokeBaseUrl` 统一入口基础上，继续补齐部署后健康检查与 smoke 演练证据
3. `Step 10` 仍未闭环，后续还需要继续完成：
   - `CP10-3` 剩余部分
   - `CP10-4`
