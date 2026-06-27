> Migrated from `docs/superpowers/plans/2026-04-17-craw-chat-unified-api-gateway.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Sdkwork IM Unified API Gateway Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a single-port built-in web gateway for all Sdkwork IM server APIs, publish live service-level and aggregate OpenAPI 3.1 contracts, and align docs and SDK inputs with the new authority-contract workflow.

**Architecture:** Introduce a dedicated `web-gateway` service and shared registry/openapi crates instead of expanding `local-minimal-node` into the long-term external API owner. Keep existing services as business owners, add live OpenAPI export to each service, centralize path ownership in a route registry, and let the gateway generate aggregate schemas and schema indexes from those live contracts. Preserve `local-minimal-node` as an embedded/local runtime profile rather than the authority schema source.

**Tech Stack:** Rust, Axum, Tokio, WebSocket upgrade support, Serde, workspace OpenAPI helpers, Markdown docs, pnpm-based docs site, existing SDK generation pipeline

---

## File Structure

### Existing files to modify

- `Cargo.toml`
- `services/session-gateway/src/lib.rs`
- `services/session-gateway/src/main.rs`
- `services/control-plane-api/src/lib.rs`
- `services/control-plane-api/src/main.rs`
- `services/conversation-runtime/src/runtime/http.rs`
- `services/conversation-runtime/src/main.rs`
- `services/projection-service/src/http.rs`
- `services/projection-service/src/main.rs`
- `services/streaming-service/src/lib.rs`
- `services/streaming-service/src/main.rs`
- `services/im-call-runtime/src/lib.rs`
- `services/im-call-runtime/src/main.rs`
- `services/media-service/src/lib.rs`
- `services/media-service/src/main.rs`
- `services/notification-service/src/lib.rs`
- `services/notification-service/src/main.rs`
- `services/automation-service/src/lib.rs`
- `services/automation-service/src/main.rs`
- `services/audit-service/src/lib.rs`
- `services/audit-service/src/main.rs`
- `services/ops-service/src/lib.rs`
- `services/ops-service/src/main.rs`
- `services/local-minimal-node/src/node.rs`
- `services/local-minimal-node/src/node/build.rs`
- `services/local-minimal-node/src/main.rs`
- `sdks/sdkwork-im-sdk/openapi/README.md`
- `docs/sites/api-reference/index.md`
- `docs/sites/sdk/index.md`

### New files to create

- `crates/sdkwork-im-api-registry/Cargo.toml`
- `crates/sdkwork-im-api-registry/src/lib.rs`
- `crates/sdkwork-im-api-registry/tests/route_registry_test.rs`
- `crates/sdkwork-im-openapi/Cargo.toml`
- `crates/sdkwork-im-openapi/src/lib.rs`
- `crates/sdkwork-im-openapi/tests/openapi_aggregate_test.rs`
- `crates/sdkwork-im-cloud-gateway-config/Cargo.toml`
- `crates/sdkwork-im-cloud-gateway-config/src/lib.rs`
- `crates/sdkwork-im-cloud-gateway-observability/Cargo.toml`
- `crates/sdkwork-im-cloud-gateway-observability/src/lib.rs`
- `services/web-gateway/Cargo.toml`
- `services/web-gateway/src/lib.rs`
- `services/web-gateway/src/main.rs`
- `services/web-gateway/tests/http_proxy_test.rs`
- `services/web-gateway/tests/websocket_proxy_test.rs`
- `services/web-gateway/tests/openapi_index_test.rs`
- `openapi/aggregate/sdkwork-im-cloud-gateway.openapi.json`
- `openapi/aggregate/openapi-index.json`
- `openapi/public/sdkwork-im-im.openapi.yaml`
- `openapi/public/sdkwork-im-control.openapi.json`
- `openapi/services/README.md`
- `docs/sites/api-reference/gateway-overview.md`
- `docs/sites/api-reference/service-contracts.md`

---

### Task 1: Add The Shared Route Registry And Schema Index Model

**Files:**
- Create: `crates/sdkwork-im-api-registry/Cargo.toml`
- Create: `crates/sdkwork-im-api-registry/src/lib.rs`
- Create: `crates/sdkwork-im-api-registry/tests/route_registry_test.rs`
- Modify: `Cargo.toml`

- [ ] **Step 1: Write the failing registry tests**

Add tests that require:
- unique ownership for each `method + external path`
- support for method-level split ownership on the same path
- visibility and sdk-target metadata on registry entries
- websocket route metadata support

Example test shape:

```rust
#[test]
fn registry_rejects_duplicate_method_path_owner() {
    let result = build_registry(vec![
        route("projection-service", "GET", "/im/v3/api/chat/conversations/{conversationId}/messages"),
        route("conversation-runtime", "GET", "/im/v3/api/chat/conversations/{conversationId}/messages"),
    ]);
    assert!(result.is_err());
}
```

- [ ] **Step 2: Run the focused registry tests**

Run:

```bash
cargo test -p sdkwork-im-api-registry route_registry_test -- --nocapture
```

Expected: FAIL because the new registry crate does not exist yet.

- [ ] **Step 3: Implement the registry crate**

Define:
- service descriptors
- route descriptors
- visibility enums
- sdk target enums
- protocol enums
- conflict validation helpers
- schema index entry types

- [ ] **Step 4: Re-run the registry tests**

Run:

```bash
cargo test -p sdkwork-im-api-registry route_registry_test -- --nocapture
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml crates/sdkwork-im-api-registry
git commit -m "feat(api): add sdkwork im route registry"
```

### Task 2: Add Live OpenAPI 3.1 Export To Standalone Services

**Files:**
- Modify: `services/session-gateway/src/lib.rs`
- Modify: `services/control-plane-api/src/lib.rs`
- Modify: `services/conversation-runtime/src/runtime/http.rs`
- Modify: `services/projection-service/src/http.rs`
- Modify: `services/streaming-service/src/lib.rs`
- Modify: `services/im-call-runtime/src/lib.rs`
- Modify: `services/media-service/src/lib.rs`
- Modify: `services/notification-service/src/lib.rs`
- Modify: `services/automation-service/src/lib.rs`
- Modify: `services/audit-service/src/lib.rs`
- Modify: `services/ops-service/src/lib.rs`
- Create: `crates/sdkwork-im-openapi/Cargo.toml`
- Create: `crates/sdkwork-im-openapi/src/lib.rs`

- [ ] **Step 1: Write a failing service-schema smoke test**

Create one focused test first for a small service, for example `session-gateway`, that requires:
- `GET /openapi.json`
- `GET /docs`
- `openapi == 3.1.0`
- route presence for `/im/v3/api/realtime/ws`

Example:

```rust
#[tokio::test]
async fn session_gateway_exports_openapi_json() {
    let app = session_gateway::build_public_app();
    let response = app.oneshot(Request::get("/openapi.json").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
```

- [ ] **Step 2: Run the focused service test**

Run:

```bash
cargo test -p session-gateway openapi -- --nocapture
```

Expected: FAIL because the route does not exist yet.

- [ ] **Step 3: Implement shared OpenAPI helpers**

Use `sdkwork-im-openapi` to provide:
- service spec metadata
- JSON serialization helpers
- docs page rendering helpers
- route inventory validation helpers

- [ ] **Step 4: Add `/openapi.json` and `/docs` to each standalone service**

Each standalone service should expose:
- `GET /openapi.json`
- `GET /docs`

Keep the schema source live and service-owned.

- [ ] **Step 5: Re-run the service-schema tests**

Run:

```bash
cargo test -p session-gateway -p control-plane-api -p conversation-runtime -p projection-service openapi -- --nocapture
```

Expected: PASS for the new schema routes on the covered services.

- [ ] **Step 6: Commit**

```bash
git add crates/sdkwork-im-openapi services/session-gateway services/control-plane-api services/conversation-runtime services/projection-service services/streaming-service services/im-call-runtime services/media-service services/notification-service services/automation-service services/audit-service services/ops-service
git commit -m "feat(api): add live openapi export to sdkwork im services"
```

### Task 3: Introduce The Single-Port Web Gateway

**Files:**
- Create: `crates/sdkwork-im-cloud-gateway-config/Cargo.toml`
- Create: `crates/sdkwork-im-cloud-gateway-config/src/lib.rs`
- Create: `crates/sdkwork-im-cloud-gateway-observability/Cargo.toml`
- Create: `crates/sdkwork-im-cloud-gateway-observability/src/lib.rs`
- Create: `services/web-gateway/Cargo.toml`
- Create: `services/web-gateway/src/lib.rs`
- Create: `services/web-gateway/src/main.rs`
- Create: `services/web-gateway/tests/http_proxy_test.rs`

- [ ] **Step 1: Write the failing gateway HTTP routing tests**

Add tests that require:
- `/healthz`
- `/readyz`
- `/backend/v3/api/control/*` proxies to `control-plane-api`
- `GET` and `POST` on the same path may resolve to different upstreams

Example:

```rust
#[tokio::test]
async fn gateway_routes_conversation_reads_and_writes_to_different_upstreams() {
    let gateway = test_gateway();
    assert_owner(&gateway, Method::GET, "/im/v3/api/chat/conversations/c_1/messages", "projection-service").await;
    assert_owner(&gateway, Method::POST, "/im/v3/api/chat/conversations/c_1/messages", "conversation-runtime").await;
}
```

- [ ] **Step 2: Run the gateway HTTP tests**

Run:

```bash
cargo test -p web-gateway http_proxy_test -- --nocapture
```

Expected: FAIL because the gateway service does not exist yet.

- [ ] **Step 3: Implement gateway config and routing**

Add:
- web bind configuration
- upstream service configuration
- registry-backed request classification
- HTTP reverse proxy handlers
- strict vs best-effort startup checks

- [ ] **Step 4: Re-run the gateway HTTP tests**

Run:

```bash
cargo test -p web-gateway http_proxy_test -- --nocapture
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml crates/sdkwork-im-cloud-gateway-config crates/sdkwork-im-cloud-gateway-observability services/web-gateway
git commit -m "feat(api): add single-port web gateway"
```

### Task 4: Add WebSocket Proxying And Realtime Protocol Metadata

**Files:**
- Modify: `services/web-gateway/src/lib.rs`
- Create: `services/web-gateway/tests/websocket_proxy_test.rs`
- Modify: `services/session-gateway/src/lib.rs`
- Modify: `crates/sdkwork-im-api-registry/src/lib.rs`

- [ ] **Step 1: Write the failing websocket proxy test**

Require:
- `GET /im/v3/api/realtime/ws` upgrades through the gateway
- auth headers are forwarded
- websocket subprotocols are preserved
- close codes from the upstream are visible

- [ ] **Step 2: Run the websocket test**

Run:

```bash
cargo test -p web-gateway websocket_proxy_test -- --nocapture
```

Expected: FAIL because websocket proxy support is not implemented yet.

- [ ] **Step 3: Implement websocket route support**

Add:
- websocket registry entries
- gateway upgrade forwarding
- protocol metadata hooks for OpenAPI vendor extensions
- runtime logging for websocket upstream resolution

- [ ] **Step 4: Re-run the websocket test**

Run:

```bash
cargo test -p web-gateway websocket_proxy_test -- --nocapture
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add services/web-gateway services/session-gateway crates/sdkwork-im-api-registry
git commit -m "feat(api): proxy realtime websocket through gateway"
```

### Task 5: Generate Aggregate OpenAPI, Schema Index, And Startup Summary

**Files:**
- Modify: `services/web-gateway/src/lib.rs`
- Modify: `services/web-gateway/src/main.rs`
- Create: `services/web-gateway/tests/openapi_index_test.rs`
- Modify: `crates/sdkwork-im-openapi/src/lib.rs`
- Modify: `crates/sdkwork-im-cloud-gateway-observability/src/lib.rs`
- Create: `openapi/aggregate/sdkwork-im-cloud-gateway.openapi.json`
- Create: `openapi/aggregate/openapi-index.json`
- Create: `openapi/public/sdkwork-im-im.openapi.yaml`
- Create: `openapi/public/sdkwork-im-control.openapi.json`

- [ ] **Step 1: Write the failing aggregate-schema tests**

Require:
- `/openapi.json`
- `/openapi/index.json`
- `/openapi/services/session-gateway.openapi.json`
- `/im/v3/openapi.json`
- startup summary includes schema and gateway URLs

- [ ] **Step 2: Run the aggregate-schema tests**

Run:

```bash
cargo test -p web-gateway openapi_index_test -- --nocapture
```

Expected: FAIL because aggregate schema routes and startup reporting do not exist yet.

- [ ] **Step 3: Implement schema index and aggregate generation**

Add:
- service schema discovery
- aggregate path merge from live contracts plus registry filtering
- public app contract extraction
- control contract extraction
- startup summary formatting helpers

- [ ] **Step 4: Re-run the aggregate tests**

Run:

```bash
cargo test -p web-gateway -p sdkwork-im-openapi openapi -- --nocapture
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add services/web-gateway crates/sdkwork-im-openapi crates/sdkwork-im-cloud-gateway-observability openapi
git commit -m "feat(api): add aggregate schemas and schema index"
```

### Task 6: Align Authority Snapshots, Docs, And SDK Inputs

**Files:**
- Modify: `services/local-minimal-node/src/node.rs`
- Modify: `services/local-minimal-node/src/node/build.rs`
- Modify: `sdks/sdkwork-im-sdk/openapi/README.md`
- Modify: `docs/sites/api-reference/index.md`
- Modify: `docs/sites/sdk/index.md`
- Create: `docs/sites/api-reference/gateway-overview.md`
- Create: `docs/sites/api-reference/service-contracts.md`

- [ ] **Step 1: Write the failing drift and documentation checks**

Add checks that require:
- public app snapshot comes from the new `openapi/public` authority location
- docs reference the single gateway base URL and new schema endpoints
- the old app schema path is no longer described as the only authority source

- [ ] **Step 2: Run the drift checks**

Run:

```bash
rg -n "sdkwork-im-im.openapi.yaml|openapi/index.json|single-port|gateway" docs sdks services/local-minimal-node
```

Expected: missing or inconsistent references.

- [ ] **Step 3: Update authority-contract references**

Make:
- `openapi/public/*` the documented authority source
- `sdks/*` a compatibility mirror
- docs site reference the gateway aggregate and schema index
- `local-minimal-node` described as embedded/local profile

- [ ] **Step 4: Re-run the drift checks**

Run:

```bash
rg -n "openapi/public|openapi/index.json|gateway-overview|service-contracts" docs sdks services/local-minimal-node
```

Expected: PASS with the new references present.

- [ ] **Step 5: Commit**

```bash
git add services/local-minimal-node sdks/sdkwork-im-sdk/openapi/README.md docs/sites/api-reference/index.md docs/sites/sdk/index.md docs/sites/api-reference/gateway-overview.md docs/sites/api-reference/service-contracts.md
git commit -m "docs(api): align gateway authority contracts"
```

### Task 7: Add CI Drift Guards And Rollout Verification

**Files:**
- Create: `crates/sdkwork-im-openapi/tests/openapi_aggregate_test.rs`
- Modify: `crates/sdkwork-im-api-registry/src/lib.rs`
- Modify: `services/web-gateway/tests/openapi_index_test.rs`
- Modify: `Cargo.toml`

- [ ] **Step 1: Write failing drift tests**

Require:
- registry and service schemas agree on owned operations
- aggregate schema excludes orphan operations
- strict mode fails startup when required schemas are unavailable

- [ ] **Step 2: Run the drift tests**

Run:

```bash
cargo test -p sdkwork-im-api-registry -p sdkwork-im-openapi -p web-gateway drift -- --nocapture
```

Expected: FAIL before the checks are fully implemented.

- [ ] **Step 3: Implement drift guards**

Add:
- registry vs service-schema reconciliation
- orphan operation detection
- strict startup validation
- best-effort degraded-state reporting

- [ ] **Step 4: Re-run the drift tests**

Run:

```bash
cargo test -p sdkwork-im-api-registry -p sdkwork-im-openapi -p web-gateway drift -- --nocapture
```

Expected: PASS

- [ ] **Step 5: Run the broader verification slice**

Run:

```bash
cargo test -p sdkwork-im-api-registry -p sdkwork-im-openapi -p web-gateway -p session-gateway -p control-plane-api -p conversation-runtime -p projection-service -- --nocapture
```

Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml crates/sdkwork-im-api-registry crates/sdkwork-im-openapi services/web-gateway
git commit -m "test(api): enforce gateway and schema drift guards"
```

## Rollout Notes

- Ship service-level live OpenAPI before routing traffic through the new gateway.
- Keep direct service ports available during rollout for debugging only.
- Switch docs and SDK regeneration to the new authority snapshots only after aggregate generation is stable.
- Do not remove `local-minimal-node` support until the gateway, public app contract, and control contract are verified in CI.

## Final Verification Checklist

- [ ] `cargo test -p sdkwork-im-api-registry -- --nocapture`
- [ ] `cargo test -p sdkwork-im-openapi -- --nocapture`
- [ ] `cargo test -p web-gateway -- --nocapture`
- [ ] `cargo test -p session-gateway -p control-plane-api -p conversation-runtime -p projection-service -- --nocapture`
- [ ] start the gateway locally and verify `/healthz`, `/openapi.json`, `/openapi/index.json`, `/docs`, and `/im/v3/api/realtime/ws`
- [ ] confirm docs and SDK inputs point at the new authority snapshots

