# Provider Health

<p class="api-page-intro">
  Provider health endpoints expose the active node's view of media and principal-profile provider
  plugins.
</p>

<div class="api-link-list">
  <a href="/api-reference/backend/ops"><code>Backend Ops</code> Broader runtime diagnostics and provider-binding views are documented separately</a>
</div>

<a id="get-media-provider-health"></a>
<section class="api-op">

## `GET /app/v3/api/media/provider_health`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/app/v3/api/media/provider_health</code>
  <span class="api-op-id">operationId: getMediaProviderHealth</span>
</div>

Returns the object-storage provider health snapshot.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / provider health</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderHealthSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ProviderHealthSnapshot" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `503` | `*_unavailable` | The provider health source is unavailable. |

</section>
<a id="get-principal-profile-provider-health"></a>
<section class="api-op">

## `GET /app/v3/api/principal/profiles/provider_health`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/app/v3/api/principal/profiles/provider_health</code>
  <span class="api-op-id">operationId: getPrincipalProfileProviderHealth</span>
</div>

Returns the principal-profile provider health snapshot.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / provider health</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderHealthSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ProviderHealthSnapshot" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `503` | `*_unavailable` | The provider health source is unavailable. |

</section>
