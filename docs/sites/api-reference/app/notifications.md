# Notifications

<p class="api-page-intro">
  Notification endpoints create notification work items and expose notification task state for the
  current principal.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/automation"><code>Automation</code> Automation executions are documented separately</a>
  <a href="/sdk/index"><code>SDK</code> Treat notifications as an HTTP-first operational surface unless a backend consumer layer is explicitly documented</a>
</div>

<a id="request-notification"></a>
<section class="api-op">

## `POST /app/v3/api/notifications/requests`

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
<a id="list-notifications"></a>
<section class="api-op">

## `GET /app/v3/api/notifications`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/app/v3/api/notifications</code>
  <span class="api-op-id">operationId: listNotifications</span>
</div>

Lists notification tasks visible to the current principal.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.notification`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Current recipient scope.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 NotificationListResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="NotificationListResponse" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller is not allowed to read the target notification scope. |
| `404` | `notification_not_found` | The requested notification task does not exist. |

</section>
<a id="get-notification"></a>
<section class="api-op">

## `GET /app/v3/api/notifications/{notificationId}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/app/v3/api/notifications/{notificationId}</code>
  <span class="api-op-id">operationId: getNotification</span>
</div>

Reads a single notification task by identifier.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.notification`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Current recipient scope.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 NotificationTask`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `notification_id` | `string` | Yes | Notification task identifier. |

### Response `200`

<ApiSchemaTable schema="NotificationTask" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller is not allowed to read the target notification scope. |
| `404` | `notification_not_found` | The requested notification task does not exist. |

</section>
