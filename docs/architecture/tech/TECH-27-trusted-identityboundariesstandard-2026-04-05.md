> Migrated from `docs/架构/27-外部认证与Trusted-Identity边界标准-2026-04-05.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 外部认证与 Trusted Identity 边界标准（2026-04-05）

## 1. 目标

冻结 `sdkwork-im` 在外部入口与内部可信链路之间的认证边界，避免以下高风险混淆：

- 把测试/内部装配用的 trusted headers 误暴露给公网入口
- 把 Bearer/JWT 入口和内部 service-to-service 身份透传混用
- 在部署脚本、README、主程序入口与代码实现之间出现不一致

## 2. 冻结标准

### 2.1 外部入口

- 所有对外 app-facing 入口默认必须走 `Bearer-only`。
- 外部入口不得依赖 `x-tenant-id`、`x-user-id`、`x-actor-id` 作为认证来源。
- 请求中的 `tenant / actor / session / device` 只能来自已校验的 Bearer token。

### 2.2 内部可信链路

- `trusted identity headers` 仅允许用于内部服务调用、测试装配、或显式声明的可信边界。
- 内部可信链路仍可继续使用：
  - `x-tenant-id`
  - `x-user-id` / `x-actor-id`
  - `x-session-id`
  - `x-device-id`
- 但这类模式不得成为公网部署的默认入口。

### 2.3 代码级约束

- 需要同时支持两种模式时，必须显式拆分 builder / profile：
  - `public app`：Bearer-only
  - `default/test app`：可用于测试装配或内部集成
- `main` 函数必须绑定 `public app`，而不是测试装配 builder。

## 3. 本轮最小落地

当前已在 `sdkwork-im-server` 落地以下边界：

- 新增 `im_auth_context::resolve_bearer_auth_context(...)`
- 新增 `local_minimal_node::build_public_app()`
- `services/sdkwork-im-cloud-gateway/src/main.rs` 已改为启动 `build_public_app()`
- `build_public_app()` 对 health 以外的入口统一要求 Bearer
- `build_default_app()` 暂保留为测试/内部装配 builder，避免一次性打断现有大量模块级离线测试

## 4. 当前明确不做的事

本轮不强行改动所有服务的默认 builder，原因是：

- 工作区内大量模块测试仍直接依赖 trusted headers
- 需要按服务分批补齐 `build_public_app()` 与 public auth 测试

## 5. 验证基线

- `cargo test -p im-auth-context --offline -- --nocapture`
- 各服务 `public_auth_test.rs` 必须通过签名 Bearer 夹具
- 部署文档与 `docs/架构/48-公网上行Bearer必须进行签名校验标准-2026-04-05.md` 保持一致

