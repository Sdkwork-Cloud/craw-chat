# IM Web Domain Routing

## Decision

`sdkwork-im/etc/sdkwork.deployment.config.json#environments` is the canonical authority for
the public IM web origin in `development`, `test`, `staging`, and `production`.
The root, PC, and H5 app manifests contain application declarations only. Renderer `etc/` files
reference the root deployment config and must not duplicate a client-specific public hostname
when both clients share adaptive web ingress.

The canonical environment origins are:

| Environment | Public origin |
| --- | --- |
| `development` | `http://im-dev.sdkwork.com:3801/` |
| `test` | `https://im-test.sdkwork.com/` |
| `staging` | `https://im-staging.sdkwork.com/` |
| `production` | `https://im.sdkwork.com/` |

These are frontend application origins. The cross-client API deployment authority is
`specs/im-api-deployment.spec.json`; API base URLs are injected by typed browser runtime
configuration and must not be inferred ad hoc inside shared packages.

Cloud deployments keep the application and API origins distinct:

| Environment | Cloud API base URL |
| --- | --- |
| `development` | `https://api-dev.sdkwork.com/` |
| `test` | `https://api-test.sdkwork.com/` |
| `staging` | `https://api-staging.sdkwork.com/` |
| `production` | `https://api.sdkwork.com/` |

Standalone deployments instead collapse IM open-api, app-api, backend-api, health checks,
and realtime WebSocket traffic onto the published application origin. The adaptive ingress
proxies these service routes before applying User-Agent renderer selection, which keeps a
single application installation independent of the cloud API gateway.

## Routing Contract

Every environment serves the application at `/`. The ingress performs an exact
Host match and selects the renderer from the request User-Agent:

- desktop browser or missing User-Agent: `sdkwork-im-pc`;
- mobile browser: `sdkwork-im-h5`.

The machine-readable policy is
`specs/im-web-ingress-domain.spec.json`. It references the manifest authorities
instead of copying the concrete origins.

## Local Domain Resolution

Binding the development listener to `0.0.0.0` is portable, but it does not create
DNS records. The client device must resolve `im-dev.sdkwork.com` to the machine
running the development ingress.

For a browser on the same development machine, add this hosts entry with
administrator privileges:

```text
127.0.0.1 im-dev.sdkwork.com
```

- Windows hosts file: `C:\Windows\System32\drivers\etc\hosts`
- macOS and Linux hosts file: `/etc/hosts`

For a physical phone or another computer, configure the LAN DNS server/router to
resolve `im-dev.sdkwork.com` to the development machine's private LAN address.
Changing only the development machine's hosts file does not affect other devices.
Firewall policy must allow inbound TCP `3801` for LAN testing.

## Verification

```bash
pnpm test:web-domain-routing-standard
```
