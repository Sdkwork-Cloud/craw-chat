# sdkwork-routes-im-realtime-open-api

## Purpose

HTTP route crate for SDKWork IM `open-api` surface at `/im/v3/api/realtime`.

## Owner

SDKWork IM maintainers.

## Allowed Content

- Path constants (`paths.rs`)
- Route manifest metadata (`manifest.rs`)
- Axum route mounting (`routes.rs`)
- IM web-framework wrapping for **HTTP** realtime routes only (`web_bootstrap.rs`)
- Websocket upgrade route composition outside the HTTP interceptor pipeline (`lib.rs`)

## Websocket Upgrade Mounting

`/im/v3/api/realtime/ws` is mounted through `session_gateway::build_realtime_websocket_router` and merged
**before** `wrap_http_router*`. Browser clients authenticate with the first `auth.init` frame after upgrade;
wrapping the websocket route with `WebFrameworkLayer` breaks Axum upgrade state and returns HTTP `426`.

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
cargo check -p sdkwork-routes-im-realtime-open-api
node scripts/dev/sdkwork-im-web-backend-standard.test.mjs
```
