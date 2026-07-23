# Authentication And Errors

<p class="api-page-intro">
  Shared authentication, request-context, success-envelope, and RFC 9457 error rules for every
  SDKWork IM Open, App, and Backend HTTP operation.
</p>

<div class="api-link-list">
  <a href="/api-reference/im-api"><code>Open API</code> IM communication operations under <code>/im/v3/api</code></a>
  <a href="/api-reference/app-api"><code>App API</code> User-facing application operations under <code>/app/v3/api</code></a>
  <a href="/api-reference/backend-api"><code>Backend API</code> Operator and management operations under <code>/backend/v3/api</code></a>
</div>

## Credential Model

`sdkwork-appbase` and `sdkwork-iam` own login, token issuance, token refresh, Sessions, users,
tenants, organizations, and permission catalogs. SDKWork IM only validates credentials and consumes
the resolved request context.

Protected App and Backend operations use both SDKWork tokens:

```http
Authorization: Bearer <auth-token>
Access-Token: <access-token>
```

| Item | Contract |
| --- | --- |
| Identity owner | `sdkwork-iam` / `sdkwork-appbase` |
| Public credential model | SDKWork dual token |
| Request framework | `sdkwork-web-framework` |
| Resolved context | Typed `AppContext` |
| IM input | Verified tenant, organization, principal, Session, application, device, data scope, and permission scope |

Protected Open API operations use the security scheme declared by their authored OpenAPI operation.
IM client SDKs obtain credentials through their approved credential provider; application code must
not handcraft auth headers when a generated SDK client is available.

## Request Context Boundary

After validating credentials, `sdkwork-web-framework` resolves a typed `AppContext`. Tenant,
organization, principal, Session, application, device, actor, data scope, and permission scope are
derived from verified credentials and trusted server configuration, not request payload fields.

Public SDKs and manual callers must not send identity-context headers. A trusted edge may pass a
private signed context only after it has validated the public credentials and discarded any
client-supplied identity values. Internal services verify the signature and the resolved scope before
handling a protected operation.

Resources such as Conversation, Message, Agent, Drive file, RTC Session, or Knowledgebase space IDs
are never authorization evidence. The owning service checks tenant, organization, membership, role,
permission, and resource lifecycle for every read or mutation.

## Permission Model

| Permission | Grants |
| --- | --- |
| `ops.read` | Read health, cluster, operational lag, runtime inspection, provider state, diagnostics, and readiness evidence |
| `audit.read` | Read audit records and export evidence |
| `audit.write` | Record audit anchors |
| `control.read` | Read protocol, provider, social, and node governance state |
| `control.write` | Mutate governed provider, social, and node state |
| `conversation.shared_channel.sync` | Execute the shared-channel synchronization command as the trusted system actor |

Conversation and Message operations additionally enforce active membership and capability checks
defined by the operation contract.

## Success Envelope

JSON success responses use the canonical `SdkWorkApiResponse` envelope:

```json
{
  "code": 0,
  "data": {
    "item": {}
  },
  "traceId": "01HXY..."
}
```

- `code` is numeric `int32` and is always `0` for an HTTP `2xx` JSON body.
- Single-resource data uses `data.item`.
- List data uses `data.items` and `data.pageInfo`.
- Command data uses `data.accepted` and may include `resourceId` or `status`.
- Async acceptance uses HTTP `202` and an operation resource.
- HTTP `204` has no body.

## Error Envelope

HTTP `4xx` and `5xx` failures use `application/problem+json` with RFC 9457 `ProblemDetail`:

```json
{
  "type": "about:blank",
  "title": "Bad Request",
  "status": 400,
  "code": 40001,
  "detail": "page_size must be between 1 and 200",
  "instance": "GET /im/v3/api/chat/conversations/{conversationId}/messages",
  "operationId": "conversations.messages.list",
  "traceId": "01HXY...",
  "i18nKey": "errors.result.40001"
}
```

- `code` is a numeric non-zero platform result code.
- `instance` uses the resolved route template and never contains raw tenant, principal, Conversation,
  Message, file, token, or provider identifiers.
- `operationId` is copied from the matched authored OpenAPI operation.
- `traceId` is server-issued or propagated from approved trace headers.
- `i18nKey` is always `errors.result.<code>`; clients localize from this key and never parse `detail`.
- A business failure never uses HTTP `2xx`, a string wire code, a `success` boolean, or a human
  `message` field.

## Platform Result Codes

| HTTP | `code` | Meaning |
| --- | --- | --- |
| `400` | `40001` | Malformed body, invalid query, or validation failure |
| `401` | `40101` | Required credentials are missing |
| `401` | `40102` | Credentials are invalid, expired, or cannot resolve a valid request context |
| `403` | `40301` | Required permission is missing |
| `403` | `40302` | Principal-to-resource binding, membership, or actor rule failed |
| `404` | `40401` | Referenced resource does not exist in the caller's scope |
| `409` | `40901` | Version, idempotency, membership, or lifecycle conflict |
| `413` | `41301` | Payload exceeds the configured limit |
| `429` | `42901` | Rate limit exceeded |
| `501` | `50101` | Requested provider capability is not implemented |
| `503` | `50301` | Required database, provider, registry, or runtime dependency is unavailable |

Endpoint pages narrow these codes with operation-specific conflict and not-found conditions. They do
not redefine the envelope.

## Generated SDK Behavior

Generated SDKs use the OpenAPI-declared security schemes and unwrap `data` by default. Typed errors
expose numeric `ProblemDetail.code`, `traceId`, `operationId`, and `i18nKey`. Use the generated raw
response option only when an application needs the complete envelope or response headers.

## Client Rules

1. Construct clients once at bootstrap and inject the approved TokenManager or credential provider.
2. Branch on numeric `code`; never branch on localized or provider text.
3. Correlate client telemetry with `traceId` and retain no credential value in logs.
4. Treat `401` as an authentication lifecycle event and `403` as an authorization decision.
5. Retry only operations whose idempotency and concurrency contract permits retry.

## What To Read Next

- [Complete IM-owned HTTP API inventory](/api-reference/index)
- [IM Open API Overview](/api-reference/im-api)
- [App API Overview](/api-reference/app-api)
- [Backend API Overview](/api-reference/backend-api)
- [SDK Overview](/sdk/index)
