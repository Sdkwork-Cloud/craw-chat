> Migrated from `docs/superpowers/specs/2026-04-09-craw-chat-admin-design.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Sdkwork IM Admin Design

## Goal

Build `apps/control-plane` as the professional IM control-plane workspace for Sdkwork IM by fully reproducing the architecture standard of `apps/sdkwork-api-router/apps/sdkwork-routes-admin`, including:

- a standalone React and Tauri workspace
- a thin root app with package-first composition
- an isolated admin login surface
- route-manifest-driven product modules
- architecture and product-surface regression tests
- strict admin SDK consumption boundaries

The result must feel like a real operator-grade IM management console rather than a CRUD scaffold.

## Non-Goals

- do not hand-build local HTTP wrappers for admin control-plane APIs
- do not collapse all admin features into a single SPA package
- do not reuse router-admin business language, routes, or entities where the IM domain differs
- do not redesign the reference architecture into a new internal pattern

## Reference Standard

`apps/sdkwork-api-router/apps/sdkwork-routes-admin` is the architectural source of truth.

`apps/control-plane` must match its standards in these areas:

- root workspace files: `package.json`, `pnpm-workspace.yaml`, `turbo.json`, `tsconfig.json`, `vite.config.ts`
- root app behavior: `src/main.tsx` bootstraps shared shell runtime; `src/App.tsx` only mounts `AppRoot`
- package-first composition under `packages/`
- shared shell and route loading in `shell`
- route manifest, store, i18n, and workbench ownership in `core`
- isolated auth pages outside the authenticated shell
- Tauri desktop host under `src-tauri/`
- regression coverage in `tests/`

## Product Scope

The first professional IM admin release will ship these product modules.

### Foundation

- `Auth`: login, registration request, forgot-password placeholder, redirect recovery
- `Overview`: platform posture, live ops snapshot, risk summary, hotspots
- `Settings`: locale, appearance, navigation, workspace preferences

### Workspace Ops

- `Tenants`: tenant, workspace, organization, and project governance
- `Users`: portal users, operator users, client route posture, activation and ban state
- `Groups`: groups, channels, membership governance, role posture
- `Announcements`: announcement templates, broadcast tasks, delivery posture

### Conversation Governance

- `Conversations`: conversation lifecycle, type, membership, handoff, archive/freeze state
- `Messages`: search, audit, export, hide, recall, evidence review
- `Moderation`: reports, keyword policy, blocklists, risk decisions, escalation queues

### Automation And Realtime

- `Automation`: bot registry, workflow execution posture, automation run history
- `Realtime`: connections, client route sessions, RTC session posture, gateway health

### System

- `System`: protocol governance, compatibility matrix, control-plane snapshot, runtime health

## Workspace Layout

The workspace will mirror the reference layout.

```text
apps/control-plane/
|- src/          # root bootstrap only
|- packages/     # foundation and IM product modules
|- tests/        # architecture and product regression tests
|- src-tauri/    # desktop host and native commands
`- dist/         # production build output
```

## Package Map

### Foundation Packages

- `sdkwork-control-plane-types`
- `sdkwork-control-plane-core`
- `sdkwork-control-plane-shell`
- `sdkwork-control-plane-admin-api`
- `sdkwork-control-plane-auth`

### Business Packages

- `sdkwork-control-plane-overview`
- `sdkwork-control-plane-tenants`
- `sdkwork-control-plane-users`
- `sdkwork-control-plane-conversations`
- `sdkwork-control-plane-messages`
- `sdkwork-control-plane-groups`
- `sdkwork-control-plane-moderation`
- `sdkwork-control-plane-automation`
- `sdkwork-control-plane-announcements`
- `sdkwork-control-plane-realtime`
- `sdkwork-control-plane-system`
- `sdkwork-control-plane-settings`

## Route And Shell Model

- Browser routes mount under `/admin/`.
- Auth pages live outside the authenticated shell.
- Authenticated pages render inside a shared shell with top header, left sidebar, and right canvas.
- Route definitions stay in `core`.
- Product-module metadata stays in `core/routeManifest.ts`.
- Shell route loading stays lazy and prefetch-aware.
- The root app never owns page composition logic.

## Login Experience

