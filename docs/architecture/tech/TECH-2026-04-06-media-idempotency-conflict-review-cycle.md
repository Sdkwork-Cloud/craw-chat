> Migrated from `docs/review/2026-04-06-media-idempotency-conflict-review-cycle.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 2026-04-06 Media Upload Idempotency Conflict Review Cycle

## 1. Finding

### 1.1 Medium: `media-service` accepted conflicting duplicate create/complete requests from the same owner

- Affected service:
  - `services/media-service`
- Root cause:
  - `create_upload(...)` returned the existing asset for the same owner without checking whether the repeated `resource` payload still matched
  - `complete_upload(...)` allowed a ready asset to be overwritten by a later request from the same owner with different:
    - `bucket`
    - `objectKey`
    - `storageProvider`
    - `url`
    - `checksum`
- Before the fix:
  - same-owner conflicting duplicate `POST /im/v3/api/media/uploads` returned `200`
  - same-owner conflicting duplicate `POST /im/v3/api/media/uploads/{id}/complete` returned `200`
  - later completion requests could silently rewrite the finalized storage target of an already-ready asset

## 2. Impact

- media asset identity was not stable once a client reused the same `mediaAssetId`
- retry semantics were inconsistent with other services that already distinguish:
  - identical retry -> idempotent success
  - conflicting retry -> `409` conflict
- a caller bug or replay with changed payload could mutate storage metadata unexpectedly

## 3. Reproduction

Regression tests were added first in:

- `services/media-service/tests/media_asset_test.rs`
  - `test_duplicate_create_upload_rejects_conflicting_resource_for_same_owner`
  - `test_duplicate_complete_upload_rejects_conflicting_storage_target`

Red evidence:

- `cargo test -p media-service --offline test_duplicate_create_upload_rejects_conflicting_resource_for_same_owner`
  - failed with actual status `200`
  - expected `409`
- `cargo test -p media-service --offline test_duplicate_complete_upload_rejects_conflicting_storage_target`
  - failed with actual status `200`
  - expected `409`

## 4. Fix Design

The correct boundary is the media asset aggregate itself.

Chosen rule:

1. same owner + same `mediaAssetId` + identical create payload:
   - idempotent success
2. same owner + same `mediaAssetId` + conflicting create payload:
   - reject with `409 media_asset_conflict`
3. same owner + ready asset + identical complete payload:
   - idempotent success
4. same owner + ready asset + conflicting complete payload:
   - reject with `409 media_asset_conflict`
5. cross-principal collision semantics remain unchanged:
   - `409 media_asset_already_exists`

## 5. Implementation

- `services/media-service/src/lib.rs`
  - added `MediaError::conflict(...)`
  - `CreateUploadRequest` and `CompleteUploadRequest` now derive equality traits for request comparison
  - `create_upload(...)` now checks whether the repeated request matches the existing owner-owned asset before returning success
  - `complete_upload(...)` now rejects conflicting re-completion for already-ready assets and only treats identical completion as idempotent
  - added helper functions:
    - `create_upload_matches_existing(...)`
    - `complete_upload_matches_existing(...)`

## 6. Verification

### Red

- `cargo test -p media-service --offline test_duplicate_create_upload_rejects_conflicting_resource_for_same_owner`
  - failed with `200` vs expected `409`
- `cargo test -p media-service --offline test_duplicate_complete_upload_rejects_conflicting_storage_target`
  - failed with `200` vs expected `409`

### Green

- `cargo test -p media-service --offline test_duplicate_create_upload_rejects_conflicting_resource_for_same_owner`
- `cargo test -p media-service --offline test_duplicate_complete_upload_rejects_conflicting_storage_target`

Observed green result:

- both regressions now return `409 media_asset_conflict`

## 7. Remaining Risks

- `media-service` still uses in-memory state in the minimal profile and does not yet implement a durable object-storage commit workflow
- this review wave only fixed request conflict semantics, not storage backend reliability

## 8. Next Wave

1. continue reviewing remaining service contracts for retry/conflict symmetry against:
   - automation
   - notification
   - streaming
   - RTC
2. verify whether media attach orchestration in `sdkwork-im-server` should explicitly document the same finalized-asset immutability rule

