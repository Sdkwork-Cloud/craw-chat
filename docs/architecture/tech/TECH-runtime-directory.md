> Migrated from `docs/sites/reference/runtime-directory.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Runtime Directory

Normalized IM business state, replay checkpoints, subscriptions, presence, stream state,
notifications, automation, and projections use their declared PostgreSQL-backed adapters. A local
runtime directory is not a second persistence authority.

## Development default

Server-only development uses topology v5 source profiles. Build output and caches stay in native
tool directories; process coordination and disposable generated configuration stay in private,
repository-keyed OS/CI temporary directories outside the source checkout.

## Packaged server

Production installs use paths declared in `deployments/templates/server.env.example`:

- data: `/var/lib/sdkwork/chat`
- logs: `/var/log/sdkwork/chat`
- run: `/run/sdkwork/chat`

See [Server Lifecycle](/deployment/server-lifecycle).
