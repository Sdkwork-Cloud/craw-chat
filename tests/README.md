# Tests

## Purpose

Cross-package, contract, integration, end-to-end, fixture, and static verification inputs for Sdkwork
IM.

## Owner

SDKWork Chat maintainers.

## Allowed Content

- Repository-level contract tests and fixtures.
- Cross-package integration and end-to-end test assets.
- Static verification fixtures that do not belong to one package.

## Forbidden Content

- Package-local unit tests that should live beside the package or crate they verify.
- Real secrets, tokens, private customer data, runtime databases, logs, caches, or generated SDK
  transport output.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/TEST_SPEC.md`
- `../sdkwork-specs/PRIVACY_SPEC.md`

## Verification

Run `pnpm run test:sdkwork-workspace-structure-standard` from the repository root.
