# Sdkwork IM 分步实施计划索引

> **Topology v2 权威入口：** [docs/topology-greenfield.md](../topology-greenfield.md) · [docs/部署/README.md](../部署/README.md) · `pnpm dev`
> 本目录为 2026-04 前后执行层归档；正文 profile / lifecycle 词汇已迁移到 Topology v2。文件名中的历史词仅用于链接定位。

## 1. 文档定位

本目录是 `docs/架构` 的执行层文档，负责把已经冻结的架构基线进一步细化成可以逐步落地的实施步骤、检查点、测试计划和结果验证口径。

本目录不是替代：

- `docs/架构/130-149` 的架构总纲与专项设计
- `docs/架构/09-实施计划.md` 的阶段性路线

而是对上述文档做执行级细化，形成“可以按顺序推进、每步可验收、每步可回滚、每步可复盘”的闭环计划体系。

## 2. 适用范围

本计划适用于当前 `sdkwork-im` 仓库的以下真实结构：

- `crates/`
- `services/`
- `adapters/`
- `sdks/`
- `tools/`
- `bin/`
- `deployments/`

本计划默认当前仓库是一个已经可本地运行、但尚未完成目标架构收敛的 Rust workspace。后续所有重构、升级、拆分、协议治理、发布治理和性能演进，都必须以本目录为执行索引。

## 3. 关联架构基线

执行时必须优先对齐以下文档：

- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
- `docs/架构/131-连接管理与分层弹性扩容架构设计-2026-04-06.md`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
- `docs/架构/133-代码结构治理与crate拆分标准-2026-04-06.md`
- `docs/架构/134-AI-Agent-IoT统一实时通信模型设计-2026-04-06.md`
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
- `docs/架构/137-部署拓扑与容量规划设计-2026-04-06.md`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`
- `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md`
- `docs/架构/142-控制面与配置治理设计-2026-04-06.md`
- `docs/架构/143-统一协议总纲与分层设计-2026-04-06.md`
- `docs/架构/144-CCP传输绑定与握手协商设计-2026-04-06.md`
- `docs/架构/145-CCP数据协议与版本兼容安全设计-2026-04-06.md`
- `docs/架构/146-CCP协议注册表与多端SDK兼容治理设计-2026-04-06.md`
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`
- `docs/架构/148-CCP控制面注册表与协议发布治理设计-2026-04-06.md`
- `docs/架构/149-多Cell多Region协议升级与灾备兼容设计-2026-04-06.md`
- `docs/架构/150-插件化提供商体系与设备接入设计-2026-04-08.md`

## 4. 总体执行顺序

| Step | 主题 | 核心目标 | 前置条件 | 主要输出 |
| --- | --- | --- | --- | --- |
| `00` | 总实施原则与执行门禁 | 固化执行方法、检查点和门禁 | 无 | 执行规则、证据口径 |
| `01` | 现状基线冻结与差距审计 | 明确当前代码与目标架构差距 | `00` | 审计清单、差距矩阵 |
| `02` | workspace 与 crate 骨架重构 | 先打通工程结构和目录边界 | `01` | 新 workspace 拓扑、薄化 `lib.rs` |
| `03` | CCP 协议基础设施与契约冻结 | 固化协议骨架和契约拆分 | `02` | `ccp-*` 与 `contract-*` 分层 |
| `04` | Link / Route Runtime 重构 | 抽离连接热路径和在线路由 | `03` | `runtime-link`、`runtime-route` |
| `05` | 消息与会话主链路重构 | 完成会话、成员、消息、已读与用户模块主链路 | `04` | message / conversation / principal-profile 核心域 |
| `06` | 流式与 RTC 实时能力重构 | 让流、RTC 与 RTC provider 成为一等能力 | `05` | stream / rtc / provider 主路径 |
| `07` | 控制面与协议治理落地 | 建立配置、能力、注册表和发布治理 | `03`、`04`、`05` | control plane、registry |
| `08` | AI / Agent / IoT 统一扩展层落地 | 接入智能主体、设备主体与协议插件 | `05`、`06`、`07` | agent / iot / device-management 统一模型 |
| `09` | 存储投影与可观测治理 | 固化存储抽象、投影和 SLO | `04`、`05`、`06` | storage / projection / observability |
| `10` | 部署脚本与多环境发布治理 | 打通本地、私有化、多环境部署与 provider 配置 | `07`、`09` | bin / deployments / profiles / provider-configs |
| `11` | 性能、高可用与灾备演练 | 完成容量、排空、故障恢复验证 | `09`、`10` | 压测、HA、DR 结果 |
| `12` | SDK / CLI 与兼容矩阵收口 | 打通客户端接入和兼容治理 | `03`、`06`、`07`、`10` | CLI、SDK、compat matrix |
| `13` | 发布就绪与持续迭代闭环 | 形成最终验收、发布与下一轮计划 | `00-12` | 发布闭环、下一轮 backlog |

## 5. 实施波次

### 波次 A：基线与骨架

- `00`
- `01`
- `02`
- `03`

目标是把“文档已清晰，但代码结构还未跟上”的状态，推进到“骨架已成型，协议与契约不再漂移”的状态。

### 波次 B：核心实时链路

- `04`
- `05`
- `06`

目标是完成连接、路由、消息、流、RTC 的主链路抽离和稳定化，让系统具备真正意义上的即时通信主路径能力。

### 波次 C：治理与扩展

- `07`
- `08`
- `09`

目标是完成控制面、协议治理、AI / Agent / IoT 扩展，以及存储投影和可观测体系。

### 波次 D：交付与验证

- `10`
- `11`
- `12`
- `13`

目标是把系统从“能开发”推进到“能部署、能验证、能演练、能发布、能兼容演进”。

