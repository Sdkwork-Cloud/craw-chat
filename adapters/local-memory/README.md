# local-memory

`local-memory` is the default in-memory adapter package used by `standalone.split-services.development`.

## What It Provides

- `MemoryCommitJournal`
- `MemoryMetadataStore`
- `MemoryTimelineProjectionStore`
- `MemoryRealtimeCheckpointStore`
- `MemoryRealtimeDisconnectFenceStore`
- `MemoryRealtimeSubscriptionStore`
- `MemoryStreamStateStore`
- `MemoryNotificationTaskStore`
- `MemoryAutomationExecutionStore`
- `MemoryPresenceStateStore`
- `MemoryStorageDomainSnapshotStore`

## Storage Snapshot Store

`MemoryStorageDomainSnapshotStore` is the in-memory reference adapter for the generic storage
management module.

It stores `StorageDomainSnapshot` values keyed by `snapshot.catalog.domain` and is intended for:

- local testing of storage runtime hydration and save flows
- sandbox and profile baselines before a persistent adapter is wired in
- validating adapter-facing behavior without duplicating provider-specific logic

The semantics are intentionally simple:

- unknown domains return `None`
- saving the same domain overwrites the previous snapshot
- different storage domains remain isolated from each other

## What It Is Not

This crate is not durable storage. It is a fast local adapter for tests, local runtime assembly,
and reference behavior. Production backends still need persistent storage adapters built on the same
`StorageDomainSnapshotStore` contract.

## SDKWork Documentation Contract

Domain: communication
Capability: im
Package type: rust-crate
Status: standardizing

### Public API

Public exports are declared in `specs/component.spec.json` under `contracts.publicExports`.

### Required SDK Surface

- None declared in `specs/component.spec.json`.

### Configuration

Configuration keys and runtime entrypoints are declared in `specs/component.spec.json`.

### SaaS/Private/Local Behavior

This module follows the canonical standards linked from `specs/component.spec.json`, including deployment and runtime configuration rules where applicable.

### Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this module.

### Extension Points

Extension points are limited to declared public exports, runtime entrypoints, SDK clients, events, and config keys.

### Verification

- `cargo test --manifest-path apps/sdkwork-im/adapters/local-memory/Cargo.toml`

### Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`.
