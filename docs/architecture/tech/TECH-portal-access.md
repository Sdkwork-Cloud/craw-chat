> Migrated from `docs/sites/api-reference/app/portal-access.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Portal Access

<p class="api-page-intro">
  Portal endpoints cover tenant portal access snapshots, public landing
  snapshots, appbase-authenticated workspace reads, and portal module snapshots for dashboard,
  conversations, realtime, media, automation, and governance.
</p>

<div class="api-link-list">
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token and AppContext projection rules are documented separately</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the app-business domain overview</a>
  <a href="/sdk/app-sdk"><code>App SDK</code> <code>sdkwork-im-app-sdk</code> exposes portal snapshots through generated <code>SdkworkAppClient.portal</code> transport modules</a>
</div>

<a id="get-home"></a>
<section class="api-op">

## `GET /app/v3/api/portal/home`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/app/v3/api/portal/home</code>
  <span class="api-op-id">operationId: getHome</span>
</div>

Reads the public tenant-portal home snapshot used for the landing experience.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.portal.home.retrieve()`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalSnapshot" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `503` | `*_unavailable` | The portal snapshot source is unavailable. |

</section>

<a id="get-access"></a>
<section class="api-op">

## `GET /app/v3/api/portal/access`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/app/v3/api/portal/access</code>
  <span class="api-op-id">operationId: getAccess</span>
</div>

Reads the public portal access snapshot. Login, token refresh, tenant, user, and organization context are supplied by sdkwork-appbase.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.portal.access.retrieve()`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalSnapshot" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `503` | `*_unavailable` | The portal snapshot source is unavailable. |

</section>

<a id="get-workspace"></a>
<section class="api-op">

## `GET /app/v3/api/portal/workspace`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/app/v3/api/portal/workspace</code>
  <span class="api-op-id">operationId: getWorkspace</span>
</div>

Reads the current authenticated tenant workspace summary used by the portal shell.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.portal.workspace.retrieve()`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalWorkspaceView`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalWorkspaceView" />

### Example Response

```json
{
  "name": "Nebula Commerce IM",
  "slug": "nebula-commerce-im",
  "tier": "Enterprise",
  "region": "CN-East / Multi-AZ",
  "supportPlan": "Platinum",
  "seats": 84,
  "activeBrands": 12,
  "uptime": "99.983%"
}
```

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `portal_access_denied`, `permission_denied` | The principal cannot access the workspace summary. |
| `503` | `*_unavailable` | The workspace snapshot source is unavailable. |

</section>

<a id="get-dashboard"></a>
<section class="api-op">

## `GET /app/v3/api/portal/dashboard`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/app/v3/api/portal/dashboard</code>
  <span class="api-op-id">operationId: getDashboard</span>
</div>

Reads the authenticated dashboard snapshot for tenant operations.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.portal.dashboard.retrieve()`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalSnapshot" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `portal_access_denied`, `permission_denied` | The principal cannot access the dashboard snapshot. |
| `503` | `*_unavailable` | The dashboard snapshot source is unavailable. |

</section>

<a id="get-conversations"></a>
<section class="api-op">

## `GET /app/v3/api/portal/conversations`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/app/v3/api/portal/conversations</code>
  <span class="api-op-id">operationId: getConversations</span>
</div>

Reads the portal conversations module snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.portal.conversationSnapshot.retrieve()`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalSnapshot" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `portal_access_denied`, `permission_denied` | The principal cannot access the conversations snapshot. |
| `503` | `*_unavailable` | The conversations snapshot source is unavailable. |

</section>

<a id="get-realtime"></a>
<section class="api-op">

## `GET /app/v3/api/portal/realtime`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/app/v3/api/portal/realtime</code>
  <span class="api-op-id">operationId: getRealtime</span>
</div>

Reads the portal realtime posture snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.portal.realtime.retrieve()`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalSnapshot" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `portal_access_denied`, `permission_denied` | The principal cannot access the realtime snapshot. |
| `503` | `*_unavailable` | The realtime snapshot source is unavailable. |

</section>

<a id="get-media"></a>
<section class="api-op">

## `GET /app/v3/api/portal/media`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/app/v3/api/portal/media</code>
  <span class="api-op-id">operationId: getMedia</span>
</div>

Reads the portal media and RTC snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.portal.media.retrieve()`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalSnapshot" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `portal_access_denied`, `permission_denied` | The principal cannot access the media snapshot. |
| `503` | `*_unavailable` | The media snapshot source is unavailable. |

</section>

<a id="get-automation"></a>
<section class="api-op">

## `GET /app/v3/api/portal/automation`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/app/v3/api/portal/automation</code>
  <span class="api-op-id">operationId: getAutomation</span>
</div>

Reads the portal automation and notification posture snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.portal.automation.retrieve()`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalSnapshot" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `portal_access_denied`, `permission_denied` | The principal cannot access the automation snapshot. |
| `503` | `*_unavailable` | The automation snapshot source is unavailable. |

</section>

<a id="get-governance"></a>
<section class="api-op">

## `GET /app/v3/api/portal/governance`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/app/v3/api/portal/governance</code>
  <span class="api-op-id">operationId: getGovernance</span>
</div>

Reads the portal governance and compliance snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.portal.governance.retrieve()`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalSnapshot" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `portal_access_denied`, `permission_denied` | The principal cannot access the governance snapshot. |
| `503` | `*_unavailable` | The governance snapshot source is unavailable. |

</section>

