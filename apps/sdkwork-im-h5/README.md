# SDKWork IM H5

Mobile browser application root for SDKWork IM chat.

## Features

- IAM authentication with `platform: "h5"` via `@sdkwork/auth-runtime-pc-react`
- Inbox via `@sdkwork/im-sdk` `conversations.list()`
- Conversation timeline and text send via REST (`listMessages`, `postText`)
  - WebSocket live updates via `@sdkwork/im-sdk` `connect()` on one shared connection; disposed on session reset
  - Inbox refresh via user-scope `events.onScope`
  - Conversation timeline via `messages.onConversation`

## Development

```powershell
pnpm install
pnpm --dir apps/sdkwork-im-h5 run dev
```

Default dev URL: `http://127.0.0.1:3010`

## Verification

```powershell
pnpm --dir apps/sdkwork-im-h5 run lint
pnpm --dir apps/sdkwork-im-h5 run build
pnpm run test:sdkwork-im-h5-architecture-standard
```

## Application identity

- App ID: `sdkwork-im-h5`
- Manifest: `sdkwork.app.config.json`

See [AGENTS.md](./AGENTS.md) for SDKWork agent entrypoint and spec index.
