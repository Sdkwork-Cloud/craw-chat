> Migrated from `docs/sites/api-reference/backend-api.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Backend API Overview

<p class="api-page-intro">
  The Backend API is the single HTTP authority for management-system, operator, audit,
  governance, control-plane, and admin-console APIs under <code>/backend/v3/api/*</code>. It maps
  to <code>sdkwork-im-backend-sdk</code>; there is no standalone admin SDK family and no standalone
  control-plane SDK family.
</p>

<div class="api-overview-grid">
  <div class="api-card">
    <h3>Operations</h3>
    <p>Inspect health, cluster topology, lag, replay status, runtime directory state, provider bindings, and diagnostic bundles.</p>
    <p><a href="/api-reference/backend/ops">Open Operator APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Audit</h3>
    <p>Record audit anchors, list audit records, verify hash-chain state, and export audit evidence bundles.</p>
    <p><a href="/api-reference/backend/audit">Open Audit APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Control Modules</h3>
    <p>Manage protocol governance, provider policy, social graph control, shared-channel runtime queues, and node route operations.</p>
    <p><a href="/api-reference/control-plane-api">Open Control Module APIs</a></p>
  </div>
</div>

## SDK Alignment

- `/backend/v3/api/*` maps to `sdkwork-im-backend-sdk`.
- `/backend/v3/api/control/*` and `/backend/v3/api/admin/*` are modules of the backend SDK family.
- `/app/v3/api/*` belongs to [App API](/api-reference/app-api) and `sdkwork-im-app-sdk`.
- `/im/v3/api/*` belongs to [IM Standard API](/api-reference/im-api) and `sdkwork-im-sdk`.
- RTC provider runtime and native driver concerns belong to [RTC SDK](/sdk/rtc-sdk).

## Backend API Domains

<div class="api-link-list">
  <a href="/api-reference/backend/ops"><code>Ops</code> Cluster health, lag, replay status, runtime directory, provider bindings, and diagnostics</a>
  <a href="/api-reference/backend/audit"><code>Audit</code> Record audit anchors, list audit records, verify hash-chain state, and export bundles</a>
  <a href="/api-reference/control-plane/protocol"><code>Control Protocol</code> Protocol registry and governance snapshots</a>
  <a href="/api-reference/control-plane/providers"><code>Control Providers</code> Provider registry, binding policies, previews, and rollback</a>
  <a href="/api-reference/control-plane/social"><code>Control Social</code> Direct-chat, external collaboration, friendship, shared-channel policy, and user block control</a>
  <a href="/api-reference/control-plane/social-runtime"><code>Control Runtime</code> Shared-channel sync runtime inventory, repair, and takeover flows</a>
  <a href="/api-reference/control-plane/nodes"><code>Control Nodes</code> Drain, activate, and route migration operations</a>
</div>

