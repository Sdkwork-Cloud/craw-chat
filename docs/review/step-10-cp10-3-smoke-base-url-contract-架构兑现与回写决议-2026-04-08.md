# Step 10 / CP10-3 smoke base-url contract 架构兑现与回写决议 - 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/137-部署拓扑与容量规划设计-2026-04-06.md`
- `docs/架构/142-控制面与配置治理设计-2026-04-06.md`

## 本轮已兑现能力力力力力
- `09`
  - `Wave D / Step 10 / CP10-3` 已开始把“可重复执行的 smoke”从底层工具脚本提升到标准 `retired-lifecycle-deploy` 交付入口
- `137`
  - 本地 Docker 交付链路现在不再把 smoke 锁死到默认地址，而是允许 operator 显式指定 smoke 目标
  - 这让本地部署拓扑可以在不改脚本内部实现的前提下，适配非默认端口、代理入口或后续 profile 变化
- `142`
  - deploy-time/operator config 现在新增了显式 `SmokeBaseUrl` 参数层
  - 这使 smoke 目标地址不再是底层脚本里的隐式硬编码，而是进入统一的 operator contract

## 本轮未兑现能力力力力力
- 真实容器链路上的 repeatable smoke 演练证据仍未完整闭环
- health/readiness 更完整的部署后验证口径仍待继续补齐
- `CP10-3` 整体尚未通过
- `CP10-4` 尚未开始闭环

## 是否偏离架构
- 无偏离。
- 本轮是在既有 `retired-lifecycle-deploy` 统一入口上继续增强 smoke 合同，而不是引入新的旁路脚本或新的平台分叉。
- 这与 `137` 的部署入口统一化、`142` 的 deploy-time/operator config 显式化是同向收敛。

## 回写决议
- `docs/架构/09-实施计划.md` 追加 `As-Built 99`
- `docs/架构/137-部署拓扑与容量规划设计-2026-04-06.md` 追加 `As-Built 3`
- `docs/架构/142-控制面与配置治理设计-2026-04-06.md` 追加 `As-Built 7`
- 本轮不追加 `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
  - 原因：本轮新增的是 smoke 目标地址合同，而不是新的恢复、灾备或 HA 语义

## 证据
- 代码：
  - `pnpm dev`
  - `pnpm dev`
  - `bin/_cmd-forward-powershell.cmd`
  - `deployments/scripts/bootstrap-local.ps1`
  - `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
- 文档：
  - `docs/部署/快速启动脚本.md`
  - `README.md`
- 验证：
  - `cargo fmt --all --check`
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File pnpm dev -Help`
  - `cmd /c bin\\retired-lifecycle-deploy.cmd --help`

## 当前判断
- 当前增量：通过
- `CP10-3`：继续执行，尚未整体闭环
- `Step 10`：继续停留在 `CP10-3`
- `Wave D`：未闭环
