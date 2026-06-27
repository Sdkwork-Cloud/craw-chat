# 2026-06-25 WebSocket 登录验证标准（SDKWork IAM 对齐）

## 1. Goal

将 Sdkwork IM 的 WebSocket 登录验证能力沉淀为 **跨前后端一致、跨网关一致、跨语言 SDK 一致** 的行业标准化能力：

- **浏览器约束优先**：浏览器原生 WebSocket 无法携带自定义 header；必须支持 `Upgrade` 后再通过首帧鉴权。
- **IAM 权威对齐**：所有登录验证必须以 `sdkwork-iam` 体系为权威来源（双 token 会话验证 + AppContext 投影）。
- **统一的连接级错误模型**：错误帧、错误码、关闭码、fatal/non-fatal 语义在 SDK 与服务端一致。
- **统一覆盖面**：适用于 **gateway registry 标记为 WebSocket 的所有路由**（不仅仅 realtime）。
- **可验证**：有契约测试矩阵（Rust gateway tests + TypeScript SDK contract tests）保证行为长期稳定。

## 2. Non-goals

- 不把 `authToken` / `accessToken` 放入 URL query、子协议（subprotocol）、或其它 handshake 元信息中。
- 不在本规范内引入新的 token 格式（本规范只定义“如何验证现有双 token 会话”）。
- 不将业务订阅/权限策略前置到 Upgrade 阶段（业务策略仍在 CCP/业务帧层完成）。

## 3. Background & Ground Truth

### 3.1 SDK 现有约定（必须保持）

TypeScript SDK（`@sdkwork/im-sdk`）的浏览器运行时约定：

- 连接 `/im/v3/api/realtime/ws` 时先完成 WebSocket Upgrade。
- WebSocket `open` 后发送首帧 `auth.init`（携带双 token + deviceId）。
- 收到 `auth.ok` 后才开始 CCP `hello/auth_bind`，并在连接进入 open 后发送 `subscriptions.sync`。

权威代码与文档：

- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/realtime.ts`
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md`
- 契约测试：`scripts/dev/sdkwork-im-sdk-websocket-contract.test.ts`

### 3.2 网关现有能力（可复用的正确形态）

`sdkwork-im-cloud-gateway` 已实现：

- 对 realtime websocket 在无 header 场景下：`Upgrade -> read auth.init -> IAM 校验 -> auth.ok -> 连接 upstream -> 代理帧`
- 对 query 清洗、header 清洗、fatal 错误关闭等具备测试覆盖

权威实现与测试：

- `services/sdkwork-im-cloud-gateway/src/lib.rs`
- `services/sdkwork-im-cloud-gateway/tests/websocket_proxy_test.rs`

## 4. Standard Terms

### 4.1 Dual-token

本标准的“会话登录验证”基于双 token：

- `authToken`：授权 token（语义上对应 `Authorization: Bearer <token>`）
- `accessToken`：访问 token（语义上对应 `Access-Token: <token>` header）

### 4.2 AppContext

`AppContext` 是 **可信权威上下文**，包含但不限于：

- tenantId / organizationId
- userId / actorId / actorKind
- sessionId
- deviceId
- permissionScope / dataScope

规则：任何业务层字段不得替代 AppContext 字段的权威性（“Authoritative context principle”）。

## 5. Normative Protocol (wire)

### 5.1 `auth.init` frame (client -> gateway)

首帧鉴权必须使用 JSON 文本帧（Text frame）。字段（camelCase）：

```json
{
  "type": "auth.init",
  "requestId": "sdkwork-im-auth-init-1",
  "authToken": "<auth-token>",
  "accessToken": "<access-token>",
  "deviceId": "<device-id>"
}
```

规则：

- `type` MUST 等于 `auth.init`。
- `authToken` MUST 存在且非空。
- `accessToken` MUST 存在且非空。
- `deviceId` SHOULD 提供（用于路由与会话围栏）；若提供，必须满足 device 绑定规则（见 7.2）。
- `requestId` SHOULD 提供；用于错误关联与调试。

禁止：

- token 不得出现在 URL query、subprotocol、或任何 header 注入（浏览器不可控且不可审计）。

### 5.2 `auth.ok` frame (gateway -> client)

网关完成 IAM 校验后回发 `auth.ok`（JSON Text frame）：

```json
{
  "type": "auth.ok",
  "requestId": "sdkwork-im-auth-init-1",
  "tenantId": "<tenant-id>",
  "principalId": "<user-id>",
  "sessionId": "<session-id>",
  "deviceId": "<device-id>"
}
```

