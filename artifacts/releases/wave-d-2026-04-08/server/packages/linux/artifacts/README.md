# Wave D Linux staging root

- bundle: `wave-d-2026-04-08`
- artifact root: `artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts`
- staging documentation state: `template_only_pending_payload`
- execution state: `template_only_pending_execution`

This staging root is where Linux packaging outputs are expected to land when the release execution
plan is run.

- canonical build command: `cargo build -p web-gateway --release --bin sdkwork-im-server --offline`

## Expected staged artifacts

- `sdkwork-im-server-linux-x86_64.tar.gz`
- `sdkwork-im-server_<version>_amd64.deb`
- `sdkwork-im-server-<version>-1.x86_64.rpm`

## Shared staging contracts

- `acceptance-manifest.json`
- `layout-tree.txt`
- `../../artifact-file-list.txt`
- `../../SHA256SUMS`

## Canonical workflow

- Step 1: build the canonical `sdkwork-im-server` binary.
- Step 2: assemble Linux package outputs into this staging root.
- Step 3: refresh `../artifact-file-list.txt`.
- Step 4: refresh `../SHA256SUMS`.
- Step 5: validate `acceptance-manifest.json`.

## Checksum example

- `sha256sum -b <artifact> >> ../SHA256SUMS`
