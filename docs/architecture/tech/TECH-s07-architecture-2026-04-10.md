> Migrated from `docs/review/S07-架构兑现与回写决议-2026-04-10.md` on 2026-06-24.
> Owner: SDKWork maintainers

# S07 架构兑现与回写决议 - 2026-04-10

## 1. 契约基线
- 来源：`docs/step/100-*`、`docs/step/101-*`、`docs/架构/150CJ-*`、`docs/架构/151CJ-*`
- Step 定义：`S07 = conversation-runtime business truth`

## 2. 已兑现能力力力
- `conversation-runtime` 已具备：
  - `reaction / pin` 的 runtime truth
  - `direct_chat -> conversation` runtime binding
  - `thread` dedicated create path、business binding、HTTP query、recovery replay
- `thread` 当前以 `group conversation + root message` 为锚点，不再依赖消息 reply 字段影子建模。

## 3. 仍未纳入本步硬闭环的能力
- `invited history visibility runtime truth`
- `shared-external history`
- `retention enforcement`
- `thread subscription / notification`

## 4. 回写决议
- `152CJ` 必须回写：
  - `conversation-runtime` 现已支持 `thread minimal runtime truth`
  - `thread` 的当前 as-built 形态是 `business binding = thread/rootMessageId + owner metadata`
  - `S07` 仍为 `not_closed / local_closure`
- `S08`、`S10` 不再把 `thread` 缺失视为当前阻塞口径。

## 5. Deferred 回挂
- `CPR07-3`：`invited history visibility`
- `CPR07-4`：`shared-external history`
- `CPR07-5`：`retention enforcement`
- owner：`S07 / S13 follow-up loops`

## 6. 再次准入条件
- 当前不得宣称 `S07 = step_closure`
- 后续只有在 `invited/shared history + retention` 三线收敛后，才允许进入新的出口检查

