> Migrated from `docs/step/01-现状基线冻结与差距审计.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 01 - 现状基线冻结与差距审计

## 1. 目标与范围

本 step 用于把当前仓库的真实工程状态与目标架构做一次完整映射，形成后续所有重构动作的输入基线。

本 step 不追求“立刻修完所有问题”，而是先回答以下问题：

- 当前仓库已经实现了什么
- 当前仓库距离目标架构还差什么
- 哪些问题是结构性问题，必须先修
- 哪些问题可以在后续 step 中渐进演进

### 1.1 执行输入

- 当前根级 `Cargo.toml` 和 workspace 成员清单
- 当前 `crates/`、`services/`、`adapters/`、`sdks/`、`tools/`、`bin/`、`deployments/`
- 当前 `docs/架构/` 与已存在的 `docs/review/` 文档
- 当前高风险文件统计结果

### 1.2 本步非目标

- 不在本 step 内直接拆大文件或迁移业务逻辑
- 不在本 step 内冻结新的协议字段细节
- 不在本 step 内调整默认部署与运行 profile

### 1.3 最小输出

- 当前基线审计结论
- 差距矩阵与优先级清单
- 高风险文件和高风险目录清单
- 可直接作为 step 02 输入的结构化审计文档

## 2. 架构对齐

本 step 重点对齐：

- `docs/架构/09-实施计划.md`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
- `docs/架构/131-连接管理与分层弹性扩容架构设计-2026-04-06.md`
- `docs/架构/133-代码结构治理与crate拆分标准-2026-04-06.md`
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`

## 3. 当前现状与问题

基于当前仓库结构，已知现状如下：

- workspace 目前仍以 `im-*` core crate 和 `services/*` 服务为主
- 当前成员包括 `im-domain-core`、`im-domain-events`、`im-platform-contracts`、`im-auth-context`、`im-time`
- 当前服务包括 `conversation-runtime`、`session-gateway`、`streaming-service`、`im-call-runtime`、`projection-service`、`control-plane-api` 等
- 本地最小运行集成体为 `services/sdkwork-im-cloud-gateway`
- 已经存在跨平台 `bin/` 脚本以及 `tools/chat-cli`

当前最突出的结构性问题：

- `services/sdkwork-im-cloud-gateway/src/lib.rs` 约 `5609` 行
- `services/conversation-runtime/src/lib.rs` 约 `4105` 行
- `services/session-gateway/src/lib.rs` 约 `1192` 行
- `CCP` crate 族还未实体化
- `contract / domain / app / interface / runtime / service` 边界还未完全落地
- 当前服务分层与目标的 `Link / Route / Messaging / Stream-AI / Projection / Storage` plane 还未一一映射

## 4. 设计

### 4.1 审计输出模型

本 step 的输出必须至少包含：

- 当前 workspace 实体清单
- 当前目录与目标目录的映射关系
- 当前高风险文件与高风险依赖列表
- 当前已有测试资产清单
- 当前脚本与部署能力清单
- 差距矩阵和优先级排序

### 4.2 差距分类

所有差距统一分成四类：

- `A 类`：阻塞后续所有重构的结构问题
- `B 类`：影响主链路正确性和扩展性的核心问题
- `C 类`：影响交付质量、部署、兼容性的问题
- `D 类`：优化类问题，可延后但需记入 backlog

### 4.3 审计对象

需要纳入本 step 的审计对象包括：

- 根级 `Cargo.toml`
- `crates/`
- `services/`
- `adapters/`
- `sdks/`
- `tools/`
- `bin/`
- `deployments/`
- `docs/架构/`
- `docs/review/`

### 4.4 输出原则

审计不是写泛泛总结，必须输出“后续谁先改什么”的结论。

因此输出必须具备：

- 真实路径
- 真实问题
- 对应架构文档引用
- 处理建议
- 推荐进入的 step

## 5. 实施落地规划

### 5.1 审计任务拆解

1. 盘点当前 workspace 成员和依赖方向
2. 盘点当前服务职责与目标 plane 的对应关系
3. 统计高风险文件、超长文件和疑似边界混写文件
4. 盘点现有测试资产和脚本资产
5. 把问题归档到 `docs/review/`，形成可执行差距矩阵

### 5.2 重点关注路径

优先审计以下路径：

- `Cargo.toml`
- `crates/im-domain-core/`
- `crates/im-platform-contracts/`
- `services/sdkwork-im-cloud-gateway/`
- `services/session-gateway/`
- `services/conversation-runtime/`
- `services/streaming-service/`
- `services/im-call-runtime/`
- `services/control-plane-api/`
- `tools/chat-cli/`
- `bin/`
- `deployments/`

### 5.3 审计输出建议

建议至少新增或更新以下 review 文档：

