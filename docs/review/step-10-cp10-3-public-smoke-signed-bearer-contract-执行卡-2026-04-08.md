# Step 10 / CP10-3 public smoke signed bearer contract 执行卡- 2026-04-08

## 当前上下文
- 当前波次：`Wave D`
- 当前 step：`Step 10`
- 当前子任务：`CP10-3`
- 当前增量：`public smoke signed bearer contract`
- 前置状态：
  - `CP10-1` 已冻结统一命令
  - `CP10-2` 已冻结多环境 profile/template 边界
  - `CP10-3` 第一增量已经`SmokeBaseUrl` / `--smoke-base-url` 提升到统一 `retired-lifecycle-deploy` 入口
  - Docker/public smoke 仍存在真实缺口：
    - `build_public_app()` 强制要求 HS256 bearer
    - `(removed compose file)` 未注`sdkwork_im_PUBLIC_BEARER_HS256_SECRET`
    - `tools/smoke/local_stack_smoke.*` 仍内容`alg=none` demo bearer
    - `docs/部署/本地最小安装与运行.md` 仍错误宣称“当前不会做签名校验证

## 本轮为什么做这个增量
- `docs/step/10-部署脚本与多环境发布治理.md` 要求 `CP10-3` 证明“部署后健康检查与 smoke 验证可重复执行”
- `docs/step/95-架构能力闭环验收标准.md` 明确把“只有脚本文件，没有真实 smoke 验证”视为伪完成
- 如果 public 路径已经切到 HS256 验签，Docker smoke 还沿unsigned bearer，则当前 smoke 不是可重复执行，而是伪成功合同
- 因此本轮最优动作是补齐 public smoke 的签名认证闭环，而不是继续在入口层打补丁

## 本轮实际完成

### 1. Docker/public smoke 已具备真实认证真实
- `(removed compose file)`
  - 新增 `sdkwork_im_PUBLIC_BEARER_HS256_SECRET: local-minimal-public-dev-secret`
  - Docker `standalone.split-services.development` profile 现在自带可重复执行的 public bearer dev secret

### 2. smoke 脚本已从 unsigned demo bearer 切换HS256 合同
- `tools/smoke/local_stack_smoke.ps1`
  - 新增 `-PublicBearerSecret` / `-BearerToken`
  - 优先级：显式 bearer -> 显式 secret -> 本地 config -> Docker dev secret
  - 通过内置 HMAC-SHA256 生成 signed bearer
- `tools/smoke/local_stack_smoke.sh`
  - 新增 `--public-bearer-secret` / `--bearer-token`
  - 同步 secret 解析优先
  - 使用 `openssl` 生成 HS256 bearer

### 3. 部署文档已追public auth 合同
- `docs/部署/本地最小安装与运行.md`
  - 删除 unsigned demo bearer 与“当前不会做签名校验”的错误表述
  - 新增 `sdkwork_im_PUBLIC_BEARER_HS256_SECRET`、Docker dev secret smoke secret 解析顺序说明
  - 新增手工生成 `DEMO_BEARER` / `OPS_AUDIT_BEARER` HS256 示例
- `docs/部署/快速启动脚md`
  - 新增 `retired-lifecycle-deploy` smoke bearer/secret 口径说明
- `docs/部署/README.md`
  - 标注本地最小安装文档已包含 HS256 public bearer 合同
- `README.md`
  - 根文档Docker 入口新增 signed bearer / dev secret 说明

### 4. 回归门禁已从“文件存在”提升到“脚本可执行卡
- `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
  - 新增文件合同测试
    - `test_local_minimal_compose_injects_public_bearer_secret_for_public_smoke_contract`
    - `test_local_stack_smoke_scripts_require_signed_public_bearer_contract`
    - `test_local_minimal_install_doc_describes_signed_public_bearer_contract`
  - 新增真实脚本执行回归
    - `test_local_stack_smoke_ps1_executes_against_public_app_with_signed_bearer`
    - `test_local_stack_smoke_sh_executes_against_public_app_with_signed_bearer`
  - 同步修正旧合同：
    - `test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints`
      不再错误要求 shell smoke 内嵌固定 `Authorization: Bearer ...`

## TDD / Red-Green 证据

### Red
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test`
  - 初始失败 3 项：
    - compose 未注`sdkwork_im_PUBLIC_BEARER_HS256_SECRET`
    - smoke 脚本仍嵌`alg=none`
    - 本地最小安装文档仍宣称不做签名校验

### Green
- 修复 compose、smoke 脚本与部署文档后，`deployment_profile_test` 全部通过
- 新增 public smoke 脚本执行回归后，PowerShell / Bash smoke 都能直接命中 `build_public_app()`

## Fresh 验证
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test public_auth_e2e_test`
- `powershell -NoProfile -ExecutionPolicy Bypass -File pnpm dev -Help`
- `cmd /c bin\\retired-lifecycle-deploy.cmd --help`
- PowerShell parser 解析 `tools/smoke/local_stack_smoke.ps1`

## 当前判断
- `CP10-3`：本轮闭环
- 已兑现：
  - Docker/public smoke 不再依赖无效 unsigned bearer
  - smoke 脚本具备真实 signed bearer 生成override 能力
  - PowerShell / Bash smoke 已在测试中真实命`build_public_app()`
  - 本地最小部署文档与快启文档已追平当前认证合
- 仍未兑现
  - `CP10-4`
  - `Step 10` 整步闭环
  - `Wave D` 总验证

## 下一轮继续做什么
1. 自动进入 `Step 10 / CP10-4`
2. 围绕 inspect / repair / restore 的多 profile 交付闭环补代码、测试、review 与架构回写
3. `Step 10` 通过后，再按 `93` 规则准备 `Wave D` 总验证
