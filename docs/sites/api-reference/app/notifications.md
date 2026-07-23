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
<a id="list-notifications"></a>
<section class="api-op">

## `GET /app/v3/api/notifications`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/app/v3/api/notifications</code>
  <span class="api-op-id">operationId: notifications.list</span>
</div>

Lists notification tasks visible to the current principal.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.notification`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Current recipient scope.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 NotificationListResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="NotificationListResponse" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller is not allowed to read the target notification scope. |
| `404` | `40401` | The requested notification task does not exist. |

</section>
<a id="get-notification"></a>
<section class="api-op">

## `GET /app/v3/api/notifications/{notificationId}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/app/v3/api/notifications/{notificationId}</code>
  <span class="api-op-id">operationId: notifications.retrieve</span>
</div>

Reads a single notification task by identifier.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
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
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller is not allowed to read the target notification scope. |
| `404` | `40401` | The requested notification task does not exist. |

</section>
