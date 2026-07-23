# `POST /app/v3/api/notifications/requests`

<p class="api-page-intro">
  Exact request and response contract for <strong>Notifications</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/notifications"><code>Notifications</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, resolved request-context, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/app/v3/api/notifications/requests</code>
  <span class="api-op-id">operationId: notifications.requests.create</span>
</div>

Creates or idempotently reuses a notification task.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.notification`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Own recipient scope or `notification.write` for delegated sends.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201 NotificationTask in data.item`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="RequestNotification" />

### Response `201`

<ApiSchemaTable schema="NotificationTask" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The notification request is invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks delegated notification authority. |
| `409` | `40901` | The idempotent notification request conflicts with existing state. |

</section>
