# artifacts/releases

`artifacts/releases` stores canonical release bundles for Sdkwork IM.

A release bundle is an auditable snapshot of what would be shipped for a named wave or release
track on a specific date. This directory is not a scratch area for arbitrary generated files. Its
job is to freeze release payload shape, machine-readable manifests, verification references, and
publication state in a form that can be reviewed, replayed, and archived.

## Bundle Naming

- Every release snapshot lives under `artifacts/releases/<bundle-id>/`.
- The recommended `<bundle-id>` format is `<wave-or-track>-<yyyy-mm-dd>`.
- Example: `wave-d-2026-04-08`.

## Minimum Bundle Contents

Every bundle should include at least the following release records:

- `bundle-manifest.md`
  - captures step, wave, date, go / no-go state, and verification commands
- review evidence references
  - examples: `step-13-release-readiness-2026-04-08.md`
  - examples: `wave-d-93-final-acceptance-2026-04-08.md`
- upgrade and rollback entrypoints
- the current release scope and known remaining gaps

## Optional Machine-Readable Release Assets

The bundle may also freeze machine-readable artifacts that support release automation and auditing.

- machine-readable evidence index
  - example: `artifacts/releases/wave-d-2026-04-08/self-hosted.split-services.development-post-release-evidence-index.json`
  - used to freeze operator templates, sample docs, and `evidenceSlots` as structured release data