规则：

- `type` MUST 等于 `auth.ok`。
- 若 client 提供了 `requestId`，server SHOULD 回显。
- `tenantId` 与 `principalId` MUST 来自 `sdkwork-iam` 校验后的权威会话投影。

### 5.3 Error frame (gateway/service -> client)

连接级错误统一使用：

```json
{
  "type": "error",
  "requestId": "<optional-request-id>",
  "code": "<stable-error-code>",
  "message": "<human-readable>"
}
```

规则：

- `code` MUST 为稳定字符串（SDK 依赖该分类决定 fatal/non-fatal 行为）。

## 6. Handshake State Machine

### 6.1 States

连接必须遵循顺序状态机（浏览器模式）：

1. `upgrade_pending`：等待 WebSocket upgrade 完成
2. `gateway_auth_pending`：等待 `auth.init`
3. `gateway_auth_ok`：已通过 IAM 校验并发送 `auth.ok`
4. `ccp_handshake_pending`：等待 CCP hello/ack/auth_ok
5. `ready`：可收发业务帧

### 6.2 Ordering Rules

- 在 `gateway_auth_pending` 期间：
  - 允许 `ping/pong`。
  - 禁止业务帧；收到业务帧 MUST 返回 `websocket_auth_required` 并关闭连接。
- 在收到 `auth.ok` 之前，SDK MUST NOT 发送 CCP `hello` 或业务订阅帧。
- 在 CCP 握手完成（`auth_ok` 控制帧之后，连接 phase 进入 `ready`）之前，SDK MUST NOT 发送 `subscriptions.sync` 或其它 `kind: cmd` 业务帧；服务端在握手阶段收到非 `control` 帧会返回 `CCP_CONTROL_REQUIRED` 并关闭连接。
- `ImSdkClient.connect()` 同步返回 `ImLiveConnection` 句柄，不代表握手已完成；应用层连接管理器 MUST 在 lifecycle `open` 之后再触发订阅同步（或由 SDK 在 `ready` phase 自动 flush 脏订阅快照）。

## 7. IAM Authority & Device Binding

### 7.1 Authority Rule (IAM-first)

所有 `auth.init` 验证 MUST 使用 `sdkwork-iam` 权威：

- 推荐路径（标准部署）：**Application Gateway** 调用 `sdkwork-iam-app-api` current-session（或等价 IAM adapter）验证双 token，并生成可信 `AppContext`。
- 兼容路径（绕过 gateway / embedded / dev 工具链）：上游服务可在受控前提下使用 IAM pool/adapter 直接验证双 token。
  - 兼容路径 MUST 遵循相同的错误码与关闭码。
  - 兼容路径不得成为生产推荐部署形态。

### 7.2 Device Rule

当 `auth.init` 提供 `deviceId` 时：

- 若 IAM 会话投影中包含 deviceId：
  - MUST 与 `auth.init.deviceId` 一致，否则返回 `device_id_mismatch`（错误帧）并关闭连接。
- 若 IAM 会话投影未包含 deviceId：
  - server MAY 采用 `auth.init.deviceId` 作为会话 device 绑定（需记录为权威上下文的一部分）。

## 8. Gateway Responsibilities (standard behavior)

本标准要求 application gateway 对所有 registry-owned WebSocket routes 提供一致能力。

### 8.1 Upgrade policy

- 浏览器模式：在缺失双 token header 的情况下，gateway MUST 允许 upgrade 先成功，随后才进入 `auth.init` 流程。
  - 不能在 HTTP upgrade 阶段强制解析 `Authorization` header（否则会导致 426/401 与浏览器不兼容）。

浏览器模式判定（normative）：

- 当请求满足：
  - `Authorization` header 不存在，且
  - `access-token` / `Access-Token` header 不存在
- gateway MUST 将该连接视为“首帧鉴权模式”，并在 upgrade 后进入 `gateway_auth_pending`。

### 8.2 Query sanitization

gateway 在连接 upstream 之前 MUST 清洗敏感 query 参数：

- 必须删除（示例）：`authToken`, `accessToken`, `authorization`, `token`, `refreshToken` 等。
- 仅允许保留 `deviceId`（realtime path 的必要字段）。

### 8.3 Header forwarding rules

gateway MUST：

