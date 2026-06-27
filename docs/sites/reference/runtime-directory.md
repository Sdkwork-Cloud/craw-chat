# Runtime Directory

When `SDKWORK_IM_RUNTIME_DIR` is set, IM services persist replay checkpoints, subscriptions,
presence, streams, RTC state, notifications, automation, and projection snapshots to disk.

## Development default

Server-only development uses profile-specific runtime directories under `.runtime/` when configured
through topology env files. Prefer explicit `SDKWORK_IM_RUNTIME_DIR` in production installs.

## Packaged server

Production installs use paths declared in `deployments/templates/server.env.example`:

- data: `/var/lib/sdkwork/chat`
- logs: `/var/log/sdkwork/chat`
- run: `/run/sdkwork/chat`

See [Server Lifecycle](/deployment/server-lifecycle).
