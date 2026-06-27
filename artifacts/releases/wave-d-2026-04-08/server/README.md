# Wave D server payload index

- bundle: `wave-d-2026-04-08`
- artifact root: `artifacts/releases/wave-d-2026-04-08/server`
- payload documentation state: `template_only_pending_payload`

This directory is the human-readable index for the server release payload. It does not claim that
archive installers or native installers already exist. The machine-readable contracts in the same
directory remain the automation truth for packaging and release gating.

## Machine-readable release contracts

- `package-catalog.json`
  - package matrix and install-root truth
  - current state: `template_only_pending_build`
- `release-execution.json`
  - canonical build command and per-platform staging plan
  - current state: `template_only_pending_execution`
- `release-provenance.json`
  - payload-defining source and contract lineage
  - current state: `template_only_pending_capture`
- `release-gate.json`
  - go / no-go inputs for server packaging
  - current state: `template_only_pending_evaluation`
  - current decision status: `pending_go_no_go`

## Canonical runtime payload

The canonical server payload is shared across all package forms.

- binaries
  - `bin/sdkwork-im-server`
  - `bin/sdkwork-im-server.exe`
  - `bin/SdkworkImServer.exe`
- templates
  - `deployments/templates/server.yaml.example`
  - `deployments/templates/server.env.example`
  - `deployments/templates/postgresql.yaml.example`
- service definitions
  - `deployments/systemd/sdkwork-im-server.service`
  - `deployments/launchd/com.sdkwork.SdkworkIm.server.plist`
  - `deployments/windows-service/SdkworkImServer.xml`

## Shared startup contract

- canonical foreground and service-managed process:
  - `sdkwork-im-server --config <config-root>/server.yaml`
- service-install entrypoint:
  - `install-service-server`
- shared configuration entrypoint:
  - `server.yaml`
- the package format may change by platform, but package wrappers may not redefine service
  identity, config semantics, or startup behavior

## Payload navigation

- executable payload details:
  - `bin/README.md`
- deployment template and service contract details:
  - `deployments/README.md`
- Windows Service wrapper details:
  - `windows-service/README.md`
- package matrix and platform delivery details:
  - `packages/README.md`
  - `packages/linux/README.md`
  - `packages/macos/README.md`
  - `packages/windows/README.md`

## Current interpretation

- the payload layout is frozen
- the package matrix is frozen
- the per-platform staging plan is frozen
- the release gate is still pending final package evaluation
- real built artifacts should be added into the existing layout instead of changing the layout
