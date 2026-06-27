# Step 10 / CP10-3 smoke base-url contract 质量审计与复盘 - 2026-04-08

## 审计范围
- `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
- `pnpm dev`
- `pnpm dev`
- `bin/_cmd-forward-powershell.cmd`
- `deployments/scripts/bootstrap-local.ps1`
- `docs/部署/快速启动脚本.md`
- `README.md`

## 审计结论
- 本轮未发现阻塞当前增量通过的剩余缺陷。
- `retired-lifecycle-deploy` 现在已经不再把 smoke 固定死在默认地址，而是显式公开了可重复执行的 smoke 目标参数。
- 这使 `CP10-3` 从“底层 smoke 脚本能改参数”推进到了“标准交付入口能改参数”。

## 正向结果
- PowerShell / CMD / Bash 三条 `retired-lifecycle-deploy` 入口现在共享同一份 smoke base-url 语义：
  - `-SmokeBaseUrl <url>`
  - `--smoke-base-url <url>`
- bootstrap 层只在显式传入时覆写 smoke 目标，不会破坏现有默认链路。
- 快启文档与根 README 已把新的 smoke 参数纳入标准示例，避免 operator 继续回退到底层 `tools/smoke/*` 直接执行。
- 测试现在同时覆盖：
  - 脚本合同
  - PowerShell 转发
  - CMD 兼容层收口
  - 快启文档参数口径

## 剩余风险
- 本轮只是 `CP10-3` 的第一增量，不代表整个 smoke/health 交付闭环已经完成。
- 当前验证仍以离线测试和帮助输出为主，没有在沙箱里执行真实 Docker 启动后的非默认地址 smoke 演练。
- Bash 侧虽然已经具备 `--smoke-base-url` 合同，但当前环境仍不适合直接做完整容器 smoke 执行证明。

## 验证证据
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test`
- `powershell -NoProfile -ExecutionPolicy Bypass -File pnpm dev -Help`
- `cmd /c bin\\retired-lifecycle-deploy.cmd --help`

## 复盘结论
- 本轮最关键的决策是没有重复造一套 `smoke-local.*` 包装脚本，而是直接把既有 smoke 的目标地址能力接入统一 `retired-lifecycle-deploy` 入口。
- 这样做的收益是：
  - operator 入口不再分裂
  - 未来无论 `standalone.split-services.development` 是否迁移到不同端口或代理入口，都可以继续复用同一套 smoke 合同
  - `CP10-3` 后续工作可以围绕现有标准入口继续加证据，而不是再治理一轮命令面漂移
