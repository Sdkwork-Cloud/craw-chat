> Migrated from `docs/review/step-10-cp10-3-public-smoke-signed-bearer-contract-质量审计与复盘-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 10 / CP10-3 public smoke signed bearer contract 质量审计与复盘 - 2026-04-08

## 审计范围
- `(removed compose file)`
- `tools/smoke/local_stack_smoke.ps1`
- `tools/smoke/local_stack_smoke.sh`
- `docs/部署/本地最小安装与运行.md`
- `docs/部署/快速启动脚本.md`
- `docs/部署/README.md`
- `README.md`
- `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`

## 审计结论
- 本轮未发现阻塞 `CP10-3` 通过的剩余缺陷。
- 之前 public smoke 的核心问题不是“缺少更多入口参数”，而是认证合同已经升级到 HS256，但 Docker 与 smoke 仍停留在 unsigned demo bearer。
- 当前增量已经把这个错位修正为可验证、可重复、可文档化的 operator contract。

## 正向结果
- Docker `standalone.split-services.development` profile 现在具备稳定 public bearer dev secret，不再要求 operator 自行推断 secret 来源。
- PowerShell / Bash smoke 脚本共享同一条 secret 解析顺序：
  - 显式 bearer
  - 显式 secret
  - 本地 config
  - Docker dev secret
- smoke 回归门禁已经从“检查脚本文本是否存在”升级为：
  - 文件合同
  - 文档合同
  - PowerShell 实际执行
  - Bash 实际执行
- `public_auth_e2e_test` fresh 通过，说明这次 smoke 修复没有破坏 public app 既有权限/签名语义。

## 仍需关注的风险
- 本轮关闭的是 `CP10-3` 的 smoke/auth 问题，不代表 `Step 10` 整步完成。
- 当前宿主环境下，单独直接调用系统 bash 做 `-n/--help` 会命中宿主 shim 的 `E_ACCESSDENIED`，因此这类 standalone shell 验证不应当作为唯一证据。
- 但该限制不影响当前判断，因为更强的 `deployment_profile_test` 已经成功执行 `local_stack_smoke.sh` 对 `build_public_app()` 的真实 smoke。

## 验证证据
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test public_auth_e2e_test`
- `powershell -NoProfile -ExecutionPolicy Bypass -File pnpm dev -Help`
- `cmd /c bin\\retired-lifecycle-deploy.cmd --help`
- PowerShell parser 成功解析 `tools/smoke/local_stack_smoke.ps1`

## 复盘结论
- 本轮最关键的决策是没有去全仓替换所有 unsigned bearer，而是把修复范围严格锁在 deployment/public smoke 链路。
- 这样做的收益是：
  - 避免误伤默认 app 路径仍允许的 jwt-like 本地演示夹具
  - 直接补上 `build_public_app()` 与 Docker smoke 之间的真实断层
  - 让 `CP10-3` 的 closing evidence 聚焦在“真实部署 smoke 可重复执行”，而不是扩散成新一轮 auth 全仓重构

