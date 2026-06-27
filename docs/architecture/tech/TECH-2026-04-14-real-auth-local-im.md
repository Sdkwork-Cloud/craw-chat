> Migrated from `docs/superpowers/plans/2026-04-14-real-auth-local-im.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Real Auth Local IM Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a real seeded-account login system for local IM and portal flows, replace mock portal auth/data with backend-backed APIs, and make chat scripts resilient enough for commercial-grade local end-to-end testing.

**Architecture:** Keep auth inside `services/local-minimal-node` so chat, realtime, RTC, and portal share one authority source. Issue HS256 access tokens compatible with existing `im-auth-context`, store opaque refresh tokens server-side, and move all clients to explicit login/bootstrap flows instead of client-side secret minting.

**Tech Stack:** Rust (`axum`, `serde`, `tokio`), existing `im-auth-context` JWT contract, PowerShell launch scripts, Node test runner, portal frontend packages.

---

### Task 1: Lock The Backend Auth Contract With Failing Tests

**Files:**
- Create: `services/local-minimal-node/tests/real_auth_e2e_test.rs`
- Modify: `services/local-minimal-node/tests/public_auth_e2e_test.rs`
- Modify: `crates/im-auth-context/tests/auth_context_test.rs`
- Reference: `services/local-minimal-node/src/node.rs`
- Reference: `services/local-minimal-node/src/node/build.rs`

- [ ] **Step 1: Write the failing auth login and refresh tests**

```rust
#[tokio::test]
async fn test_seeded_im_user_can_log_in_and_fetch_me() {
    let app = local_minimal_node::build_default_app_with_runtime_dir(temp_runtime_dir());

    let login = post_json(
        &app,
        "sdkwork-appbase login verification",
        json!({
            "tenantId": "100001",
            "login": "guest",
            "password": "Guest#2026",
            "clientKind": "im_user",
            "clientRouteId": "d_guest",
            "sessionId": "s_guest"
        }),
    ).await;

    assert_eq!(login.status(), StatusCode::OK);
    let body = read_json(login).await;
    let access_token = body["accessToken"].as_str().expect("access token");

    let me = get_with_bearer(&app, "sdkwork-iam context", access_token).await;
    assert_eq!(me.status(), StatusCode::OK);
    let me_body = read_json(me).await;
    assert_eq!(me_body["user"]["id"], "2");
}

#[tokio::test]
async fn test_refresh_rotates_and_revokes_previous_refresh_token() {
    // login -> refresh once -> old refresh rejected -> new refresh accepted
}
```

- [ ] **Step 2: Run the auth test file and verify it fails for missing routes**

Run: `cargo test -p local-minimal-node --test real_auth_e2e_test --quiet`
Expected: FAIL with `404`, missing route wiring, or missing auth types.

- [ ] **Step 3: Extend bearer contract tests for issued tokens**

```rust
#[test]
fn test_public_bearer_contract_accepts_actor_kind_and_permissions_claims_from_login_tokens() {
    let token = encode_hs256_bearer_token(&json!({
        "tenant_id": "100001",
        "sub": "2",
        "actor_kind": "user",
        "sid": "s_guest",
        "did": "d_guest",
        "permissions": ["conversation.*"],
        "iss": "sdkwork-im",
        "aud": "sdkwork-im-public"
    }), "secret").unwrap();
    // assert resolver reads the contract correctly
}
```

- [ ] **Step 4: Run the bearer/unit tests and verify they fail only for the new contract gaps**

Run: `cargo test -p im-auth-context auth_context_test --quiet`
Expected: FAIL on the newly added assertions only.

- [ ] **Step 5: Commit the red test baseline**

```bash
git add services/local-minimal-node/tests/real_auth_e2e_test.rs services/local-minimal-node/tests/public_auth_e2e_test.rs crates/im-auth-context/tests/auth_context_test.rs
git commit -m "test(auth): lock real local login contract"
```

### Task 2: Implement Auth Store, Token Issuance, And Portal Snapshot APIs

**Files:**
- Create: `services/local-minimal-node/src/node/auth.rs`
- Create: `services/local-minimal-node/src/node/portal.rs`
- Modify: `services/local-minimal-node/src/node.rs`
- Modify: `services/local-minimal-node/src/node/build.rs`
- Modify: `services/local-minimal-node/src/lib.rs`
- Modify: `services/local-minimal-node/Cargo.toml`
- Test: `services/local-minimal-node/tests/real_auth_e2e_test.rs`

- [ ] **Step 1: Add the minimal auth domain types used by the failing tests**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAccountRecord {
    pub tenant_id: String,
    pub account_id: String,
    pub login: String,
    pub client_kind: String,
    pub actor_id: String,
    pub actor_kind: String,
    pub password_hash: String,
    pub password_salt: String,
    pub password_iterations: u32,
    pub permissions: Vec<String>,
    pub disabled: bool,
}
```

