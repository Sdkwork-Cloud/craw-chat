> Migrated from `docs/step/94-Step并行执行编排与车道拆分建议.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 并行执行编排与车道拆分建议

## 1. 文档定位

本文件用于回答一个非常实际的问题：

`00-13` 的 step 顺序已经完整，但如果完全按单线程串行推进，整体周期会过长；如果盲目并行，又会出现多人或多 agent 同时改同一块代码、协议边界互相打架、验证结果互相污染的问题。

因此，本文件的目标是定义：

- 哪些 step 必须串行推进
- 哪些能力可以在同一波次内并行推进
- 如何按代码写入范围拆分并行车道
- 如何保证并行执行后还能稳定收口

## 2. 总体结论

最快且风险可控的执行方式，不是“所有 step 全并行”，而是：

`串行主脊柱 + 波次内并行车道 + 单独验证车道 + 统一集成收口`

也就是说：

- step 之间保持架构依赖顺序
- 每个 step 内或每个波次内，根据写入范围并行拆分
- 保留一条独立验证车道，不和实现车道混写
- 最终由集成 owner 做统一收口

## 3. 哪些 Step 必须串行

以下 step 属于强依赖主脊柱，必须保持串行：

- `00`：先冻结执行门禁，否则后续执行口径不统一
- `01`：先做差距审计，否则后续并行容易改偏
- `02`：先打骨架，否则后续协议和主链路没有稳定落点
- `03`：先冻结协议与契约骨架，否则运行时和主链路会反复返工
- `13`：最终发布就绪与闭环必须最后做

这些 step 可以在内部并行做子任务，但 step 级准入不能跳过。

## 4. 哪些 Step 适合波次内并行

### 4.1 波次 A 内并行

波次 A：`00-03`

可并行车道：

- `A1-文档与审计车道`
  - `01` 的基线审计、差距矩阵、review 文档
- `A2-骨架车道`
  - `02` 的 workspace 骨架、crate 模板、目录化拆分起点
- `A3-协议与契约车道`
  - `03` 的 `ccp-*`、`contract-*` 骨架与测试模板

硬约束：

- `A2` 与 `A3` 只能在 `01` 的结论基础上启动
- `A3` 的最终协议收口必须等 `A2` 的 crate 落点明确后合并

### 4.2 波次 B 内并行

波次 B：`04-06`

可并行车道：

- `B1-Link Runtime 车道`
  - handshake、shard、queue、backpressure、resume
- `B2-Route Runtime 车道`
  - route ownership、epoch、fencing、drain、rebalance
- `B3-消息主链路车道`
  - conversation、member、message、read-cursor、presence
- `B4-流与 RTC 车道`
  - stream lifecycle、materialization、rtc signaling
- `B5-投影兼容车道`
  - timeline、unread、summary 的兼容衔接

硬约束：

- `B1` 和 `B2` 可并行，但共享 route / session 模型时必须统一 owner
- `B3` 依赖 `B1/B2` 给出稳定 runtime hook
- `B4` 依赖 `B3` 的 sender / scope / message bridge 规则
- `B5` 不得先于 `B3` 定稿投影语义

### 4.3 波次 C 内并行

波次 C：`07-09`

可并行车道：

- `C1-控制面与注册表车道`
- `C2-AI / Agent 车道`
- `C3-IoT / MQTT 车道`
- `C4-存储抽象车道`
- `C5-投影与观测车道`

硬约束：

- `C2` 和 `C3` 都依赖 `07` 的 capability / registry / rollout 结果
- `C4` 和 `C5` 可以并行，但投影 rebuild 不能绕过统一存储抽象
- `C2/C3` 的事件写回必须遵守 `C5` 的观测与投影口径

### 4.4 波次 D 内并行

波次 D：`10-13`

可并行车道：

- `D1-脚本与部署车道`
- `D2-压测与灾备车道`
- `D3-CLI / SDK / compat 车道`
- `D4-发布与回滚文档车道`

硬约束：

- `D2` 依赖 `D1` 的统一部署入口
- `D3` 依赖 `D1` 的稳定启动链路和 `D2` 的基本稳定性
- `D4` 必须最后汇总 `D1/D2/D3` 结果

## 5. 推荐并行车道拆分原则

并行必须按“写入范围”拆，而不是按“概念主题”拆。

推荐规则：

- 一个车道只拥有一组主写入目录
- 不允许两个车道同时主写同一高风险文件
- 共享边界文件必须先指定单一 owner
- 验证车道尽量只写测试、脚本和 review 文档

## 6. 推荐主写入范围

### 6.1 协议车道

主写入范围：

- `crates/*ccp*`
- `crates/*contract*`
- `crates/im-auth-context/`

### 6.2 连接运行时车道

主写入范围：

- `services/session-gateway/`
- 未来 `runtime-link/`
- 未来 `runtime-route/`

### 6.3 消息域车道

主写入范围：

- `services/conversation-runtime/`
- `crates/im-domain-core/src/conversation.rs`
- `crates/im-domain-core/src/message.rs`
- `crates/im-domain-core/src/realtime.rs`

### 6.4 流与 RTC 车道

主写入范围：

- `services/streaming-service/`
- `services/im-call-runtime/`
- `crates/im-domain-core/src/stream.rs`
- `crates/im-domain-core/src/rtc.rs`

### 6.5 控制面车道

主写入范围：

- `services/control-plane-api/`
- `services/ops-service/`
- `services/audit-service/`

### 6.6 存储与投影车道

主写入范围：

- `adapters/`
- `services/projection-service/`
- 未来 `storage-*`

### 6.7 脚本与交付车道

主写入范围：

- `bin/`
- `deployments/`
- `README.md`
- `docs/部署/`

### 6.8 CLI / SDK 车道

主写入范围：

- `tools/chat-cli/`
- `sdks/`
- `bin/chat-*`

## 7. 推荐角色分工

如果是多人团队或多 agent 并行，建议最少配以下角色：

- `总集成 Owner`
  - 负责主脊柱顺序、接口边界拍板、合并与收口
- `协议 / 契约 Owner`
  - 负责 `CCP`、`contract-*`、权威字段边界
- `运行时 Owner`
  - 负责 Link / Route / gateway runtime
- `消息域 Owner`
  - 负责 conversation / message / read-cursor / presence
- `流式 / RTC Owner`
  - 负责 stream / rtc 一等能力
- `控制面 Owner`
  - 负责 registry / rollout / drain / audit
- `存储 / 投影 Owner`
  - 负责 projection / rebuild / backup / restore / observability
- `交付 Owner`
  - 负责 `bin/`、`deployments/`、CLI / SDK / release 文档
- `验证 Owner`
  - 负责测试矩阵、压测、灾备演练、review 复盘

## 8. 推荐最快执行模型

### 8.1 小团队 / 2-3 人

建议：

- `Owner A`：协议 + 连接运行时
- `Owner B`：消息域 + 流/RTC
- `Owner C`：控制面 + 存储/投影 + 部署/验证

执行方式：

- 以波次推进
- 每个波次末统一收口

### 8.2 中型团队 / 4-6 人

建议：

- `A`：协议 / 契约
- `B`：Link / Route Runtime
- `C`：消息主链路
- `D`：流 / RTC
- `E`：控制面 + 存储 / 投影
- `F`：部署 / CLI / SDK / 验证

执行方式：

- 波次内多车道并行
- 每个 step 收尾用 `91` 打分
- 每个波次收尾用 `93` 做总验收

### 8.3 多 agent 模式

建议：

- 主 agent 只做总协调、边界拍板、最终集成
- 子 agent 只做单车道、单写入范围的任务
- 子 agent 结果回收后，由主 agent 做统一 review 和合并

## 9. 并行执行时的硬约束

并行推进时，必须坚持以下规则：

- 协议和权威字段模型只能有一个最终 owner
- 高风险大文件只能由单一车道主拆分
- 任何车道不得绕过 `90/91/92/93`
- 验证车道不能和实现车道混在同一个提交里无限放大
- 波次未通过总验收，不得进入下一波次

## 10. 最快且完整的建议路径

如果目标是“尽快且完整地把 step 执行完毕”，推荐如下顺序：

1. `00-01` 快速冻结与审计
2. `02-03` 骨架与协议并行推进
3. `04-06` 分成运行时、消息域、流/RTC 三车道并行
4. `07-09` 分成控制面、AI/IoT、存储/投影 三车道并行
5. `10-12` 分成交付、演练、CLI/SDK 三车道并行
6. `13` 统一收口

这是当前风险和速度平衡最优的路径。

## 11. 与其他治理文档的关系

- `90`：告诉你某项能力应该落在哪
- `91`：告诉你做得够不够好
- `92`：告诉你本轮 step 怎么控输入输出和阻塞
- `93`：告诉你波次是否可过关
- `94`：告诉你如何在不打架的前提下尽快并行执行

## 12. 结论

真正高效的并行执行，不是把 step 全部拆散，而是：

- 保持主脊柱顺序
- 在每个波次内按写入范围拆车道
- 让验证和集成独立
- 最后统一收口

本文件的作用，就是把这套并行执行方法固化下来。

