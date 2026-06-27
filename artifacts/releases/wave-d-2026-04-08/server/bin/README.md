# Wave D server executable payload

- bundle: `wave-d-2026-04-08`
- artifact root: `artifacts/releases/wave-d-2026-04-08/server/bin`
- executable-payload documentation state: `template_only_pending_payload`

This directory documents the executable payload contract for `sdkwork-im-server`. The files listed
here are required runtime payload identities even when the current bundle still contains only
template-level package staging.

## Required executable identities

- `bin/sdkwork-im-server`
  - canonical Unix-like foreground and service-managed executable
- `bin/sdkwork-im-server.exe`
  - canonical Windows foreground executable
- `bin/SdkworkImServer.exe`
  - dedicated Windows Service host wrapper

## Shared process contract

- wrapped or direct startup command:
  - `sdkwork-im-server --config <config-root>/server.yaml`
- service identity that must not drift on Windows:
  - `SdkworkImServer`

## Relationship to other release contracts

- package inventory and package naming:
  - `../package-catalog.json`
- staging and build sequence:
  - `../release-execution.json`
- Windows wrapper details:
  - `../windows-service/README.md`

## Current interpretation

- executable identities are frozen
- actual packaged binaries are not yet materialized in the archive staging roots
- any future installer must preserve these executable names and process roles

