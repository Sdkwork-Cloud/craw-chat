# Wave D 2026-04-08 Bundle Manifest

## Release Snapshot

- bundle id: `wave-d-2026-04-08`
- wave: `wave-d`
- archive baseline scope: `Step 13`, `Wave D / 93`
- bundle archive decision: `go`
- server package gate decision: `pending_go_no_go`
- SDK publication state: `template_only_pending_generation`

This manifest freezes the current release-bundle snapshot for repository audit, rollback planning,
and traceability. It does not claim that all server installers are built or that any SDK package is
already published.

## Review And Evidence References

- `docs/review/step-13-release-readiness-2026-04-08.md`
- `docs/review/step-13-go-no-go清单-2026-04-08.md`
- `docs/review/step-13-next-wave-backlog-2026-04-08.md`
- `docs/review/wave-d-93-总验收-2026-04-08.md`
- `docs/review/step-13-架构兑现-2026-04-08.md`
- `docs/review/step-13-架构回写决议-2026-04-08.md`
- `docs/review/continuous-optimization-local-default-machine-readable-evidence-index-2026-04-08.md`
- `docs/review/continuous-optimization-release-bundle-evidence-index-schema-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-release-catalog-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-release-catalog-schema-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-leaf-readmes-release-boundary-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-container-readmes-release-boundary-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-release-version-placeholder-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-release-version-decision-source-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-leaf-readmes-version-decision-source-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-container-readmes-version-decision-source-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-leaf-readmes-version-placeholder-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-container-readmes-version-placeholder-2026-04-08.md`
- `docs/review/continuous-optimization-evidence-slot-metadata-contract-2026-04-08.md`
- `docs/review/continuous-optimization-evidence-artifact-root-contract-2026-04-08.md`
- `docs/review/continuous-optimization-evidence-slot-suggested-path-contract-2026-04-08.md`
- `docs/review/continuous-optimization-evidence-slot-size-bytes-contract-2026-04-08.md`
- `docs/review/continuous-optimization-checksum-manifest-contract-2026-04-08.md`
- `docs/review/continuous-optimization-artifact-file-list-contract-2026-04-08.md`
- `docs/review/continuous-optimization-collection-summary-contract-2026-04-08.md`
- `docs/review/continuous-optimization-collection-summary-slot-consistency-contract-2026-04-08.md`
- `docs/部署/性能与灾备演练场景.md`
- `docs/部署/性能与灾备演练场景.md`

## Frozen Bundle Artifact Inventory

### SDK Release Artifacts

