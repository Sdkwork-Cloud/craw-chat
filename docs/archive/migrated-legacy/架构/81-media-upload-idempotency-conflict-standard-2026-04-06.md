# 81. Media Upload Idempotency and Conflict Standard (2026-04-06)

## 1. Goal

Media asset creation and completion must follow the same retry discipline as the rest of the platform:

- identical retries are idempotent
- conflicting retries are rejected

This protects asset identity stability and prevents silent storage-target rewrites.

## 2. Asset Identity Rule

`mediaAssetId` identifies a single owner-scoped media asset under:

- `tenantId`
- `principalId`
- `principalKind`

Once an owner has created that asset id, later requests must not silently reinterpret what that id means.

## 3. Create Upload Rule

For `POST /im/v3/api/media/uploads`:

### 3.1 Same owner, identical create payload

If the same owner retries the same `mediaAssetId` with the same `resource`, the service must return the existing asset idempotently.

### 3.2 Same owner, conflicting create payload

If the same owner retries the same `mediaAssetId` with a different `resource`, the service must reject the request with:

- HTTP `409`
- code `media_asset_conflict`

### 3.3 Different owner, same asset id

Cross-principal collision remains a distinct rule:

- HTTP `409`
- code `media_asset_already_exists`

This preserves ownership isolation semantics while distinguishing it from same-owner request conflict.

## 4. Complete Upload Rule

For `POST /im/v3/api/media/uploads/{mediaAssetId}/complete`:

### 4.1 Pending asset

If the asset is still pending upload, completion may finalize:

- `bucket`
- `objectKey`
- `storageProvider`
- `url`
- `checksum`

### 4.2 Ready asset, identical completion payload

If the asset is already `ready` and the repeated completion request matches the stored finalized state, the service must return the existing asset idempotently.

### 4.3 Ready asset, conflicting completion payload

If the asset is already `ready` and the new completion request differs from the stored finalized state, the service must reject the request with:

- HTTP `409`
- code `media_asset_conflict`

The service must not silently overwrite finalized storage metadata for a ready asset.

## 5. Storage Provider Normalization

For conflict comparison:

- omitted `storageProvider` is normalized to `"local"`

This ensures the same logical completion request is treated consistently whether the caller sends the explicit default or not.

## 6. Relationship To Ownership Isolation

This standard complements the ownership isolation rule:

- ownership determines who may read or finalize an asset
- idempotency/conflict determines whether a repeated owner request is the same command or a contradictory command

Both rules are required for a stable commercial contract.

## 7. Verification Standard

`media-service` must keep regression coverage for:

1. same-owner conflicting duplicate create -> `409 media_asset_conflict`
2. same-owner conflicting duplicate complete -> `409 media_asset_conflict`
3. cross-principal same-id create collision -> `409 media_asset_already_exists`
4. cross-principal read/complete -> not found semantics

## 8. Design Consequence

After an asset id is created, callers may safely retry without creating ambiguity.

At the same time, any client bug, replay drift, or mismatched storage callback that attempts to redefine the asset is surfaced immediately as an explicit conflict instead of mutating persisted truth.
