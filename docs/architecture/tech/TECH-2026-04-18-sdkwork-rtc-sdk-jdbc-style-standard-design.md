> Migrated from `docs/superpowers/specs/2026-04-18-sdkwork-rtc-sdk-jdbc-style-standard-design.md` on 2026-06-24.
> Owner: SDKWork maintainers

# SDKWork RTC SDK JDBC-Style Standard Design

## Goal

Define the long-term `sdkwork-rtc-sdk` standard for the new Sdkwork IM application as a
provider-pluggable RTC SDK family with these properties:

- one stable, JDBC-inspired driver standard across languages
- one unified provider selection model aligned with the existing provider registry
- one official semantic consumer package per language workspace
- provider adapters that wrap official vendor SDKs instead of re-implementing media engines
- explicit capability negotiation, degradation, and vendor-extension escape hatches
- a root workspace layout and verification model consistent with `sdkwork-im-sdk`

The first executable landing is a TypeScript baseline that proves the standard through:

- a root workspace contract
- a structural verifier
- a typed RTC SPI
- a driver manager and data source model
- built-in provider adapters for `volcengine`, `aliyun`, and `tencent`

## Non-Goals

- re-implement vendor media engines, transports, or device pipelines
- force every vendor-specific advanced capability into one flat common API
- pretend all languages can ship full media runtime bridges on day one
- make OpenAPI generation the center of the RTC SDK architecture
- preserve compatibility with any older RTC SDK shape

## Canonical Inputs

- app SDK workspace reference: `sdks/sdkwork-im-sdk`
- SDK workspace overview: `sdks/README.md`
- current provider contract: `crates/im-platform-contracts/src/provider.rs`
- current RTC runtime provider selection: `services/im-call-runtime/src/lib.rs`
- existing built-in RTC adapters:
  - `adapters/rtc-volcengine`
  - `adapters/rtc-aliyun`
  - `adapters/rtc-tencent`

## Problem Statement

RTC integration cannot be standardized safely by hiding all provider differences behind an oversized
"universal client" API.

Three specific problems must be solved together:

1. Entry and selection must be standardized.
   Business code should not scatter provider-specific bootstrapping, package names, conditional
   imports, or `if provider == ...` branching across the application.
2. Capability differences must stay explicit.
   Recording, cloud mix, screen share, relay, data channel, spatial audio, and AI enhancements are
   not uniformly supported. The SDK must expose these differences as capability metadata rather than
   accidental runtime failures.
3. Vendor extensions must stay available without polluting the core standard.
   A strict common-denominator API is too weak, but a giant merged interface becomes unmaintainable.

The correct standard is therefore JDBC-inspired rather than "universal engine" inspired:

- standardize the connection and driver model
- standardize metadata and capabilities
- standardize error and selection behavior
- preserve provider escape hatches through `unwrap()`

## Chosen Direction

Adopt a JDBC-style architecture with six layers:

1. `core`
   Stable provider-neutral contracts, models, and errors.
2. `spi`
   The official driver and adapter extension points.
3. `driver-manager`
   The registry, selection, and discovery layer, equivalent to `DriverManager`.
4. `provider-adapter`
   One manual adapter package per vendor, wrapping the official vendor SDK.
5. `control`
   Provider selection, credential, session, recording-artifact, and health integration for server and
   control-plane usage.
6. `runtime-bridge`
   Platform-specific runtime glue for languages or platforms that actually have official vendor SDKs.

This lets the SDK do three things well:

- unify provider access
- make capability and degradation explicit
- keep advanced vendor-specific power accessible through extension points

## JDBC Mapping

The standard should intentionally mirror proven JDBC patterns.

| JDBC | RTC standard |
| --- | --- |
| `Driver` | `RtcProviderDriver` |
| `DriverManager` | `RtcDriverManager` |
| `DataSource` | `RtcDataSource` |
| `Connection` | `RtcClient` |
| `DatabaseMetaData` | `RtcProviderMetadata` |
| `SQLException` | `RtcSdkException` |
| `unwrap()` | `unwrap()` |

Important difference:

RTC requires a first-class capability model, so the JDBC analogy must be extended with:

- `RtcCapabilitySet`
- provider degradation rules
- provider extension keys