- 不转发客户端提供的 `x-sdkwork-*` 内部上下文头（防伪造）。
- 不转发 `Authorization` / `Access-Token` 到 upstream（避免泄漏）。
- 只转发 gateway 生成的可信 AppContext 投影头（包含签名机制时必须遵循签名规范）。

### 8.4 Error & close behavior

gateway 在连接级错误上必须保证：

- 发送 error frame（尽量带 requestId）
- 使用稳定 close code 关闭连接（见 10）。

## 9. Upstream Service Responsibilities

上游 WebSocket 服务（如 session-gateway）应满足：

- 如果收到了 gateway 投影的可信 AppContext：直接进入自身协议（如 CCP）处理。
- 如果处于兼容路径并接收 `auth.init`：必须走 IAM authority（IAM pool/adapter），不得走“生产环境 header-only fallback”。

## 10. Error Code & Close Code Standard

### 10.1 Stable error codes (partial baseline)

以下 error code MUST 作为稳定契约（SDK 已按前缀做 fatal 分类，服务端不得随意改名）：

- `websocket_auth_required`：在 `auth.init` 之前收到业务帧或缺少必须字段
- `websocket_auth_failed`：双 token 无效、会话过期、签名/上下文验证失败
- `websocket_auth_timeout`：在规定时间内未收到 `auth.init` 或未完成 `auth.ok`
- `websocket_upstream_unavailable`：网关无法连接 upstream
- `websocket_connect_timeout`：连接建立超时（SDK 侧）
- `CCP_CONTROL_REQUIRED`：CCP 握手未完成时收到 `kind: cmd` / `kind: ack` 等业务帧（常见于过早 `subscriptions.sync`）

还需要规范化的业务/策略类错误（通常 non-fatal）：

- `subscription_forbidden`
- `realtime_scope_access_denied`

### 10.2 Close codes

标准 close codes MUST 稳定，并与 SDK 的重连策略一致：

- `4401`：鉴权失败/鉴权要求（auth required/failed）
- `4408`：超时（auth timeout / heartbeat timeout / connect timeout）

注：WebSocket RFC 标准 close code 是 1000/1008 等；本标准采用应用层 close code（与现有 SDK 行为一致）。服务端实现需确保跨平台兼容性（部分浏览器对非标准 close code 的可见性不同，但仍应发送 error frame 做主渠道）。

## 11. Coverage: registry-owned WebSocket routes

本标准适用于 gateway registry 标注协议为 `Websocket` 的所有路径。

分类规则：

- **Gateway-owned WebSocket route**：由 gateway proxy 负责连接与帧代理。
- **Service-owned WebSocket route**：不在 gateway registry 中的 websocket 路由，其标准化由该服务自行承担，但仍建议复用同一 auth-init gate（避免分裂）。

## 12. Compatibility & Migration

### 12.1 Compatibility matrix

| Runtime | Transport | Upgrade headers | Query params | Post-upgrade auth | CCP start |
| --- | --- | --- | --- | --- | --- |
| Browser (`globalThis.WebSocket`) | 无自定义 header | 无 token query | 仅 `deviceId` | `auth.init` → `auth.ok` | `auth.ok` 后 |
| PC/Tauri/Node（`webSocketFactory`） | `Authorization` + `Access-Token` | 仅 `deviceId` | 跳过 `auth.init` | Upgrade 后立即 `hello` |
| Flutter/iOS/Android（`IOWebSocketChannel`） | `Authorization` + `Access-Token` | 仅 `deviceId` | 跳过 `auth.init` | 连接就绪后立即 `hello` |
| 匿名/测试（`ImWebSocketAuthOptions.none()`） | 无 | 无 token | 可选 `deviceId` | 无 IAM 校验 | 直接 `hello`（仅 dev/test） |
| 无 query 参数（非 parameter） | 视运行时 | 无 `deviceId` query | 无 token query | `deviceId` 来自 `auth.init` 帧或 JWT `device_id` claim | 服务端 `resolve_requested_device_id` |
| 仅 query `deviceId`（非 token parameter） | 浏览器/Native | 无 token header | `?deviceId=` | 浏览器：`auth.init` 可省略帧内 `deviceId`（回退 query）；Native：header 鉴权 + query 绑定 | 见 `coalesce_websocket_device_id` |
| 遗留 query token（**非推荐**） | 无 | token query（需 `SDKWORK_IM_GATEWAY_ALLOW_WEBSOCKET_QUERY_TOKENS=true`，且 **非** realtime path） | gateway 转 header 后代理 | 等同 header 模式 |

