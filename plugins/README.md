# Plugins

## Purpose

Application/runtime plugin source packages for Sdkwork IM.

## Owner

SDKWork Chat maintainers.

## Allowed Content

- Application runtime plugin source packages.
- Plugin component specs, tests, documentation, and packaging metadata.

## Forbidden Content

- Repository/application agent plugins, which belong under `.sdkwork/plugins/`.
- Generated SDK output, runtime data, caches, secrets, or user-private files.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/APPLICATION_SPEC.md`
- `../sdkwork-specs/COMPONENT_SPEC.md`

## Verification

Run `pnpm run test:sdkwork-workspace-structure-standard` from the repository root.