## Standard Package Boundary

The workspace boundary must stay consistent with the existing SDK families:

- root workspace owns standards, assembly metadata, docs, and verification
- one language workspace per official language
- one official semantic consumer package per language workspace
- generated or third-party transport/runtime code must stay behind manual workspace-owned boundaries
- business code imports the semantic package, not vendor-private source paths

Unlike `sdkwork-im-sdk`, RTC is not OpenAPI-generation-first. The key boundary here is not
`generated/server-openapi` versus `composed`; it is:

- `core`
- `spi`
- `provider-*`
- `composed` or root semantic surface

## Workspace Layout Standard

The root workspace should converge on this structure:

```text
sdkwork-rtc-sdk/
  README.md
  .sdkwork-assembly.json
  docs/
    README.md
    package-standards.md
    provider-adapter-standard.md
    multilanguage-capability-matrix.md
    verification-matrix.md
  bin/
    verify-sdk.mjs
    verify-sdk.ps1
    verify-sdk.sh
  sdkwork-rtc-sdk-typescript/
  sdkwork-rtc-sdk-flutter/
  sdkwork-rtc-sdk-rust/
  sdkwork-rtc-sdk-java/
  sdkwork-rtc-sdk-csharp/
  sdkwork-rtc-sdk-swift/
  sdkwork-rtc-sdk-kotlin/
  sdkwork-rtc-sdk-go/
  sdkwork-rtc-sdk-python/
```

### TypeScript Standard

The TypeScript workspace is the executable reference implementation.

```text
sdkwork-rtc-sdk-typescript/
  package.json
  tsconfig.build.json
  README.md
  bin/
    package-task.mjs
  src/
    index.ts
    errors.ts
    types.ts
    capabilities.ts
    driver.ts
    driver-manager.ts
    data-source.ts
    client.ts
    providers/
      volcengine.ts
      aliyun.ts
      tencent.ts
  test/
    driver-manager.test.mjs
    data-source.test.mjs
    built-in-providers.test.mjs
```

Rules:

- `src/**` outside `src/providers/**` is provider-neutral
- `src/providers/**` may depend only on `src/**` and official vendor SDK entrypoints
- the semantic package is `@sdkwork/rtc-sdk`
- business integrations import only from `@sdkwork/rtc-sdk`

### Other Languages

The official language family remains:

- `typescript`
- `flutter`
- `rust`
- `java`
- `csharp`
- `swift`
- `kotlin`
- `go`
- `python`

In the first landing, TypeScript is executable and the rest are standard-governed workspace
materializations with README-level contract guidance and reserved semantic boundaries.

## Package Naming Standard

The root package names should follow the same business-derived naming rules already used by the
existing SDK families.

Recommended public package names:

- TypeScript: `@sdkwork/rtc-sdk`
- Flutter: `rtc_sdk`
- Rust: `rtc_sdk`
- Java: `com.sdkwork:rtc-sdk`
- C#: `Sdkwork.Rtc.Sdk`
- Swift: `RtcSdk`
- Kotlin: `com.sdkwork:rtc-sdk`
- Go: `github.com/sdkwork/rtc-sdk`
- Python: `sdkwork-rtc-sdk`

Provider adapter package names should be explicit and one-provider-only.

TypeScript examples:

- `@sdkwork/rtc-sdk-provider-volcengine`
- `@sdkwork/rtc-sdk-provider-aliyun`
- `@sdkwork/rtc-sdk-provider-tencent`
- `@sdkwork/rtc-sdk-provider-agora`
- `@sdkwork/rtc-sdk-provider-zego`
- `@sdkwork/rtc-sdk-provider-livekit`

No "fat all providers" package is allowed.

## Core Contract Standard

The provider-neutral standard must freeze these types and interfaces:

- `RtcProviderId`
- `RtcProviderDescriptor`
- `RtcProviderMetadata`
- `RtcCapabilitySet`
- `RtcDataSourceConfig`
- `RtcClientConfig`
- `RtcSessionDescriptor`
- `RtcJoinOptions`
- `RtcParticipant`
- `RtcTrackPublication`
- `RtcSdkException`
- `RtcProviderDriver`
- `RtcClient`
- `RtcDataSource`