判定规则（normative）：

- `has_websocket_upgrade_auth_headers(headers)` 为 true → **header 鉴权模式**，不得要求 `auth.init`。
- 否则且 realtime path → **首帧鉴权模式**，允许匿名 Upgrade，首帧 MUST 为 `auth.init`。
- `deviceId` MAY 出现在 query；header 模式下若 JWT 未携带 `device_id`，server MUST 接受 query `deviceId` 并与 IAM 投影对齐（`resolve_requested_device_id`）。
- `auth.init` 模式下，帧内 `deviceId` 优先于 query `deviceId`（`coalesce_websocket_device_id`）；两者皆无时回退 JWT `device_id` claim。

权威共享实现：

- `crates/sdkwork-im-websocket-auth-gate/` — frame gate, query policy, device binding, Axum I/O
- `crates/im-app-context/src/lib.rs` — upgrade header detection and query `deviceId` helpers

### 12.2 Migration strategy

阶段化：

1. 将 auth-init gate 抽象为 gateway 共享组件，cloud/standalone 共用
2. 所有 registry websocket routes 接入统一 gate
3. session-gateway 保留最小兼容模式（非推荐部署）
4. 文档与 SDK contract 形成稳定契约；新增 breaking change 必须走 ADR/版本治理

## 13. Verification & Test Matrix (must-pass)

### 13.1 Rust gateway tests

基线已有：`services/sdkwork-im-cloud-gateway/tests/websocket_proxy_test.rs`

必须补齐（对所有 registry websocket routes）：

- upgrade succeeds without headers
- auth.init required before business frame
- ping before auth.init allowed
- auth.init timeout
- deviceId mismatch
- query sanitization
- internal header stripping

### 13.2 TypeScript SDK contract test

基线已有：`scripts/dev/sdkwork-im-sdk-websocket-contract.test.ts`

必须保持并扩展：

- browser waits for auth.ok before marking open
- `subscriptions.sync` MUST NOT be sent before CCP `ready` (even if `syncConversations` is called early)
- fatal error closes socket & emits lifecycle error
- non-fatal server error does not close socket
- heartbeat liveness semantics remain stable

## 14. Work items (implementation guide)

本规范的实现工作应输出：

- 共享 gateway auth-init 组件（cloud/standalone 共用）
- 全路由接入（registry websocket routes）
- 完整测试矩阵
- 文档对外说明（协议 + 错误模型 + 安全）

### 14.1 Implemented (2026-06-25)

- **Websocket route mounting**: `/im/v3/api/realtime/ws` is mounted outside `WebFrameworkLayer` in
  `sdkwork-routes-im-realtime-open-api` and unified gateways mount the embedded websocket router before the wrapped
  business router. This prevents Axum upgrade-state loss and HTTP `426` in browser clients.
- **Embedded dispatch rule**: `dispatch_embedded_session_gateway` skips `REALTIME_WS`; websocket upgrades are served from
  the merged router, while other `/im/v3/api/realtime/*` and `/im/v3/api/presence/*` HTTP routes continue to use oneshot
  dispatch.
- **Regression tests**: `session-gateway/tests/websocket_http_framework_upgrade_test.rs` and
  `sdkwork-im-standalone-gateway` embedded-plane websocket handshake test.

### 14.2 PC client shared connection (2026-06-25)

- **Manager**: `apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/pcRealtimeConnectionManager.ts` owns the sole
  browser WebSocket for chat, contacts, and incoming-call watch.
- **Consumers**: `ChatService`, `ContactService`, and `CallService` subscribe through the manager; they do not call
  `ImSdkClient.connect()` directly.
- **Recovery**: browser `online` / `visibilitychange` and exponential reconnect with circuit breaker live in the manager;
  recovery is skipped while the connection is already `open` or `connecting`.
- **Contract tests**: `apps/sdkwork-im-pc/scripts/pc-realtime-connection-contract.test.ts` and
  `apps/sdkwork-im-pc/scripts/chat-auth-realtime-guard-contract.test.ts`.

### 14.3 Postgres checkpoint org-scope binding (2026-06-25)

- **Symptom**: browser `auth.init` returned `checkpoint_store_unavailable` with
  `expected 3 parameters but got 2`, causing client reconnect loops.
- **Cause**: `im_realtime_checkpoints` queries were migrated to
  `(tenant_id, organization_id, client_route_scope_key)` but `im-adapters-postgres-realtime` still bound only
  `tenant_id` and `client_route_scope_key`.
