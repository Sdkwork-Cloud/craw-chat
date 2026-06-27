# Component Specs

This directory is the local SDKWork component contract for `@sdkwork/im-pc`.

- Component root: `sdkwork-im/apps/sdkwork-im-pc`
- Canonical standards: `../../../sdkwork-specs/README.md`
- Machine-readable contract: `specs/component.spec.json`

Read `specs/component.spec.json` before changing this component's public exports, runtime entrypoints, SDK clients, generated artifacts, config keys, or verification commands.

Do not copy root standards into this directory. Link to files under `../../../sdkwork-specs/` instead.

## Workspace Authority

The PC app root is registered in the repository root `pnpm-workspace.yaml`
(`apps/sdkwork-im-pc` and `apps/sdkwork-im-pc/packages/*`). The app-local
`pnpm-workspace.yaml` only carries relative capability-package and sibling-SDK
entries; it must not duplicate or override the root workspace authority.

## PC Client Package Naming

Capability packages follow the SDKWork PC architecture segment. Canonical naming:

- Console surface: `sdkwork-im-console-*` (normalized PC target `sdkwork-im-pc-console-*`).
- Admin surface: `sdkwork-im-admin-*` (normalized PC target `sdkwork-im-pc-admin-*`).
- PC-native capabilities: `sdkwork-im-pc-*`.

Historical `sdkwork-clawchat-console-*` and `sdkwork-clawchat-admin-*` package
names were retired by the `sdkwork-im → sdkwork-im` rebrand; new packages must
use the canonical `sdkwork-im-*` vocabulary.