The stable minimum behavior surface is:

- driver registration and selection
- explicit provider metadata lookup
- provider capability lookup
- client creation through a driver or data source
- `unwrap()` for vendor-native client access

The stable minimum runtime surface is:

- `join`
- `leave`
- `publish`
- `unpublish`
- `muteAudio`
- `muteVideo`
- `capabilities`
- `metadata`
- `unwrap`

## SPI Standard

The SPI must stay intentionally narrow.

Required SPI contracts:

- `RtcProviderDriver`
- `RtcClientFactory`
- `RtcRuntimeAdapter`

Provider adapters must not expose vendor-private types in the core contract.

Provider adapters may expose vendor-specific advanced helpers only through:

- `unwrap()`
- provider-specific extension objects
- provider capability declarations

## Capability Standard

RTC capability differences must be modeled explicitly.

### Required capabilities

Every official provider adapter must declare support state for:

- `session`
- `join`
- `publish`
- `subscribe`
- `mute`
- `basic-events`
- `health`
- `unwrap`

### Optional capabilities

Optional capabilities must be declared individually:

- `screen-share`
- `recording`
- `cloud-mix`
- `cdn-relay`
- `data-channel`
- `transcription`
- `beauty`
- `spatial-audio`
- `e2ee`

### Degradation Rule

If a provider does not support a capability:

- the capability must be absent or explicitly false in metadata
- the core contract must not silently emulate it
- the provider may expose a richer extension path, but the core API must not pretend support exists

## Provider Selection Standard

`sdkwork-rtc-sdk` must align with the existing platform provider registry model rather than invent a
parallel selection system.

Selection priority:

1. explicit provider URL
2. explicit provider key
3. tenant override
4. deployment profile
5. workspace default

The workspace default remains `volcengine`.

Provider identifiers must stay normalized:

- provider key example: `volcengine`
- plugin id example: `rtc-volcengine`
- driver id example: `sdkwork-rtc-driver-volcengine`

## Supported Provider Matrix

### Tier A

Tier A providers are the standard baseline:

- `volcengine`
- `aliyun`
- `tencent`

They are the only providers that must ship as built-in adapters in the first landing because they
already align with the current platform runtime contracts.

### Tier B

Tier B providers are official extension targets with reserved adapter positions:

- `agora`
- `zego`
- `livekit`
- `twilio`
- `jitsi`

### Tier C

Tier C providers are SPI-level future targets:

- `janus`
- `mediasoup`

## Multi-Language Capability Standard

Language support must distinguish control capabilities from runtime bridge capabilities.

### Control SDK

Every official language workspace may ship:

- provider descriptors
- capability metadata
- driver registration
- data-source configuration
- control-plane alignment helpers

### Runtime bridge

A language workspace may ship a runtime bridge only when the target provider has an official and
stable SDK for that platform.

The standard must never claim runtime support where no official vendor SDK exists.

## Error Standard

`RtcSdkException` must normalize:

- stable SDK error code
- message
- provider key
- provider plugin id when known
- cause
- details

Required stable code families:

- `driver_already_registered`
- `driver_not_found`
- `provider_not_supported`
- `provider_selection_failed`
- `capability_not_supported`
- `invalid_provider_url`
- `native_sdk_not_available`
- `vendor_error`

## Verification Standard

The root workspace must verify:

- required docs exist
- `.sdkwork-assembly.json` remains aligned with materialized language workspaces
- TypeScript package metadata and public exports remain stable
- built-in provider adapters exist for `volcengine`, `aliyun`, and `tencent`
- docs and assembly stay aligned on provider and language matrices

The first landing does not require live vendor SDK integration tests.
It does require structural tests that prove:

- the SPI works
- driver selection works
- capability and metadata propagation work
- built-in provider descriptors stay aligned with the docs and assembly snapshot

## Implementation Sequence

The first implementation should land in this order:

1. root workspace docs, README, and assembly metadata
2. root verification scripts and contract tests
3. TypeScript core contract and driver manager
4. TypeScript data source and built-in provider adapters
5. multi-language skeleton workspaces and docs alignment

This keeps the standard system real from the start without pretending full multi-provider runtime
parity before the core contract is stable.

