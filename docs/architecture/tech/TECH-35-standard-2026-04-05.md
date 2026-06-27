> Migrated from `docs/架构/35-自动化执行幂等与冲突标准-2026-04-05.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 自动化执行幂等与冲突标准 - 2026-04-05

## 1. 问题定义

在上一轮完成自动化执行最小权限和 owner principal 隔离之后，继续 review 发现 `automation-service` 仍存在执行标识语义缺口：

- 同一主体重复提交相同 `executionId` 时，运行时会再次追加 `automation.execution_requested` 和 `automation.execution_completed` 事件。
- 同一主体重复提交相同 `executionId` 但请求内容不同，运行时会直接覆盖已有执行结果。
- `sdkwork-im-server` 聚合入口在每次请求后都会继续写 audit 和 notification side-effect，导致幂等重放也会重复落副作用。

这会带来三类问题：

- 审计记录数量膨胀，无法区分首次创建与重试重放
- 执行结果被后写覆盖，破坏 `executionId` 的稳定引用语义
- side-effect 与主执行结果不再一致，出现“逻辑同一条执行，多次发通知/多次记审计”的偏差

## 2. 标准结论

### 2.1 `executionId` 是主体内稳定幂等键

自动化执行的幂等域定义为：

- `tenant_id + principal_id + execution_id`

在这个域内，`executionId` 必须满足稳定引用语义：

- 相同请求重试不能创建第二条执行
- 已有执行不能被不同请求内容覆盖

### 2.2 相同请求视为幂等重放

若以下字段完全一致：

- `triggerType`
- `targetKind`
- `targetRef`
- `inputPayload`

且 `tenant_id + principal_id + execution_id` 相同，则视为幂等重放：

- 返回已存在的执行结果
- HTTP 返回 `200`
- 不追加新的自动化事件
- 不追加新的 audit side-effect
- 不追加新的 notification side-effect

### 2.3 相同 `executionId` 但请求内容不同视为冲突

若 `tenant_id + principal_id + execution_id` 相同，但请求内容不一致，则必须返回：

- HTTP `409`
- `code = automation_execution_conflict`

目的：

- 阻止“后写覆盖前写”
- 阻止客户端把 `executionId` 同时当作幂等键和可变请求键使用
- 让调用方显式生成新的 `executionId`，而不是隐式复用旧执行

## 3. 聚合入口要求

`sdkwork-im-server` 复用 `AutomationRuntime` 时，必须显式区分：

- 首次创建
- 幂等重放

仅在首次创建时才允许产生以下 side-effect：

- `audit.record`
- `automation.result` 站内通知

幂等重放时这些 side-effect 必须被抑制。

## 4. 实现要求

实现层建议统一暴露一个带结果语义的 runtime 返回值，例如：

- `execution`
- `is_new`

这样独立服务和聚合入口都能共享同一份判定，不会把“幂等判定”散落在各个 handler 中重复实现。

## 5. 本轮落地行为

本轮标准落地后，至少满足以下回归行为：

1. 相同主体、相同 `executionId`、相同请求内容，第二次提交返回 `200` 且结果与首次一致
2. 相同主体、相同 `executionId`、不同请求内容，返回 `409 automation_execution_conflict`
3. 幂等重放不会重复追加自动化事件
4. `sdkwork-im-server` 幂等重放不会重复写 audit 记录
5. `sdkwork-im-server` 幂等重放不会产生第二条 `automation.result` 通知可见结果

## 6. 变更文件

- `services/automation-service/src/lib.rs`
- `services/automation-service/tests/automation_execution_test.rs`
- `services/automation-service/tests/http_smoke_test.rs`
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
- `services/sdkwork-im-cloud-gateway/tests/task10_capabilities_e2e_test.rs`

## 7. 验证命令

- `cargo test -p automation-service --test automation_execution_test --offline`
- `cargo test -p automation-service --test http_smoke_test --offline`
- `cargo test -p sdkwork-im-cloud-gateway --test task10_capabilities_e2e_test --offline`
- `cargo test --workspace --offline`

## 8. 下一轮 review 建议

自动化执行链路本轮完成后，下一轮优先继续看两个地方：

- `notification-service` 自身是否也需要对 `notificationId` 建立显式幂等/冲突语义，避免内部 side-effect 通道重复写事件
- `media-service` 独立服务与文档对 `POST /im/v3/api/media/{id}/attach` 的能力描述是否一致，需要继续做文档与实现对账

