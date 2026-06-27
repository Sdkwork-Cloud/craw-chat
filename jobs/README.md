# Jobs

## Purpose

Schedules, queue bindings, batch descriptors, maintenance runbooks, and non-Rust job packages for
Sdkwork IM.

## Owner

SDKWork Chat maintainers.

## Allowed Content

- Job schedules and queue binding metadata.
- Batch job descriptors and maintenance runbooks.
- Non-Rust job package roots when they are not request/response services.

## Forbidden Content

- Rust worker implementations, which belong under `crates/sdkwork-<domain>-<capability>-worker/`.
- Runtime queue state, logs, caches, secrets, or local credentials.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/APPLICATION_SPEC.md`
- `../sdkwork-specs/RUNTIME_DIRECTORY_SPEC.md`

## Verification

Run `pnpm run test:sdkwork-workspace-structure-standard` from the repository root.
