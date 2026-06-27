# 150P control-plane provider snapshot status 设计
## 1. 背景

`07-C10` 到 `07-C12` 已经把 provider-policy 的 success/error 路径都补齐显式 `status`，但 `GET /backend/v3/api/control/provider-registry` 与 `GET /backend/v3/api/control/provider_bindings` 仍然没有统一状态语义。这样 provider control-plane 仍然存在一小块读面不一致。

## 2. 目标

- 为 provider snapshot 读面补显式 `status`
- 让 registry / bindings 读面也能用同一消费方式读取
- 保持原有字段结构与路由语义不变

## 3. 非目标

- 不重构为单一 envelope
- 不改变 provider-policy 路由
- 不修改 ops runtime 消费链路

## 4. HTTP 面

- `GET /backend/v3/api/control/provider-registry`
  - 返回 `status=registry`
- `GET /backend/v3/api/control/provider_bindings`
  - 返回 `status=bindings`

原有字段继续平铺：

- registry:
  - `interfaceVersion`
  - `plugins`
  - `effectiveBindings`
  - `precedence`
- bindings:
  - `interfaceVersion`
  - `tenantId`
  - `effectiveBindings`
  - `precedence`

## 5. 设计取舍

- 对 registry 采用 HTTP 包装层补 `status`
- 对 bindings GET 直接在本地响应模型中补 `status`
- 不把路由语义下沉到共享 contracts，保持 contracts 纯粹

## 6. 演进关系

- `07-C10` 收口 provider-policy preview/commit 状态
- `07-C11` 收口 provider-policy history/diff/rollback 状态
- `07-C12` 收口 provider-policy error 状态
- `07-C13` 收口 provider-registry / provider-bindings 读面状态
- 完成后，provider control-plane 所有主要读写路径都具备显式 `status`
