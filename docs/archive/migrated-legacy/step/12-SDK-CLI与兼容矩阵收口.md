# Step 12 - SDK / CLI 与兼容矩阵收口

## 1. 目标与范围

本 step 用于把协议、契约、服务端实现、命令行工具和 SDK 统一到同一套兼容治理口径下，形成可对外稳定交付的接入面。

本 step 主要覆盖：

- `tools/chat-cli`
- `bin/chat-cli*`
- `bin/chat-window*`
- `bin/open-chat-test*`
- `sdks/sdkwork-im-sdk`
- `sdks/sdkwork-control-plane-sdk`
- compatibility matrix

### 1.1 执行输入

- step 03 的 `CCP` 与 `contract-*` 基线
- step 10 的标准部署入口和 step 11 的稳定验证环境
- 当前 `tools/chat-cli`、`bin/chat-cli*`、`sdks/` 现状
- 当前控制面关于 registry 与 compatibility 的治理结果

### 1.2 本步非目标

- 不在本 step 内新增新一轮服务端业务能力
- 不在本 step 内重构桌面或移动端完整应用产品
- 不在本 step 内覆盖所有潜在第三方 SDK 生态

### 1.3 最小输出

- 对齐当前协议与权威字段模型的 CLI
- 两套 SDK 目录的清晰契约边界与 README
- 可核对的 compatibility matrix
- 多终端聊天和流式验证脚本

## 2. 架构对齐

本 step 重点对齐：

- `docs/架构/146-CCP协议注册表与多端SDK兼容治理设计-2026-04-06.md`
- `docs/架构/148-CCP控制面注册表与协议发布治理设计-2026-04-06.md`
- `docs/架构/149-多Cell多Region协议升级与灾备兼容设计-2026-04-06.md`

## 3. 当前现状与问题

当前仓库已存在：

- `tools/chat-cli`
- 多平台 `bin/chat-cli*` 与 `bin/open-chat-test*`
- SDK 目录骨架

但还需要进一步收口：

- CLI 是否完全遵从新的 CCP 契约和权威字段规则
- SDK 与服务端契约是否已形成明确版本边界
- 多端兼容矩阵还未真正沉淀为可验证资产

## 4. 设计

### 4.1 接入层原则

SDK / CLI 必须遵守：

- 所有请求都服从 CCP 与 contract-* 契约
- 不允许在客户端自行拼接权威字段
- 协议能力依赖 capability negotiation 决定
- 版本兼容通过 registry 与 matrix 治理，而不是客户端硬猜

### 4.2 SDK 分层

建议 SDK 至少分层为：

- transport binding client
- CCP envelope / codec
- contract DTO
- facade API
- 场景示例与测试工具

### 4.3 CLI 职责

CLI 不是临时脚本，而是标准验证工具，至少用于：

- 登录与会话绑定
- 创建或选择会话
- 发消息
- 收消息
- 观察流式输出
- 多终端对话验证

### 4.4 兼容矩阵

至少需要定义：

- 服务端版本 -> 协议版本 -> schema 版本
- binding 支持情况
- feature flag / capability 依赖关系
- 废弃周期和回滚策略

## 5. 实施落地规划

### 5.1 任务拆解

1. 让 `tools/chat-cli` 基于最新 contract / CCP 能力改造
2. 让 `bin/chat-cli*`、`bin/chat-window*`、`bin/open-chat-test*` 与最新 CLI 对齐
3. 梳理两个 SDK 目录的契约、封装和示例
4. 形成 compatibility matrix 文档与测试资产
5. 为协议升级和回滚建立 CLI / SDK 兼容验证场景

### 5.2 重点路径

重点涉及：

- `tools/chat-cli/`
- `bin/chat-cli*`
- `bin/chat-window*`
- `bin/open-chat-test*`
- `sdks/sdkwork-im-sdk/`
- `sdks/sdkwork-control-plane-sdk/`
- `services/control-plane-api/`

### 5.3 收口顺序

推荐顺序：

1. 先 CLI 与服务端主路径对齐
2. 再 SDK facade 与契约对齐
3. 再形成兼容矩阵和回归场景

### 5.4 文档同步

本 step 完成时，必须同步更新：

