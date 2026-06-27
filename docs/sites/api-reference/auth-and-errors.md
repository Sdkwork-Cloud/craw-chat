# Authentication and Errors

<p class="api-page-intro">
  This page defines the security model and error envelope shared across the Sdkwork IM HTTP APIs.
  Individual operation pages document only endpoint-specific authorization requirements or conflict
  conditions on top of these defaults.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/portal-access"><code>App</code> Portal access snapshots, SDKWork credentials, and AppContext projection rules</a>
  <a href="/api-reference/app-api"><code>App</code> User-facing runtime domains and their operation groups</a>
  <a href="/api-reference/control-plane-api"><code>Control Plane</code> Administrative endpoints that use the same SDKWork dual-token and AppContext model with <code>control.read</code> and <code>control.write</code></a>
</div>

## How To Use This Page

Use this page for the shared contract first:

1. Start here when you need AppContext projection rules, SDKWork token semantics, or the common error
   envelope.
2. Switch to operation pages when your next question is endpoint-specific permission, conflict, or
   resource-not-found behavior.
3. Switch to the SDK docs only when your next question is language surface, package naming, or
   publication state.

## Security Schemes

### `SdkworkDualToken`

`sdkwork-appbase` owns login, IAM sessions, users, tenants, organizations, dual-token validation,
and the authoritative IAM context. Public clients authenticate with the SDKWork auth token and
access token:

- `Authorization: Bearer <auth-token>`
- `Access-Token: <access-token>`

Tenant, user, session, app, device, actor, data-scope, and permission-scope context is resolved from
those token claims. `sdkwork-im` does not own login or token issuance.

| Item | Value |
| --- | --- |
| External auth owner | `sdkwork-appbase` |
| Required public token model | SDKWork dual token |
| Sdkwork IM input | Dual-token claims, or private signed trusted-edge projection |
| Resolver | `resolve_app_context()` |

### `AppContextProjection`

AppContext projection is an internal trusted-edge or service-to-service contract. Public SDKs and
manual API callers must not send identity projection headers for tenant, user, session, device,
actor, or permission scope. The trusted edge may create a private signed projection only after it has
validated the SDKWork dual tokens and discarded any client-supplied projection values.

The private projection carries the same context represented in the dual-token claims: tenant,
organization, login scope, user, session, app, environment, deployment mode, auth level, actor,
device, data scope, and permission scope.

## Permission Model

### Control Plane permissions

| Permission | Grants |
| --- | --- |
| `control.read` | Read protocol registry, governance state, provider registry, bindings, and policy history |
| `control.write` | Mutate provider policies and execute node lifecycle operations |

### Platform and operator permissions

| Permission | Grants |
| --- | --- |
| `audit.read` | Read audit records and export bundles |
| `audit.write` | Write audit anchors |
| `ops.read` | Read operator health, cluster, lag, and diagnostics endpoints |
| `conversation.shared_channel.sync` | Execute shared-channel linked-member sync. Reserved for system actor `control-plane-sync`. |

## Security-Specific Error Codes

| HTTP | `code` | When it appears |
| --- | --- | --- |
| `401` | `app_context_missing` | Required AppContext projection headers are missing after SDKWork auth validation. |
| `401` | `app_context_invalid` | AppContext projection is malformed or incomplete. |
| `403` | `shared_channel_sync_permission_denied` | Missing permission `conversation.shared_channel.sync` for shared-channel sync endpoint. |
| `403` | `shared_channel_sync_actor_invalid` | Caller actor is not the system actor `control-plane-sync`. |
| `429` | `shared_channel_sync_rate_limited` | Shared-channel sync exceeded per-tenant rate limit window. |

## Error Envelope

### IM, App, and Backend APIs

The application-facing APIs return a compact error object:

```json
{
  "code": "conversation_not_found",
  "message": "conversation summary not found: conv_demo_001"
}
```

<ApiSchemaTable schema="ApiError" />

### Control Plane APIs

The control plane adds a `status` discriminator that mirrors the control-plane error category:

```json
{
  "status": "forbidden",
  "code": "permission_denied",
  "message": "missing required permission: control.write"
}
```

`status` can be one of:

- `unauthorized`
- `forbidden`
- `invalid`
- `conflict`
- `not_found`
- `unavailable`

## Common HTTP Statuses

| HTTP | Meaning |
| --- | --- |
| `200` | Success |
| `400` | Validation failure, malformed request body, or invalid query value |
| `401` | Missing or invalid SDKWork auth context, or incomplete AppContext projection |
| `403` | Permission denied or principal-to-resource binding violation |
| `404` | Referenced conversation, message, media asset, node, or policy version does not exist |
| `409` | Version conflict, membership conflict, route migration conflict, or invalid lifecycle transition |
| `501` | Requested provider capability is not implemented |
| `503` | Provider, registry, or runtime dependency is unavailable |

## Client Guidance

1. Branch on `code` for application handling. Do not depend on the exact wording of `message`.
2. For control-plane clients, use both the HTTP status code and the response `status` field.
3. Treat operation pages as the source for endpoint-specific conflicts and resource-not-found cases.
4. Use the SDK pages as the source for language-surface questions, and this page as the source for
   shared auth and error semantics.

## What To Read Next

- [App API Overview](/api-reference/app-api)
- [Control Plane API Overview](/api-reference/control-plane-api)
- [SDK Overview](/sdk/index)
