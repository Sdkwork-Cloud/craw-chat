# 150M control-plane provider-policy 结果状态设计
## 1. 背景

`07-C6` 引入了 `POST /backend/v3/api/control/provider-policies/preview`，`07-C8` 补齐 committed 回显，`07-C9` 又加入 no-op 抑制。但调用方仍需要通过不同路由、`applied` 布尔值和 HTTP 语义组合判断当前到底是 `preview`、真实写入还是 `noop`。

## 2. 目标

- 为成功路径提供统一 `status`。
- 明确冻结三种结果值:
  - `preview`
  - `applied`
  - `noop`
- 保持已有 `applied`、`currentVersion`、`diff` 等字段继续可用。

## 3. 非目标

- 不重构所有 control-plane 响应 envelope。
- 不改变 `409 + provider_policy_conflict` 的冲突面。
- 不为 rollback / history / diff 新增额外状态字段。

## 4. 模型

- 新增 `ProviderPolicyResultStatus`
  - `preview`
  - `applied`
  - `noop`
- `ProviderPolicyPreview`
  - `status`
  - `baseVersion`
  - `previewVersion`
  - `previewBinding`
  - `diff`
- `ProviderPolicyCommit`
  - `status`
  - `applied`
  - `currentVersion`
  - `committedBinding`
  - `diff`

## 5. HTTP 面

- `POST /backend/v3/api/control/provider-policies/preview`
  - 返回 `status=preview`
- `POST /backend/v3/api/control/provider_bindings`
  - 真写入返回 `status=applied`
  - no-op 返回 `status=noop`
  - 继续返回 `applied` 布尔值，兼容旧调用方

## 6. 兼容性

- 现有依赖 `applied=true|false` 的调用方无需修改即可继续运行。
- 新调用方应优先读取 `status`，避免把路由、HTTP 状态和字段组合混在一起推断。

## 7. 副作用约束

- `status=preview` 不得触发 ops / audit。
- `status=noop` 不得推进版本，不得追加 history，不得写 audit。
- `status=applied` 才表示真实写入已经落到 provider policy。

## 8. 演进关系

- `07-C6` 解决 preview。
- `07-C8` 解决 committed 回显。
- `07-C9` 解决 noop 抑制。
- `07-C10` 用 `ProviderPolicyResultStatus` 把 preview / applied / noop 收口为统一结果状态。
