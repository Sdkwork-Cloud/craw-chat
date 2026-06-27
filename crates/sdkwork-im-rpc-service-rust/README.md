# sdkwork-im-rpc-service-rust

Domain: communication
Capability: im
Package type: rust-crate
Status: standardizing

This README is the SDKWork module entrypoint for `sdkwork-im-rpc-service-rust`. The
machine-readable component contract is `specs/component.spec.json`; canonical standards are
under `../../../sdkwork-specs/`.

## Purpose

Rust server-side binding scaffold for the `sdkwork-im-rpc-sdk` family. It wires the generated
`tonic`/`prost` RPC service surfaces (from `sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-rust`)
to runtime dispatch adapters: unary and server-streaming method dispatch, deadline/metadata
extraction, gRPC health reflection, and a method/service manifest that keeps the live service
bindings aligned with the generated SDK contract.

Implements the Rust RPC profile from `sdkwork-specs/RUST_RPC_SPEC.md` and the service catalog
rules from `sdkwork-specs/RPC_SPEC.md`.

## Public API

- `ImRpcServerConfig`, `ImRpcClientConfig` — typed RPC bootstrap configuration.
- `ImRpcRuntimeDispatcher`, `dispatch_unary_rpc`, `dispatch_server_stream_rpc` — runtime
  method dispatch.
- `RpcServiceBinding`, `RpcMethodBinding`, `bind_all_rpc_services`, `bind_all_rpc_methods` —
  service and method binding surface.
- `ImRpcHealthService`, `build_im_rpc_health_server` — gRPC health reflection.
- `RpcMetadata` and the `METADATA_*` constants — request metadata extraction.
- `ImRpcError`, `map_rpc_error_to_status` — error mapping at the RPC boundary.
- `RPC_METHOD_BINDINGS`, `RPC_SERVICE_BINDINGS`, `RPC_SDK_FAMILY` — manifests that keep live
  bindings aligned with the generated SDK family.

## Required SDK Surface

- `sdkwork-im-rpc-sdk-rust` (generated RPC SDK family, sibling path).

## Configuration

Configuration keys, runtime entrypoints, and integration contracts are declared in
`specs/component.spec.json`. RPC services must receive configuration through typed bootstrap
(`ImRpcServerConfig`/`ImRpcClientConfig`) or the service-host boundary rather than reading
host-local environment state directly.

## SaaS/Private/Local Behavior

This component follows the deployment and runtime rules referenced by its `canonicalSpecs`
entries. SaaS, private, and local behavior must stay compatible with the relevant SDKWork
specs before implementation changes are made.

## Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this
module. Access tokens flow only through the approved `RpcMetadata` extraction boundary
(`METADATA_ACCESS_TOKEN`, `METADATA_AUTHORIZATION`) and are validated by the protected API
layer, not by this binding scaffold.

## Extension Points

Extension points are limited to public exports, runtime entrypoints, the method/service
manifest, and config keys declared in `specs/component.spec.json`. Do not hand-edit generated
tonic/prost output; change the proto source contract and regenerate the sibling SDK family.

## Verification

- `cargo test -p sdkwork-im-rpc-service-rust`
- `cargo clippy -p sdkwork-im-rpc-service-rust --tests -- -D warnings`

## Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`. Update that contract
before changing public integration behavior.
