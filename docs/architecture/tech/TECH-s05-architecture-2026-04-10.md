> Migrated from `docs/review/S05-架构兑现与回写决议-2026-04-10.md` on 2026-06-24.
> Owner: SDKWork maintainers

# S05 架构兑现与回写决议 - 2026-04-10

## 1. 契约基线
- 来源：`docs/step/100-*`、`docs/step/101-*`、`docs/step/95-*`、`docs/step/97-*`
- Step 定义：`S05 = 关系域 durable truth`

## 2. 已兑现能力力力
- `im-domain-core::social` 是关系域契约层。
- `im-domain-events::social` 是关系域事件出口层。
- `services/control-plane-api` 已成为 social durable truth 的运行时与 repair owner。
- 已兑现链路：
  - `friend_request / friendship / user_block / direct_chat`
  - `external_connection / external_member_link / shared_channel_policy`
  - social HTTP write/read
  - audit anchor
  - `social-state.json`
  - `social-commit-journal.json`
  - `social-transaction-marker.json`
  - HTTP `repair-derived-snapshot`
  - standalone `repair-social-runtime-dir`
  - unified `repair-runtime-local.*`

## 3. 仍未纳入本步硬闭环的能力
- 更强 `staged / manifest` 级事务
- 这些能力属于 durability hardening，而不是当前 `S05` 的最低能力闭环标准

## 4. 回写决议
- `S05.status = step_closure`
- `S05.closure_level = step_closure`
- `152CJ` 必须回写：
  - social durable truth 已不再只是 `file-backed snapshot + journal` 样板，而是达到 `journal-authoritative replay + committed ack + failpoint + repair-marker + operator surface` 的最小闭环
  - `transactionMarkerCleared` 已成为 operator-visible repair contract
  - 更强 `staged / manifest` 级事务保持 deferred，不再作为 `S05` step-blocking gap
- `150CJ / 151CJ` 无需回写目标设计；本轮主要是将 as-built closure 与 deferred backlog 分层写清。

## 5. Deferred 回挂
- `CPR05-4`：更强 staged/manifest transaction proof
- owner：`durability hardening backlog`
- downstream：不再阻塞 `S05`，仅作为后续优化输入

## 6. 再次准入条件
- 当前无需再次准入判断；`S05` 已达到 `step_closure`
- 后续若重新打开 `S05`，只能以新增 durability hardening 范围而非“当前能力未闭环”为理由

