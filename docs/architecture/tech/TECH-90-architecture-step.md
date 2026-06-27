> Migrated from `docs/step/90-架构能力-Step-代码目录-证据映射矩阵.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 架构能力 - Step - 代码目录 - 证据映射矩阵

## 1. 文档定位

本文件用于解决一个常见问题：

计划看起来完整，但执行时不知道某项架构能力究竟落在哪个 step、会修改哪些目录、最终要提供哪些验证证据。

因此，本文件把四类信息收敛到一张矩阵中：

- 架构能力
- 对应 step
- 主要代码目录
- 主要验证与复盘证据

本文件不是替代 `00-13` 的 step 文档，而是 step 执行前和 step 验收前的辅助索引。

## 2. 使用规则

执行任一 step 前，必须回答四个问题：

1. 当前要落地的能力，对应哪条架构文档
2. 这项能力属于哪个 step 或哪些 step
3. 主要触达哪些仓库目录
4. 完成后要在哪些地方留下证据

若以上四个问题无法回答清楚，不应开始代码实施。

## 3. 总体能力矩阵

| 能力域 | 核心架构文档 | 对应 Step | 主要代码目录 | 主要证据输出 |
| --- | --- | --- | --- | --- |
| 执行规则与门禁 | `130` `133` `140` `142` | `00` | `docs/step/` `docs/review/` | step 索引、门禁文档、review 模板 |
| 现状审计与差距分析 | `09` `130` `131` `133` `147` | `01` | 根 `Cargo.toml` `crates/` `services/` `bin/` `deployments/` | 基线审计文档、差距矩阵、高风险文件清单 |
| workspace / crate 骨架 | `130` `133` `147` | `02` | `Cargo.toml` `crates/` `services/` | 新 crate 骨架、依赖边界、文件长度治理 |
| CCP 协议基础设施 | `143` `144` `145` `146` `147` `148` | `03` | `crates/*contract*` `crates/*ccp*` `services/session-gateway/` | 协议契约、编解码测试、权威字段防护测试 |
| Link Plane | `130` `131` `136` | `04` | `services/session-gateway/` `runtime-link` | 握手、resume、背压、metrics 证据 |
| Route Plane | `131` `136` `149` | `04` `07` `11` | `runtime-route` `services/control-plane-api/` | route epoch、drain、rebalance 演练结果 |
| 消息与会话主链路 | `130` `134` `136` `139` | `05` | `services/conversation-runtime/` `crates/im-domain-core/` `services/projection-service/` | conversation / message / read-cursor 测试、access control 测试 |
| sender / tenant 权威收口 | `139` `143` `145` | `03` `05` | `crates/im-auth-context/` `contract-*` `interface-*` | 权威字段覆盖防护测试、接口兼容说明 |
| 流式能力 | `130` `134` `143` | `06` | `services/streaming-service/` `domain-stream` `app-stream` | stream lifecycle、checkpoint、materialize 测试 |
| RTC 信令 | `130` `136` `143` | `06` | `services/im-call-runtime/` `domain-rtc` | rtc signal flow、membership gate 测试 |
| 控制面治理 | `139` `142` `148` `149` | `07` | `services/control-plane-api/` `services/ops-service/` `services/audit-service/` | registry、rollout、drain、audit 测试 |
| AI / Agent | `130` `134` `135` | `08` | `runtime-agent` `app-agent` `automation-service` | agent 对话、tool call、handoff 证据 |
| IoT / 设备接入 | `130` `134` `143` `145` | `08` | `runtime-iot` `interface-mqtt` `app-iot` | device auth、telemetry、command 测试 |
| 存储抽象 | `132` `141` | `09` | `adapters/` `storage-*` | storage port、rebuild、backup / restore 证据 |
| 投影与时间线 | `136` `140` `141` | `05` `09` | `services/projection-service/` | timeline、unread、summary 测试 |
| 可观测与 SLO | `140` | `09` `11` | `services/*` `bin/` `deployments/` | metrics、tracing、SLO 报告 |
| 部署与多环境 | `137` `138` `142` | `10` | `bin/` `deployments/` `README.md` | 安装、启动、停止、修复、恢复脚本验证 |
| 性能、高可用、灾备 | `131` `137` `138` `140` `149` | `11` | `services/*` `bin/` `tools/chat-cli/` | 压测、排空、恢复、升级演练报告 |
| SDK / CLI / 兼容矩阵 | `146` `148` `149` | `12` | `tools/chat-cli/` `sdks/` `bin/` | CLI E2E、SDK README、compat matrix |
| 行业对标与终局能力收口 | `135` | `08` `11` `13` | `docs/架构/` `docs/step/` `docs/review/` | 对标差距分析、终局能力验收说明 |
| 发布就绪与持续迭代 | `09` `137` `138` `146` `149` | `13` | 全仓 | 发布清单、升级/回滚说明、下一轮 backlog |

## 4. 高风险目录优先映射

当前仓库的高风险目录与最优先 step 对应关系如下：

| 高风险路径 | 主要问题 | 优先 Step | 补充 Step |
| --- | --- | --- | --- |
| `services/sdkwork-im-cloud-gateway/src/lib.rs` | 超大文件、装配和运行时细节混写 | `01` `02` | `04` `10` |
| `services/conversation-runtime/src/lib.rs` | 消息主链路和服务边界混写 | `01` `02` `05` | `09` |
| `services/session-gateway/src/lib.rs` | 连接热路径、协议接入、运行时边界混写 | `01` `02` `03` `04` | `11` |
| `crates/im-platform-contracts/` | 业务契约与目标 `CCP` 契约仍需收口 | `03` | `12` |
| `crates/im-domain-core/` | 领域模型需要继续细分到目标 domain crate | `02` `05` `06` `08` | - |
| `bin/` | 脚本资产多，但需统一治理与 smoke 口径 | `10` | `11` `12` |

## 5. Step 执行前核对表

在正式进入某个 step 之前，至少要核对：

- 当前能力是否已在本矩阵中找到对应 step
- 当前改动目录是否在本矩阵允许的主要代码目录中
- 当前验证证据是否能落到本矩阵指定的证据输出中
- 当前变更是否会提前触碰本应属于后续 step 的能力域

如果答案是否定的，说明 step 边界还不够清晰，需要先修订计划。

## 6. Step 完成后证据清单

建议每个 step 在 `docs/review/` 中至少形成一个对应的复盘文件，命名推荐：

- `docs/review/step-00-执行门禁复核-YYYY-MM-DD.md`
- `docs/review/step-01-基线审计复盘-YYYY-MM-DD.md`
- `docs/review/step-02-workspace-crate重构复盘-YYYY-MM-DD.md`
- `docs/review/step-03-ccp与契约冻结复盘-YYYY-MM-DD.md`
- `docs/review/step-04-link-route运行时复盘-YYYY-MM-DD.md`
- `docs/review/step-05-消息主链路复盘-YYYY-MM-DD.md`
- `docs/review/step-06-stream-rtc复盘-YYYY-MM-DD.md`
- `docs/review/step-07-控制面治理复盘-YYYY-MM-DD.md`
- `docs/review/step-08-ai-agent-iot复盘-YYYY-MM-DD.md`
- `docs/review/step-09-存储投影与观测复盘-YYYY-MM-DD.md`
- `docs/review/step-10-部署发布复盘-YYYY-MM-DD.md`
- `docs/review/step-11-性能灾备复盘-YYYY-MM-DD.md`
- `docs/review/step-12-sdk-cli兼容复盘-YYYY-MM-DD.md`
- `docs/review/step-13-发布就绪复盘-YYYY-MM-DD.md`

## 7. 使用建议

推荐工作流：

1. 先看 `README` 中的执行顺序
2. 再用本矩阵定位当前能力、目录和证据
3. 然后执行对应 step 文档
4. 最后用 `91-Step质量审计清单与复盘模板` 做收尾校对

## 8. 结论

一套实施计划只有在“能力、路径、证据、门禁”四个维度都能落地时，才称得上真正可执行。

本矩阵的作用，就是让 `docs/step/` 不只是漂亮的计划集合，而是可以被真实工程持续使用的执行导航系统。

