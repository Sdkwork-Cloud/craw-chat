# sdkwork-api-product-runtime

Domain: communication
Capability: im
Package type: rust-crate
Status: standardizing

This README is the SDKWork module entrypoint for `sdkwork-api-product-runtime`. The machine-readable component contract is `specs/component.spec.json`; canonical standards are under `../../../sdkwork-specs/`.

## Public API

- `.`

## Required SDK Surface

- None declared in `specs/component.spec.json`.

## Configuration

Configuration keys, runtime entrypoints, and integration contracts are declared in
`specs/component.spec.json`. `SDKWORK_ADMIN_PROXY_TARGET` selects the real backend-admin upstream.
`SDKWORK_ADMIN_SANDBOX` and `SDKWORK_ADMIN_SANDBOX_STORAGE_FILE` are explicit development/test
tools, not production storage configuration.

## Deployment Profile And Runtime Target Behavior

Standalone desktop development may opt into the admin sandbox. Production-like standalone and cloud
runtimes fail startup when the sandbox is enabled and require a configured admin upstream. The
sandbox never serves as billing, metering, audit, tenant, or storage source of truth for a production
deployment.

## Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this
module. Protected API and SDK access must use the generated SDK or approved service boundary declared
in the component contract. File-backed sandbox state must contain development-only data.

## Extension Points

Extension points are limited to public exports, runtime entrypoints, SDK clients, events, and config keys declared in `specs/component.spec.json`.

## Verification

- `cargo test --manifest-path apps/sdkwork-im/crates/sdkwork-api-product-runtime/Cargo.toml`

## Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`. Update that contract before changing public integration behavior.
