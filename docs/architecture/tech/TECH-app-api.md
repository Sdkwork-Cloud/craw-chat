> Migrated from `docs/sites/api-reference/app-api.md` on 2026-06-24.
> Owner: SDKWork maintainers

# App API Overview

<p class="api-page-intro">
  The App API is the generated app-business HTTP surface under <code>/app/v3/api/*</code>. It
  contains non-management APIs that are outside the IM standardized development contract and maps
  to the <code>sdkwork-im-app-sdk</code> family.
</p>

<div class="api-overview-grid">
  <div class="api-card">
    <h3>Portal Access</h3>
    <p>Tenant portal access snapshots, public landing snapshots, workspace reads, and authenticated portal module snapshots.</p>
    <p><a href="/api-reference/app/portal-access">Open Portal Access APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Notifications</h3>
    <p>Create notification requests and inspect dispatched notification tasks.</p>
    <p><a href="/api-reference/app/notifications">Open Notification APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Automation</h3>
    <p>Trigger automation executions, agent responses, and agent tool-call workflows.</p>
    <p><a href="/api-reference/app/automation">Open Automation APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Provider Health</h3>
    <p>Check media and principal-profile provider plugin health from the active node.</p>
    <p><a href="/api-reference/app/provider-health">Open Provider Health APIs</a></p>
  </div>
</div>

## SDK Alignment

- App endpoints under `/app/v3/api/*` map to `sdkwork-im-app-sdk`.
- The generated TypeScript client is `SdkworkAppClient`.
- AIoT device, twin, command, and event capability belongs to `sdkwork-aiot-app-sdk`, consumed as a dependency SDK.
- IM standardized endpoints under `/im/v3/api/*` map to [IM Standard API](/api-reference/im-api) and `sdkwork-im-sdk`.
- Backend management, control, admin, operator, and audit endpoints map to [Backend API](/api-reference/backend-api) and `sdkwork-im-backend-sdk`.
- App API routes do not own login, account, tenant, organization, or token refresh; those contexts are supplied by the upstream appbase identity system.
- In packaged installs, the same App API surface is exposed through the unified `sdkwork-im-server`
  / `sdkwork-im-cloud-gateway` public origin rather than a separate public app-node port.

## How To Use This Page

Use the App API pages for route semantics first:

1. Read this section and the linked operation pages for request or response contracts.
2. Switch to [App SDK](/sdk/app-sdk) when you need generated package boundaries, language workspaces, or `SdkworkAppClient` usage.
3. Switch to [SDK Overview](/sdk/index) when you need to decide between IM, app, backend, and RTC SDK families.

The API reference remains the authority for HTTP behavior even when the SDK pages describe a
higher-level helper method on top of it.

## What To Read Next

- [SDK Overview](/sdk/index)
- [App SDK](/sdk/app-sdk)
- [IM Standard API](/api-reference/im-api)
- [Backend API](/api-reference/backend-api)

## App API Domains

<div class="api-link-list">
  <a href="/api-reference/app/portal-access"><code>Portal</code> Tenant access snapshots and portal reads</a>
  <a href="/api-reference/app/notifications"><code>Notifications</code> Request, list, and inspect notification tasks</a>
  <a href="/api-reference/app/automation"><code>Automation</code> Request executions, drive agent responses, and complete tool calls</a>
  <a href="/api-reference/app/provider-health"><code>Provider Health</code> Media and principal-profile provider health snapshots</a>
</div>

