# Crates

## Purpose

Rust crates for Sdkwork IM contracts, services, repositories, gateways, route helpers, runtime
adapters, and reusable Rust libraries.

## Owner

SDKWork Chat maintainers.

## Allowed Content

- Cargo workspace members declared from the repository root `Cargo.toml`.
- Rust source, package-local tests, component specs, and crate README files.
- Route, service, repository, gateway, worker, native host, and reusable library crates.

## Forbidden Content

- Generated SDK language output.
- Runtime databases, logs, caches, secrets, or user-private files.
- Cross-repository SDKWork source dependencies copied into this repository.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/RUST_CODE_SPEC.md`
- `../sdkwork-specs/NAMING_SPEC.md`
- `../sdkwork-specs/TEST_SPEC.md`

## Verification

Run `cargo fmt --all --check` and the narrow crate tests for changed crates. Run
`pnpm run test:sdkwork-workspace-structure-standard` after root layout changes.