- machine-readable SDK release catalog
  - example: `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
  - freezes SDK audience, language, workspace, generation state, and publication state for the
    bundle
  - if versions are not assigned yet, keep the release-freeze fields explicit:
    - `plannedVersion = null`
    - `versionStatus = version_unassigned_pending_freeze`
    - `versionDecisionSourcePath = null`
- machine-readable server package catalog
  - `artifacts/releases/wave-d-2026-04-08/server/package-catalog.json`
  - freezes platform, package type, artifact path, install roots, service manager, and startup
    command
  - current server package catalog state may remain `template_only_pending_build` until archive and
    native installers are produced
- machine-readable server package acceptance manifests
  - `artifacts/releases/wave-d-2026-04-08/server/packages/<platform>/artifacts/acceptance-manifest.json`
  - freezes per-package acceptance checks and template-only validation placeholders
- machine-readable server release execution manifest
  - `artifacts/releases/wave-d-2026-04-08/server/release-execution.json`
  - freezes canonical build metadata plus Linux, macOS, and Windows staging execution surfaces
- machine-readable server release provenance manifest
  - `artifacts/releases/wave-d-2026-04-08/server/release-provenance.json`
  - freezes the payload-defining source files and machine-readable release contract paths
- machine-readable server release gate manifest
  - `artifacts/releases/wave-d-2026-04-08/server/release-gate.json`
  - freezes go / no-go inputs, review doc links, and platform acceptance requirements
  - `bin/plan-release-server.ps1` and `bin/plan-release-server.sh` consume
    `release-gate.json`, `package-catalog.json`, and `release-execution.json` to emit a dry-run
    release plan

## Release Schemas

Machine-readable bundle artifacts must stay schema-bound.

- SDK release catalog schema
  - `artifacts/releases/schemas/sdk-release-catalog.schema.json`
  - `sdk-release-catalog.json` must bind through `$schema`
- evidence index schema
  - `artifacts/releases/schemas/post-release-evidence-index.schema.json`
  - evidence indexes must bind through `$schema`
  - the top level should also freeze `artifactRoot`
  - `collectionSummary` is derived from `evidenceSlots[*].required` and `evidenceSlots[*].status`
  - optional stable paths may include:
    - `checksumManifestPath`
    - `artifactFileListPath`
    - `collectionSummary`
  - recommended slot metadata includes:
    - `artifactPath`
    - `suggestedRelativePath`
    - `collectedAt`
    - `sizeBytes`
    - `checksumSha256`
- server package catalog schema
  - `artifacts/releases/schemas/server-package-catalog.schema.json`
  - `server/package-catalog.json` must bind through `$schema`
- server package acceptance schema
  - `artifacts/releases/schemas/server-package-acceptance.schema.json`
  - acceptance manifests must bind through `$schema`
- server release execution schema
  - `artifacts/releases/schemas/server-release-execution.schema.json`
  - `server/release-execution.json` must bind through `$schema`
- server release provenance schema
  - `artifacts/releases/schemas/server-release-provenance.schema.json`
  - `server/release-provenance.json` must bind through `$schema`
- server release gate schema
  - `artifacts/releases/schemas/server-release-gate.schema.json`
  - `server/release-gate.json` must bind through `$schema`

## Server Edition Payload Standard

The release bundle is the canonical payload source for `sdkwork-im-server`.

- canonical payload contains binaries, templates, service units, migrations, docs, checksums, and
  bundle-manifest metadata
- archive installers remain first-class artifacts:
  - `linux`: `tar.gz`
  - macOS: `tar.gz`
  - `windows`: `zip`
- native installers are derived wrappers around the same payload:
  - `linux`: `deb`, `rpm`
  - macOS: `pkg`
  - `windows`: `msi`

The canonical payload may not redefine configuration or service semantics per platform.

## Server Payload Layout

Every server bundle must freeze a single canonical payload layout for `sdkwork-im-server`.

- bundle-level server payload index:
  - `artifacts/releases/<bundle-id>/server/README.md`
  - this is the canonical payload layout index for the server bundle
- bundle-level server package matrix index:
  - `artifacts/releases/<bundle-id>/server/packages/README.md`
  - freezes the mapping between platform installers and canonical initialization entrypoints
- bundle-level server machine-readable package catalog:
  - `artifacts/releases/<bundle-id>/server/package-catalog.json`
  - freezes package metadata across archive and native installer forms
- bundle-level server machine-readable release execution manifest:
  - `artifacts/releases/<bundle-id>/server/release-execution.json`
  - freezes the canonical build source and platform execution graph for release staging
- bundle-level server machine-readable release provenance manifest:
  - `artifacts/releases/<bundle-id>/server/release-provenance.json`
  - freezes which source files and release contracts define the server payload
- bundle-level server machine-readable release gate manifest:
  - `artifacts/releases/<bundle-id>/server/release-gate.json`
  - freezes machine-readable go / no-go gate inputs for the server bundle
- required runtime payload:
  - `bin/sdkwork-im-server` or `bin/sdkwork-im-server.exe`
  - `deployments/templates/server.yaml.example`
  - `deployments/templates/server.env.example`
  - `deployments/templates/postgresql.yaml.example`
  - `deployments/systemd/sdkwork-im-server.service`
  - `deployments/launchd/com.sdkwork.SdkworkIm.server.plist`
  - `deployments/windows-service/SdkworkImServer.xml`
- Windows Service payload is `wrapper-required`:
  - bundle or installer must ship `bin/SdkworkImServer.exe`
  - `install-service-server` renders `generated/SdkworkImServer.xml`,
    `install-SdkworkImServer.ps1`, and `uninstall-SdkworkImServer.ps1`
  - the wrapper must keep the same process contract:
    `sdkwork-im-server --config <config-root>/server.yaml`
- derived installers such as `tar.gz`, `zip`, `deb`, `rpm`, `pkg`, and `msi` may repackage the
  same files, but may not rename the canonical service identity or change `server.yaml` startup
  semantics
  - Linux package matrix must stay aligned with `install-server.sh`, `init-config-server.sh`,
    `init-storage-server.sh`, and `install-service-server.sh`
  - macOS package matrix must stay aligned with `install-server.sh`, `init-config-server.sh`,
    `init-storage-server.sh`, and `install-service-server.sh`
  - Windows package matrix must stay aligned with `install-server.ps1`,
    `init-config-server.ps1`, `init-storage-server.ps1`, `install-service-server.ps1`, and the
    corresponding `.cmd` wrappers

## SDK Release Catalog Standard

The SDK release catalog is a machine-readable release snapshot, not a hand-maintained spreadsheet.

- authority source:
  - the checked-in `.sdkwork-assembly.json` files under each SDK workspace
- current workspaces:
  - `sdks/sdkwork-im-sdk`
  - `sdks/sdkwork-im-app-sdk`
  - `sdks/sdkwork-im-backend-sdk`
  - `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk`
- synchronization script:
  - `node artifacts/releases/sync-sdk-release-catalog.mjs --bundle <bundle-id>`
- drift check:
  - `node artifacts/releases/sync-sdk-release-catalog.mjs --bundle <bundle-id> --check`

The release-state semantics are:

- `template_only_pending_generation`
  - one or more tracked language lines do not yet have generated workspace output
- `generated_pending_publication`
  - all tracked language lines are generated, but at least one artifact is still not published
- `published`
  - every tracked language line is published

## Current Status

- `wave-d-2026-04-08` already includes a checked-in `sdk-release-catalog.json`
- the current SDK release-catalog state is `template_only_pending_generation`
- the tracked artifact set covers the IM standard, App API, Backend API, and RTC provider-runtime
  SDK families recorded in `sdk-release-catalog.json`
- the generated HTTP SDK families currently show:
  - `generationStatus = generated`
  - `releaseStatus = not_published`
- the independent RTC provider-runtime SDK family currently shows:
  - `generationStatus = template_only_pending_generation`
  - `releaseStatus = not_published`
- version-freeze fields remain explicit until publication is planned:
  - `plannedVersion = null`
  - `versionStatus = version_unassigned_pending_freeze`
  - `versionDecisionSourcePath = null`

The release bundle archive goals remain the same:

- auditable
- rollback-ready
- traceable

When real published SDK versions are assigned, keep the same directory structure and advance only
the release catalog fields and publication evidence.
