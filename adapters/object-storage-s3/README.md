# object-storage-s3

通用 `S3-compatible` 对象存储插件基线。

## 覆盖的插件实例

- `object-storage-aliyun`
- `object-storage-tencent`
- `object-storage-volcengine`
- `object-storage-aws`
- `object-storage-google`
- `object-storage-microsoft`

## 设计约束

- 业务层只依赖 `ObjectStorageProvider`。
- provider 选择由 `ProviderRegistry` 决定。
- URL 签发统一走 `signed_download_url(...)`。
- Google / Microsoft 使用 `s3-gateway` 能力标识。

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
