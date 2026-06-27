# Continuous Optimization - runtime ops status profile and ps1 smoke - 2026-04-08

## 1. 本轮背景

- `Step 13` 与 `Wave D / 93` 已收口，但 `step-13-next-wave-backlog-2026-04-08.md` 仍要求把 `status / inspect / repair / archive / prune / preview / restore` 从 help 面验证继续推进到最小 smoke 行为证据。
- 当前仓库里：
  - `inspect-runtime-local.ps1`、`inspect-runtime-local.cmd` 已有 profile 行为回归；
  - `repair-runtime-local.sh` 已有 Bash profile 行为回归；
  - `retired-lifecycle-status.*` 仍固定停留在 `standalone.split-services.development`；
  - `repair/archive/prune/preview/restore` 的 PowerShell wrapper 仍缺少统一的最小 smoke 证明。

## 2. 实际落地

### 2.1 retired-lifecycle-status 已对齐 runtime ops profile 合同

- 更新：`bin/retired-lifecycle-status.ps1`
- 更新：`bin/retired-lifecycle-status.sh`
- 更新：`docs/部署/快速启动脚本.md`
- 当前新增能力：
  - `retired-lifecycle-status.ps1` 支持 `-ProfileName <standalone.split-services.development|standalone.split-services.development>` 与 `-RuntimeDir <path>`
  - `retired-lifecycle-status.sh` 支持 `--profile <standalone.split-services.development|standalone.split-services.development>` 与 `--runtime-dir <path>`
  - 状态输出会根据所选 profile 解析：
    - config 路径
    - bind 地址
    - healthz URL
    - stdout / stderr log 路径
    - 后续 runtime ops 命令建议
  - 当选择 `standalone.split-services.development` 时，后续建议命令会显式保留同一 profile 参数，避免排障链路回退到默认 `standalone.split-services.development`

### 2.2 runtime ops PowerShell wrapper 最小 smoke 已冻结

- 更新：`services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
- 当前新增或扩展回归：
  - `test_runtime_operation_scripts_expose_profile_selection_contract`
    - 已把 `retired-lifecycle-status.ps1` / `retired-lifecycle-status.sh` 纳入 runtime ops profile 合同门禁
  - `test_status_local_ps1_uses_local_default_profile_config_when_requested`
  - `test_status_local_cmd_supports_profile_switch`
  - `test_status_local_sh_uses_local_default_profile_config_when_requested`
    - 在无可用 Bash runtime 的机器上允许跳过，不伪造结果
  - `test_runtime_operation_ps1_wrappers_forward_profile_and_backup_arguments`
    - 真实执行 `repair/archive/prune/preview/restore` PowerShell wrapper
    - 通过 fake cargo 捕获并冻结：
      - standalone.split-services.development runtime dir 解析
      - `backup-dir`
      - `retention-days`
      - `legal-hold`
      - `expected-preview-fingerprint`
      - `--json`

### 2.3 当前收口边界

- 这一轮补到的是“最小可信 smoke 行为证据”，不是完整的跨平台真实节点 E2E。
- 尤其是 Bash 真实执行仍受当前 Windows 机器可用 Bash runtime 限制：
  - `retired-lifecycle-status.sh` 新增了最小 smoke 测试
  - 若环境没有可用 Bash，测试会明确 skip，而不是伪装通过

## 3. 当前判断

- `retired-lifecycle-status` 不再是 runtime ops 里唯一停留在 `standalone.split-services.development` 固定视角的入口。
- `repair/archive/prune/preview/restore` 的 PowerShell wrapper 现在已经从“仅 help/字符串合同”提升为“可执行 smoke 证据”。
- 本轮是对 `runtime ops minimal smoke` backlog 的实质推进，不是文档粉饰。
- 下一轮可继续推进：
  - `standalone.split-services.development` 对称发布后验证样本
  - 可用 Bash 环境上的真实 runtime ops shell 证据补全

## 4. fresh evidence

- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test -- --nocapture`
