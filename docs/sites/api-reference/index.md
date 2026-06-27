# API Reference

<p class="api-page-intro">
  This API reference stays aligned to the HTTP surface implemented by the current repository state.
  It documents request and response shapes in an OpenAPI-style layout, with expandable nested
  schemas for deeply structured payloads.
</p>

<div class="api-overview-grid">
  <div class="api-card">
    <h3>Gateway OpenAPI</h3>
    <p>Aggregate schema discovery, service schema index, per-service schema proxies, and rendered docs on the unified gateway port.</p>
    <p><a href="/api-reference/gateway-openapi">Open Gateway OpenAPI</a></p>
  </div>
  <div class="api-card">
    <h3>IM Standard API</h3>
    <p><code>/im/v3/api</code> runtime routes for Realtime Presence, realtime delivery, chat, media, streams, and RTC.</p>
    <p><a href="/api-reference/im-api">Open IM Standard API overview</a></p>
  </div>
  <div class="api-card">
    <h3>App API</h3>
    <p><code>/app/v3/api</code> generated app-business routes for non-management APIs outside IM standardization.</p>
    <p><a href="/api-reference/app-api">Open App API overview</a></p>
  </div>
  <div class="api-card">
    <h3>Backend API</h3>
    <p><code>/backend/v3/api</code> management, operator, audit, control, and admin endpoints.</p>
    <p><a href="/api-reference/backend-api">Open Backend API overview</a></p>
  </div>
  <div class="api-card">
    <h3>Backend Control Modules</h3>
    <p>Protocol governance, provider registry and policy management, social graph and shared-channel runtime control, plus node drain, activate, and route migration operations.</p>
    <p><a href="/api-reference/control-plane-api">Open backend control module overview</a></p>
  </div>
</div>

## Standards Used

- Operation pages are grouped by runtime domain and linked individually in the sidebar.
- The unified `sdkwork-im-cloud-gateway` also publishes an aggregate OpenAPI 3.1 document, a service schema index, and service-specific schema/docs routes.
- For packaged installs, start with [Gateway OpenAPI](/api-reference/gateway-openapi):
  `/im/v3/api`, `/app/v3/api`, `/backend/v3/api`, backend control modules, and
  `/backend/v3/api/admin/*` discovery all converge on the same unified public origin.
- Request and response payloads are rendered from a shared schema registry to keep documentation aligned with implementation.
- Nested request and response fields are expandable so large payloads remain readable on desktop and mobile.
- Shared auth rules and error envelope semantics are documented in [Authentication and Errors](/api-reference/auth-and-errors).
- SDK families remain boundary-specific: `sdkwork-im-sdk` maps to `/im/v3/api`,
  `sdkwork-im-app-sdk` maps to `/app/v3/api`, `sdkwork-im-backend-sdk` maps to `/backend/v3/api`,
  and `sdkwork-rtc-sdk` maps to provider-runtime integration rather than an OpenAPI route family.
  `/backend/v3/api/control/*` and `/backend/v3/api/admin/*` are backend SDK modules.

## How To Use This Page

Read the API docs in this order:

1. Start with [Authentication and Errors](/api-reference/auth-and-errors) for SDKWork dual-token, AppContext projection, and error-envelope rules.
2. Open the domain overview that matches the runtime surface you are integrating: IM standard, app, or backend.
3. Use operation pages for the exact request and response contract.
4. Switch to the SDK docs only when your next question becomes package names, language parity,
   helper methods, or publication state.

That split keeps HTTP semantics and consumer package surfaces from being mixed together.

## When To Switch To SDK Docs

- Use [TypeScript SDK](/sdk/typescript-sdk) or [Flutter SDK](/sdk/flutter-sdk) when you need checked-in import paths and client examples for the app runtime.
- Use [Backend SDK](/sdk/backend-sdk) when you need ops, audit, control-plane, or admin client boundaries.
- Use [App API SDK](/sdk/app-sdk) when you need app-business generated transport for `/app/v3/api/*`.
- Use [RTC SDK](/sdk/rtc-sdk) when you need RTC provider runtime and native driver boundaries.
- Use [SDK Overview](/sdk/index) when you need to decide whether a repo package also implies public registry availability.

## What To Read Next

<div class="api-link-list">
  <a href="/api-reference/gateway-openapi"><code>Gateway</code> Aggregate schema, schema index, and service-level docs discovery</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Authentication, AppContext projection headers, and the error envelope</a>
  <a href="/api-reference/im/session-and-realtime"><code>IM</code> Realtime Presence transport semantics</a>
  <a href="/api-reference/im/conversations"><code>IM</code> Conversation creation and handoff flows</a>
  <a href="/api-reference/app/portal-access"><code>App</code> Portal sign-in and tenant portal snapshot endpoints</a>
  <a href="/api-reference/app/notifications"><code>App</code> Notification request and task inspection</a>
  <a href="/api-reference/backend/ops"><code>Backend</code> Operator diagnostics and runtime inspection</a>
  <a href="/api-reference/control-plane/providers"><code>Control Plane</code> Provider registry, binding policies, and preview or rollback flows</a>
  <a href="/api-reference/control-plane/social-runtime"><code>Control Plane</code> Shared-channel sync runtime inventory, repair, and takeover flows</a>
</div>
