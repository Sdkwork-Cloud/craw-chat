# @sdkwork/im-h5-core

Domain: communication
Capability: im-h5-core
Package type: node-package

IM H5 **host runtime**: session storage, IM SDK client, app SDK client, drive SDK client, and realtime connection manager for the SDKWork IM H5 mobile browser app.

Machine-readable contract: `specs/component.spec.json`. Canonical standards: `../../../../../sdkwork-specs/`.

## Modules

| Module | Role |
| --- | --- |
| `session` | H5 auth session storage, token manager, and request context |
| `appSdkClient` | IM app SDK client (platform: `h5`) |
| `imSdkClient` | IM SDK client (platform: `h5`) |
| `driveAppSdkClient` | Drive app SDK client for media uploads (platform: `h5`) |
| `chatRealtimeConnection` | Shared WebSocket realtime connection manager with reconnect, circuit breaker, and inbox refresh subscriptions |
