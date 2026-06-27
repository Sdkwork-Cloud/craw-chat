# Wave D server deployment payload

- bundle: `wave-d-2026-04-08`
- artifact root: `artifacts/releases/wave-d-2026-04-08/server/deployments`
- deployment-payload documentation state: `template_only_pending_payload`

This directory documents the deployment templates and service definitions that every server package
form must preserve.

## Required configuration templates

- `deployments/templates/server.yaml.example`
- `deployments/templates/server.env.example`
- `deployments/templates/postgresql.yaml.example`

## Required service definitions

- `deployments/systemd/sdkwork-im-server.service`
- `deployments/launchd/com.sdkwork.SdkworkIm.server.plist`
- `deployments/windows-service/SdkworkImServer.xml`

## Shared deployment invariants

- `server.yaml` remains the single startup configuration contract
- installer-specific helpers may render instance-specific files, but may not redefine the service
  identity or canonical startup semantics
- packaged delivery must remain aligned with:
  - `install-server`
  - `init-config-server`
  - `init-storage-server`
  - `install-service-server`

## Relationship to other release contracts

- payload lineage:
  - `../release-provenance.json`
- package inventory:
  - `../package-catalog.json`
- Windows wrapper-specific materialization:
  - `../windows-service/README.md`

## Current interpretation

- deployment contracts are frozen and checked in
- package-specific generated copies are still template-only until platform packaging runs