- [ ] **Step 2: Implement seeded account bootstrap and file-backed persistence**

Run: no command yet
Expected: `auth-accounts.json` and `auth-refresh-sessions.json` are created under runtime `state/` lazily and deterministically.

- [ ] **Step 3: Implement password hashing, login, me, and refresh handlers**

```rust
pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    let issued = state.auth_runtime.login(request)?;
    Ok(Json(issued))
}
```

- [ ] **Step 4: Add router wiring and split public-vs-authenticated routes correctly**

Run: `cargo test -p local-minimal-node --test real_auth_e2e_test --quiet`
Expected: PASS for login, refresh, me, invalid-password, and client-kind rejection tests.

- [ ] **Step 5: Add portal backend snapshot endpoints and narrow local CORS**

```rust
.route("/app/v3/api/portal/home", get(portal::get_home))
.route("/app/v3/api/portal/access", get(portal::get_auth))
.route("/app/v3/api/portal/workspace", get(portal::get_workspace))
```

- [ ] **Step 6: Run focused backend regression coverage**

Run: `cargo test -p local-minimal-node --test real_auth_e2e_test --test public_auth_e2e_test --quiet`
Expected: PASS.

- [ ] **Step 7: Commit the backend implementation**

```bash
git add services/local-minimal-node/src/node/auth.rs services/local-minimal-node/src/node/portal.rs services/local-minimal-node/src/node.rs services/local-minimal-node/src/node/build.rs services/local-minimal-node/src/lib.rs services/local-minimal-node/Cargo.toml services/local-minimal-node/tests/real_auth_e2e_test.rs services/local-minimal-node/tests/public_auth_e2e_test.rs crates/im-auth-context/tests/auth_context_test.rs
git commit -m "feat(auth): add real local login and portal snapshots"
```

### Task 3: Add CLI Login Support And Real Token Plumbing

**Files:**
- Modify: `tools/chat-cli/src/command.rs`
- Modify: `tools/chat-cli/src/lib.rs`
- Create: `tools/chat-cli/tests/chat_cli_auth_test.rs`
- Modify: `tools/chat-cli/tests/chat_cli_contract_test.rs`
- Reference: `tools/chat-cli/src/main.rs`

- [ ] **Step 1: Write failing CLI parse and execution tests for `login`**

```rust
#[tokio::test]
async fn test_login_command_posts_credentials_and_returns_tokens() {
    let command = parse_cli_args([
        "sdkwork-im-cli",
        "--base-url", server.base_url(),
        "login",
        "--tenant-id", "100001",
        "--login", "guest",
        "--password", "Guest#2026",
        "--client-kind", "im_user",
    ]).unwrap();

    let output = execute_command(command).await.unwrap();
    let json = unwrap_json(output);
    assert_eq!(json["user"]["id"], "2");
    assert!(json["accessToken"].as_str().is_some());
}
```

- [ ] **Step 2: Run the CLI auth tests and verify `login` is unknown**

Run: `cargo test -p sdkwork-im-cli --test chat_cli_auth_test --quiet`
Expected: FAIL with unknown command or missing flags.