The login experience will follow the reference structure, but the copy and visual identity will become IM-specific.

- left panel: brand, trust signals, QR/operator access framing, operations summary
- right panel: email/password login, registration request, forgot-password recovery
- redirect support for protected routes
- development prefill behavior kept for local operator flows

The first release will keep the same interaction model as the reference app:

- credential login works immediately
- registration flow is a request-for-access placeholder
- forgot-password flow routes back into login until backend recovery is ready

## Data And SDK Boundaries

The admin app must consume admin control-plane APIs through the admin SDK boundary only.

Required path:

`control-plane app -> sdkwork-control-plane-admin-api -> sdkwork-control-plane-sdk`

Rules:

- no raw `fetch` or local `/admin/*` wrappers inside product packages
- no duplicated admin DTOs in the app workspace
- if a capability is missing, extend the admin SDK path rather than bypassing it
- protocol governance and compatibility matrix surfaces remain admin-facing and flow through the admin SDK

## Workbench Model

`sdkwork-control-plane-core` will own:

- route constants and route metadata
- sidebar state and theme preferences
- admin locale provider
- workbench snapshot builders
- workbench actions
- shell-wide loading and error state contracts

`sdkwork-control-plane-admin-api` will own:

- typed admin service functions
- normalization of backend errors into operator-readable failures
- transport and auth setup for the admin SDK facade

Business packages will stay presentation-focused:

- page components
- registry sections
- detail drawers
- dialogs
- page-local filters and view state

## Product Experience Requirements

The app should feel operator-grade.

- `Overview` must highlight online users, message throughput, moderation backlog, realtime health, top tenants, and hot conversations
- `Users` must expose activation, client route posture, last active time, and governance actions
- `Conversations` must expose member posture, handoff state, archive/freeze state, and quick operator actions
- `Messages` must provide fast search and evidence-style detail views
- `Moderation` must organize report triage, blocked entities, keyword rules, and outcome tracking
- `Realtime` must show live session and gateway posture rather than static configuration only
- `System` must surface protocol governance and compatibility matrix as first-class operators concepts

## Error Handling

- auth expiration redirects to login with redirect preservation
- module-level failures render shared empty/error states, not ad hoc raw exceptions
- destructive actions require explicit confirmation dialogs
- long-running operations expose operator-readable progress and completion states
- snapshot loading and action execution remain separated so read failures do not silently mask write failures

## Testing Strategy

Testing will follow the reference workspace pattern and start before production code changes.

### Architecture Tests

- workspace root files exist
- required packages exist
- root app remains thin
- shell owns router/layout/theme integration
- route manifest exposes product-module metadata
- lazy route loading remains package-first
- login routes stay isolated outside authenticated shell
- Vite and Tauri configuration keep standalone app semantics
- admin SDK boundary is preserved

### Product Surface Tests

- IM module names and navigation groups match the design
- overview exposes IM-specific posture panels
- users, conversations, messages, moderation, automation, realtime, and system modules all expose first-class operator workflows
- login page copy and IM visual identity replace router-admin business language

## Delivery Strategy

Implementation should proceed in this order:

1. copy and rename the reference workspace skeleton into `apps/control-plane`
2. write failing architecture tests that define the IM admin workspace contract
3. make the copied workspace satisfy the renamed contract
4. replace router-admin module map with IM module map
5. rework auth copy, brand language, shell labels, and navigation groups
6. add first-pass IM module pages with professional placeholder content
7. connect `admin-api` to the IM admin SDK boundary
8. tighten regression coverage and verification commands

## Risks

- direct copy from the reference app can leave router-domain names behind unless tests force them out
- missing admin SDK capabilities can tempt local HTTP bypasses unless the boundary is enforced early
- shell parity is easy to regress if root files absorb business logic
- product quality can collapse into placeholder CRUD if module requirements are not encoded in tests

## Acceptance Criteria

- `apps/control-plane` matches the structural architecture of `sdkwork-routes-admin`
- login exists and is routed outside the authenticated shell
- all primary IM admin modules are present and navigable
- route manifest and package boundaries are package-first
- admin business calls are funneled through `sdkwork-control-plane-admin-api`
- the workspace builds and its targeted regression tests pass

