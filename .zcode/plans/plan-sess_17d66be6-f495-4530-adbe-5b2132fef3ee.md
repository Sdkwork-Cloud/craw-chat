# H5 验证码登录端到端打通

## 背景
- 已修复密码登录（补 `grantType: "password"`）。
- 验证码登录（`grantType: phone_code/email_code`）后端 `sdkwork-routes-iam-app-api` 的 `create_session` 不支持（handlers.rs 硬校验只认 password），且 H5 前端未注入 `verificationCodeClient`（发码 fail-closed）。

## 设计决策
1. **grant 契约**：`sessions.create` body 增加 `grantType: "phone_code"|"email_code"` + `phone|email` + `code`（H5 controller 已发送此结构，与 PC 契约一致）。
2. **验证码校验双路径**（复用既有原语）：
   - 开发/测试模式：`SDKWORK_IAM_DEV_FIXED_VERIFY_CODE` 固定码 + `iam_ephemeral_artifact`（新 kind `code_login`）原子消费防重放（镜像现有 password_reset 机制）；
   - 生产模式：`verify_and_consume_messaging_challenge` scene `LOGIN`（新增 `MESSAGING_VERIFICATION_SCENE_LOGIN` 常量；发码归 messaging 仓库，本地无法验证，作为架构预留）。
3. **发码端点**：新增 `POST /app/v3/api/auth/verification_code_requests`（`verificationCodeRequests.create`），body `{scene, target, channel}`（scene: LOGIN/REGISTER/RESET_PASSWORD，channel: sms/email）。dev 固定码模式返回 `{accepted: true, devCode}` 供前端提示演示码；生产返回 `{accepted: true}`。
4. **用户与校验**：按 phone/email 精确匹配 `iam_user`，要求 `phone_verified`/`email_verified`；会话签发复用 `create_authenticated_session_response`（含多组织 challenge 路径）。sqlite 分支镜像实现（固定码直比，无防重放，嵌入式场景可接受）。
5. **auth_level**：验证码会话沿用 `AuthLevel::Password`（共享枚举无 code 变体，改枚举影响 JWT 消费方，本次不引入）。
6. **策略**：`retrieve_verification_policy` 的 `phoneCodeLoginEnabled/emailCodeLoginEnabled` 按固定码/messaging 开关置位。

## 改动清单

### A. 后端 Rust（sdkwork-iam / sdkwork-routes-iam-app-api）
1. `src/manifest.rs` — 新增路由条目 + `IAM_CREDENTIAL_ENTRY_OPERATION_IDS`
2. `src/handlers.rs` — `create_session` grant 分发重构（password / phone_code / email_code）；新 handler `create_verification_code_request`；code 登录认证（postgres）；`retrieve_verification_policy` 置位
3. `src/sqlite_sessions.rs` — `authenticate_code_and_create_session`（镜像密码版）
4. `src/ephemeral.rs` — `KIND_CODE_LOGIN` + upsert/consume（镜像 password_reset）
5. `src/passwords.rs`（或新模块）— `verify_code_login`：messaging LOGIN scene / dev 固定码两路径 + verified 检查
6. `src/utils.rs` — grant/body 解析辅助（如需要）
7. `crates/sdkwork-iam-web-adapter/src/messaging_verification.rs` — 新增 `MESSAGING_VERIFICATION_SCENE_LOGIN`
8. `tests/iam_local_app_router_test.rs` — 集成测试：发码端点（devCode 返回）、code 登录成功、错误码拒绝、固定码重放拒绝、未验证账号拒绝

### B. 契约链（sdkwork-iam）
9. `pnpm api:materialize:openapi` — 重新物化 OpenAPI（apis/ + sdks 的 sdkgen 输入）
10. `LANGUAGES=typescript pnpm sdk:generate:app` — 重新生成 TS SDK（生成器已确认存在于 ../sdkwork-sdk-generator）
11. `apps/sdkwork-iam-common/packages/sdkwork-iam-service/src/index.ts` — `SdkworkIamService.auth.verificationCodeRequests.create`（callRaw 模式）
12. `apps/sdkwork-iam-common/packages/sdkwork-iam-sdk-ports/src/index.ts` — `IamAppSdkClient.auth.verificationCodeRequests?.create?`

### C. 前端 + 配置（sdkwork-im）
13. `apps/sdkwork-im-h5/src/bootstrap/imH5AuthController.ts` — 注入 `verificationCodeClient`：`send({scene,target,verifyType})` → `runtime.service.auth.verificationCodeRequests.create({scene, target, channel})`（PHONE→sms, EMAIL→email）；响应含 devCode 时 `showToast` 提示演示码
14. `etc/topology/standalone.development.env` + `standalone.test.env` — 加 `SDKWORK_IAM_DEV_FIXED_VERIFY_CODE=654321`（与 IAM 测试固定码一致）

### D. 验证
- `cargo test -p sdkwork-routes-iam-app-api`（新增测试；需要工作区 postgres test env）
- H5 `pnpm run typecheck` + iam-h5-auth controller 测试

## 不做的事
- 不改 messaging 仓库（发码发送/校验表属其域；scene LOGIN 仅作为常量预留，本地 dev 走固定码）
- 不动 `registrations.create` 的注册验证码校验（端点接受 REGISTER 场景，验证逻辑维持现状）
- 不引入新的 auth_level 枚举值