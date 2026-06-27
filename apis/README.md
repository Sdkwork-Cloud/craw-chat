# APIs

## Purpose

Author-owned API contracts, API examples, changelogs, route materialization inputs, and validation
fixtures for Sdkwork IM.

## Owner

SDKWork Chat maintainers.

## Authority Layout

| Surface | Authority path |
| --- | --- |
| open-api (IM) | `apis/open-api/im/sdkwork-im-im.openapi.yaml` |
| app-api | `apis/app-api/communication/sdkwork-im-app-api.openapi.yaml` |
| backend-api | `apis/backend-api/communication/sdkwork-im-backend-api.openapi.yaml` |
| rpc | `apis/rpc/` (proto contracts; discovery deferred until hosted gRPC ships) |

SDK mirrors under `sdks/*-sdk/openapi/` must match the `apis/` authority byte-for-byte until mirrors are retired.

## Allowed Content

- OpenAPI source contracts for `open-api`, `app-api`, and `backend-api` surfaces.
- RPC/proto contract inputs when promoted from local experimental roots.
- API examples, changelogs, validation fixtures, and route authority inputs.

## Forbidden Content

- Generated SDK language output.
- Generated SDK `.sdkwork/sdkwork-generator-*.json` control-plane files.
- Runtime services, controller implementations, repositories, caches, secrets, or local state.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/API_SPEC.md`
- `../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md`
- `../sdkwork-specs/TEST_SPEC.md`

## Verification

Run `pnpm run test:apis-authority-standard`, `pnpm run test:sdkwork-workspace-structure-standard`, `pnpm run test:web-framework-standard`, and `pnpm run test:database-framework-standard` from the repository root.
