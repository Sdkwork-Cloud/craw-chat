> Migrated from `docs/sites/deployment/production-domain-binding.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Production Domain Binding

Use topology v2 surface keys for every production deployment. Do not use legacy `SDKWORK_IM_SERVER_*`
URL keys or mixed `/sdkwork/chat` mount roots.

## SaaS canonical hosts

| Surface | Host | Env keys |
| --- | --- | --- |
| IM application HTTP/WebSocket | `im.sdkwork.com` | `SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL`, `SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL` |
| Platform API gateway | `api.sdkwork.com` | `SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` |

`chat.sdkwork.com` is reserved for LLM conversational apps, not IM.

## Single-host production example

```env
SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND=0.0.0.0:18080
SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=https://im.sdkwork.com
SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=wss://im.sdkwork.com
SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=https://api.sdkwork.com
SDKWORK_IM_BROWSER_ORIGINS=https://im.sdkwork.com
```

## SDK base URL mapping

| SDK family | Server env | Browser env | SDK path suffix |
| --- | --- | --- | --- |
| IAM / platform modules | `SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` | `VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` | module-specific `/app/v3/api` or service route |
| IM app API | `SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL` | `VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL` | `/app/v3/api` |
| IM realtime | `SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL` | `VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL` | `/im/v3/api/realtime/ws` |

Document base URLs without SDK-owned path suffixes. The generated SDK clients append their own
`/app/v3/api`, `/im/v3/api`, and `/im/v3/api/realtime/ws` segments.

## Release build propagation

`scripts/release/run-sdkwork-im-pc-release-build.mjs` resolves IAM/release env through
`sdkwork-im-iam-env.mjs`, which materializes:

- `VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL`
- `VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL`
- `VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL`

If `SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL` is omitted but
`SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL` is set, the resolver derives WebSocket base URL by
converting `https://` to `wss://` and `http://` to `ws://`.

## Installed server packages

For installed server packages, prefer the explicit `SDKWORK_IM_APPLICATION_PUBLIC_*` and
`SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` values in `/etc/sdkwork/chat/server.env` so generated
server config, logs, docs, and release manifests all point at the same public surface contract.

Copy from `deployments/templates/server.env.example` and adjust hostnames for your deployment profile.

