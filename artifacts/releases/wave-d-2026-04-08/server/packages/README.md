# Wave D server package matrix

- bundle: `wave-d-2026-04-08`
- artifact root: `artifacts/releases/wave-d-2026-04-08/server/packages`
- package-matrix documentation state: `template_only_pending_payload`

This directory is the human-readable package matrix for `sdkwork-im-server`. It maps platform
delivery forms back to the canonical server payload and the shared startup contract.

## Package-matrix contracts

- `../package-catalog.json`
  - package inventory, file-name templates, install roots, and service managers
- `../release-execution.json`
  - per-platform staging and checksum workflow examples
- `../release-gate.json`
  - packaging go / no-go inputs
- `linux/artifacts/acceptance-manifest.json`
- `macos/artifacts/acceptance-manifest.json`
- `windows/artifacts/acceptance-manifest.json`
- `release-checklist.md`
  - human-reviewed sequencing for packaging and verification
- `artifact-file-list.txt`
  - frozen artifact listing contract
- `SHA256SUMS`
  - frozen checksum manifest contract
- `layout-tree.txt`
  - package layout snapshot contract

## Platform package families

- `linux`
  - `tar.gz`
  - `deb`
  - `rpm`
- macOS
  - `tar.gz`
  - `pkg`
- `windows`
  - `zip`
  - `msi`

## Frozen artifact names

- `sdkwork-im-server-linux-x86_64.tar.gz`
- `sdkwork-im-server_<version>_amd64.deb`
- `sdkwork-im-server-<version>-1.x86_64.rpm`
- `sdkwork-im-server-darwin-universal.tar.gz`
- `sdkwork-im-server-<version>.pkg`
- `sdkwork-im-server-windows-x86_64.zip`
- `sdkwork-im-server-<version>-x64.msi`

## Platform indexes

- `linux/README.md`
- `linux/artifacts/README.md`
- `macos/README.md`
- `macos/artifacts/README.md`
- `windows/README.md`
- `windows/artifacts/README.md`

## Shared invariants

- all package lines map back to the same canonical payload
- all package lines preserve the same `server.yaml` startup contract
- all package lines preserve the same service identity semantics
- Windows package lines additionally preserve the dedicated wrapper contract for
  `SdkworkImServer.exe`

## Current interpretation

- the package matrix is frozen for review and automation
- the package contract paths are real and checked in
- the archive and installer artifacts themselves are still template-only staged outputs until
  actual packaging runs populate the staging roots