## 6. 辅助治理文档

除 `00-13` 的主步骤外，本目录还引入以下两份长期治理文档，用于把“完整计划”提升为“可校对、可审计、可持续打磨”的执行体系：

- [`90-架构能力-Step-代码目录-证据映射矩阵`](./90-架构能力-Step-代码目录-证据映射矩阵.md)
- [`91-Step质量审计清单与复盘模板`](./91-Step质量审计清单与复盘模板.md)
- [`92-Step输入输出与阻塞升级规则`](./92-Step输入输出与阻塞升级规则.md)
- [`93-波次里程碑与阶段总验收矩阵`](./93-波次里程碑与阶段总验收矩阵.md)
- [`94-Step并行执行编排与车道拆分建议`](./94-Step并行执行编排与车道拆分建议.md)
- [`95-架构能力闭环验收标准`](./95-架构能力闭环验收标准.md)
- [`96-Step并行执行周计划与排班建议`](./96-Step并行执行周计划与排班建议.md)
- [`97-Step完成后的架构回写与能力兑现清单`](./97-Step完成后的架构回写与能力兑现清单.md)
- [`98-Step并行车道交付包与集成交接标准`](./98-Step并行车道交付包与集成交接标准.md)

使用原则如下：

- `90` 用于执行前核对“当前能力在哪个 step 落地、触达哪些目录、需要哪些证据”
- `91` 用于 step 完成前做质量评分、自审、review 和复盘归档
- `92` 用于当前 step 的输入、非目标、输出和阻塞升级控制
- `93` 用于波次级里程碑和跨 step 总验收门禁
- `94` 用于并行执行编排、车道拆分和写入边界控制
- `95` 用于判定每个 step 是否真正闭环实现了对应架构能力
- `96` 用于把 `94` 的并行方法具体化为周节奏、角色排班、收口窗口和最快执行路径
- `97` 用于要求每个 step 完成后回写 `docs/架构/` 的受影响能力文档，并给出“架构承诺是否已兑现”的证据
- `98` 用于规定并行车道在实现、验证、合并、升级阻塞时必须提交的交付包和集成交接标准
- 任何 step 若未通过 `91` 的质量门禁，不应进入下一个 step

## 7. 每一步必须产出的内容

每个 step 文档都必须明确：

- 执行输入
- 本步非目标
- 最小输出
- 推荐 review 产物
- 推荐并行车道
- 架构能力闭环判定
- 快速并行执行建议
- 完成后必须回写的架构文档
- 设计
- 实施落地规划
- 测试计划
- 结果验证
- 检查点
- 风险与回滚
- 完成定义
- 下一步准入条件

任何没有这些内容的 step，都视为未达执行标准。

## 8. 执行证据要求

每个 step 完成时，必须同步产出以下证据中的适用项：

- `docs/review/` 中的问题清单、决策记录、复盘记录
- 新增或更新的 `docs/架构/` 文档
- 对应代码目录下的测试、基准、脚本
- `bin/`、`deployments/`、`tools/` 的验证脚本
- 可重复执行的验证命令及结果摘要

推荐证据分层如下：

- 架构类证据：`docs/架构/`
- 审计与复盘类证据：`docs/review/`
- 自动化验证类证据：代码仓内测试与脚本
- 临时运行证据：`tmp/` 下的局部验证产物

## 9. 极致打磨规则

为满足“反复检查、打磨到极致”的要求，后续所有 step 在执行和复盘时，必须额外通过以下九个维度的自查：

- 架构对齐是否清晰到具体文档编号
- 代码触达范围是否精确到目录和高风险文件
- 协议、权限和权威字段边界是否明确
- 测试是否覆盖单元、集成、E2E 和脚本化验证中的适用项
- 结果验证是否可重复，不依赖人工口头说明
- 检查点是否具备“可判定完成/未完成”的标准
- 风险与回滚是否真实可执行，而不是泛泛而谈
- 与前置 step 和后续 step 的依赖是否清楚
- 是否形成 `docs/review/` 中的审计和复盘沉淀

建议在每次 step 收尾时，结合 [`91-Step质量审计清单与复盘模板`](./91-Step质量审计清单与复盘模板.md) 打分，目标分数不低于 `90/100`。

若 step 涉及高风险路径、协议边界或主链路切换，建议同时补一份基于 [`92-Step输入输出与阻塞升级规则`](./92-Step输入输出与阻塞升级规则.md) 的执行卡。

若目标是加快整体推进速度，建议在波次内结合 [`94-Step并行执行编排与车道拆分建议`](./94-Step并行执行编排与车道拆分建议.md) 组织并行车道。

若目标是确认“不是名义完成，而是真的实现了架构能力”，建议在 step 收尾时结合 [`95-架构能力闭环验收标准`](./95-架构能力闭环验收标准.md) 做能力闭环判定。

## 10. 当前阶段重点约束

结合现状与架构基线，当前执行时必须额外遵守以下约束：

- 不允许继续把新逻辑堆进 `services/sdkwork-im-cloud-gateway` 的 monolith 装配层（应拆到独立 `services/*` crate）
- 不允许继续把新逻辑堆进 `services/conversation-runtime/src/lib.rs`
- 不允许继续把新逻辑堆进 `services/session-gateway/src/lib.rs`
- 单文件超过 `1000` 行视为红线问题
- `tenantId`、`sender`、`actor` 等权威字段必须来源于认证上下文
- 协议骨架必须收敛到 `CCP`，不能让每个入口各搞一套帧格式
- `services/*` 只能做装配和运行入口，不能再成为大杂烩业务层
- 新增厂商或协议接入必须先走 `plugin / provider` 契约、registry 和 conformance 文档回写，禁止直接写私有分支

