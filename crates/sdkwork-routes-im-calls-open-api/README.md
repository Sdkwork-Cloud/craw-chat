# sdkwork-routes-im-calls-open-api

## Purpose

HTTP route crate for SDKWork IM call signaling at `/im/v3/api/calls`.

## Owner

SDKWork IM maintainers.

## Allowed Content

- Path constants, route manifest metadata, Axum route mounting, and IM web-framework wrapping

## Forbidden Content

- RTC media runtime or provider SDK wiring outside `im-calls-service`

## Related Specs

- `../../sdkwork-specs/WEB_BACKEND_SPEC.md`
- `../../sdkwork-specs/API_SPEC.md`

## Verification

```bash
cargo check -p sdkwork-routes-im-calls-open-api
node scripts/dev/sdkwork-im-web-backend-standard.test.mjs
```
