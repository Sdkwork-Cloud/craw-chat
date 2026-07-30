# Runtime Directory

The runtime directory is a deployment-owned boundary for process files, diagnostics, logs, and
bounded temporary runtime material. It is not part of the IM business database and must not be used
as the authority for Conversation, Message, Member, ReadCursor, notification, automation, stream,
or RTC business state.

Normalized durable state belongs in PostgreSQL. Production profiles require their durable adapters
and fail closed when those adapters cannot be initialized. The runtime directory does not contain a
second copy of current IM state and is not a source for rebuilding that state.

## Development And Test

Development and test business state uses the declared PostgreSQL profile. Build output and caches
remain in tool-native ignored directories such as Cargo `target/`, Vite `node_modules/.vite/`, and
Flutter `.dart_tool/`. Process coordination and disposable generated configuration use private,
repository-keyed OS/CI temporary directories and are cleaned by their owning lifecycle.

Repository, application, and nested source-module runtime-state directories are forbidden. Ignore
rules remain as defense against accidental commits, but they do not authorize creating such a
directory.

## Packaged Server Paths

Packaged installations retain the deployment paths declared in
`deployments/templates/server.env.example`:

- data: `/var/lib/sdkwork/chat`
- logs: `/var/log/sdkwork/chat`
- run: `/run/sdkwork/chat`

These paths describe filesystem ownership and process layout, not database ownership. See
[Server Lifecycle](/deployment/server-lifecycle).