- **Fix**: bind `organization_id` in all checkpoint/subscription/event-window clear/load paths; include
  `organization_id` in checkpoint/subscription SELECT projections.

### 14.4 Postgres route binding serialization (2026-06-25)

- **Symptom**: browser `auth.init` returned `route_store_write_failed` with
  `error serializing parameter 9` while persisting `im_route_bindings`.
- **Cause**: route binding upsert used unstable temporary bind values (`&(route_epoch as i64)`, string
  `bound_at` with `$10::timestamptz`) instead of stable `i64` / `DateTime<Utc>` parameters aligned with
  other postgres-realtime adapters.
- **Fix**: `adapters/postgres-realtime/src/route_store.rs` now binds `organization_id`, `route_epoch`,
  `session_id`, and `bound_at` through locals and chrono timestamps.

### 14.5 Shared websocket auth gate (2026-06-25)

- **Crate**: `crates/sdkwork-im-websocket-auth-gate` — single owner for `auth.init` frame parsing,
  IAM dual-token header projection, `auth.ok` / error frames, query sanitization, and device binding.
- **Consumers**: `sdkwork-im-cloud-gateway` (registry websocket proxy) and `session-gateway` (embedded
  compat path when upgrade arrives without headers).
- **Helpers in `im-app-context`**: `has_websocket_upgrade_auth_headers`,
  `websocket_query_device_id_from_path_and_query`, `coalesce_websocket_device_id`.
- **Verification**: `cargo test -p sdkwork-im-websocket-auth-gate`, `cargo test -p sdkwork-im-cloud-gateway websocket`,
  `cargo test -p session-gateway --test websocket_auth_init_test`, `pnpm test:sdkwork-im-sdk-websocket-contract`.

### 14.6 CCP handshake subscription guard (2026-06-25)

- **Symptom**: after `auth.ok`, server returned `CCP_CONTROL_REQUIRED` with
  `expected control envelope, got kind cmd` (schema `cc.control.error.v1`).
- **Cause**: TypeScript SDK `flushSubscriptionSync()` only checked websocket
  `OPEN` state, not `connectionPhase === 'ready'`. PC `pcRealtimeConnectionManager`
  called `syncWireSubscriptions()` as soon as `connect()` resolved (before CCP
  `hello` / `auth_bind` / `auth_ok` completed).
- **Fix**:
  - `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/realtime.ts` — defer
    `subscriptions.sync` until CCP `ready` (aligned with Flutter SDK).
  - `apps/sdkwork-im-pc/.../pcRealtimeConnectionManager.ts` — bind lifecycle
    `open` before wire subscription sync; do not mark connection `open` early.
- **Verification**: `pnpm test:sdkwork-im-sdk-websocket-contract`,
  `apps/sdkwork-im-pc/scripts/pc-realtime-connection-contract.test.ts`.

## 16. Gateway Refactor Plan (cloud + standalone reuse)

**Status: implemented (2026-06-25).** Shared gate crate shipped; cloud gateway and session-gateway compat path
both delegate to `sdkwork-im-websocket-auth-gate`. Upstream session-gateway `auth.init` handler is compat-only
for embedded/unified deployments that bypass the public gateway.

### 16.1 Refactor boundaries

- **Shared unit**（建议新增 crate / 模块，供两个 gateway 复用）：
  - 负责：
    - browser-like websocket 判定（无 auth headers）
    - `auth.init` 读取、大小限制、超时、ping/pong 容忍
    - 调用 IAM authority 校验双 token
    - 生成 `auth.ok` / `error` 帧与 close behavior
    - query 清洗（token 字段）与 header forwarding 策略（禁止 `x-sdkwork-*` 注入）
  - 不负责：
    - 具体 upstream websocket 协议（CCP / echo / custom）
    - 业务订阅语义

- **Gateway-owned WebSocket proxy**（现有 proxy 路由层）：
  - 在连接 upstream 前，调用 shared unit 获取 “可信 AppContext + deviceId + sanitized path/query + upstream headers”。
  - 之后做 upstream connect，并进入双向 frame proxy。

### 16.2 Implemented code layout

Auth-init gate logic lives in `crates/sdkwork-im-websocket-auth-gate/`:

- frame parsing / validation (`frame`)
- query sanitization and routing policy (`policy`)
- device binding (`device`)
- Axum read/send helpers (`axum_gate`)

Consumers:

