> Migrated from `docs/review/step-10-cp10-1-bin-command-surface-freeze-架构兑现与回写决议-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 10 / CP10-1 bin command surface freeze 架构兑现与回写决议 - 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/137-部署拓扑与容量规划设计-2026-04-06.md`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
- `docs/架构/142-控制面与配置治理设计-2026-04-06.md`

## 本轮已兑现能力力力力力
- `09`
  - `Step 10 / CP10-1` 现在已经把 `standalone.split-services.development` 命令面冻结为统一入口：
    - `install`
    - `init`
    - `start`
    - `status`
    - `restart`
    - `stop`
    - runtime inspect / repair / backup / archive / restore loop
- `137`
  - 部署入口现在不再只是“存在一些脚本”，而是有了跨平台命令矩阵和明确执行顺序
- `138`
  - runtime backup / archive / prune / restore 入口现在正式纳入标准运维命令面，不再是散落的附加能力
- `142`
  - `init-config-local` 与快启命令矩阵已把配置初始化、运行时入口和运维入口固定到统一的控制面/配置治理话语体系

## 本轮未兑现能力力力力力
- 多环境 profile/template 仍未完成真实收敛
- Docker / compose smoke 的跨 profile 重复执行仍待后续 `CP10-3`
- repair / restore / inspect 脚本虽然已进入命令矩阵，但完整标准运维演练仍待 `CP10-4`

## 是否偏离架构
- 无偏离。
- 本轮属于把既有脚本与运行时能力拉回到文档和测试共同约束的统一命令面，而不是引入新的平台语义分叉。

## 回写决议
- `docs/架构/09-实施计划.md` 追加 `As-Built 97`
- `docs/架构/137-部署拓扑与容量规划设计-2026-04-06.md` 追加 `As-Built 1`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md` 追加 `As-Built 12`
- `docs/架构/142-控制面与配置治理设计-2026-04-06.md` 追加 `As-Built 5`

## 证据
- 代码：
  - `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
  - `bin/retired-lifecycle-status.sh`
- 文档：
  - `docs/部署/快速启动脚本.md`
- 验证：
  - `cargo fmt --all --check`
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test`
  - `powershell -NoProfile -ExecutionPolicy Bypass -File bin/retired-lifecycle-status.ps1 -Help`
  - `cmd /c bin\\retired-lifecycle-status.cmd --help`

## 当前判断
- `CP10-1`：通过
- `Step 10`：仍未闭环，需要继续进入 `CP10-2`
- `Wave D`：继续执行 `Step 10`

