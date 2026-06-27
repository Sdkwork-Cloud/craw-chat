# Scripts

## Purpose

Thin command entrypoints for build, verification, generation, migration, packaging, release, and
local development workflows.

## Owner

SDKWork Chat maintainers.

## Allowed Content

- Deterministic shell, Node.js, and PowerShell command wrappers.
- Static verification tests that exercise repository workflow contracts.
- Small orchestration scripts that delegate reusable logic to tools, packages, or crates.

## Forbidden Content

- Large reusable libraries that belong in `tools/` or a package.
- Runtime state, generated SDK output, caches, secrets, local credentials, or machine-specific
  absolute paths.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md`
- `../sdkwork-specs/TEST_SPEC.md`

## Verification

Run script-specific tests and `pnpm run test:sdkwork-workspace-structure-standard` after root
layout changes.
