> Migrated from `docs/review/S08-架构兑现与回写决议-2026-04-10.md` on 2026-06-24.
> Owner: SDKWork maintainers

# S08 架构兑现与回写决议 - 2026-04-10

## 1. 契约基线
- 来源：`docs/step/100-*`、`docs/step/101-*`、`docs/step/95-*`、`docs/step/97-*`
- `S08` 定义：`projection / notification / read model / rebuild`

## 2. 本轮确认可兑现的架构能力
- 会话投影面：`timeline / inbox / read cursor / unread / member snapshot / client-route sync / handoff summary / member_directory / contacts`
- `contacts` 当前 as-built：friendship-derived 双向联系人、`direct_chat.bound` conversation enrich/backfill、HTTP 查询、snapshot persist/restore、鉴权读取
- 重建与恢复面：`projection snapshot restore / rebuild duration / live lag / update-delay`
- 通知投影面：`request / idempotency / fanout / recipient resolution / automation result / rebuild restore`

## 3. 暂不回写为 Step 完整兑现的能力
- 无

## 4. 回写决议
- `S08.status = step_closure`
- `S08` 本轮从 `local_closure` 提升为 `step_closure`
- `contacts read model` 从 deferred 清单移出，正式回写为 `projection-service` 的 as-built 能力
- `S09` 维持既有判断，不受本轮推翻
- `reaction/pin summary` 正式回写为 `projection-service` 的 as-built 能力
- `S10` 维持 `暂不准入`；阻塞已收敛为仅剩 `S09` 未闭环

## 5. Deferred 回挂
- `CPR08-5`：`done`
- owner：`S08 / L4`
- verification owner：`LV`
- review owner：`L0`

## 6. 再次准入条件
- `S09` 完成 `step_closure`
- `projection-service` 与 `notification-service` fresh 回归继续全绿
- `S10` 准入判断完成 fresh review 回写

