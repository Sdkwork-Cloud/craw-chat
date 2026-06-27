# Wave D Linux package line

- bundle: `wave-d-2026-04-08`
- artifact root: `artifacts/releases/wave-d-2026-04-08/server/packages/linux`
- package-line documentation state: `template_only_pending_payload`
- service manager: `systemd`

## Delivery forms

- `tar.gz`
- `deb`
- `rpm`

## Canonical artifact names

- `sdkwork-im-server-linux-x86_64.tar.gz`
- `sdkwork-im-server_<version>_amd64.deb`
- `sdkwork-im-server-<version>-1.x86_64.rpm`

## Canonical operator entrypoints

- `install-server.sh`
- `init-config-server.sh`
- `init-storage-server.sh`
- `install-service-server.sh`

## Default install-root mapping

- install root: `/opt/sdkwork-im`
- config root: `/etc/sdkwork-im/default`
- data root: `/var/lib/sdkwork-im/default`
- log root: `/var/log/sdkwork-im/default`
- run root: `/var/run/sdkwork-im/default`

## Related staging contracts

- `artifacts/README.md`
- `artifacts/acceptance-manifest.json`
- `artifacts/layout-tree.txt`

## Current interpretation

- the Linux package line is frozen for archive and native-installer packaging
- actual Linux build outputs are still template-only until packaging execution populates the staging
  root
- all Linux package forms must preserve the canonical payload layout and the shared
  `server.yaml` startup contract

