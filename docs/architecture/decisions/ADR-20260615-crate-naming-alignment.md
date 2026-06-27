# ADR-20260615-crate-naming-alignment

Status: superseded by ADR-20260615-sdkwork-im-to-sdkwork-im-rebrand
Owner: SDKWork IM maintainers
Date: 2026-06-15
Specs: NAMING_SPEC.md, RUST_CODE_SPEC.md, ARCHITECTURE_DECISION_SPEC.md, GOVERNANCE_SPEC.md, MIGRATION_SPEC.md

> **Superseded.** This ADR chose `<app> = chat` as the canonical app token.
> ADR-20260615-sdkwork-im-to-sdkwork-im-rebrand superseded that choice by
> adopting `<app> = im` everywhere (product and app token), renaming the
> breaking contract IDs (Postgres database/role/binary, WebSocket subprotocol,
> URN namespace), and renaming the `sdkwork_im_*` / `SDKWORK_CHAT_*` environment
> variables to `SDKWORK_IM_*`. The migration-batch structure below is retained
> for history; the accepted target naming is in the superseding ADR.

## Context

`sdkwork-im` is the SDKWork Chat instant-messaging backend. Its `specs/component.spec.json`
declares `component.domain = "communication"`, `component.capability = "chat"`, and the
product/app key is `chat`. The IM domain vocabulary across the codebase is `im`
(every `im-*` crate, the `im_` database table prefix in
`specs/database-prefix-registry.json`, and the `im` capability references in
`specs/im-app-api-sdk-integration.spec.md`).

The Rust workspace currently names its 55 crates with three non-standard prefixes:

1. `sdkwork-im-*` — the legacy product codename (contract, codec, runtime-link, gateway
   helpers, openapi, api-registry).
2. `im-*` — a domain prefix missing the SDKWork owner token (domain-core, domain-events,
   platform-contracts, storage-*, app-context, time, adapters-*).
3. Bare responsibility names for services and adapters (`conversation-runtime`,
   `session-gateway`, `audit-service`, `local-disk`, etc.).

`sdkwork-specs/README.md` §4 "Minimum Rules" and `NAMING_SPEC.md` require Rust crates to be
named by responsibility under the SDKWork owner token:

- Business logic: `sdkwork-<domain>-<capability>-service`
- SQLx/repository access: `sdkwork-<domain>-<capability>-repository-sqlx`
- HTTP route adapters: `sdkwork-routes-<capability>-<surface>`
- HTTP server processes: `sdkwork-<app>-standalone-gateway`
- In-process service containers: `sdkwork-<app>-service-host`
- Background jobs: `sdkwork-<domain>-<capability>-worker`
- Gateway/proxy: `sdkwork-<app>-gateway`

Generic suffixes are explicitly forbidden: `-product`, `-runtime`, `-backend`, `-core`,
`-common`, `-manager`. Six crates currently use a forbidden suffix:

| Current name | Forbidden suffix |
| --- | --- |
| `sdkwork-im-contract-core` | `-core` |
| `sdkwork-im-ccp-core` | `-core` |
| `im-domain-core` | `-core` |
| `im-storage-runtime` | `-runtime` |
| `sdkwork-api-product-runtime` | `-product`, `-runtime` |
| `conversation-runtime` | `-runtime` |

This is a public-naming change. Per `ARCHITECTURE_DECISION_SPEC.md` §5 it requires human
review, and per `MIGRATION_SPEC.md` it needs a compatibility window because the crate names
are referenced by: workspace `Cargo.toml` members and `[workspace.dependencies]`, every
dependent crate's `Cargo.toml`, every `use <crate>::` / `extern crate` in Rust source,
`sdks/sdkwork-im-rpc-sdk` family inputs, `apps/sdkwork-chat-pc` build wiring, release
planning scripts under `scripts/release/`, and the RPC SDK family manifest.

## Decision

Adopt the SDKWork-compliant crate vocabulary in batches, with this ADR as the migration
authority. The canonical `<domain>` is `im` and the canonical `<app>` is `chat`.

### Target naming

| Layer | Pattern | Examples (current → target) |
| --- | --- | --- |
| IM contracts | `sdkwork-im-<capability>-contract` | `sdkwork-im-contract-message` → `sdkwork-im-message-contract`; drop forbidden `-core` from `sdkwork-im-contract-core` → `sdkwork-im-contract` |
| IM domain model | `sdkwork-im-domain` / `sdkwork-im-domain-events` | `im-domain-core` → `sdkwork-im-domain`; `im-domain-events` → `sdkwork-im-domain-events` |
| CCP transport | `sdkwork-im-ccp-*` | `sdkwork-im-ccp-core` → `sdkwork-im-ccp`; `sdkwork-im-ccp-binding-ws` → `sdkwork-im-ccp-binding-ws` |
| Storage | `sdkwork-im-storage-*` | `im-storage-contracts` → `sdkwork-im-storage-contract`; `im-storage-runtime` → `sdkwork-im-storage-service-host` |
| Repository (DB) | `sdkwork-im-<capability>-repository-<tech>` | `im-adapters-postgres-realtime` → `sdkwork-im-realtime-repository-postgres` |
| HTTP services | `sdkwork-im-<capability>-service` | `conversation-runtime` → `sdkwork-im-conversation-service`; `projection-service` → `sdkwork-im-projection-service` |
| Gateway | `sdkwork-<app>-gateway` | `session-gateway` → `sdkwork-chat-session-gateway`; `web-gateway` → `sdkwork-chat-web-gateway` |
| Service host | `sdkwork-<app>-service-host` | `sdkwork-im-server` (retired unified node) -> `sdkwork-im-service-host` |
| App config | `sdkwork-chat-*` | `sdkwork-im-cloud-gateway-config` → `sdkwork-chat-gateway-config` |

