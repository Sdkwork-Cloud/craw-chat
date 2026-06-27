# `POST /app/v3/api/notifications/requests`

<p class="api-page-intro">
  Exact request and response contract for <strong>Notifications</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/notifications"><code>Notifications</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, AppContext projection, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/app/v3/api/notifications/requests</code>
  <span class="api-op-id">operationId: requestNotification</span>
</div>

Creates or idempotently reuses a notification task.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.notification`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Own recipient scope or `notification.write` for delegated sends.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 NotificationTask`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="RequestNotification" />

### Response `200`

<ApiSchemaTable schema="NotificationTask" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The notification request is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks delegated notification authority. |
| `409` | `notification_conflict` | The idempotent notification request conflicts with existing state. |

</section>
