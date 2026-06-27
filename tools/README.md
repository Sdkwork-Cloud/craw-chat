# Tools

## Purpose

Developer, validation, generation, migration, and operator tools that are not shipped as Sdkwork IM
runtime code.

## Owner

SDKWork Chat maintainers.

## Allowed Content

- Reusable validators, generators, migration helpers, parsers, CLIs, and operator utilities.
- Tool-local tests, docs, and component specs.

## Forbidden Content

- Thin command entrypoints that simply launch existing tools; those belong in `scripts/`.
- Runtime services, generated SDK output, caches, secrets, or local credentials.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/CODE_STYLE_SPEC.md`
- `../sdkwork-specs/TEST_SPEC.md`

## Verification

Run the tool-specific test command and `pnpm run test:sdkwork-workspace-structure-standard` after
root layout changes.