- `docs/review/当前基线审计-YYYY-MM-DD.md`
- `docs/review/workspace-差距矩阵-YYYY-MM-DD.md`
- `docs/review/高风险文件清单-YYYY-MM-DD.md`

### 5.4 审计后的立即决策

本 step 完成后必须明确：

- 哪些 crate 需要新增
- 哪些现有 crate 需要保留为兼容层
- 哪些 service 必须优先薄化
- 哪些协议与契约必须在 step 03 前冻结
- 哪些脚本和工具可直接复用到后续步骤

## 6. 测试计划

本 step 的测试重点是“审计结果是否建立在真实代码基础之上”。

建议执行：

- `cargo metadata --format-version 1`
- `cargo test --workspace --no-run`
- 关键服务 smoke 测试清点
- 高风险文件行数统计脚本
- `bin/retired-lifecycle-start.*`、`bin/retired-lifecycle-status.*`、`tools/chat-cli` 存在性检查

## 7. 结果验证

本 step 完成后，需要能够明确回答：

- 当前仓库最先要动哪些目录
- 哪些已有能力可以保留并渐进重构
- 哪些功能已经具备主路径基础，不需要推倒重来
- 哪些地方是典型的结构性阻塞点

如果这些问题仍回答不清，本 step 不算完成。

## 8. 检查点

- `CP01-1`：完成当前 workspace、services、adapters、tools、bin 的全量盘点
- `CP01-2`：形成“当前结构 -> 目标架构”的差距矩阵
- `CP01-3`：形成高风险文件和优先整改目录清单
- `CP01-4`：在 `docs/review/` 中沉淀至少一轮基线审计结果

### 8.1 推荐 review 产物

- `docs/review/step-01-执行卡-YYYY-MM-DD.md`
- `docs/review/step-01-基线审计-YYYY-MM-DD.md`
- `docs/review/step-01-差距矩阵-YYYY-MM-DD.md`
- `docs/review/step-01-高风险文件清单-YYYY-MM-DD.md`

### 8.2 推荐并行车道

- `01-A`：仓库结构、Cargo/workspace、crate/service 盘点
- `01-B`：架构能力映射、差距矩阵、后续 step 归属判断
- `01-C`：测试、脚本、部署资产与验证入口盘点
- 收口要求：仅允许 `01-Owner` 汇总统一差距矩阵，`02/03` 只能以同一版审计结论为输入，避免并行审计口径漂移。
- 车道编排参考：[`94-Step并行执行编排与车道拆分建议`](./94-Step并行执行编排与车道拆分建议.md)

### 8.3 架构能力闭环判定

- 必须完成“当前能力 -> 目标架构 -> 对应 step -> 代码目录 -> 证据产物”的五段映射，才能作为后续重构输入。
- 如果只有概览或问题列表，无法直接支撑 `02-workspace与crate骨架重构` 的目录拆分和优先级决策，不算闭环。
- 闭环验收以 [`95-架构能力闭环验收标准`](./95-架构能力闭环验收标准.md) 中 Step 01 条目为准。

### 8.4 快速并行执行建议

- 审计阶段只做盘点和判断，不做代码重构，避免审计口径被中途实现污染。
- 推荐分成“仓库盘点”“架构映射”“测试脚本盘点”三车道，并在同一天内产出统一差距矩阵。
- 本步完成当日就把结论直接交给 Step 02 和 Step 03，避免基线审计与骨架设计脱节。

### 8.5 完成后必须回写的架构文档

- 强制范围：本文件 `## 2. 架构对齐` 中列出的全部架构文档。
- 回写重点：当前仓库 reality 与架构蓝图、连接分层、crate 拆分、CCP 落点之间的真实差距，必须回写到实施计划与相关架构基线。
- 必备证据：`docs/review/step-01-架构兑现-YYYY-MM-DD.md` 与 `docs/review/step-01-架构回写决议-YYYY-MM-DD.md`。

## 9. 风险与回滚

### 9.1 风险

- 只看文档不看代码，会导致计划脱离真实仓库
- 只看服务功能不看目录边界，会漏掉后续大规模返工点
- 不把测试和脚本纳入审计，会导致后续验证缺失

### 9.2 回滚

本 step 主要是审计与文档输出，无需代码回滚。若差距矩阵判断失误，应先修订审计文档，再继续后续重构。

## 10. 完成定义

以下条件全部满足时，本 step 才算完成：

- 当前仓库结构、主要服务和主要工具已被盘点
- 高风险文件与边界问题已被定级
- 差距矩阵已对应到具体后续 step
- review 文档已产出并可供后续步骤引用

## 11. 下一步准入条件

进入 step 02 前必须确认：

- 高风险大文件已被列为优先拆分对象
- 目标 crate 拓扑和迁移原则已经有清晰结论

