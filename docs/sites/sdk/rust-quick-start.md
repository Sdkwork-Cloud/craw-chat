# Rust Quick Start

## Audience

Use this page when you are integrating Sdkwork IM from a Rust backend, service, or systems-oriented
application and want the preferred composed crate.

## Crate

- preferred public crate: `im-sdk`
- generated transport crate: `sdkwork-im-sdk-generated`
- workspace path: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed`

## Add the dependency

If you are consuming from a local checkout before public publication, point Cargo at the composed
crate directly:

```toml
[dependencies]
im-sdk = { path = "../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed" }
```

## Create a client

```rust
use im_sdk::ImSdkClient;

let client = ImSdkClient::new_with_base_url("http://127.0.0.1:18079")?;
client.set_auth_token(token);
```

## First read call

```rust
let inbox = client.inbox().list().await?;
```

## First write call

```rust
client
  .conversations()
  .post_text("conv-demo-01", "hello from Rust", Default::default())
  .await?;
```

## Common module entrypoints

```rust
use im_sdk::PostTextOptions;

client.presence().current().await?;
client
  .conversations()
  .post_text("conv-demo-01", "hello from Rust", PostTextOptions::default())
  .await?;
```

## Builder helpers

The crate re-exports builder helpers for common payload creation:

```rust
use im_sdk::{
  PostTextOptions,
  TextFrameOptions,
  build_text_message,
  build_text_stream_frame,
};
```

## Next Steps

- [Auth and Client Init](/sdk/auth-and-client-init)
- [Module Map](/sdk/module-map)
- [Messages Module](/sdk/modules/messages)
- [Streams](/api-reference/im/streams)
- [Calls](/api-reference/im/calls)

