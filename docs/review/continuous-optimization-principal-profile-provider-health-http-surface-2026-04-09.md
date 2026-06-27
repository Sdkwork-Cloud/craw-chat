# Continuous Optimization: User-Module Provider Health HTTP Surface

## Context

- `principal-profile` already supported runtime provider selection and unavailable semantics.
- `sdkwork-im-server` already exposed `provider-health` routes for RTC, media, IoT access, and IoT protocol.
- `principal-profile` still had no operator-facing health route.

## Confirmed Bug

- `GET /backend/v3/api/principal_profile/provider_health` returned `404`.
- That hid `principal-profile-external-catalog` unavailable state behind business-route failures instead of giving operators a direct read path.

## Root Cause

- `build.rs` never registered a principal-profile health route.
- `AppState` had no principal-profile health accessor.
- `principal_profile.rs` implemented provider health snapshots but had no HTTP handler.
- Env-driven principal-profile tests also lacked a serialization guard, so parallel execution could leak env state between tests.

## Decision

- Add `/backend/v3/api/principal_profile/provider_health`.
- Reuse `ProviderHealthSnapshot` and the same auth model as other provider-health routes.
- Keep HTTP `200`; expose availability through `status=healthy|unavailable`.
- Add minimal `providerKind` details and preserve external error/config details.
- Serialize env-mutating principal-profile tests with a per-binary async mutex.

## Changed Files

- `services/sdkwork-im-cloud-gateway/src/node.rs`
- `services/sdkwork-im-cloud-gateway/src/node/build.rs`
- `services/sdkwork-im-cloud-gateway/src/node/principal_profile.rs`
- `services/sdkwork-im-cloud-gateway/tests/principal_profile_provider_http_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/principal_profile_provider_runtime_selection_test.rs`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test principal_profile_provider_http_test -- --nocapture
```

- Failed before the patch with `404 != 200`.

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test principal_profile_provider_http_test -- --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline --test principal_profile_provider_runtime_selection_test -- --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Remaining Gap

- Invalid `SDKWORK_IM_PRINCIPAL_PROFILE_PROVIDER` still panics fast.
- Unified provider registry / selected-plugin visibility is still not surfaced through a single ops or control-plane view.
