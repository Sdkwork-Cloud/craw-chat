> Migrated from `docs/sites/architecture/overview.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Architecture Overview

Sdkwork IM is a multi-service Rust workspace, not a single binary with optional extras. The current
documentation is easiest to understand through five architectural lenses:

1. The workspace layout and contract crates
2. Topology v2 connectivity planes (`application.public-ingress` + `platform.api-gateway`)
3. The separate `control-plane-api`
4. The unified `sdkwork-im-cloud-gateway` / `sdkwork-im-server` application ingress
5. The runtime-directory persistence contract and shared storage baseline

## Core Architecture Facts

| Fact | Current implementation |
| --- | --- |
| Application ingress binary | `services/sdkwork-im-cloud-gateway` with `[[bin]] name = "sdkwork-im-server"` |
| Default IM open-platform prefix | `/im/v3/api/*` |
| Default app-development prefix | `/app/v3/api/*` |
| Default backend/operator prefix | `/backend/v3/api/*` |
| Default dev application ingress | `127.0.0.1:18079` (`standalone.split-services.development`) |
| Default dev platform gateway | `127.0.0.1:3900` (sibling `sdkwork-api-cloud-gateway`) |
| Production IM host | `im.sdkwork.com` |
| Production platform gateway | `api.sdkwork.com` |
| Standalone control-plane bind address | `127.0.0.1:18081` |
| Public auth model | SDKWork dual token at appbase boundary; verified AppContext projection inside sdkwork-im |
| Control-plane permissions | `control.read` and `control.write` |

## Application Ingress

`sdkwork-im-server` is the packaged application ingress. In `split-services` layouts it proxies to
internal IM services declared in `specs/topology.spec.json`. In `unified-process` layouts it runs the
assembled runtime in one process for smoke and local verification.

Domains exposed through the ingress include:

- client route recovery, presence, and realtime delivery
- conversation lifecycle, inbox projection, membership, and read state
- messages, media, streams, and RTC
- notifications, automation, audit, and operator diagnostics
- principal-profile, object-storage, RTC, and IoT-related provider health surfaces

Routing is implemented in `services/sdkwork-im-cloud-gateway`.

## Control Plane

`services/control-plane-api` is a distinct governance service. It is responsible for:

- protocol registry snapshots
- protocol governance snapshots
- provider registry and effective binding views
- provider policy preview, commit, diff, history, and rollback
- realtime node drain, activate, and route migration

This surface is implemented in `services/control-plane-api/src/lib.rs` and started by a separate
binary that binds `127.0.0.1:18081` in `services/control-plane-api/src/main.rs`.

## Unified Gateway And Packaged Server

`services/sdkwork-im-cloud-gateway` publishes the packaged application ingress boundary. Its discovery surface
includes `GET /openapi.json`, `GET /openapi/index.json`, and `GET /openapi/runtime-summary.json`,
along with rendered docs and per-service OpenAPI proxies.

## Runtime Directory Is Architectural, Not Auxiliary

When `SDKWORK_IM_RUNTIME_DIR` is set, the app node switches from in-memory defaults to file-backed
stores for replay, realtime checkpoints, subscriptions, presence, streams, RTC, notifications,
automation, and projection snapshots.

That means the runtime directory is part of the runtime contract, not just a convenience folder for
logs.

## Storage Management Is Now A Shared Module Baseline

Storage configuration management is no longer treated as app-specific admin glue. The current
repository state already includes:

- `im-storage-contracts` for provider schema, typed input payloads, secret redaction, effective
  resolution, and store contracts
- `im-storage-runtime` for validation, save and delete orchestration, audit capture, and
  snapshot-backed hydration
- compatibility re-exports, admin sandbox wiring, and a standalone admin storage module that consume
  the shared storage model

The architectural implication is that tenant/global storage behavior, provider credential semantics,
and future upload issuance flows should converge on the same storage runtime instead of rebuilding
provider logic in each consumer surface.

Read [Storage Management](/architecture/storage-management) before changing admin storage flows,
provider fallback rules, or media upload wiring.

## Provider Defaults

The platform-default provider registry currently selects these defaults:

| Domain | Selected plugin |
| --- | --- |
| `rtc` | `rtc-volcengine` |
| `object-storage` | `object-storage-volcengine` |
| `principal-profile` | `principal-profile-upstream-context` (default), `principal-profile-external-catalog` (read-only catalog mode) |

These defaults come from the platform provider registry contract and are surfaced through runtime
tests for app, ops, and control-plane endpoints.

## Retired Local Profiles

Pre-topology-v2 minimal-node/minimal/default profile ids are removed. Use topology profile ids under
`configs/topology/` and `pnpm dev` / `pnpm dev:server` for development.

## What To Read Next

- [Runtime Topology](/architecture/runtime-topology)
- [Module Map](/architecture/module-map)
- [Runtime Directory](/reference/runtime-directory)

