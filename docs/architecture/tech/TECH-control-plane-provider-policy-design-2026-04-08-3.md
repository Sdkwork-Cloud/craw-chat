> Migrated from `docs/架构/150I-control-plane-provider-policy预览设计-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150I control-plane provider-policy 预览设计
## 1. 背景

`07-C5` 已经能比较两个已提交版本，但控制面在真正提交 `provider policy` 之前，仍没有标准方式查看“如果现在提交，会产生什么变化”。这使前端或运营台只能自己拼装预览逻辑，且无法保证和真实写接口使用同一套校验。

## 2. 目标

- 提供标准 `POST /backend/v3/api/control/provider-policies/preview`。
- 复用真实写接口的校验规则。
- 返回统一的 `previewBinding` 与 `diff`。
- 保证预览无副作用。

## 3. 非目标

- 不在本轮支持 rollback preview。
- 不引入草稿工作区。
- 不在本轮处理多条 policy 一次性预演。

## 4. 模型

- `ProviderPolicyPreview`
  - `baseVersion`
  - `previewVersion`
  - `tenantId`
  - `previewBinding`
  - `diff`

其中：

- `previewBinding` 表示按当前请求虚拟应用后的目标 binding。
- `diff` 直接复用 `ProviderPolicyDiff`，确保预览与已提交 diff 共用同一套变化语义。

## 5. 计算规则

- 先读取当前最新快照，得到 `baseVersion`。
- 在内存中克隆当前 policy 状态。
- 把请求中的 `tenantId / domain / pluginId` 虚拟写入克隆状态。
- 生成虚拟 `previewVersion` 与预览快照。
- 计算 `baseVersion -> previewVersion` 的 `diff`。
- 预览完成后，不得修改真实 state。

## 6. HTTP 面

- 路径: `POST /backend/v3/api/control/provider-policies/preview`
- 权限: `control.write`
- Body 与真实写接口一致：
  - `tenantId`
  - `domain`
  - `pluginId`
- Response：
  - `baseVersion`
  - `previewVersion`
  - `previewBinding`
  - `diff`

## 7. 副作用约束

- 不追加 history item。
- 不写 audit。
- 不触发 `replace_provider_binding_snapshots`。
- 不改变当前 `effectiveBinding`。

## 8. 后续演进

- `07-C6` 解决“写之前怎么看”。
- 下一轮应补 `expectedBaseVersion` 或同等 preview confirm 机制，解决“看完以后怎么安全提交”。