- `services/sdkwork-im-cloud-gateway/src/lib.rs` — registry websocket proxy
- `services/session-gateway/src/websocket_auth_init.rs` — embedded compat path

Shared helpers in `crates/im-app-context/src/lib.rs`:

- `has_websocket_upgrade_auth_headers`
- `websocket_query_device_id_from_path_and_query`
- `coalesce_websocket_device_id`

### 16.3 Standalone gateway alignment

当 `sdkwork-im-standalone-gateway` 作为 public ingress 时：

- MUST 使用同一 shared unit 处理所有 registry websocket routes。
- embedded/upstream 的 session-gateway 仍可保留兼容能力，但标准路径必须经过 gateway gate。

### 16.4 Deprecation policy

- 当 cloud/standalone 网关都完成共享 gate 迁移后：
  - 上游服务（session-gateway）内的 auth-init 兼容实现进入 “compat only” 状态；
  - 新增 WebSocket 服务不得再自行实现一套 auth-init gate。

## 17. Test Matrix Plan (Rust + SDK contract)

目标：把“行业标准能力”变成 **可回归的契约**，避免未来改动再次出现 `426/401` 这类浏览器握手不兼容问题。

### 17.1 Rust: Gateway proxy + auth.init gate

基线（已存在，继续扩展）：

- `services/sdkwork-im-cloud-gateway/tests/websocket_proxy_test.rs`

新增覆盖（必须）：

- **Registry WebSocket route coverage**
  - 至少新增 1 条“非 realtime 的 registry websocket 路由”测试，用同一套 auth-init gate 保护：
    - upgrade ok without headers
    - auth.init ok -> auth.ok -> upstream connect ok
- **Auth-init frame guardrails**
  - oversized auth.init（> 8KiB） -> error + close
  - invalid JSON -> error + close
  - missing authToken/accessToken -> `websocket_auth_required` + close
  - auth.init timeout -> `websocket_auth_timeout` + close
  - second auth.init after auth.ok -> `websocket_auth_failed` + close
- **Device rules**
  - deviceId mismatch -> `device_id_mismatch` + close
  - deviceId omitted but IAM context has device -> ok (device derived from context)
- **Header & query sanitization**
  - query includes authToken/accessToken/etc -> upstream sees only allowed keys (e.g. deviceId)
  - client-sent `x-sdkwork-*` headers never reach upstream
- **Resource safety**
  - connection semaphore saturated -> controlled error response (no panic)

### 17.2 Rust: Upstream service compatibility (session-gateway)

基线（已存在，保持）：

- `services/session-gateway/tests/websocket_auth_init_test.rs`（compat mode）

新增覆盖（可选，但推荐）：

- auth.init replay rejection
- auth.init invalid json/oversize guardrails

### 17.3 TypeScript: SDK contract test

基线（已存在，必须保持稳定）：

- `scripts/dev/sdkwork-im-sdk-websocket-contract.test.ts`

新增覆盖（必须）：

- browser runtime：
  - `auth.ok` payload requirements（tenantId/principalId/sessionId/deviceId）
  - server returns `error` before close for auth failures
- non-realtime registry websocket route（如果 SDK 暴露此能力）：
  - SDK 不必直接支持所有 websocket route，但至少要保证“realtime 路由的行业标准能力”不回退。

### 17.4 CI / Verification commands (narrow-first)

- Rust：`cargo test -p sdkwork-im-cloud-gateway`（网关 proxy 行为）
- Rust：`cargo test -p session-gateway`（上游 compat 与 CCP 行为）
- Node：`pnpm test:sdkwork-im-sdk-websocket-contract`（若存在对应 script；否则将该脚本纳入现有 verify 链路）


## 15. Security Requirements (normative)

### 15.1 Token confidentiality

gateway MUST treat `authToken` / `accessToken` as secrets:

- MUST NOT log raw tokens（包括但不限于 query、frame payload、header）
- MUST NOT forward them to upstream
- MUST NOT allow them in URL query

### 15.2 Replay / re-auth on a socket

- A websocket connection MUST accept at most one successful authentication transition.
- After `gateway_auth_ok`, receiving another `auth.init` MUST fail closed with `websocket_auth_failed`.

### 15.3 DoS & resource safety

auth-init gate MUST enforce:

- max auth.init frame bytes（recommended: 8 KiB）
- auth.init timeout（recommended: 10 s）
- max total websocket message/frame sizes aligned with existing guardrails
- connection admission control（semaphore）must fail closed with controlled error codes (no panics, no resource leak)

