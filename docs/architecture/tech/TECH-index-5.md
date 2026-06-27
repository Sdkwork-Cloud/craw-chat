> Migrated from `docs/sites/index.md` on 2026-06-24.
> Owner: SDKWork maintainers

---
layout: home

hero:
  name: Sdkwork IM
  text: Implementation-aligned product documentation
  tagline: "Open-source docs aligned to the current Sdkwork IM repository state: runtime architecture, HTTP APIs, SDK workspaces, and deployment workflows."
  actions:
    - theme: brand
      text: SDK Overview
      link: /sdk/index
    - theme: alt
      text: TypeScript Quick Start
      link: /sdk/typescript-quick-start
    - theme: alt
      text: API Reference
      link: /api-reference/index

features:
  - title: Implementation First
    details: Every page is written against the current repository behavior, not against roadmap assumptions or placeholder directories.
  - title: OpenAPI-style API Docs
    details: HTTP operations are grouped by runtime domain, linked from the sidebar, and documented with request and response schemas plus nested field expansion.
  - title: SDK Boundary Clarity
    details: The docs distinguish release-catalog artifacts, repo package contracts, and consumer package surfaces so repository state is not confused with registry publication.
  - title: SDK Quick Starts And Examples
    details: TypeScript, Flutter, and Rust now have dedicated quick-starts, shared client bootstrap guidance, module pages, and scenario-driven integration examples.
  - title: Operable Deployment Guides
    details: Local binary, Docker Compose, runtime inspection, repair, backup, preview, and restore flows are documented from the existing scripts and binaries.
  - title: Runtime-centric Architecture
    details: The site explains the Rust workspace, service boundaries, runtime-directory contract, provider model, and control-plane governance through the code that runs today.
  - title: Mature Open-source Structure
    details: The site is organized as a product documentation portal with clear entry points for onboarding, operations, architecture, APIs, SDKs, and reference material.
---

## What Sdkwork IM Is

Sdkwork IM is a Rust workspace with three primary runtime surfaces in the current repository state,
plus checked-in SDK workspaces built from those services:

- `services/sdkwork-im-cloud-gateway`, the unified application ingress that publishes the canonical
  `sdkwork-im-server` binary, aggregates OpenAPI discovery, and fronts split or unified runtime
  layouts.
- `services/control-plane-api`, the separate governance surface for protocol registry, provider
  policy, and node lifecycle operations.
- `sdks/`, which currently contains the IM standard, App API, Backend API, and independent RTC SDK
  families used by product, app-business, backend/operator, admin, governance, and provider-runtime
  integrations.

The default development profile is `standalone.split-services.development` (`pnpm dev`). Topology
authority lives in `specs/topology.spec.json` and `configs/topology/*.env`.

For most new integrations, the fastest reading order is:

1. [SDK Overview](/sdk/index)
2. [Auth and Client Init](/sdk/auth-and-client-init)
3. one language quick start:
   [TypeScript](/sdk/typescript-quick-start),
   [Flutter](/sdk/flutter-quick-start),
   [Rust](/sdk/rust-quick-start)
4. [Module Map](/sdk/module-map)
5. the matching API reference domain pages

<div class="landing-grid">
  <div class="fact-card">
    <h3>Default App Listener</h3>
    <p><code>127.0.0.1:18079</code> for the default development application ingress (<code>pnpm dev</code>).</p>
  </div>
  <div class="fact-card">
    <h3>Unified Server Listener</h3>
    <p><code>0.0.0.0:18080</code> in the frozen <code>server.yaml.example</code> contract for <code>sdkwork-im-server</code> and the unified <code>web-gateway</code>.</p>
  </div>
  <div class="fact-card">
    <h3>Control Plane Listener</h3>
    <p><code>127.0.0.1:18081</code> when the standalone <code>control-plane-api</code> binary is run directly.</p>
  </div>
  <div class="fact-card">
    <h3>Public Auth Model</h3>
    <p>Public clients authenticate through SDKWork dual tokens; sdkwork-im receives only verified <code>x-sdkwork-*</code> AppContext projection headers.</p>
  </div>
  <div class="fact-card">
    <h3>SDK Delivery State</h3>
    <p>The official IM consumer TypeScript package is <code>@sdkwork/im-sdk</code>. <code>sdkwork-im-app-sdk</code> owns <code>/app/v3/api</code>, <code>sdkwork-im-backend-sdk</code> owns all <code>/backend/v3/api</code> control/admin modules, and <code>sdkwork-rtc-sdk</code> remains an independent provider-runtime SDK.</p>
  </div>
</div>

## What To Read Next

1. Start with [Getting Started](/getting-started/index) to understand supported runtime modes,
   prerequisites, and auth expectations.
