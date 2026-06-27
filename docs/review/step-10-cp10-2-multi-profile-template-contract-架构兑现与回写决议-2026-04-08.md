# Step 10 / CP10-2 multi-profile template contract 架构兑现与回写决议 - 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/137-部署拓扑与容量规划设计-2026-04-06.md`
- `docs/架构/142-控制面与配置治理设计-2026-04-06.md`

## 本轮已兑现能力力力力力
- `09`
  - `Wave D / Step 10 / CP10-2` 已把多环境 profile/template 的“真实边界”从口头约定推进为：
    - 文档矩阵
    - 模板文件
    - 统一脚本参数
    - 回归测试门禁
- `137`
  - 本地交付拓扑不再只有单一 `standalone.split-services.development` 命令面，而是已经具备：
    - `standalone.split-services.development`
    - `standalone.split-services.development`
    两个真实可引用的本地部署 profile 入口
  - 其中 `standalone.split-services.development` 当前虽然仍复用 `standalone.split-services.development` 服务合同，但其 compose/template/脚本入口已经固定，不再漂移
- `142`
  - 本地 deploy-time config 已从“初始化时生成一个 env 文件”推进到“按 profile 拥有稳定模板入口”
  - `retired-lifecycle-deploy` 的 profile selector 也让 deploy-time config 与 operator action 之间的接口变得显式，而不再依赖手工改脚本或硬编码 compose 路径

## 本轮未兑现能力力力力力
- `standalone.split-services.development` 尚未演进为独立依赖拓扑或独立 smoke/recovery 闭环
- `private-saas-single-cell`、`cloud-shared-cell`、`cloud-dedicated-cell` 仍未形成真实 compose/template/script 资产
- `Step 10 / CP10-3` 的健康检查与 smoke 重复执行证据尚未闭环
- `Step 10 / CP10-4` 的 inspect / repair / restore 演练尚未按多 profile 收口

## 是否偏离架构
- 无偏离。
- 本轮采取的是“先冻结当前真实 profile 边界，再继续推进 smoke/health 闭环”的路线，与 `137` 的部署拓扑收敛、`142` 的配置模板化治理完全一致。
- 本轮没有把尚未落地的未来 profile 伪装成已完成能力，反而降低了架构与交付口径偏离的风险。

## 回写决议
- `docs/架构/09-实施计划.md` 追加 `As-Built 98`
- `docs/架构/137-部署拓扑与容量规划设计-2026-04-06.md` 追加 `As-Built 2`
- `docs/架构/142-控制面与配置治理设计-2026-04-06.md` 追加 `As-Built 6`
- 本轮不追加 `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
  - 原因：本轮新增的是 profile/template 边界与 deploy 入口合同，不是新的 HA/DR 恢复语义

## 证据
- 代码：
  - `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
  - `pnpm dev`
  - `pnpm dev`
  - `bin/_cmd-forward-powershell.cmd`
  - `deployments/scripts/bootstrap-local.ps1`
- 文档：
  - `docs/部署/多环境Profile与配置模板.md`
  - `docs/部署/快速启动脚本.md`
  - `docs/部署/README.md`
  - `README.md`
- 模板：
  - `deployments/templates/standalone.split-services.development.env.example`
  - `deployments/templates/standalone.split-services.development.env.example`
- 验证：
  - `cargo fmt --all --check`
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File pnpm dev -Help`
  - `cmd /c bin\\retired-lifecycle-deploy.cmd --help`

## 当前判断
- `CP10-2`：通过
- `Step 10`：继续执行 `CP10-3`
- `Wave D`：未闭环，继续停留在 `Step 10`