## 11. 当前仓库执行起点

基于当前仓库已知现状，执行计划的真实起点是：

- 当前 workspace 仍以 `im-*` 与 `service-*` 为主
- `CCP` crate 族尚未完整落地
- 高风险大文件仍然存在
- 跨平台 CLI（`chat-cli` / `chat-window`）与 topology v2 开发栈（`pnpm dev`）已作为验证基线
- 架构文档已经足够清晰，下一步重点是把它们转化为工程结构和实施节奏

## 12. 使用方式

推荐使用顺序：

1. 先阅读 [`00-总实施原则与执行门禁`](./00-总实施原则与执行门禁.md)
2. 再执行 [`01-现状基线冻结与差距审计`](./01-现状基线冻结与差距审计.md)
3. 执行前使用 [`90-架构能力-Step-代码目录-证据映射矩阵`](./90-架构能力-Step-代码目录-证据映射矩阵.md) 做能力对照
4. 关键 step 执行前，使用 [`92-Step输入输出与阻塞升级规则`](./92-Step输入输出与阻塞升级规则.md) 形成执行卡
5. 若同一波次内需要提速，先使用 [`94-Step并行执行编排与车道拆分建议`](./94-Step并行执行编排与车道拆分建议.md) 确定并行车道，再使用 [`96-Step并行执行周计划与排班建议`](./96-Step并行执行周计划与排班建议.md) 和 [`98-Step并行车道交付包与集成交接标准`](./98-Step并行车道交付包与集成交接标准.md) 固化节奏与交接包
6. 每完成一个 step，使用 [`91-Step质量审计清单与复盘模板`](./91-Step质量审计清单与复盘模板.md)、[`95-架构能力闭环验收标准`](./95-架构能力闭环验收标准.md) 和 [`97-Step完成后的架构回写与能力兑现清单`](./97-Step完成后的架构回写与能力兑现清单.md) 做自检、闭环判定和架构回写
7. 需要查阅当前 review 归档入口时，先打开 [`docs/review/README.md`](../review/README.md)
8. 每完成一个波次，使用 [`93-波次里程碑与阶段总验收矩阵`](./93-波次里程碑与阶段总验收矩阵.md) 做总验收
9. 之后严格按编号顺序推进
10. 当 `00-13` 全部闭环后，进入持续优化模式；每一轮只挑一个最小 provider/plugin 或主链缺口增量继续推进，并继续复用 `91 / 93 / 95 / 97`
11. 持续优化样例可直接参考 [`continuous-optimization-pre-release-capacity-tier-gates-2026-04-09`](./continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md)
    当前 Pre-Release Tier 为 `evidence_collected_gate_blocked`；默认预发布 profile 为 `standalone.split-services.development`，目标容量环境为 `capacity-dedicated`
    对应部署文档为 `docs/部署/性能与灾备演练场景.md`，对应 gate 模板为 `tools/perf/step-11-pre-release-tier-gate.json` 与 `tools/perf/step-11-capacity-tier-gate.json`
12. Step 12 的持续优化样例可直接参考 [`continuous-optimization-chat-cli-token-only-contract-2026-04-09`](./continuous-optimization-chat-cli-token-only-contract-2026-04-09.md)
    本轮收敛 `sdkwork-im-cli token --token-only` 的真实行为边界，明确默认 header 形态与裸 token 形态
    对应架构文档为 `docs/架构/09AS-chat-cli-token-only-contract-implementation-plan-2026-04-09.md` 与 `docs/架构/150AS-chat-cli-token-only-contract-design-2026-04-09.md`
    对应 catalog 为 `tools/perf/step-11-scenario-catalog.json`，未来高阶 `artifactRoot` 证据归档根目录为 `artifacts/perf/step-11/pre-release` 与 `artifacts/perf/step-11/capacity`
    对应 schema 为 `tools/perf/schemas/step-11-scenario-catalog.schema.json` 与 `tools/perf/schemas/step-11-tier-gate.schema.json`
    同时冻结 `collectionSummary`、`evidenceSlots`、`pending_collection`、`checksumSha256` 等 evidence-slot 契约字段，等待真实采集填充
    `collectionSummary` 公开 `totalSlots`、`requiredSlots`、`optionalSlots`、`collectedSlots`、`pendingSlots`、`skippedOptionalSlots` 六个统计字段
13. Step 12 的下一轮持续优化可直接参考 [`continuous-optimization-chat-cli-lowercase-bearer-normalization-contract-2026-04-09`](./continuous-optimization-chat-cli-lowercase-bearer-normalization-contract-2026-04-09.md)
    本轮收敛 `--bearer-token "bearer <token>"` 的大小写无关前缀归一化，固定默认 `authorization = Bearer <token>` 与 `token = <token>` 的边界
    同时冻结 `token --token-only` 在小写 bearer 输入下仍只返回裸 token，不能把前缀泄漏进结果字段
    对应架构文档为 `docs/架构/09AT-chat-cli-lowercase-bearer-normalization-contract-implementation-plan-2026-04-09.md` 与 `docs/架构/150AT-chat-cli-lowercase-bearer-normalization-contract-design-2026-04-09.md`
14. Step 12 的又一轮持续优化可直接参考 [`continuous-optimization-chat-cli-provided-token-claims-boundary-2026-04-09`](./continuous-optimization-chat-cli-provided-token-claims-boundary-2026-04-09.md)
    本轮收敛 `providedBearerToken` 的 `claims` 真实性边界，明确 CLI 只对本地生成 token 暴露 claims 载荷
    当 `source = providedBearerToken` 时，`claims` 必须返回 `null`，不能把 `tenant-id / user-id / session-id / device-id` 等本地输入伪装成外部 token 已解码结果
    对应架构文档为 `docs/架构/09AU-chat-cli-provided-token-claims-boundary-implementation-plan-2026-04-09.md` 与 `docs/架构/150AU-chat-cli-provided-token-claims-boundary-design-2026-04-09.md`
    当前冻结值为 `totalSlots = 7`、`requiredSlots = 7`、`optionalSlots = 0`、`collectedSlots = 0`、`pendingSlots = 7`、`skippedOptionalSlots = 0`
    evidence slot 元数据还包含 `artifactPath`、`suggestedRelativePath`、`collectedAt`、`sizeBytes` 等回填字段
