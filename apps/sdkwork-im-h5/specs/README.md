# Component Specs

This directory is the local SDKWork component contract for `@sdkwork/im-h5`.

- Component root: `sdkwork-im/apps/sdkwork-im-h5`
- Canonical standards: `../../../sdkwork-specs/README.md`
- Machine-readable contract: `specs/component.spec.json`

Read `specs/component.spec.json` before changing this component's public exports, runtime entrypoints, SDK clients, generated artifacts, config keys, or verification commands.

Do not copy root standards into this directory. Link to files under `../../../sdkwork-specs/` instead.

## Workspace Authority

The H5 app root and every `sdkwork-im-h5-*` package are registered in the
repository root `pnpm-workspace.yaml` (`apps/sdkwork-im-h5` and
`apps/sdkwork-im-h5/packages/*`). Sibling SDK and capability-package source paths
are declared once at the repository root; application roots must not add nested
`pnpm-workspace.yaml` files or npm `"workspaces"` fields.

## H5 Client Package Naming

Capability packages follow the SDKWork H5 architecture segment. Canonical naming:

- H5-native capabilities: `sdkwork-im-h5-*`.

Historical `sdkwork-clawchat-mobile-*` package names were retired by the `sdkwork-im` rebrand;
new packages must use the canonical `sdkwork-im-h5-*` vocabulary.
