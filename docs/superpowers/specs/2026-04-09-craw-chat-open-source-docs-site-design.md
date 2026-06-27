# Sdkwork IM Open Source Docs Site Design

**Date:** 2026-04-09

## Goal

Create a complete VitePress documentation site under `docs/sites` for the current `sdkwork-im` application. The site must read like a mature open source product site while staying strictly aligned with the current repository state.

## Canonical Sources

- Product and workspace framing: `README.md`, `Cargo.toml`
- Public app API and local runtime assembly: `services/local-minimal-node/src/node/build.rs`
- Control-plane API: `services/control-plane-api/src/lib.rs`
- Public auth behavior: `services/local-minimal-node/tests/public_auth_e2e_test.rs`, `services/control-plane-api/tests/public_auth_test.rs`
- End-to-end HTTP behavior: `services/local-minimal-node/tests/http_e2e_test.rs`
- Runtime and deployment commands: `bin/*.ps1`, `deployments/scripts/bootstrap-local.ps1`, `deployments/docker-compose/*.yml`
- Environment templates: `deployments/templates/*.env.example`
- SDK boundary and release status: `sdks/**/*.md`, `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`

## Chosen Approach

Build a standalone VitePress site that treats the codebase as the source of truth and uses the existing internal docs only as supplemental context. The documentation will prefer:

1. Exact command surfaces from the scripts.
2. Exact route surfaces from the router builders.
3. Exact SDK status from the checked-in SDK catalog.
4. Explicitly documented current limitations where generation or release work is still pending.

## Information Architecture

- `index.md`
  Product landing page, positioning, capability summary, quick links.
- `getting-started/`
  Prerequisites, quick start, local run, first API call.
- `architecture/`
  System overview, workspace map, runtime topology, auth and data flow.
- `features/`
  Capability-oriented docs for conversation, realtime, media, stream, RTC, automation, audit, ops, IoT, provider model.
- `api-reference/`
  Full route inventory grouped into app, platform, IoT, ops, and control-plane surfaces.
- `sdk/`
  App SDK family, admin SDK family, TypeScript, Flutter, and current release state.
- `deployment/`
  Local binary install, Docker deployment, profile matrix, runtime operations.
- `reference/`
  CLI/runtime command reference, environment variables, runtime directory contract.

## Editorial Standards

- Product-first landing page with opinionated information scent.
- Every page must tell the reader what is implemented now versus planned later.
- API pages must include auth model, source-of-truth note, and grouped endpoint tables.
- SDK pages must not imply published packages when the repo only contains placeholders and release metadata.
- Deployment pages must show platform-specific commands for PowerShell, Bash, and CMD when those entrypoints exist in `bin/`.

## Verification Strategy

- Run `npm install` and `npm run docs:build` inside `docs/sites`.
- Re-check page content against route builders, scripts, templates, and SDK catalog before finalizing.
- Avoid undocumented claims that are not grounded in current source files.
