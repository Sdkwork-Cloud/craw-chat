# object-storage-s3

通用 `S3-compatible` 对象存储插件基线。

## 覆盖的插件实例

- `object-storage-aliyun`
- `object-storage-tencent`
- `object-storage-volcengine`
- `object-storage-aws`
- `object-storage-google`
- `object-storage-microsoft`

## Design constraints

- Business layer only depends on `ObjectStorageProvider` contract interfaces.
- Provider selection is decided by `ProviderRegistry`.
- URL signing uses `signed_download_url(...)`.
- Google / Microsoft use the `s3-gateway` capability identifier.

## Production upload boundary

This crate is **not** a production file-upload path for Sdkwork IM applications.

- All user-facing and app-surface uploads **must** go through `sdkwork-drive` (`/app/v3/api/drive/*`) via `@sdkwork/drive-app-sdk` or approved server-side Drive Uploader facades per `DRIVE_SPEC.md`.
- `media-service` and chat/community clients **must not** call this adapter for upload lifecycle.
- Allowed uses here: contract tests, provider registry conformance checks, and platform storage plugin conformance under `adapters/object-storage-s3/tests/`.

## SDKWork Documentation Contract

Domain: drive
Capability: im
Package type: rust-crate
Status: standardizing

### Public API

Public exports are declared in `specs/component.spec.json` under `contracts.publicExports`.

### Required SDK Surface

- None declared in `specs/component.spec.json`.

### Configuration

Configuration keys and runtime entrypoints are declared in `specs/component.spec.json`.

### SaaS/Private/Local Behavior

This module follows the canonical standards linked from `specs/component.spec.json`, including deployment and runtime configuration rules where applicable.

### Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this module.

### Extension Points

Extension points are limited to declared public exports, runtime entrypoints, SDK clients, events, and config keys.

### Verification

- `cargo test --manifest-path apps/sdkwork-im/adapters/object-storage-s3/Cargo.toml`

### Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`.