- `README.md`
- SDK README
- CLI 使用说明
- 兼容矩阵说明

## 6. 测试计划

建议重点测试：

- CLI 单聊 / 群聊发送与接收测试
- 多窗口聊天验证测试
- 流式输出 CLI 展示测试
- 旧协议版本与新服务端兼容测试
- capability 未开启时的降级与报错测试

优先复用或扩展：

- `tools/chat-cli/tests/chat_cli_e2e_test.rs`
- `bin/open-chat-test.*`
- 本地最小节点的 E2E 用例

## 7. 结果验证

本 step 完成后，需要验证：

- CLI 能真实验证当前服务端的聊天与流式主路径
- SDK 已有明确的契约边界和版本边界
- 兼容矩阵不再只是文档，而是具备脚本或测试支撑
- 多端验证可以支撑发布前验收

## 8. 检查点

- `CP12-1`：CLI 与最新协议和契约模型对齐
- `CP12-2`：SDK 目录具备清晰 facade 和 README
- `CP12-3`：兼容矩阵有文档、有测试、有控制面映射
- `CP12-4`：多终端聊天与流式验证脚本可重复执行

### 8.1 推荐 review 产物

- `docs/review/step-12-执行卡-YYYY-MM-DD.md`
- `docs/review/step-12-cli-sdk兼容收口-YYYY-MM-DD.md`
- `docs/review/step-12-e2e对话验证-YYYY-MM-DD.md`
- `docs/review/step-12-compat-matrix验证-YYYY-MM-DD.md`

### 8.2 推荐并行车道

- `12-A`：CLI 交互链路、对话 smoke、脚本封装
- `12-B`：SDK 契约边界、示例、版本兼容策略
- `12-C`：兼容矩阵、控制面映射、跨平台 E2E 验证
- 收口要求：CLI 与 SDK 都必须走统一主链路，兼容矩阵必须同时有文档、测试和控制面映射三类证据。
- 车道编排参考：[`94-Step并行执行编排与车道拆分建议`](./94-Step并行执行编排与车道拆分建议.md)

### 8.3 架构能力闭环判定

- CLI 必须能够真实验证主链路，SDK 边界必须清晰且与控制面兼容治理对齐。
- 如果只有 README、示例列表，没有真实 E2E 验证或兼容控制链路，本 step 不算闭环。
- 闭环验收以 [`95-架构能力闭环验收标准`](./95-架构能力闭环验收标准.md) 中 Step 12 条目为准。

### 8.4 快速并行执行建议

- 先冻结 CLI 场景清单、SDK 契约边界和兼容矩阵维度，再并行推进 CLI、SDK、E2E 验证三车道。
- 推荐 CLI 与 SDK 都复用同一条主链路验证资产，避免各自维护不同的 smoke 语义。
- 本步收尾必须至少保留一套可复用的终端验证路径，不能只靠文档示例证明完成。

### 8.5 完成后必须回写的架构文档

- 强制范围：本文件 `## 2. 架构对齐` 中列出的全部架构文档。
- 回写重点：SDK 契约边界、CLI 验证路径、协议注册表兼容规则和多 region 兼容矩阵是否已经被真实工具链消费。
- 必备证据：`docs/review/step-12-架构兑现-YYYY-MM-DD.md` 与 `docs/review/step-12-架构回写决议-YYYY-MM-DD.md`。

## 9. 风险与回滚

### 9.1 风险

- 如果 CLI 继续走旧模型，会让现场验证结果失真
- 如果 SDK 没有版本边界，后续协议治理无法落地
- 如果兼容矩阵只写文档，不做测试，升级风险仍然不可控

### 9.2 回滚

- 保留旧 CLI 入口作为兼容层，但默认走新实现
- SDK 先增加版本化 facade，再逐步清理旧接口
- 兼容矩阵先覆盖关键主路径，再扩展到边缘能力

## 10. 完成定义

满足以下条件时，本 step 完成：

- CLI、SDK、协议、控制面治理已经基本收口
- 多终端对话、流式验证和兼容验证具备稳定工具链
- 发布前验收不再依赖手工拼装命令

## 11. 下一步准入条件

进入 step 13 前必须确认：

- 服务端、脚本、CLI、SDK、兼容矩阵已经具备发布前收口条件
