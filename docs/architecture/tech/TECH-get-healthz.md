> Migrated from `docs/sites/api-reference/operations/im/session-and-realtime/get-healthz.md` on 2026-06-24.
> Owner: SDKWork maintainers

<p class="api-page-intro">
  Exact request and response contract for <strong>Realtime Presence</strong> in the <strong>IM Standard API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/im/session-and-realtime"><code>Realtime Presence</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/im-api"><code>IM Standard API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, AppContext projection, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/healthz</code>
  <span class="api-op-id">operationId: getHealthz</span>
</div>

Returns process liveness for the app runtime.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Open endpoint</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>Direct HTTP probe</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Not required</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 HealthResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="HealthResponse" />

</section>

