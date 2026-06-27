> Migrated from `docs/review/continuous-optimization-install-deploy-cmd-help-gnu-surface-contract-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Install/Deploy CMD Help GNU-Surface Contract

## Context

- Current loop: continue Step 12 Windows operator seam tightening after the `start/status` and `principal-profile` fixes.
- Surfaces: `(retired lifecycle script)`, `pnpm dev`
- Contract: Windows `.cmd` entrypoints must expose the same GNU-style named flags that docs and operators already use.

## Confirmed Gap

- `cmd /c .\bin\retired-lifecycle-install.cmd --help` only showed:
  - `Usage: powershell -ExecutionPolicy Bypass -File (retired lifecycle script) [-Release] [-BindAddress <host:port>]`
- `cmd /c .\bin\retired-lifecycle-deploy.cmd --help` only showed:
  - `Usage: powershell -ExecutionPolicy Bypass -File pnpm dev [-ProfileName <standalone.split-services.development|standalone.split-services.development>] [-SkipSmoke] [-SmokeBaseUrl <url>]`
- That left the same drift already closed for `retired-lifecycle-start.cmd` and `retired-lifecycle-status.cmd`:
  - docs taught `--release`, `--bind-addr`, `--profile`, `--skip-smoke`, `--smoke-base-url`
  - local Windows help still surfaced only PowerShell syntax

## Decision

- Keep `_cmd-forward-powershell.cmd` and runtime execution flow unchanged.
- Patch only the `-Help` branch in `(retired lifecycle script)` and `pnpm dev`.
- Freeze the visible Windows help contract with two regression tests in `deployment_profile_test.rs`.

## Changed Files

- `(retired lifecycle script)`
- `pnpm dev`
- `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
- `docs/review/continuous-optimization-install-deploy-cmd-help-gnu-surface-contract-2026-04-09.md`
- `docs/step/continuous-optimization-install-deploy-cmd-help-gnu-surface-contract-2026-04-09.md`
- `docs/架构/09BI-install-deploy-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150BI-install-deploy-cmd-help-gnu-surface-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline cmd_help_surfaces_gnu_style_named_flags -- --nocapture
```

- Failed before the patch because both tests only saw PowerShell usage lines.

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline cmd_help_surfaces_gnu_style_named_flags -- --nocapture
cmd /c .\bin\retired-lifecycle-install.cmd --help
cmd /c .\bin\retired-lifecycle-deploy.cmd --help
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
cargo fmt --all --check
```

## Remaining Gap

- This loop closes Windows help discoverability for `retired-lifecycle-install.cmd` and `retired-lifecycle-deploy.cmd`.
- The next higher-value gap is no longer operator help parity; it is deeper runtime/provider maturity:
  - invalid `principal-profile-external-catalog` bootstrap semantics and health reporting
  - real RTC / object-storage / IoT provider runtime depth beyond current contract baseline