15. Step 12 的下一轮 wrapper 收敛可直接参考 [`continuous-optimization-chat-cli-cmd-help-pass-through-contract-2026-04-09`](./continuous-optimization-chat-cli-cmd-help-pass-through-contract-2026-04-09.md)
16. Step 12 的下一轮 open-chat operator 收敛可直接参考 [`continuous-optimization-open-chat-test-cmd-gnu-flag-contract-2026-04-09`](./continuous-optimization-open-chat-test-cmd-gnu-flag-contract-2026-04-09.md)
    本轮收敛 `bin/open-chat-test.cmd` 的 GNU-style named flag 入口边界，固定 `.cmd` 路径必须真正进入 scripted validation，而不是在忽略参数后回落到默认开窗流程
    对应架构文档为 `docs/架构/09AW-open-chat-test-cmd-gnu-flag-contract-implementation-plan-2026-04-09.md` 与 `docs/架构/150AW-open-chat-test-cmd-gnu-flag-contract-design-2026-04-09.md`
17. Step 12 的下一轮 chat-window wrapper 收敛可直接参考 [`continuous-optimization-chat-window-cmd-gnu-flag-contract-2026-04-09`](./continuous-optimization-chat-window-cmd-gnu-flag-contract-2026-04-09.md)
    本轮收敛 `bin/chat-window.cmd` 的 GNU-style named flag 入口边界，固定 `.cmd` 路径必须真正进入 interactive chat-session，而不是在参数绑定失败后直接打印 usage
    对应架构文档为 `docs/架构/09AX-chat-window-cmd-gnu-flag-contract-implementation-plan-2026-04-09.md` 与 `docs/架构/150AX-chat-window-cmd-gnu-flag-contract-design-2026-04-09.md`
18. Step 12 的下一轮 open-chat scripted-validation 保真收敛可直接参考 [`continuous-optimization-open-chat-test-cmd-validation-message-special-char-contract-2026-04-09`](./continuous-optimization-open-chat-test-cmd-validation-message-special-char-contract-2026-04-09.md)
    本轮收敛 `bin/open-chat-test.cmd` 的 `--validation-message` 特殊字符边界，固定 Windows `.cmd` wrapper 不得吞掉 `!` 这类操作员输入内容
    对应架构文档为 `docs/架构/09AY-open-chat-test-cmd-validation-message-special-char-contract-implementation-plan-2026-04-09.md` 与 `docs/架构/150AY-open-chat-test-cmd-validation-message-special-char-contract-design-2026-04-09.md`
19. Step 12 的下一轮 chat-window interactive 保真收敛可直接参考 [`continuous-optimization-chat-window-cmd-message-prefix-special-char-contract-2026-04-09`](./continuous-optimization-chat-window-cmd-message-prefix-special-char-contract-2026-04-09.md)
    本轮收敛 `bin/chat-window.cmd` 的 `--message-prefix` 特殊字符边界，固定 Windows `.cmd` wrapper 不得吞掉 `!` 这类操作员输入前缀
    对应架构文档为 `docs/架构/09AZ-chat-window-cmd-message-prefix-special-char-contract-implementation-plan-2026-04-09.md` 与 `docs/架构/150AZ-chat-window-cmd-message-prefix-special-char-contract-design-2026-04-09.md`
20. Step 12 的下一轮 chat-window help discoverability 收敛可直接参考 [`continuous-optimization-chat-window-cmd-help-gnu-surface-contract-2026-04-09`](./continuous-optimization-chat-window-cmd-help-gnu-surface-contract-2026-04-09.md)
    本轮收敛 `bin/chat-window.cmd --help` 的帮助面边界，固定 Windows operator 在本地 help 中也能直接看到 GNU-style named flags
    对应架构文档为 `docs/架构/09BA-chat-window-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md` 与 `docs/架构/150BA-chat-window-cmd-help-gnu-surface-contract-design-2026-04-09.md`
21. Step 12 的下一轮 open-chat-test help discoverability 收敛可直接参考 [`continuous-optimization-open-chat-test-cmd-help-gnu-surface-contract-2026-04-09`](./continuous-optimization-open-chat-test-cmd-help-gnu-surface-contract-2026-04-09.md)
    本轮收敛 `bin/open-chat-test.cmd --help` 的帮助面边界，固定 Windows operator 在本地 help 中也能直接看到 `--owner-user-id`、`--scripted-validation`、`--validation-message`、`--json` 这类 GNU-style named flags
    对应架构文档为 `docs/架构/09BB-open-chat-test-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md` 与 `docs/架构/150BB-open-chat-test-cmd-help-gnu-surface-contract-design-2026-04-09.md`
22. Step 12 的下一轮 chat-window-gui help discoverability 收敛可直接参考 [`continuous-optimization-chat-window-gui-cmd-help-gnu-surface-contract-2026-04-09`](./continuous-optimization-chat-window-gui-cmd-help-gnu-surface-contract-2026-04-09.md)
    本轮收敛 `bin/chat-window-gui.cmd --help` 的帮助面边界，固定 Windows operator 在本地 help 中也能直接看到 `--conversation-id`、`--user-id`、`--message-prefix` 这类 GNU-style named flags
    对应架构文档为 `docs/架构/09BC-chat-window-gui-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md` 与 `docs/架构/150BC-chat-window-gui-cmd-help-gnu-surface-contract-design-2026-04-09.md`
