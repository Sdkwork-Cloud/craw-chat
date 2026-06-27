# Step 10 / CP10-2 multi-profile template contract 质量审计与复盘 - 2026-04-08

## 审计范围
- `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
- `pnpm dev`
- `pnpm dev`
- `bin/_cmd-forward-powershell.cmd`
- `deployments/scripts/bootstrap-local.ps1`
- `deployments/templates/standalone.split-services.development.env.example`
- `deployments/templates/standalone.split-services.development.env.example`
- `docs/部署/多环境Profile与配置模板.md`
- `docs/部署/快速启动脚本.md`
- `docs/部署/README.md`
- `README.md`

## 审计结论
- 本轮未发现阻塞 `CP10-2` 通过的剩余缺陷。
- 当前交付已经把“profile 名称存在”推进到“profile 边界、模板入口、脚本参数、文档入口与测试门禁一致”。
- `retired-lifecycle-deploy` 的 profile 选择语义已经在 PowerShell / CMD / Bash 三条入口上收口到同一份 operator contract。

## 正向结果
- `standalone.split-services.development` 与 `standalone.split-services.development` 的当前职责边界已被文档明确冻结。
- `standalone.split-services.development` 不再只是 README 里的名字，而是具备：
  - `deployments/docker-compose/standalone.split-services.development.yml`
  - `deployments/templates/standalone.split-services.development.env.example`
  - `retired-lifecycle-deploy` 统一入口可选 profile
- `deployment_profile_test` 现在同时覆盖：
  - profile/template 文档合同
  - retired-lifecycle-deploy 参数合同
  - PowerShell 转发合同
  - CMD 兼容层收口
  - 快启文档中的 profile selector 说明
- fresh verification 过程中暴露出的旧硬编码断言也已被修复，避免“实现已升级但资产测试还锁死旧路径”的假回归。

## 剩余风险
- `standalone.split-services.development` 当前仍是兼容基线 profile，不是独立依赖拓扑，也不是独立 smoke/恢复闭环。
- 本轮没有在沙箱内执行真实 Docker `up/down`，因此验证范围是：
  - 离线测试
  - 帮助输出
  - profile 参数转发合同
  而不是容器级运行时 smoke。
- `private-saas-single-cell`、`cloud-shared-cell`、`cloud-dedicated-cell` 仍只有规划边界，没有 compose/template/script 落地。

## 验证证据
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline test_quick_start_doc_freezes_full_local_command_surface -- --exact`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test`
- `powershell -NoProfile -ExecutionPolicy Bypass -File pnpm dev -Help`
- `cmd /c bin\\retired-lifecycle-deploy.cmd --help`

## 复盘结论
- 本轮最关键的决策仍然是“不夸大未来 profile 的完成度”，而是只冻结当前仓库里真实存在、可验证、可回归的 profile 资产。
- 先把 `standalone.split-services.development/standalone.split-services.development`、模板入口和 `retired-lifecycle-deploy` 参数合同锁定，再进入 `CP10-3` 做 smoke/health 复用，能显著降低后续跨平台脚本继续分叉的风险。
- 因为本轮在 fresh verification 里主动吸收了旧断言和旧文档口径的偏差，`CP10-2` 现在的证据质量高于“代码实现过了，但文档没有追平”的半闭环状态。
