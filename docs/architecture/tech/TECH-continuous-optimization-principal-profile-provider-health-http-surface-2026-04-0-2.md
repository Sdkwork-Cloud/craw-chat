> Migrated from `docs/step/continuous-optimization-principal-profile-provider-health-http-surface-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 12: User-Module Provider Health HTTP Surface

## Goal

- Expose principal-profile provider health directly to operators.

## Loop

1. Reproduce missing route with `GET /backend/v3/api/principal/profiles/provider_health`.
2. Freeze local healthy and external-unavailable contracts in HTTP tests.
3. Add route, handler, and `AppState` accessor.
4. Surface minimal principal-profile provider details in `ProviderHealthSnapshot`.
5. Add env-test serialization to remove parallel leakage.
6. Re-run format and package verification.

## Exit

- local provider health returns `200` + `principal-profile-upstream-context`
- unavailable external provider health returns `200` + `status=unavailable`
- env-driven principal-profile tests are stable inside the same test binary