23. Step 12 的下一轮 chat-window-gui literal-fidelity 收敛可直接参考 [`continuous-optimization-chat-window-gui-cmd-label-special-char-contract-2026-04-09`](./continuous-optimization-chat-window-gui-cmd-label-special-char-contract-2026-04-09.md)
    本轮收敛 `bin/chat-window-gui.cmd` 的 label 保真边界，固定 Windows GUI wrapper 不得吞掉 `!` 这类 `-Label` / `--label` 内容
    对应架构文档为 `docs/架构/09BD-chat-window-gui-cmd-label-special-char-contract-implementation-plan-2026-04-09.md` 与 `docs/架构/150BD-chat-window-gui-cmd-label-special-char-contract-design-2026-04-09.md`
24. Step 12 的下一轮 chat-window-gui GNU-style runtime 收敛可直接参考 [`continuous-optimization-chat-window-gui-cmd-gnu-flag-contract-2026-04-09`](./continuous-optimization-chat-window-gui-cmd-gnu-flag-contract-2026-04-09.md)
    本轮收敛 `bin/chat-window-gui.cmd` 的 GNU-style named flag 入口边界，固定 `.cmd` 路径必须真正进入 GUI 脚本，而不是在参数绑定失败后直接打印 usage
    对应架构文档为 `docs/架构/09BE-chat-window-gui-cmd-gnu-flag-contract-implementation-plan-2026-04-09.md` 与 `docs/架构/150BE-chat-window-gui-cmd-gnu-flag-contract-design-2026-04-09.md`
    本轮收敛 `bin/chat-cli.cmd --help` 的 Windows 入口边界，固定 `.cmd` wrapper 必须把 CLI 原始参数透传给 `chat-cli-local.ps1`
    禁止 wrapper 再把 `--help` 改写成 `-Help` 这类 `sdkwork-im-cli` 不接受的 PowerShell 风格参数
    对应架构文档为 `docs/架构/09AV-chat-cli-cmd-help-pass-through-contract-implementation-plan-2026-04-09.md` 与 `docs/架构/150AV-chat-cli-cmd-help-pass-through-contract-design-2026-04-09.md`
    evidence slot 语义字段还包含 `scenarioFamily`、`required`、`reportId`，用于区分场景槽位与报告槽位
    最小示例值统一冻结为 `scenarioFamily = connection` / `scenarioFamily = failover`、`required = true`、`reportId = capacity_report` / `reportId = recovery_report`
    `reportId` 与 `artifactKind` 的对应关系包括 `capacity_report -> report_markdown`、`recovery_report -> report_markdown`
    `reportId` 与建议路径的对应关系包括 `capacity_report -> reports/capacity-report.md`、`recovery_report -> reports/recovery-report.md`
    `reportId` 与 `requiredSections` 的对应关系包括 `capacity_report -> input_scale / throughput_summary / tail_latency_summary`、`recovery_report -> recovery_window / rto_rpo_summary / operator_follow_up`
    `suggestedRelativePath` 示例包括 `connection/metrics.json`、`failover/drill.json`、`reports/capacity-report.md`、`reports/recovery-report.md`
    Capacity Tier 额外示例路径包括 `connection/capacity.json`、`restore-recovery/recovery.json`、`failover/recovery.json`
    Capacity Tier 剩余 capacity 路径示例包括 `message/capacity.json`、`stream/capacity.json`
    evidence slot 还冻结主键 `id`，代表值包括 `connection_metrics`、`connection_capacity`、`failover_recovery`
    Capacity Tier 其余代表性 slot id 还包括 `message_capacity`、`stream_capacity`、`restore_recovery_recovery`
  - Pre-Release Tier collected slot examples now include `message_metrics` and `stream_metrics`
  - Pre-Release Tier collected path examples now include `message/metrics.json` and `stream/metrics.json`
  - Pre-Release Tier current state is now `evidence_collected_gate_blocked`
  - Capacity Tier current state is now `evidence_collected_gate_passed`
  - Both Step 11 tier artifact roots now carry truthful local evidence; dedicated topology runs still gate full commercial sign-off.
    evidence slot 还公开 `artifactKind`，代表值包括 `metrics_json`、`drill_json`、`capacity_json`、`recovery_json`、`report_markdown`
    机器契约还冻结 `requiredFields` / `requiredSections`，示例值包括 `runId`、`connectP95Ms`、`input_scale`、`operator_follow_up`
    额外字段示例包括 `messageTps`、`frameP95Ms`、`recovery_window`、`rto_rpo_summary`
    report section 代表值还包括 `throughput_summary`、`tail_latency_summary`、`recovery_window`、`operator_follow_up`
    更细一级字段示例还包括 `fanoutP95Ms`、`streamFramesPerSecond`、`previewDiffAccuracy`、`rollbackActivationSeconds`
    drill / rollback 字段示例还包括 `drainCompletionSeconds`、`restoreRtoSeconds`、`compatibilityMatrixPassRate`、`postRollbackProtocolErrorRate`
    `artifactKind` 与代表字段/section 的对应关系包括 `metrics_json -> connectP95Ms / messageTps / frameP95Ms`、`drill_json -> drainCompletionSeconds / rollbackActivationSeconds`、`capacity_json -> fanoutP95Ms / streamFramesPerSecond`、`recovery_json -> restoreRtoSeconds / previewDiffAccuracy`、`report_markdown -> throughput_summary / rto_rpo_summary`
    `artifactKind` 与建议路径的对应关系包括 `metrics_json -> connection/metrics.json / message/metrics.json`、`drill_json -> failover/drill.json / restore-recovery/drill.json`、`capacity_json -> connection/capacity.json / message/capacity.json`、`recovery_json -> failover/recovery.json / restore-recovery/recovery.json`、`report_markdown -> reports/capacity-report.md / reports/recovery-report.md`
    `artifactKind` 与代表性 `slot id` 的对应关系包括 `metrics_json -> connection_metrics / message_metrics`、`drill_json -> failover_drill / restore_recovery_drill`、`capacity_json -> connection_capacity / message_capacity`、`recovery_json -> failover_recovery / restore_recovery_recovery`、`report_markdown -> capacity_report / recovery_report`
    `artifactKind` 与 `requiredFields / requiredSections` 的对应关系包括 `metrics_json -> runId / connectionCount / successCount`、`drill_json -> runId / drainCompletionSeconds / takeoverDurationMs`、`capacity_json -> runId / peakActiveConnections / messageTps`、`recovery_json -> runId / restoreRtoSeconds / staleSessionRejectionRate`、`report_markdown -> input_scale / throughput_summary / operator_follow_up`
    `requiredScenarioFamilies = connection / message / stream / drain-rebalance / restore-recovery / failover / upgrade-rollback`
    `requiredScenarioFamilies = connection / message / stream / restore-recovery / failover`
    `requiredReports = capacity_report / recovery_report`
    `requiredOutputs` 以 `scenarioFamily -> artifactKind -> requiredFields` tuple 冻结最小输出契约，代表项包括 `connection -> metrics_json -> runId / connectionCount / successCount`、`restore-recovery -> recovery_json -> runId / restoreRtoSeconds / dataLossRpoEvents / previewDiffAccuracy`
    `operatorDocPath = docs/部署/性能与灾备演练场景.md`，`scenarioCatalogPath = tools/perf/step-11-scenario-catalog.json`
    `profile = standalone.split-services.development / capacity-dedicated`
    `reviewBackwrite = docs/step/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md / docs/review/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md / docs/架构/09AR-pre-release-capacity-tier-gates-implementation-plan-2026-04-09.md / docs/架构/150AR-pre-release-capacity-tier-gates-design-2026-04-09.md`
    `scenarioFamily` 与 `artifactKind` 的对应关系包括 `connection -> metrics_json / capacity_json`、`failover -> drill_json / recovery_json`、`restore-recovery -> drill_json / recovery_json`
    `scenarioFamily` 与 `requiredFields / requiredSections` 的对应关系包括 `connection -> runId / connectP95Ms`、`failover -> runId / takeoverDurationMs`、`restore-recovery -> runId / restoreRtoSeconds / previewDiffAccuracy`
    `scenarioFamily` 与 slot id 的对应关系包括 `connection -> connection_metrics / connection_capacity`、`failover -> failover_drill / failover_recovery`、`restore-recovery -> restore_recovery_drill / restore_recovery_recovery`
    代表性 `slot id` 与 `artifactKind` 的对应关系包括 `connection_metrics -> metrics_json`、`failover_drill -> drill_json`、`restore_recovery_recovery -> recovery_json`
    代表性 `slot id` 与 `requiredFields / requiredSections` 的对应关系包括 `connection_metrics -> runId / connectP95Ms`、`failover_drill -> runId / takeoverDurationMs`、`capacity_report -> input_scale / throughput_summary / tail_latency_summary`
    `scenarioFamily` 与建议路径的对应关系包括 `connection -> connection/metrics.json / connection/capacity.json`、`failover -> failover/drill.json / failover/recovery.json`、`restore-recovery -> restore-recovery/drill.json / restore-recovery/recovery.json`
    代表性 `slot id` 与建议路径的对应关系包括 `connection_metrics -> connection/metrics.json`、`failover_drill -> failover/drill.json`、`restore_recovery_recovery -> restore-recovery/recovery.json`
    默认命名关系为 `artifactPath = artifactRoot + "/" + suggestedRelativePath`
    在真实采集前，`artifactPath`、`collectedAt`、`sizeBytes`、`checksumSha256` 继续保持 `null`

