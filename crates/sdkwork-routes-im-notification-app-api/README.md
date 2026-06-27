# sdkwork-routes-im-notification-app-api

## Purpose

HTTP route crate for SDKWork IM `app-api` surface at `/app/v3/api/notifications`.

## Owner

SDKWork IM maintainers.

## Allowed Content

- Path constants (`paths.rs`)
- Route manifest metadata (`manifest.rs`)
- Axum route mounting (`routes.rs`)
- IM web-framework wrapping (`web_bootstrap.rs`)

## Forbidden Content

- Business logic, persistence, or provider clients
- Raw HTTP credential parsing outside `sdkwork-web-framework`
- Generated SDK imports for the same API authority

## Related Specs

- `../../sdkwork-specs/WEB_BACKEND_SPEC.md`
- `../../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`
- `../../sdkwork-specs/API_SPEC.md`

## Verification

```bash
cargo check -p sdkwork-routes-im-notification-app-api
node scripts/dev/sdkwork-im-web-backend-standard.test.mjs
```
