# postgres-rtc-state

## Purpose

PostgreSQL adapter for durable RTC call session state used by IM call signaling.

## Owner

SDKWork IM maintainers.

## Allowed Content

- `StateStore` implementation for `im-domain-core::rtc`
- SQL mapping for `im_rtc_sessions` and related RTC lifecycle columns

## Forbidden Content

- HTTP handlers or OpenAPI route definitions
- Direct frontend or generated SDK imports

## Related Specs

- `../../sdkwork-specs/DATABASE_SPEC.md`
- `../../sdkwork-specs/WEB_BACKEND_SPEC.md`

## Verification

```bash
cargo test -p postgres-rtc-state
```
