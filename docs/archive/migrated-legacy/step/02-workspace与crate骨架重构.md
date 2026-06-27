# Step 02 - workspace 与 crate 骨架重构

## 1. 目标与范围

本 本 step 用于把当前“服务偏重、crate 分层不足、超大文件承载过多职责”的工程形态，重构为符合目标架构的 workspace 骨架

step 主要解决

- `contract / domain / app / interface / runtime / service / adapter` 分层具备真实目录承载
- `services/*` 回归装配层，而不是继续承担业务大杂烩
- 让超大 `lib.rs` 被目录化拆分
- `CCP`、Link Plane、Route Plane、AI / IoT 扩展等后续 step 预留明确落点

### 1.1 执行输入

- step 01 的差距矩阵与高风险文件清单
- 当前 workspace 成员列表与服务职责盘点
- 当前超大 `lib.rs` 和混写目录的真实统计结果
- `130`、`133`、`147` 的目标骨架文档

### 1.2 本步非目标

- 不在本 step 内完成全部业务迁移
- 不在本 step 内切换完整消息主链路或流式主链路
- 不在本 step 内完成性能优化与高可用演练

### 1.3 最小输出

- 可编译的目标 workspace 骨架
- 新旧 crate 迁移路径
- 高风险服务的目录化拆分起
- 文件长度与模块边界治理的落地项

## 2. 架构对齐

step 重点对齐

- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
- `docs/架构/133-代码结构治理与crate拆分标准-2026-04-06.md`
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`

## 3. 当前现状与问题

当前 workspace 已有能力较多，但骨架仍偏向第一阶段快速实现：

- `crates/` 内只有少量通用 crate
- 许多业务逻辑仍压在`services/*` 
- 运行时、接口层、契约层的边界不够硬
- 高风险 `lib.rs`： 已明显超过治理标准

如果不先做骨架重构，后续所有协议重构和主链路拆分都会继续堆回现有服务文件中。

## 4. 设计

### 4.1 目标拓扑

建议采用“双轨迁移”策略：

- 新增 `sdkwork-im-*` crate 家族承载目标分层
- 现有 `im-*` crate 在过渡期保留为兼容 facade 或迁移中间层

推荐骨架类别：

- 基础横切层：`sdkwork-im-kernel`、`sdkwork-im-config`、`sdkwork-im-observability`、`sdkwork-im-auth`、`sdkwork-im-policy`
- 契约与协议层：`sdkwork-im-contract-*`、`sdkwork-im-ccp-*`
- 领域层：`sdkwork-im-domain-*`
- 应用层：`sdkwork-im-app-*`
- 接口层：`sdkwork-im-interface-*`
- 运行时层：`sdkwork-im-runtime-*`
- 存储与适配层：`sdkwork-im-storage-*`、`sdkwork-im-adapter-*`
- 服务装配层：保留 `services/*`

### 4.2 迁移原则

- crate 先建空骨架并保证可编译
- 旧逻辑按能力域渐进迁移，不做一次性搬迁
- `lib.rs` 只保留模块声明、导出和极少量facade
- 所有新逻辑必须优先落到新目录，而不是继续写入旧大文件

### 4.3 高风险优先级

优先处理：

- `services/sdkwork-im-cloud-gateway/src/lib.rs`
- `services/conversation-runtime/src/lib.rs`
- `services/session-gateway/src/lib.rs`

拆分建议：

- `router.rs`
- `state.rs`
- `error.rs`
- `runtime/`
- `service/`
- `projection/`
- `recovery/`
- `websocket/`
- `http/`

### 4.4 依赖方向

必须逐步建立以下硬约束：

- `domain-*` 不依赖`axum`
- `domain-*` 不依赖具体数据库驱动
- `interface-*` 不直接依赖具体存储实
- `runtime-*` 负责运行态，不承载稳定契约定
- `services/*` 只装配，不定义领域逻辑

## 5. 实施落地规划

### 5.1 任务拆解

1. 调整根级 `Cargo.toml`，引入目crate 目录规划
2. 新建空的 `sdkwork-im-*` crate 骨架和模块模
3. 为高风险服务建立目录化模块结
4. 先把`lib.rs` 中最易拆的内容下沉到子模
5. 建立文件长度检查和模块边界检查的脚本或约
6. 更新 README、架构映射和审计文档

### 5.2 代码触达范围

重点涉及

- `Cargo.toml`
- `crates/`
- `services/sdkwork-im-cloud-gateway/`
- `services/conversation-runtime/`
- `services/session-gateway/`
- `services/streaming-service/`
- `services/im-call-runtime/`
- `tools/chat-cli/`

### 5.3 过渡期策

为避免大面积中断，允许阶段性存在：

- `im-*` crate 提供过渡导出
- `services/*` 同时装配旧实现和新实
- 新旧模块通过 feature、facade adapter 渐进切换

但必须明确：过渡层只用于迁移，不能成为永久结构

### 5.4 工程治理落地

本 step 必须同步落地以下约束

- 单文件绝对上`1000` 
- 推荐控制帧`200-500` 
- `lib.rs` 不再承载复杂业务逻辑
- 新增 crate 使用一致的目录模板

## 6. 测试计划

建议测试与验证动作：

- `cargo metadata --format-version 1`
- `cargo check --workspace`
- `cargo test --workspace --no-run`
- 关键服务smoke 测试继续可执行
- 文件长度检查脚本验证高风险文件开始收口

建议补充自动化检查：

- `file-length-check`
- `module-boundary-check`
- workspace 成员完整性检查

## 7. 结果验证

本 step 完成后，需要达成以下结果：

- workspace 结构已经能承载目标架构，不再只有“文档有分层、代码没落点
- 至少三类高风险服务开始目录化拆分
- 新增 crate 可以被后续 step 直接复用
- 后续协议、运行时、主链路改造不再被迫堆回旧 `lib.rs`

## 8. 检查点

- `CP02-1`：目workspace 拓扑crate 清单已经落地到仓库结
- `CP02-2`：高风险大文件开始拆分，`lib.rs` 只保留瘦 facade
- `CP02-3`：workspace 仍可成功 `cargo check`
- `CP02-4`：文件长度与模块边界治理开始纳入检查流

### 8.1 推荐 review 产物

- `docs/review/step-02-执行卡YYYY-MM-DD.md`
- `docs/review/step-02-workspace骨架重构设计-YYYY-MM-DD.md`
- `docs/review/step-02-crate边界迁移清单-YYYY-MM-DD.md`
- `docs/review/step-02-librs拆分跟踪-YYYY-MM-DD.md`

### 8.2 推荐并行车道

- `02-A`：workspace、crate 拓扑Cargo 依赖面重
- `02-B`：`services/*` 装配层收敛与目录边界整理
- `02-C`：超大`lib.rs` 拆分、编译入口与测试入口保活
- 收口要求：共享目录树crate 命名规则`02-Owner` 统一拍板，任何车道不得绕过既定边界继续向`lib.rs` 堆积逻辑。
- 车道编排参考：[`94-Step并行执行编排与车道拆分建议`](./94-Step并行执行编排与车道拆分建立md)

### 8.3 架构能力闭环判定

- 新骨架必须能承接 Step 03 之后的协议、运行时、主链路改造，而不是只创建空目录
- 如果高风险大文件仍继续增长，或新代码仍回流到 `lib.rs`，则 step 未闭环
- 闭环验收口[`95-架构能力闭环验收标准`](./95-架构能力闭环验收标准.md) Step 02 条目为准

### 8.4 快速并行执行建立

- 先冻结目workspace 目录树、crate 命名和主写入边界，再放开拆分车道
- 推荐并行执行“Cargo/依赖拓扑”“services 装配层收敛”“大 `lib.rs` 拆分”三条线，每天至少一次编译收口径
- 不允许一边定义新骨架，一边继续往旧热文件堆逻辑；一旦出现，优先修正边界再继续

### 8.5 完成后必须回写的架构文档

- 强制范围：本文件 `## 2. 架构对齐` 中列出的全部架构文档
- 回写重点：crate 家族、workspace 目录树、`services/*` 装配层定位、兼容 facade 保留策略是否已从设计态转为实际工程态
- 必备证据：`docs/review/step-02-架构兑现-YYYY-MM-DD.md` `docs/review/step-02-架构回写决议-YYYY-MM-DD.md`

## 9. 风险与回写

### 9.1 风险

- 过早大规模迁移逻辑，可能导致编译面过大、问题定位困
- 只建 crate 不迁移边界，容易形成“新目录 + 旧坏味道并存
- 只拆文件不改依赖方向，会留下伪分

### 9.2 回滚

- 新增 crate 可以先保留空壳，不影响旧实现运行
- 旧实现保facade，必要时可临时回写
- 每次拆分应按模块逐段提交，避免大爆炸式不可回滚变

## 10. 完成定义

以下条件全部满足时，step 完成

- 根级 workspace 已引入目crate 分层骨架
- 关键服务的大文件已开始收口
- 新旧结构的迁移路径清单
- 基本编译smoke 验证未被破坏

## 11. 下一步准入条件

进入 Step 03 前必须确认：

- 目标协议层和契约层已有明确crate 落点
- `services/*` 不再继续承担协议定义职责
