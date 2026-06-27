> Migrated from `docs/review/step-10-cp10-3-public-smoke-signed-bearer-contract-架构兑现与回写决议-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 10 / CP10-3 public smoke signed bearer contract 架构兑现与回写决议 - 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/137-部署拓扑与容量规划设计-2026-04-06.md`
- `docs/架构/142-控制面与配置治理设计-2026-04-06.md`

## 本轮已兑现能力力力力力
- `09`
  - `Wave D / Step 10 / CP10-3` 已从“入口可传 smoke base-url”推进到“public smoke 认证闭环真实可执行”
- `137`
  - 本地 Docker/public 链路现在具备固定 dev secret 与 signed bearer smoke 合同
  - 这意味着部署拓扑中的 public smoke 不再依赖已失效的 unsigned demo bearer
- `142`
  - deploy-time/operator config 已新增 public bearer secret 真源与 smoke secret 解析顺序
  - smoke auth 现在从隐式脚本常量，提升为显式配置合同

## 本轮未兑现能力力力力力
- `CP10-4` 的 inspect / repair / restore 多 profile 交付闭环尚未开始
- `Step 10` 整步尚未通过
- `Wave D` 尚未达到 `93` 总验收条件

## 是否偏离架构
- 无偏离。
- 本轮沿用既有 `retired-lifecycle-deploy` / `local_stack_smoke.*` / `standalone.split-services.development.yml` 交付面收敛认证合同，没有引入新的旁路脚本，也没有制造新的平台差异。

## 回写决议
- `docs/架构/09-实施计划.md` 追加 `As-Built 100`
- `docs/架构/137-部署拓扑与容量规划设计-2026-04-06.md` 追加 `As-Built 4`
- `docs/架构/142-控制面与配置治理设计-2026-04-06.md` 追加 `As-Built 8`
- 本轮不追加 `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
  - 原因：本轮新增的是 public smoke auth contract，而非新的恢复/灾备/HA 语义

## 证据
- 代码：
  - `(removed compose file)`
  - `tools/smoke/local_stack_smoke.ps1`
  - `tools/smoke/local_stack_smoke.sh`
  - `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
- 文档：
  - `docs/部署/本地最小安装与运行.md`
  - `docs/部署/快速启动脚本.md`
  - `docs/部署/README.md`
  - `README.md`
- 验证：
  - `cargo fmt --all --check`
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test`
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test public_auth_e2e_test`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File pnpm dev -Help`
  - `cmd /c bin\\retired-lifecycle-deploy.cmd --help`

## 当前判断
- 当前增量：通过
- `CP10-3`：通过，进入 `CP10-4`
- `Step 10`：继续执行，尚未整步闭环
- `Wave D`：未闭环