2. Use [Quick Start](/getting-started/quick-start) to start the development stack and verify health.
3. Read [Architecture Overview](/architecture/overview) and
   [Runtime Topology](/architecture/runtime-topology) before changing runtime wiring, providers, or
   deployment assumptions.
4. Read [Storage Management](/architecture/storage-management) before changing tenant provider
   resolution, admin storage behavior, or upload issuance assumptions. Keep
   [Admin Storage Contract](/reference/admin-storage-contract) open when you need the current
   `/backend/v3/api/admin/storage/*` route set and sandbox promotion boundary.
5. Use [Server Lifecycle](/deployment/server-lifecycle) when validating the packaged
   `sdkwork-im-server` install contract, PostgreSQL-backed storage wiring, or unified gateway
   endpoints.
6. Use [API Reference](/api-reference/index) for the OpenAPI-style operation catalog with sidebar
   links to every documented endpoint.
7. Read [SDK Overview](/sdk/index), then use
   [TypeScript Quick Start](/sdk/typescript-quick-start),
   [Flutter Quick Start](/sdk/flutter-quick-start), or
   [Rust Quick Start](/sdk/rust-quick-start) before promising package availability, import paths,
   or feature parity to consumers.
8. Keep [Deployment](/deployment/index) and [Reference](/reference/cli-and-scripts) open when
   running, diagnosing, backing up, or restoring a local environment.

## Choose Your Entry Point

| If you are... | Start here | Why |
| --- | --- | --- |
| Integrating against the public app runtime | [SDK Overview](/sdk/index) | Separates contract authority, repo packages, and release state |
| Building a browser or Node integration today | [TypeScript Quick Start](/sdk/typescript-quick-start) | Jumps straight into the production package, flat config, message-first send model, and live receive setup |
| Building a Flutter client for app-runtime flows | [Flutter Quick Start](/sdk/flutter-quick-start) | Starts from `im_sdk_generated` (HTTP) and `im_sdk_composed` (CCP WebSocket live) |
| Building a Rust integration or service-side adapter | [Rust Quick Start](/sdk/rust-quick-start) | Clarifies the generated transport boundary, composed reserve, and current Rust package contract |
| Working directly with HTTP endpoints | [API Reference](/api-reference/index) | Links every documented operation page from the sidebar |
| Operating or diagnosing a local deployment | [Deployment](/deployment/index) | Covers binary startup, Docker Compose, runtime inspection, backup, repair, preview, and restore |
| Changing runtime or governance behavior | [Architecture Overview](/architecture/overview) | Anchors service boundaries, runtime topology, and source-of-truth files before code changes |

## Current Delivery Surface

| Area | What is currently implemented |
| --- | --- |
| App runtime | Presence heartbeat, realtime delivery, conversations, membership, messages, media, streams, RTC, notifications, automation, audit, ops, and provider health |
| Control plane | Protocol registry, protocol governance, provider registry, effective bindings, provider policy preview and rollback, plus node drain, activate, and route migration |
| Unified gateway and server | `sdkwork-im-cloud-gateway` publishes the canonical `sdkwork-im-server` binary, the aggregate OpenAPI and discovery routes, service-schema proxies, rendered docs, and the single-port server install contract |
| Deployment | Local binary lifecycle scripts, Docker Compose bootstrap, server install/service-management scripts, runtime inspection, repair, backup listing, archive, preview, and restore |
| SDK workspaces | IM standard SDK for `/im/v3/api`; App API SDK for `/app/v3/api`; Backend SDK for `/backend/v3/api` including control and admin modules; independent RTC provider-standard SDK for provider runtime integration |
| Frontend apps | `apps/sdkwork-im-admin` already provides a verified standalone operator shell, a first-class storage-management workflow, and a documented `/backend/v3/api/admin/storage/*` contract surface, while `apps/sdkwork-im-portal` exists in-repo but is not yet documented here as a mature product surface |

::: warning Scope rule
This documentation intentionally describes only what can be verified from the current repository
state. Placeholder directories, future plans, or unpublished SDK artifacts are explicitly marked as
such rather than documented as delivered features.
:::

<div class="source-note">
  <strong>Implementation sources:</strong>
  App routing is aligned to <code>services/sdkwork-im-cloud-gateway</code>.
  Control-plane routing is aligned to <code>services/control-plane-api/src/lib.rs</code>.
  Unified gateway and packaged server entry are aligned to <code>services/sdkwork-im-cloud-gateway/src/lib.rs</code>,
  <code>services/sdkwork-im-cloud-gateway/src/main.rs</code>, and <code>deployments/templates/server.yaml.example</code>.
  Development orchestration is aligned to <code>scripts/im-dev.mjs</code>, <code>scripts/im-server-dev.mjs</code>,
  and <code>configs/topology/*.env</code>.
</div>

