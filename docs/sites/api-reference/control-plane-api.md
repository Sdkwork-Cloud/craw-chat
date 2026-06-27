# Backend Control Module Overview

<p class="api-page-intro">
  Backend control modules manage runtime governance outside the app node request path under
  <code>/backend/v3/api/control/*</code>. They expose
  protocol registry and governance snapshots, provider registry and provider policy management,
  social graph and shared-channel runtime control, and node lifecycle operations for drain,
  activate, and route migration.
</p>

<div class="api-overview-grid">
  <div class="api-card">
    <h3>Protocol Governance</h3>
    <p>Read protocol registry inventory, client compatibility, quota profiles, rollout policies, and kill-switch state.</p>
    <p><a href="/api-reference/control-plane/protocol">Open Protocol Governance APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Provider Governance</h3>
    <p>Inspect provider plugins, read effective bindings, apply binding policies, preview diffs, and roll back versions.</p>
    <p><a href="/api-reference/control-plane/providers">Open Provider Governance APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Social Graph Control</h3>
    <p>Bind direct chats, establish external connections, manage friendships, apply shared-channel policies, and enforce user blocks.</p>
    <p><a href="/api-reference/control-plane/social">Open Social Graph Control APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Social Runtime</h3>
    <p>Inspect pending, delivered, and dead-letter shared-channel sync queues, then repair, reclaim, republish, requeue, or take over targeted work.</p>
    <p><a href="/api-reference/control-plane/social-runtime">Open Social Runtime APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Node Operations</h3>
    <p>Drain nodes, reactivate nodes, and migrate owned realtime routes between nodes.</p>
    <p><a href="/api-reference/control-plane/nodes">Open Node Operation APIs</a></p>
  </div>
</div>

## SDK Alignment

- These endpoints are backend SDK control modules inside `sdkwork-im-backend-sdk`.
- Read [Backend SDK](/sdk/backend-sdk) for the generated SDK boundary.
- The live control-plane source remains `services/control-plane-api`, exposed at `/openapi.json`
  and `/backend/v3/api/control/openapi.json`, then consolidated into
  `sdks/sdkwork-im-backend-sdk/openapi/sdkwork-im-backend-api.openapi.yaml`.
- Read and write permissions are split between `control.read` and `control.write`.
- Standalone governance development can call `control-plane-api` directly, but packaged installs
  expose the same governance routes through the unified `sdkwork-im-server` / `sdkwork-im-cloud-gateway` public
  origin.

## How To Use This Page

Use the control-plane API docs as the semantic authority first:

1. Read this overview and the linked control-plane operation groups for request, response, and permission behavior.
2. Use [Backend SDK](/sdk/backend-sdk) to confirm package boundaries, source-of-truth files, and release-state limits.
3. Treat `/backend/v3/api/control/*` and `/backend/v3/api/admin/*` as modules of the same backend SDK family.

## What To Read Next

- [Backend SDK](/sdk/backend-sdk)
- [Backend API](/api-reference/backend-api)
- [SDK Overview](/sdk/index)
- [Authentication and Errors](/api-reference/auth-and-errors)

## Control Plane Domains

<div class="api-link-list">
  <a href="/api-reference/control-plane/protocol"><code>Protocol</code> Health, protocol registry, and governance snapshots</a>
  <a href="/api-reference/control-plane/providers"><code>Providers</code> Registry, effective bindings, policy history, diff, preview, and rollback</a>
  <a href="/api-reference/control-plane/social"><code>Social</code> Direct-chat, external-collaboration, friendship, shared-channel-policy, and user-block control</a>
  <a href="/api-reference/control-plane/social-runtime"><code>Social Runtime</code> Shared-channel sync queue inventory, repair, reclaim, republish, requeue, and takeover flows</a>
  <a href="/api-reference/control-plane/nodes"><code>Nodes</code> Drain, activate, and route migration operations</a>
</div>