- [ ] **Step 3: Implement the new command and reuse returned bearer tokens**

```rust
CommandOperation::Login {
    tenant_id,
    login,
    password,
    client_kind,
}
```

- [ ] **Step 4: Keep legacy secret minting only as an explicit fallback path**

Run: `cargo test -p sdkwork-im-cli --test chat_cli_auth_test --test chat_cli_contract_test --quiet`
Expected: PASS for new login flow and existing token contract tests.

- [ ] **Step 5: Commit the CLI auth changes**

```bash
git add tools/chat-cli/src/command.rs tools/chat-cli/src/lib.rs tools/chat-cli/tests/chat_cli_auth_test.rs tools/chat-cli/tests/chat_cli_contract_test.rs
git commit -m "feat(cli): add real login flow for local chat auth"
```

### Task 4: Upgrade PowerShell Chat Helpers To Auto-Start And Log In

**Files:**
- Modify: `bin/chat-window-gui.ps1`
- Modify: `bin/open-chat-test.ps1`
- Modify: `bin/start-local.ps1`
- Modify: `.runtime/local-minimal/config/local-minimal.env`
- Create: `tests/bin/chat-window-gui.ps1.test.ps1` or `apps/sdkwork-im-portal/tests/chat-helper-contract.test.mjs`

- [ ] **Step 1: Write a failing script contract test or deterministic validation harness**

```powershell
# Pseudocode
# 1. stop local service
# 2. invoke helper in scripted mode
# 3. assert it auto-starts the service
# 4. assert it calls `chat-cli login`
# 5. assert send-message succeeds with bearer returned from login
```

- [ ] **Step 2: Run the helper validation and verify it fails with the current connect error**

Run: `powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\open-chat-test.ps1 -ScriptedValidation -Json`
Expected: FAIL with connect error or missing login support.

- [ ] **Step 3: Add credential bootstrap and health recovery to the scripts**

```powershell
$login = Invoke-ChatCliJson -Arguments @(
  "--base-url", $resolvedBaseUrl,
  "login",
  "--tenant-id", $TenantId,
  "--login", $ResolvedLogin,
  "--password", $ResolvedPassword,
  "--client-kind", "im_user"
)
$bearerToken = $login.accessToken
```

- [ ] **Step 4: Thread bearer tokens into timeline, send, watch, and scripted validation flows**

Run: `powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\open-chat-test.ps1 -ScriptedValidation -Json`
Expected: PASS with `watchDelivered=true` and `timelineContainsValidationMessage=true`.

- [ ] **Step 5: Commit the helper-script hardening**

```bash
git add bin/chat-window-gui.ps1 bin/open-chat-test.ps1 bin/start-local.ps1 .runtime/local-minimal/config/local-minimal.env
git commit -m "feat(scripts): auto-start local service and log in seeded users"
```

### Task 5: Replace Portal Mock Auth And Mock Snapshots With Real HTTP Integration

**Files:**
- Create: `apps/sdkwork-im-portal/packages/sdkwork-im-portal-portal-api/src/runtime/dataSources/httpPortalDataSource.js`
- Modify: `apps/sdkwork-im-portal/packages/sdkwork-im-portal-portal-api/src/runtime/createPortalDataSource.js`
- Modify: `apps/sdkwork-im-portal/packages/sdkwork-im-portal-portal-api/src/index.js`
- Modify: `apps/sdkwork-im-portal/packages/sdkwork-im-portal-core/src/store/usePortalAuthStore.js`
- Modify: `apps/sdkwork-im-portal/packages/sdkwork-im-portal-auth/src/index.js`
- Modify: `apps/sdkwork-im-portal/packages/sdkwork-im-portal-core/src/application/app/PortalProductApp.js`
- Modify: `apps/sdkwork-im-portal/tests/portal-routing-and-state.test.mjs`

- [ ] **Step 1: Write failing portal tests for real login form and HTTP data source behavior**

