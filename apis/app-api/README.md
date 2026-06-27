# App API Contracts

## Purpose

Author-owned OpenAPI contracts for the Sdkwork IM `/app/v3/api` surface.

## Owner

SDKWork Chat maintainers.

## Allowed Content

- OpenAPI authority documents grouped by canonical domain.
- Route manifests, shared schemas, examples, changelogs, and validation fixtures.

## Forbidden Content

- Generated SDK transport output.
- Runnable HTTP handlers, services, repositories, or runtime state.

## Related Specs

- `../../sdkwork-specs/API_SPEC.md`
- `../../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`

## Verification

Run `pnpm run test:apis-authority-standard` from the repository root.