- `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
- `artifacts/releases/schemas/sdk-release-catalog.schema.json`
- `artifacts/releases/sync-sdk-release-catalog.mjs`

### Evidence Artifacts

- `artifacts/releases/wave-d-2026-04-08/self-hosted.split-services.development-post-release-evidence-index.json`
- `artifacts/releases/wave-d-2026-04-08/evidence/self-hosted.split-services.development/README.md`
- `artifacts/releases/wave-d-2026-04-08/evidence/self-hosted.split-services.development/checksum-manifest.txt`
- `artifacts/releases/wave-d-2026-04-08/evidence/self-hosted.split-services.development/artifact-file-list.txt`
- `artifacts/releases/schemas/post-release-evidence-index.schema.json`

### Server Release Artifacts

- `artifacts/releases/wave-d-2026-04-08/server/README.md`
- `artifacts/releases/wave-d-2026-04-08/server/package-catalog.json`
- `artifacts/releases/wave-d-2026-04-08/server/release-execution.json`
- `artifacts/releases/wave-d-2026-04-08/server/release-provenance.json`
- `artifacts/releases/wave-d-2026-04-08/server/release-gate.json`
- `artifacts/releases/wave-d-2026-04-08/server/bin/README.md`
- `artifacts/releases/wave-d-2026-04-08/server/deployments/README.md`
- `artifacts/releases/wave-d-2026-04-08/server/windows-service/README.md`
- `artifacts/releases/wave-d-2026-04-08/server/packages/README.md`
- `artifacts/releases/wave-d-2026-04-08/server/packages/SHA256SUMS`
- `artifacts/releases/wave-d-2026-04-08/server/packages/artifact-file-list.txt`
- `artifacts/releases/wave-d-2026-04-08/server/packages/release-checklist.md`
- `artifacts/releases/wave-d-2026-04-08/server/packages/layout-tree.txt`
- `artifacts/releases/wave-d-2026-04-08/server/packages/linux/README.md`
- `artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/README.md`
- `artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/layout-tree.txt`
- `artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/acceptance-manifest.json`
- `artifacts/releases/wave-d-2026-04-08/server/packages/macos/README.md`
- `artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/README.md`
- `artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/layout-tree.txt`
- `artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/acceptance-manifest.json`
- `artifacts/releases/wave-d-2026-04-08/server/packages/windows/README.md`
- `artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/README.md`
- `artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/layout-tree.txt`
- `artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/acceptance-manifest.json`
- `artifacts/releases/schemas/server-package-catalog.schema.json`
- `artifacts/releases/schemas/server-package-acceptance.schema.json`
- `artifacts/releases/schemas/server-release-execution.schema.json`
- `artifacts/releases/schemas/server-release-provenance.schema.json`
- `artifacts/releases/schemas/server-release-gate.schema.json`

## Verification Commands

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features --offline -- -D warnings`
- `cargo test --workspace --offline`
- `powershell -NoProfile -ExecutionPolicy Bypass -File pnpm dev -Help`
- `pnpm dev`
- `powershell -NoProfile -ExecutionPolicy Bypass -File pnpm dev:server -Help`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/retired-lifecycle-status.ps1 -Help`
- `pnpm dev:server (topology v2)`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/restore-runtime-local.ps1 -Help`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/open-chat-test.ps1 -Help`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/plan-release-server.ps1 -ReleaseGatePath artifacts/releases/wave-d-2026-04-08/server/release-gate.json`
- `node artifacts/releases/sync-sdk-release-catalog.mjs --bundle wave-d-2026-04-08 --check`
- `node docs/sites/sdk/verify-sdk-site-docs.mjs`
- `node sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs`
- `node sdks/sdkwork-im-app-sdk/bin/verify-sdk.mjs`
- `node sdks/sdkwork-im-backend-sdk/bin/verify-sdk.mjs`
- `node <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/bin/verify-sdk.mjs`

## Rollout And Recovery Entrypoints

### Local Rollout

- `pnpm dev`
- `pnpm dev:server`
- `bin/retired-lifecycle-status.ps1`

### Recovery

- `bin/restore-runtime-local.ps1`

### Server Release Planning

- `bin/plan-release-server.ps1`
- `bin/plan-release-server.sh`

## Frozen State Summary

### Bundle Scope

- the bundle is a release archive baseline, not a complete end-to-end publication pipeline
- `self-hosted.split-services.development` still uses topology v2 `self-hosted.split-services.development` service contract shape
- the archive is already sufficient for audit, rollback planning, and traceability

### SDK State

- the SDK family release snapshot is frozen in `sdk-release-catalog.json`
- current catalog state: `template_only_pending_generation`
- current generated HTTP SDK artifact state: `generationStatus = generated`
- current RTC provider-runtime SDK artifact state: `generationStatus = template_only_pending_generation`
- current tracked artifact publication state: `releaseStatus = not_published`
- version-freeze fields remain explicit:
  - `plannedVersion = null`
  - `versionStatus = version_unassigned_pending_freeze`
  - `versionDecisionSourcePath = null`
- the release catalog must be derived from the checked-in `.sdkwork-assembly.json` files through
  `sync-sdk-release-catalog.mjs`, not edited by hand

### Server Packaging State

- server package catalog state: `template_only_pending_build`
- server release gate state: `template_only_pending_evaluation`
- server release decision status: `pending_go_no_go`
- the canonical payload layout and platform package matrix are frozen, but archive and native
  installer artifacts are still template-only placeholders

## Archive Goals

- auditable
- rollback-ready
- traceable

When package publication or real server installer builds happen later, this bundle layout should be
extended in place rather than replaced with a new ad hoc structure.
