> Migrated from `docs/架构/49-真实时间戳与通知列表排序标准-2026-04-05.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 49-真实时间戳与通知列表排序标准-2026-04-05

## 1. 问题背景

在本轮 review 中发现：

- `notification-service` 之前使用固定的 `requested_at / dispatched_at` 常量。
- `automation-service` 之前使用固定的 `requested_at / completed_at` 常量。
- `audit-service` 之前使用固定的 `recorded_at / exported_at` 常量。
- `notification-service` 的通知列表返回顺序没有显式契约，实际依赖 `HashMap` 迭代顺序。

这会带来以下问题：

- 审计时间失真，无法反映真实发生时间。
- 同一租户连续产生多条记录时，时间字段完全相同，破坏可观测性。
- 通知列表在多条数据场景下无法保证“最新优先”，客户端排序和分页语义不稳定。

## 2. 标准要求

### 2.1 时间字段必须使用真实 UTC 时间

所有 runtime 在产生领域对象或审计对象时：

- 必须生成真实的 UTC 当前时间。
- 输出格式必须为固定宽度、可字典序比较的 RFC3339 UTC 毫秒时间串。
- 禁止继续使用演示常量、硬编码时间串或伪时间占位值。

当前纳入本标准的字段包括：

- `notification.requested_at`
- `notification.dispatched_at`
- `automation_execution.requested_at`
- `automation_execution.completed_at`
- `audit_record.recorded_at`
- `audit_export_bundle.exported_at`

### 2.2 通知列表必须显式定义排序契约

`GET /im/v3/api/notifications` 的返回顺序必须稳定，不能依赖内存容器遍历顺序。

排序规则定义为：

1. 以 `dispatched_at` 倒序优先。
2. 若 `dispatched_at` 不存在，则退化为 `requested_at` 倒序。
3. 若时间完全相同，则以 `notification_id` 倒序作为稳定 tie-breaker。

该规则的目标是保证：

- 客户端无需额外猜测“最新一条通知”。
- 同步、调试、审计和分页具有稳定语义。

## 3. 落地方式

本轮实现新增共享工具 crate：

- `crates/im-time`

统一提供：

- `utc_now_rfc3339_millis()`

各服务必须复用共享工具，不允许在模块内部重复实现不同格式的时间生成逻辑。

## 4. 验证要求

至少需要覆盖以下测试：

- 连续创建两条通知时，时间戳必须不同。
- 连续创建两条 automation execution 时，时间戳必须不同。
- 连续写入两条 audit record 时，时间戳必须不同。
- 通知列表在两条通知场景下必须按最新优先返回。

## 5. 后续 review 方向

本标准修复了最小实现中的时间与排序失真问题。下一步继续检查：

- 其他模块是否仍存在固定时间戳占位实现。
- 需要分页的列表接口是否仍依赖 `HashMap` 或 `Vec` 当前插入顺序。
- 后续持久化实现是否需要把“时间生成”和“排序键生成”下沉到统一基础设施层。

