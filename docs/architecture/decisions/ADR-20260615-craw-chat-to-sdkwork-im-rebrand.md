# ADR-20260615-sdkwork-im-to-sdkwork-im-rebrand

Status: accepted
Owner: SDKWork IM maintainers
Date: 2026-06-15
Supersedes: ADR-20260615-crate-naming-alignment
Specs: NAMING_SPEC.md, RUST_CODE_SPEC.md, ARCHITECTURE_DECISION_SPEC.md, GOVERNANCE_SPEC.md, MIGRATION_SPEC.md, ENVIRONMENT_SPEC.md, DEPLOYMENT_SPEC.md

## Context

This repository was historically branded `sdkwork-im` / `Sdkwork IM`, with the PC
client workspace at `apps/sdkwork-chat-pc` and client packages scoped as
`@sdkwork/clawchat-*`. The prior ADR-20260615-crate-naming-alignment began the
crate-naming cleanup but chose `<app> = chat` as the canonical app token, leaving
the product still identifiable as "chat" rather than the IM product token `im`.

`NAMING_SPEC.md` §2 requires a single canonical product token. The IM domain
vocabulary across the codebase is `im` (every `im-*` crate, the `im_` database
prefix, the `im` capability). Continuing to brand the repository `sdkwork-im`
while its domain is `im` produced a mixed identity that contradicted the naming
standard and made ownership non-self-describing.

This is a public-naming and breaking-contract change. Per
`ARCHITECTURE_DECISION_SPEC.md` §5 it requires human review, and per
`MIGRATION_SPEC.md` it needs a migration record because the renamed identifiers
are referenced across the workspace, the PC app, release scripts, deployment
templates, and persisted/runtime contracts.

## Decision

Adopt `sdkwork-im` as the canonical product **and** app token, everywhere.

### Naming targets

| Layer | Current | Target |
| --- | --- | --- |
| Repository root / package.json name | `sdkwork-im` | `sdkwork-im` |
| App key (sdkwork.app.config.json) | `chat` | `im` |
| Display name | `SDKWork Chat` / `Sdkwork IM` | `Sdkwork IM` |
| PC app directory | `apps/sdkwork-chat-pc` | `apps/sdkwork-im-pc` |
| PC app package scope | `@sdkwork/chat-pc` | `@sdkwork/im-pc` |
| Client packages | `sdkwork-clawchat-<surface>-<cap>` / `@sdkwork/clawchat-<surface>-<cap>` | `sdkwork-im-<surface>-<cap>` / `@sdkwork/im-<surface>-<cap>` |
| Rust crates | `sdkwork-im-*` | `sdkwork-im-*` |
| CLI tool | `sdkwork-im-cli` (`sdkwork_im_cli`) | `sdkwork-im-cli` (`sdkwork_im_cli`) |

### Breaking contract identifiers (renamed, no compatibility alias)

The following wire-protocol, persistence, and deployment identifiers were
renamed as breaking changes (human-approved). There is **no** compatibility
alias window.

| Identifier | Current | Target |
| --- | --- | --- |
| WebSocket subprotocol | `sdkwork-im.ccp.ws.v1` | `sdkwork-im.ccp.ws.v1` |
| Message schema URN namespace | `urn:sdkwork:sdkwork-im:message:*` | `urn:sdkwork:sdkwork-im:message:*` |
| Default `app_id` | `sdkwork-im` | `sdkwork-im` |
| Postgres database name | `sdkwork_im` | `sdkwork_im` |
| Postgres app role | `sdkwork_im_app` | `sdkwork_im_app` |
| Postgres CI test database/role | `sdkwork_im_test` | `sdkwork_im_test` |
| Server binary | `sdkwork-im-server` | `sdkwork-im-server` |
| Windows install root | `C:\Program Files\SdkworkIm` | `C:\Program Files\SdkworkIm` |
| macOS launchd bundle id | `com.sdkwork.SdkworkIm.server` | `com.sdkwork.im.server` |

### Environment variables

`sdkwork_im_*` and `SDKWORK_CHAT_*` are renamed to `SDKWORK_IM_*` (including the
`VITE_sdkwork_im_*` browser-prefixed variants →`VITE_SDKWORK_IM_*`).

### Deferred: forbidden-suffix removal

Two Rust crates still carry the `-core` suffix forbidden by `NAMING_SPEC.md`
§4.3 (`sdkwork-im-contract-core`, `sdkwork-im-ccp-core`). Their suffix removal
is a follow-up under this ADR, tracked separately, because it changes the crate
identifier shape and touches every `use` site.

## Migration execution

Executed as a single coordinated change (no alias window per the breaking-ID
decision). All directory renames used `git mv` to preserve history. All
in-file identifier renames used byte-fidelity tools to avoid encoding
corruption of non-ASCII content.

## Consequences

- **Benefits:** ownership and domain become self-describing; the repository,
  packages, crates, app identity, and contracts all align on the `im` token;
  `NAMING_SPEC.md` §2 single-token rule is satisfied.
- **Costs:** breaking change for any deployment/client using the old Postgres
  database name, WebSocket subprotocol, URN namespace, binary name, or
  environment variables. Existing databases must be migrated/recreated;
  clients must upgrade in lockstep.
- **Risk:** a stale reference missed in the rename breaks a build, release
  artifact, or runtime connection. Mitigated by the comprehensive residual
  scan (10 residual tracked files, all either auto-regenerated lockfiles or
  immutable dated historical records).

## Verification

- Rust: all 24 renamed crate directories resolve; every `path =` dependency in
  every `Cargo.toml` resolves to an existing target; zero residual
  `sdkwork_im_` / `sdkwork-im-` identifiers in `.rs` / `.toml` source.
  (`cargo check --workspace` is additionally blocked only by pre-existing
  cross-repository layout drift in `../sdkwork-rtc` and `../sdkwork-appbase`,
  unrelated to this rename.)
- TS/PC: `apps/sdkwork-im-pc` workspace, all 43 packages renamed, tsconfig and
  vite alias maps updated, enforcing `sdkwork-workspace-structure-standard`
  test updated to the new canonical naming.
- Residual scan: 10 tracked files remain, all categorized as auto-regenerated
  lockfiles (`Cargo.lock`, `pnpm-lock.yaml`, `docs/sites/package-lock.json`)
  or immutable dated historical documents under `docs/superpowers/{plans,specs}`.

## Supersedes / Superseded By

Supersedes: ADR-20260615-crate-naming-alignment.
