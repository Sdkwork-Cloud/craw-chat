> Migrated from `docs/review/continuous-optimization-runtime-lifecycle-profile-selection-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Runtime Lifecycle Profile Selection

## Context

- `retired-lifecycle-status.*`、`inspect-runtime-local.*`、`repair-runtime-local.*` 已支持 `standalone.split-services.development` / `standalone.split-services.development` profile。
- `init-config-local.*`、`retired-lifecycle-install.*`、`retired-lifecycle-start.*`、`retired-lifecycle-stop.*`、`retired-lifecycle-restart.*` 仍写死 `standalone.split-services.development`。

## Confirmed Bug

- 选择 `standalone.split-services.development` 时，运行时运维脚本与生命周期脚本会落到不同配置入口。
- `retired-lifecycle-restart.*` 无法把选定 profile 传给 `stop/start`。
- `init-config-local.*` 无法生成 `.runtime/standalone.split-services.development/config/standalone.split-services.development.env`。

## Root Cause

- runtime profile 公共解析器只在部分脚本落地。
- PowerShell 生命周期脚本继续使用字符串数组透传命名参数，导致 `-ProfileName` 被当成普通位置参数。

## Fix

- 为 `init/install/start/stop/restart` 补齐 `-ProfileName` / `--profile`。
- 统一接入 `_runtime-profile-common.ps1` / `_runtime-profile-common.sh`，按 profile 解析 config 与 runtime-dir。
- `standalone.split-services.development` 写入独立 config 文件，但仍保持当前 `SDKWORK_IM_RUNTIME_DIR -> .runtime/standalone.split-services.development` 兼容合同。
- `retired-lifecycle-restart.*` 显式把 profile 传给 `stop/start`；PowerShell 改为真正的命名参数调用。
- 新增回归测试：
  - `test_restart_local_ps1_forwards_profile_name_to_stop_and_start_scripts`
  - `test_restart_local_sh_forwards_profile_selection_to_stop_and_start_scripts`
  - `test_init_config_local_ps1_uses_local_default_profile_when_requested`
  - 扩展 `test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_restart_local_ps1_forwards_profile_name_to_stop_and_start_scripts -- --exact --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_init_config_local_ps1_uses_local_default_profile_when_requested -- --exact --nocapture
```

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test -- --nocapture
cargo fmt --all
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Result

- lifecycle 与 runtime ops 现在共享同一 profile 解析合同。
- `standalone.split-services.development` 已具备对称的 config/init/install/start/stop/restart 入口。
- 现有 Windows/CMD/PowerShell 回归全部通过。

## Boundary

- 当前会话没有可用 Bash 运行时，若干 `.sh` 真实执行测试仍按既有机制跳过。
- 本轮没有把 `standalone.split-services.development` 升级为独立 runtime topology，只修正选择入口与兼容回退。