如果中途发生重大架构偏移，必须先回写 `docs/架构/` 和 `docs/step/`，再继续编码。
# 2026-04-09 Step 12 Addendum

- [continuous-optimization-retired-lifecycle-start-cmd-help-gnu-surface-contract-2026-04-09](./continuous-optimization-retired-lifecycle-start-cmd-help-gnu-surface-contract-2026-04-09.md)
  - close the `retired-lifecycle-start.cmd --help` GNU-style discoverability gap on Windows
  - verification anchored in `test_start_local_cmd_help_surfaces_gnu_style_named_flags`
- [continuous-optimization-retired-lifecycle-status-cmd-help-gnu-surface-contract-2026-04-09](./continuous-optimization-retired-lifecycle-status-cmd-help-gnu-surface-contract-2026-04-09.md)
  - close the `retired-lifecycle-status.cmd --help` GNU-style discoverability gap on Windows
  - verification anchored in `test_status_local_cmd_help_surfaces_gnu_style_named_flags`
- [continuous-optimization-principal-profile-runtime-provider-selection-2026-04-09](./continuous-optimization-principal-profile-runtime-provider-selection-2026-04-09.md)
  - close the default runtime gap between `principal-profile-upstream-context` and `principal-profile-external-catalog`
  - verification anchored in `test_default_app_uses_configured_external_principal_profile_provider`
