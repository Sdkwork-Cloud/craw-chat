> Migrated from `docs/sites/sdk/generation-and-ownership.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Generation and Ownership

This page defines the maintenance boundary for the app SDK family. It is intended for SDK
maintainers and contributors, not just downstream consumers.

## Generator-Owned Surface

`generated/server-openapi` is generator-owned output for every supported language:

- TypeScript: `sdkwork-im-sdk-typescript/generated/server-openapi`
- Flutter: `sdkwork-im-sdk-flutter/generated/server-openapi`
- Rust: `sdkwork-im-sdk-rust/generated/server-openapi`

Do not hand-edit files in these directories to fix product behavior.

## Manual-Owned Surface

`composed` is the manual-owned SDK authoring layer for every non-TypeScript language and the
pre-assembly authoring layer for TypeScript:

- TypeScript public package: `@sdkwork/im-sdk`
- Flutter composed package: `im_sdk_composed`
- Rust composed crate: `im-sdk`

This layer owns:

- business-facing semantic clients such as `ImSdkClient`
- semantic modules
- builder helpers
- public consumer documentation and examples

## Regeneration Rule

When the transport contract or generated transport surface needs to change:

1. change the authority contract or generator inputs
2. regenerate the affected language workspaces
3. re-run the workspace verification flow
4. update manual docs only where public behavior or examples changed

Do not patch generated files by hand.

## Source-Of-Truth Hierarchy

1. `openapi/sdkwork-im-im.openapi.yaml`
2. derived sdkgen inputs under `openapi/`
3. root generation and verification wrappers under `bin/`
4. generated transport output under `generated/server-openapi`
5. manual composition under `composed`

## Documentation Rule

Every SDK page in this site should make it obvious whether it is documenting:

- generated transport behavior
- manual convenience layers
- or a transport contract that is documented but not wrapped manually in this round

## Related Pages

- [App SDK](/sdk/app-sdk)
- [Language Support](/sdk/language-support)
- [Auth and Client Init](/sdk/auth-and-client-init)

