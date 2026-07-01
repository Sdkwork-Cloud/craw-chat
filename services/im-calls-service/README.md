# im-calls-service

## Purpose

IM-owned call signaling service for RTC session lifecycle and signal relay (`/im/v3/api/calls/*`).

## Owner

SDKWork IM maintainers.

## Allowed Content

- Call session state machines, signal delivery, and HTTP handlers delegated from `sdkwork-routes-im-calls-open-api`

## Forbidden Content

- RTC media runtime paths owned by `sdkwork-rtc`
- Retired interaction-service HTTP surfaces

## Related Specs

- `../../sdkwork-specs/WEB_BACKEND_SPEC.md`
- `../../sdkwork-specs/API_SPEC.md`
- `../sdkwork-rtc/docs/rtc-im-boundary.md`

## Verification

```bash
cargo test -p im-calls-service
```
