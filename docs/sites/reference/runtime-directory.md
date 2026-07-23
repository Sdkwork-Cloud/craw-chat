# Runtime Directory

The runtime directory is a deployment-owned boundary for process files, diagnostics, logs, and
bounded temporary runtime material. It is not part of the IM business database and must not be used
as the authority for Conversation, Message, Member, ReadCursor, notification, automation, stream,
or RTC business state.

Normalized durable state belongs in PostgreSQL. Production profiles require their durable adapters
and fail closed when those adapters cannot be initialized. The runtime directory does not contain a
second copy of current IM state and is not a source for rebuilding that state.

## Development And Test

Development and test profiles may configure service-specific, single-node file fallbacks under
`.runtime/`. The current social runtime can also use `SDKWORK_IM_RUNTIME_DIR` when explicitly
configured. These facilities support local verification only; they are not production persistence
and must not be promoted into a shared or highly available deployment.

## Packaged Server Paths

Packaged installations retain the deployment paths declared in
`deployments/templates/server.env.example`:

- data: `/var/lib/sdkwork/chat`
- logs: `/var/log/sdkwork/chat`
- run: `/run/sdkwork/chat`

These paths describe filesystem ownership and process layout, not database ownership. See
[Server Lifecycle](/deployment/server-lifecycle).