- [continuous-optimization-install-deploy-cmd-help-gnu-surface-contract-2026-04-09](./continuous-optimization-install-deploy-cmd-help-gnu-surface-contract-2026-04-09.md)
  - close the remaining Windows help-surface drift for `retired-lifecycle-install.cmd` and `retired-lifecycle-deploy.cmd`
  - verification anchored in `test_install_local_cmd_help_surfaces_gnu_style_named_flags` and `test_deploy_local_cmd_help_surfaces_gnu_style_named_flags`
- [continuous-optimization-principal-profile-external-catalog-missing-catalog-unavailable-contract-2026-04-09](./continuous-optimization-principal-profile-external-catalog-missing-catalog-unavailable-contract-2026-04-09.md)
  - close the missing external principal-profile catalog-path drift where app assembly still panicked
  - verification anchored in `test_default_app_boots_with_external_principal_profile_provider_missing_catalog_path_and_returns_provider_unavailable`
## 2026-04-09 Addendum

- [continuous-optimization-principal-profile-provider-health-http-surface-2026-04-09](./continuous-optimization-principal-profile-provider-health-http-surface-2026-04-09.md)
  - close the missing `GET /backend/v3/api/principal/profiles/provider_health` surface
  - verification anchored in `test_local_minimal_profile_gets_principal_profile_provider_health_over_http`
## 2026-04-09 Addendum

- [continuous-optimization-local-minimal-ops-provider-bindings-runtime-visibility-2026-04-09](./continuous-optimization-local-minimal-ops-provider-bindings-runtime-visibility-2026-04-09.md)
- [continuous-optimization-local-minimal-ops-provider-bindings-http-surface-2026-04-09](./continuous-optimization-local-minimal-ops-provider-bindings-http-surface-2026-04-09.md)
## 2026-04-09 Addendum

- [continuous-optimization-step11-tier-gate-doc-state-alignment-2026-04-09](./continuous-optimization-step11-tier-gate-doc-state-alignment-2026-04-09.md)
  - close the Step 11 doc-state drift after tier-level `artifactRoot` had already shipped in the catalog
  - verification anchored in `test_continuous_optimization_step11_step_doc_marks_artifact_root_gap_closed`
## 2026-04-09 Addendum

- [continuous-optimization-step11-tier-artifact-root-materialization-2026-04-09](./continuous-optimization-step11-tier-artifact-root-materialization-2026-04-09.md)
  - materialize the published high-tier Step 11 `artifactRoot` paths inside the repo
  - verification anchored in `test_continuous_optimization_materializes_step11_tier_artifact_roots_in_repo`
## 2026-04-09 Addendum

- [continuous-optimization-step11-tier-machine-readable-evidence-index-2026-04-09](./continuous-optimization-step11-tier-machine-readable-evidence-index-2026-04-09.md)
  - co-locate machine-readable Step 11 tier evidence indexes with the high-tier artifact roots
  - verification anchored in `test_continuous_optimization_co_locates_step11_tier_evidence_indexes_with_artifact_roots`
## 2026-04-09 Addendum

- [continuous-optimization-step11-pre-release-failover-collected-evidence-2026-04-09](./continuous-optimization-step11-pre-release-failover-collected-evidence-2026-04-09.md)
  - materialize one truthful `Pre-Release Tier` collected `failover/drill.json` artifact
  - verification anchored in `test_continuous_optimization_materializes_pre_release_failover_collected_evidence`
- [continuous-optimization-step11-pre-release-restore-recovery-collected-evidence-2026-04-09](./continuous-optimization-step11-pre-release-restore-recovery-collected-evidence-2026-04-09.md)
  - materialize one truthful `Pre-Release Tier` collected `restore-recovery/drill.json` artifact
  - verification anchored in `test_continuous_optimization_materializes_pre_release_restore_recovery_collected_evidence`
- [continuous-optimization-step11-pre-release-drain-rebalance-collected-evidence-2026-04-09](./continuous-optimization-step11-pre-release-drain-rebalance-collected-evidence-2026-04-09.md)
  - materialize one truthful `Pre-Release Tier` collected `drain-rebalance/drill.json` artifact
  - verification anchored in `test_continuous_optimization_materializes_pre_release_drain_rebalance_collected_evidence`
- [continuous-optimization-step11-pre-release-upgrade-rollback-collected-evidence-2026-04-09](./continuous-optimization-step11-pre-release-upgrade-rollback-collected-evidence-2026-04-09.md)
  - materialize one truthful `Pre-Release Tier` collected `upgrade-rollback/drill.json` artifact
  - verification anchored in `test_continuous_optimization_materializes_pre_release_upgrade_rollback_collected_evidence`
- [continuous-optimization-step11-pre-release-connection-metrics-collected-evidence-2026-04-09](./continuous-optimization-step11-pre-release-connection-metrics-collected-evidence-2026-04-09.md)
  - materialize one truthful `Pre-Release Tier` collected `connection/metrics.json` artifact
  - verification anchored in `test_continuous_optimization_materializes_pre_release_connection_metrics_collected_evidence`
- [continuous-optimization-step11-pre-release-message-metrics-collected-evidence-2026-04-09](./continuous-optimization-step11-pre-release-message-metrics-collected-evidence-2026-04-09.md)
  - materialize one truthful `Pre-Release Tier` collected `message/metrics.json` artifact
  - verification anchored in `test_continuous_optimization_materializes_pre_release_message_metrics_collected_evidence`
- [continuous-optimization-step11-pre-release-stream-metrics-collected-evidence-2026-04-09](./continuous-optimization-step11-pre-release-stream-metrics-collected-evidence-2026-04-09.md)
  - materialize one truthful `Pre-Release Tier` collected `stream/metrics.json` artifact and move the tier into `evidence_collected_gate_blocked`
  - verification anchored in `test_continuous_optimization_materializes_pre_release_stream_metrics_collected_evidence`
