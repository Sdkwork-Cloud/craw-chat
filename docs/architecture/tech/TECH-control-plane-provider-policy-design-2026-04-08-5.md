> Migrated from `docs/架构/150K-control-plane-provider-policy提交结果设计-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150K control-plane provider-policy 提交结果设计
## 1. 背景

`07-C7` 之后，控制面已经能用 `expectedBaseVersion` 安全确认写入，但成功响应仍然只是 provider bindings 快照。调用方如果想知道“这次提交写到了哪个版本、落下了什么 diff、最终 binding 是什么”，还需要再读 history 或 diff。

## 2. 目标

- 让成功提交直接返回 committed 结果。
- 保证 committed 结果与真实写入原子一致。
- 保持 GET `provider-bindings` 的只读快照职责不变。

## 3. 非目标

- 不处理 no-op 重复提交。
- 不改 preview 的响应结构。
- 不增加新的写接口。

## 4. 模型

- `ProviderPolicyCommit`
  - `currentVersion`
  - `tenantId`
  - `committedBinding`
  - `diff`

其中：

- `currentVersion` 是本次提交后的真实版本。
- `committedBinding` 是本次提交实际生效的 binding。
- `diff` 是本次提交对应的单次真实版本差异。

## 5. 计算规则

- 在同一条真实写路径里拿到提交前快照。
- 完成真实写入并记录新快照。
- 直接在同一原子提交结果里返回 `currentVersion / committedBinding / diff`。
- 不允许通过写后再单独查 history/diff 的方式拼装 committed 结果。

## 6. HTTP 面

- 路径: `POST /backend/v3/api/control/provider_bindings`
- 权限: `control.write`
- Success Response:
  - `currentVersion`
  - `committedBinding`
  - `diff`
  - `effectiveBindings`
  - `precedence`

## 7. 兼容性

- 旧的 GET `provider-bindings` 响应不变。
- POST 继续返回 `effectiveBindings / precedence`，降低现有控制台迁移成本。
- 新增 committed 字段后，控制台可以在一次 POST 中拿到最终提交结果。

## 8. 演进关系

- `07-C6` 解决 preview。
- `07-C7` 解决 confirm。
- `07-C8` 解决 committed 结果回显。
- 下一轮应补 no-op 提交语义，减少无效版本膨胀。