```javascript
test('auth store signs in with credentials and persists backend-issued bearer token', async () => {
  // inject http data source double, call signIn({ tenantId, login, password })
  // assert sdkwork-appbase login verification shape and token persistence
});

test('portal auth page renders tenant, login, and password inputs', async () => {
  const html = await renderPortalAuthPage();
  assert.match(html, /name="tenantId"/);
  assert.match(html, /name="login"/);
  assert.match(html, /type="password"/);
});
```

- [ ] **Step 2: Run the portal test suite and verify current mock-only behavior fails**

Run: `node --test --experimental-test-isolation=none apps/sdkwork-im-portal/tests/portal-routing-and-state.test.mjs`
Expected: FAIL on the new sign-in and HTTP bootstrap assertions.

- [ ] **Step 3: Implement the HTTP portal data source and explicit auth store inputs**

```javascript
export async function loginPortalUser(credentials) {
  return activePortalDataSource.loginPortalUser(credentials);
}

async signIn(credentials) {
  const session = await loginPortalUser(credentials);
  // validate, persist token, fetch workspace
}
```

- [ ] **Step 4: Convert `/login` UI from demo button to real form submission**

Run: `node --test --experimental-test-isolation=none apps/sdkwork-im-portal/tests/portal-routing-and-state.test.mjs`
Expected: PASS for auth store, routing, retry, and sign-out regressions.

- [ ] **Step 5: Commit the portal integration**

```bash
git add apps/sdkwork-im-portal/packages/sdkwork-im-portal-portal-api/src/runtime/dataSources/httpPortalDataSource.js apps/sdkwork-im-portal/packages/sdkwork-im-portal-portal-api/src/runtime/createPortalDataSource.js apps/sdkwork-im-portal/packages/sdkwork-im-portal-portal-api/src/index.js apps/sdkwork-im-portal/packages/sdkwork-im-portal-core/src/store/usePortalAuthStore.js apps/sdkwork-im-portal/packages/sdkwork-im-portal-auth/src/index.js apps/sdkwork-im-portal/packages/sdkwork-im-portal-core/src/application/app/PortalProductApp.js apps/sdkwork-im-portal/tests/portal-routing-and-state.test.mjs
git commit -m "feat(portal): switch local portal to real backend auth"
```

### Task 6: End-To-End Verification, Cleanup, And Delivery

**Files:**
- Modify: `docs/superpowers/specs/2026-04-14-real-auth-local-im-design.md`
- Modify: `docs/superpowers/plans/2026-04-14-real-auth-local-im.md`
- Modify: any touched runtime docs or env samples that need the seeded account list

- [ ] **Step 1: Start the local service and verify health**

Run: `powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\start-local.ps1 -ProfileName local-minimal`
Expected: health URL printed and `/healthz` returns `200`.

- [ ] **Step 2: Verify seeded IM login and message delivery**

Run: `powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\open-chat-test.ps1 -ScriptedValidation -Json`
Expected: JSON shows `watchDelivered=true` and `timelineContainsValidationMessage=true`.

- [ ] **Step 3: Verify portal login bootstrap**

Run: `node --test --experimental-test-isolation=none apps/sdkwork-im-portal/tests/portal-routing-and-state.test.mjs`
Expected: PASS.

- [ ] **Step 4: Verify RTC flow with issued bearer tokens**

Run: `cargo test -p local-minimal-node rtc --quiet`
Expected: PASS for existing RTC tests plus any new auth-backed cases.

- [ ] **Step 5: Run the final targeted regression set**

Run: `cargo test -p local-minimal-node --test real_auth_e2e_test --test public_auth_e2e_test --quiet`
Expected: PASS.

Run: `cargo test -p sdkwork-im-cli --test chat_cli_auth_test --test chat_cli_contract_test --quiet`
Expected: PASS.

- [ ] **Step 6: Commit, merge to the feature branch tip, and push**

```bash
git status --short
git add -A
git commit -m "feat(auth): ship real local login across chat and portal"
git push origin feature/real-auth-im
```

