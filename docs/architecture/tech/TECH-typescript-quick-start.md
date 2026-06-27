> Migrated from `docs/sites/sdk/typescript-quick-start.md` on 2026-06-24.
> Owner: SDKWork maintainers

# TypeScript Quick Start

## Audience

Use this page when you are integrating Sdkwork IM from a TypeScript or JavaScript application and
want the preferred public SDK surface.

## Package

- preferred public package: `@sdkwork/im-sdk`
- primary client: `ImSdkClient`
- local workspace path before publication: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript`
- generator-owned authoring boundary:
  `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/generated/server-openapi`
  This boundary is internal-only and not a consumer package.

## Install

If you are consuming from a local repository checkout before public publication, wire the package as
a workspace dependency:

```json
{
  "dependencies": {
    "@sdkwork/im-sdk": "workspace:*"
  }
}
```

## Create a client

```ts
import { ImSdkClient } from "@sdkwork/im-sdk";

const sdk = new ImSdkClient({
  baseUrl: "http://127.0.0.1:18079",
  authToken: process.env.SDKWORK_IM_TOKEN,
});
```

## First read call

```ts
const workspace = await sdk.portal.getWorkspace();
console.log(workspace.name);
```

## First write call

```ts
await sdk.conversations.postText("conv-demo-01", "hello from TypeScript");
```

## Message-first send path

```ts
const message = sdk.createTextMessage({
  conversationId: "conv-demo-01",
  text: "hello from TypeScript",
});

await sdk.send(message);
```

## Generated transport when needed

```ts
const inbox = await sdk.inbox.getInbox();
console.log(inbox.items.length);
```

## Common entrypoints

```ts
// Tokens are issued by sdkwork-appbase and passed into Sdkwork IM.
await sdk.sync.catchUp({ limit: 20 });

const live = await sdk.connect({
  clientRouteId: "device-web-01",
});

live.messages.on((message, context) => {
  console.log(message.type, context.sequence);
});
```

## Next Steps

- [Auth and Client Init](/sdk/auth-and-client-init)
- [Module Map](/sdk/module-map)
- [Messages Module](/sdk/modules/messages)
- [Message and Media](/sdk/examples/message-and-media)