- [continuous-optimization-step11-closure-claim-supersession-2026-04-09](./continuous-optimization-step11-closure-claim-supersession-2026-04-09.md)
  - supersede stale Step 11 closure wording with the current high-tier evidence state
  - verification anchored in `test_continuous_optimization_supersedes_stale_step11_closure_claims_in_historical_docs`
## 2026-04-09 Addendum

- [continuous-optimization-shell-process-identity-portability-2026-04-09](./continuous-optimization-shell-process-identity-portability-2026-04-09.md)
  - close the Bash lifecycle portability drift caused by truncation-prone `ps -o comm=` matching
  - verification anchored in `test_shell_lifecycle_scripts_use_args_based_process_identity_for_portability`
- [continuous-optimization-runtime-lifecycle-profile-selection-2026-04-09](./continuous-optimization-runtime-lifecycle-profile-selection-2026-04-09.md)
  - close the remaining Step 10 lifecycle gap where `standalone.split-services.development` profile selection did not reach init/install/start/stop/restart
  - verification anchored in `test_init_config_local_ps1_uses_local_default_profile_when_requested` and `test_restart_local_ps1_forwards_profile_name_to_stop_and_start_scripts`
- [continuous-optimization-lifecycle-profile-doc-contract-alignment-2026-04-09](./continuous-optimization-lifecycle-profile-doc-contract-alignment-2026-04-09.md)
  - close the remaining Step 10 operator-doc gap where lifecycle profile examples lagged behind the shipped scripts
  - verification anchored in `test_quick_start_doc_surfaces_local_default_profile_examples_across_lifecycle_commands`
- [continuous-optimization-retired-lifecycle-start-ps1-health-timeout-test-stability-2026-04-09](./continuous-optimization-retired-lifecycle-start-ps1-health-timeout-test-stability-2026-04-09.md)
  - stabilize the Step 10 health-timeout rollback regression by restoring a realistic test scheduling window
  - verification anchored in `test_start_local_ps1_stops_background_process_and_clears_pid_file_when_health_check_times_out`
## 2026-04-09 Addendum

- [continuous-optimization-restore-runtime-cmd-expected-preview-fingerprint-2026-04-09](./continuous-optimization-restore-runtime-cmd-expected-preview-fingerprint-2026-04-09.md)
  - close the Step 10 Windows restore-wrapper gap where `.cmd` did not preserve the documented preview fingerprint confirmation flag
  - verification anchored in `test_restore_runtime_local_cmd_normalizes_expected_preview_fingerprint_switch`
## 2026-04-09 Addendum

- [continuous-optimization-inspect-runtime-cmd-help-gnu-surface-contract-2026-04-09](./continuous-optimization-inspect-runtime-cmd-help-gnu-surface-contract-2026-04-09.md)
  - close the Step 10 Windows inspect-runtime help discoverability gap for GNU-style operator flags
  - verification anchored in `test_inspect_runtime_local_cmd_help_surfaces_gnu_style_named_flags`
## 2026-04-09 Addendum

- [continuous-optimization-retired-lifecycle-start-ps1-health-timeout-window-recalibration-2026-04-09](./continuous-optimization-retired-lifecycle-start-ps1-health-timeout-window-recalibration-2026-04-09.md)
  - re-stabilize the Step 10 Windows health-timeout rollback proof after the prior synthetic startup window stayed too small
  - verification anchored in `test_start_local_ps1_stops_background_process_and_clears_pid_file_when_health_check_times_out`
## 2026-04-09 Addendum

- [continuous-optimization-repair-runtime-cmd-help-gnu-surface-contract-2026-04-09](./continuous-optimization-repair-runtime-cmd-help-gnu-surface-contract-2026-04-09.md)
  - close the Step 10 Windows repair-runtime help discoverability gap for GNU-style operator flags
  - verification anchored in `test_repair_runtime_local_cmd_help_surfaces_gnu_style_named_flags`
## 2026-04-09 Addendum

- [continuous-optimization-open-chat-test-detached-gui-start-process-fallback-2026-04-09](./continuous-optimization-open-chat-test-detached-gui-start-process-fallback-2026-04-09.md)
  - close the Step 10 Windows popup-launch gap by inserting a stable `Start-Process` fallback into `open-chat-test.ps1`
  - verification anchored in `test_open_chat_test_ps1_uses_detached_gui_launcher_for_default_windows_mode`
## 2026-04-09 当前推荐入口

- 当前应优先从 [100-面向DDD的IM重构升级总实施计划-2026-04-09](./100-面向DDD的IM重构升级总实施计划-2026-04-09.md) 进入。
- 真正开工前，必须基于 [101-Step执行卡与并行交付模板-2026-04-10](./101-Step执行卡与并行交付模板-2026-04-10.md) 为当前 `Sxx` 生成执行卡。
- 执行前最小阅读顺序建议固定为：`100-* -> 101-* -> 95 -> 97`。
- 若新总计划与旧 `00-13` step 文档冲突，以 `100-*` 为准。
- 旧 `00-13` 继续保留，角色是历史执行资产、细节参考、已落地证据索引，不再代表当前最优主脊。

## 2026-04-09 Addendum

- [99-step-loop-execution-and-closure-protocol-2026-04-09](./99-step-loop-execution-and-closure-protocol-2026-04-09.md)
  - 定义 loop、step、wave、release 四层闭环与禁止伪完成的统一协议
- [101-Step执行卡与并行交付模板-2026-04-10](./101-Step执行卡与并行交付模板-2026-04-10.md)
  - 提供单个 `Sxx` 开工前必须实例化的执行卡与车道交付包模板
- [../prompts/反复执行Step指令.md](../prompts/反复执行Step指令.md)
  - 提供可重复输入给大模型的循环执行提示词模板，要求持续迭代直到真实闭环