### Migration batches

Each batch is independently shippable and verified before the next starts.

1. **Batch A — forbidden-suffix removal (6 crates).** Highest priority: these violate an
   explicit MUST NOT. Rename only the six forbidden-suffix crates, drop the suffix, and fix
   all references. Directory names may stay during this batch to limit churn.
2. **Batch B — IM contracts and domain (leaves).** The `sdkwork-im-contract-*` and
   `im-domain-*` crates have the fewest inbound runtime dependencies and are the natural
   leaves of the dependency graph.
3. **Batch C — CCP codec/binding/registry layer.** Self-contained transport family.
4. **Batch D — IM services and storage runtime.** `*-service` crates and
   `sdkwork-im-storage-service-host`.
5. **Batch E — gateways, service host, adapters, tools.** Process crates and infrastructure
   adapters; touch release scripts and deployment templates last.

### Compatibility rule

During the migration window a renamed crate MAY keep a one-release `[package] name =` alias
plus a `pub use` re-export shim under the old name, so downstream consumers (the RPC SDK
family, `apps/sdkwork-chat-pc`, sibling repositories) upgrade without a flag day. The alias
is removed once all consumers are updated, tracked as a follow-up under this ADR.

## Alternatives

- **Register a governance exception and keep `sdkwork-im-*`/`im-*`.** Rejected: the prefixes
  also omit the SDKWork owner token, so the names are not just stylistically off — they do
  not encode ownership at all, and the forbidden generic suffixes are a hard MUST NOT.
- **One-shot rename of all 55 crates.** Rejected: the blast radius (workspace deps, RPC SDK
  family, app build wiring, release scripts, deployment templates) is too large to verify in
  a single change; a failed partial rename would leave the workspace in a non-building state.
- **Rename directories too.** Deferred: rename `[package].name` and Rust identifiers first;
  directory renames fold into the final batch once references are stable, to keep diffs
  reviewable.

## Consequences

- **Benefits:** crate ownership and responsibility become self-describing; the forbidden
  generic suffixes stop contradicting `NAMING_SPEC.md`; new contributors can map a crate to
  its SDKWork role without reading its README; static scans in `TEST_SPEC.md` that key off
  the `sdkwork-` vocabulary start matching.
- **Costs:** a multi-batch migration with a compatibility window; every batch must re-run
  `cargo check --workspace`, the affected crate tests, the RPC SDK family verifier
  (`sdks/sdkwork-im-rpc-sdk`), and `apps/sdkwork-chat-pc` build; release scripts and
  deployment templates that hard-code crate names need updating in the relevant batch.
- **Risk:** a stale reference (a missed `use`, a release planner, an SDK family manifest)
  breaks the build or a release artifact. Mitigated by per-batch verification and the
  compatibility alias during the window.

## Verification

### Batch-A pilot (verified 2026-06-15)

The `im-storage-runtime` → `sdkwork-im-storage-service-host` rename was applied end-to-end
as the migration-pattern pilot and verified:

- `[package].name` set to `sdkwork-im-storage-service-host`; `[lib].name = "im_storage_runtime"`
  kept as the import-path alias during the window.
- Workspace `Cargo.toml` dependency entry carries `package = "sdkwork-im-storage-service-host"`
  under the legacy `im-storage-runtime` key.
- `cargo check --workspace`, `cargo test -p sdkwork-im-storage-service-host -p sdkwork-api-product-runtime --lib`,
  `cargo clippy ... -- -D warnings`, `sdkwork-chat-component-spec-consistency`, and
  `sdkwork-workspace-structure-standard` all pass.
- Consumer `sdkwork-api-product-runtime` required no source change (alias absorbs it).

This confirms the alias-based migration pattern works; the remaining five forbidden-suffix
crates and the broader prefix migration follow the same mechanism.

Per batch, before the batch is marked accepted:

- `cargo check --workspace` and `cargo clippy --workspace --tests -- -D warnings`.
- `cargo test -p <each-renamed-crate>` and `cargo test -p <each-direct-dependent>`.
- `node scripts/dev/sdkwork-chat-component-spec-consistency.test.mjs` (README + manifest
  presence still holds).
- For Batch C and E: the RPC SDK family verifier command documented in
  `sdks/README.md`.
- For Batch E: `pnpm test:source-server-deploy` and the release planner dry-run
  (`pnpm release:package:check`) to confirm no stale crate-name reference leaks into a
  release artifact.

## Supersedes / Superseded By

_None._
