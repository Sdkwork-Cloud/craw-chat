> Migrated from `docs/sites/api-reference/platform-api.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Legacy Platform API Grouping

<p class="api-page-intro">
  This page is retained only as a legacy grouping note. Current API authority is split into
  <code>/app/v3/api/*</code> for app-business routes and <code>/backend/v3/api/*</code> for
  management, operator, audit, control, and admin routes.
</p>

<div class="api-overview-grid">
  <div class="api-card">
    <h3>Notifications</h3>
    <p>Create notification requests and inspect dispatched notification tasks.</p>
    <p><a href="/api-reference/app/notifications">Open Notification APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Automation</h3>
    <p>Trigger automation executions and inspect execution state.</p>
    <p><a href="/api-reference/app/automation">Open Automation APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Audit and Ops</h3>
    <p>Read and export audit records, inspect runtime health, lag, cluster topology, and diagnostic bundles.</p>
    <p><a href="/api-reference/backend/ops">Open Operator APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Provider Health</h3>
    <p>Check media, RTC, and principal-profile provider plugin health from the active node.</p>
    <p><a href="/api-reference/app/provider-health">Open Provider Health APIs</a></p>
  </div>
</div>

## SDK Alignment

- Management endpoints are consumed through `sdkwork-im-backend-sdk`; non-management app-business
  endpoints are consumed through `sdkwork-im-app-sdk`.
- Permission requirements are documented in [Authentication and Errors](/api-reference/auth-and-errors) and repeated on operation pages when they are mandatory.
- This site does not document a separate Platform API or Platform SDK family.
- `/backend/v3/api/ops/*`, `/backend/v3/api/audit/*`, `/backend/v3/api/automation/governance`,
  `/backend/v3/api/control/*`, and `/backend/v3/api/admin/*` belong to `sdkwork-im-backend-sdk`.
- Provider health, IoT protocol, app-facing notifications, app-facing automation execution, and
  app-facing RTC provider routes belong to `sdkwork-im-app-sdk` under `/app/v3/api/*`.
- In packaged installs, these routes are still reached through the unified `sdkwork-im-server` /
  `sdkwork-im-cloud-gateway` public origin even though the implementation remains on the app-node side of the
  runtime.
- Platform routes do not have a standalone SDK family.

## How To Use This Page

- Start with [Authentication and Errors](/api-reference/auth-and-errors) for SDKWork dual-token, AppContext projection, and permission rules.
- Use the linked operation groups below for exact route semantics.
- Switch to [SDK Overview](/sdk/index) only when you need to understand whether a backend-facing
  package surface is documented as published or only present as repo workspace state.

## What To Read Next

- [Authentication and Errors](/api-reference/auth-and-errors)
- [SDK Overview](/sdk/index)

## Current Domains

<div class="api-link-list">
  <a href="/api-reference/app/notifications"><code>Notifications</code> Request, list, and inspect notification tasks</a>
  <a href="/api-reference/app/automation"><code>Automation</code> Request and inspect automation executions</a>
  <a href="/api-reference/backend/audit"><code>Audit</code> Record audit anchors, list audit records, and export bundles</a>
  <a href="/api-reference/backend/ops"><code>Ops</code> Cluster health, lag, replay status, runtime directory, provider bindings, and diagnostics</a>
  <a href="/api-reference/app/provider-health"><code>Health</code> Provider plugin health snapshots for active integrations</a>
</div>

