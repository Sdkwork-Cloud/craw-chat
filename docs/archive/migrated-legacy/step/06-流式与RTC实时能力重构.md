# Step 06 - 流式RTC 实时能力重构

## 1. 目标与范围

本 本 step 用于把 `stream` 与 `rtc` 从“附属功能”升级为 `sdkwork-im` 的一等实时能力，并纳入统一协议、统一运行时和统一投影体系

本 step 必须覆盖：

- `stream.open`
- `stream.delta`
- `stream.patch`
- `stream.checkpoint`
- `stream.finalize`
- `stream.abort`
- `rtc.invite`
- `rtc.accept`
- `rtc.reject`
- `rtc.end`

### 1.1 执行输入

- Step 05 已稳定的消息主链路与 sender / tenant 边界
- 当前 `streaming-service`、`im-call-runtime`、`im-domain-core` 中的流与 RTC 实现
- `143` 统一协议总纲与当`CCP` 控制帧、envelope 能力
- `150` 插件化提供商体系与设备接入标准
- 当前 stream / rtc 生命周期相关测试资产

### 1.2 本步非目标

- 不在本 step 内完成 AI / Agent 治理与 device 接入治理
- 不在本 step 内引入媒体面处理或复杂音视频基础设施
- 不在本 step 内完成 SDK CLI 的兼容收口

### 1.3 最小输出

- stream rtc 的契约、领域与应用边界
- 生命周期、checkpoint、materialization 规则
- 与消息主链路的桥接规
- RTC provider plugin 契约、默provider 规则和录制产物落盘边界
- 可重复执行的 stream / rtc 自动化验证

## 2. 架构对齐

step 重点对齐

- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
- `docs/架构/134-AI-Agent-IoT统一实时通信模型设计-2026-04-06.md`
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
- `docs/架构/143-统一协议总纲与分层设计-2026-04-06.md`
- `docs/架构/150-插件化提供商体系与设备接入设计-2026-04-08.md`

## 3. 当前现状与问题

当前仓库已经有：

- `services/streaming-service`
- `services/im-call-runtime`
- `im-domain-core` 中的 `stream.rs`、`rtc.rs`

但距离目标形态仍存在差距：

- 流式能力还需要更明确的协议级地位
- 流与消息的物化、关联和恢复逻辑需要统一
- RTC 应只承载信令，不应与媒体逻辑混写
- AI 流、普通流、设备流需要共享同一套骨
- RTC provider 还需要统一抽象plugin 体系，冻`火山引擎 / 阿里/ 腾讯云` 的选择边界

## 4. 设计

### 4.1 流模

统一流模型应支持

- 普通渐进式内容输出
- LLM token streaming
- 长任务进度流
- 结构patch 
- 设备 telemetry 
- Agent 工具调用过程

统一核心字段

- `stream_id`
- `stream_type`
- `scope_kind`
- `scope_id`
- `sender`
- `frame_seq`
- `resume_token`
- `materialize_target`

### 4.2 RTC 模型

RTC 只负责信令：

- session 生命周期
- invite / accept / reject / close
- signal relay
- conversation 绑定
- recording / artifact 回流消息体系

同时必须冻结 provider/plugin 边界：

- signaling、权限、成员治理由平台统一掌控
- 房间、凭证、录制、回调由 RTC provider plugin 负责
- 当前支持 `火山引擎 / 阿里/ 腾讯云`
- 全局默认 RTC provider `火山引擎`

### 4.3 与消息主链路的关系

必须明确：

- 流不是消息字段，而是并列的一等模
- 流可以物化为消息、卡片、摘要或任务状
- RTC 信令conversation-bound 能力，受成员治理约束
- 流与 RTC 都要服从同一套auth / sender / scope 规则

### 4.4 RTC provider 插件边界

- 任何 RTC provider 差异都只能停留在 `RtcProviderPort` 后面
- RTC 录制、转写、截图、回放地址等产物必须统一回流 `BlobStore` / `ObjectStorageProvider`
- 不允许把 `Volcengine / Aliyun / Tencent` 私有对象或回调字段直接暴露到消息和流领域模型

## 5. 实施落地规划

### 5.1 任务拆解

1. 建立或完`contract-stream` `contract-rtc`
2. 补齐 `RtcProviderPort`、provider registry 和默provider 选择语义
3. 拆分 `domain-stream` `domain-rtc`
4. 拆分 `app-stream` `app-rtc`
5. `streaming-service`、`im-call-runtime` 接入新契约和runtime
6. 建立 stream materialization 规则
7. 建立 stream resume / checkpoint / finalize 行为校验

### 5.2 重点路径

重点涉及

- `crates/im-domain-core/src/stream.rs`
- `crates/im-domain-core/src/rtc.rs`
- `services/streaming-service/`
- `services/im-call-runtime/`
- `services/projection-service/`
- `services/notification-service/`

### 5.3 流物化策

至少应定义以下物化模式：

- 仅实时推送，不写 timeline
- 写入 timeline 的流式消
- 完成后聚合为最终消
- 中途产checkpoint 供断线恢

### 5.4 AI 和设备预

虽然 AI / IoT 的完整落地在 Step 08，但 step 必须先把接口预留好：

- stream type 支持 AI / device / task / rtc-artifact
- sender / actor 模型兼容 `agent` `device`
- materialize target 支持 timeline / card / task / twin

## 6. 测试计划

建议重点测试

- 流生命周期测
- checkpoint / resume 测试
- abort / finalize 测试
- 普通流物化timeline 测试
- RTC 信令全流程测
- RTC conversation membership 约束测试
- RTC provider selection / degrade behavior / artifact 回流边界测试

建议优先复用或扩展：

- `services/streaming-service/tests/stream_lifecycle_test.rs`
- `services/im-call-runtime/tests/rtc_signal_flow_test.rs`
- `services/im-call-runtime/tests/rtc_runtime_persistence_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/stream_runtime_persistence_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/rtc_runtime_persistence_test.rs`

## 7. 结果验证

本 step 完成后，需要验证：

- 流式输出已经是协议和主链路内建能
- RTC 信令已经与会话和成员治理正确绑定
- RTC provider 已被限制在统一 plugin 边界内，默认 provider 与覆盖规则清单
- 普通流、AI 流、设备流可以共用同一骨架
- 断线重连后，流的恢复语义可验证

## 8. 检查点

- `CP06-1`：统一流模型、RTC 模型RTC provider 契约已经落实到契约与领域
- `CP06-2`：stream 生命周期materialization 规则已可自动化验证
- `CP06-3`：RTC invite / accept / reject / end provider 选择测试跑
- `CP06-4`：流与消息、RTC 与消息的桥接规则已稳

### 8.1 推荐 review 产物

- `docs/review/step-06-执行卡YYYY-MM-DD.md`
- `docs/review/step-06-stream-rtc能力收口-YYYY-MM-DD.md`
- `docs/review/step-06-stream生命周期验证-YYYY-MM-DD.md`
- `docs/review/step-06-rtc消息桥接验证-YYYY-MM-DD.md`

### 8.2 推荐并行车道

- `06-A`：stream lifecycle、增量输出、materialization
- `06-B`：RTC signaling、权限校验、会话态协商、provider contract
- `06-C`：与消息主链路的桥接、兼容回放与 CLI 验证
- 收口要求：`06-Owner` 统一定义“流与消息”“RTC 与消息”的桥接规则，任何车道不得把流式能力退化成消息字段附属物
- 车道编排参考：[`94-Step并行执行编排与车道拆分建议`](./94-Step并行执行编排与车道拆分建立md)

### 8.3 架构能力闭环判定

- stream RTC 必须成为一等实时能力，生命周期、权限、桥接语义都可单独验证的
- 如果 stream 仍只是消息上的一个字段，RTC 仍混入媒体侧内部逻辑而没有标准 signaling 面，RTC provider 仍散落在业务代码中，step 不算闭环
- 闭环验收口[`95-架构能力闭环验收标准`](./95-架构能力闭环验收标准.md) Step 06 条目为准

### 8.4 快速并行执行建立

- 先冻stream lifecycle RTC signaling 状态，再并行推进流式、RTC、消息桥接三车道
- 推荐把“生命周期验证”和“桥接验证”作为独立回归项，每日与主链路一起收口径
- 在流式状态不稳定前，不要继续扩张 SDK 表面能力，先保住服务端语义稳定

### 8.5 完成后必须回写的架构文档

- 强制范围：本文件 `## 2. 架构对齐` 中列出的全部架构文档
- 回写重点：stream 一等能力、RTC signaling、消息桥接和统一协议分层是否已经从设计目标变成可重复验证的运行能力
- 必备证据：`docs/review/step-06-架构兑现-YYYY-MM-DD.md` `docs/review/step-06-架构回写决议-YYYY-MM-DD.md`

## 9. 风险与回写

### 9.1 风险

- 把流错误降级成普通消息字段，会破坏未AI / IoT 扩展
- RTC 信令与媒体逻辑耦合，会拉高系统复杂
- checkpoint / resume 规则设计不稳，会导致断线恢复错误

### 9.2 回滚

- 保留现有 streaming / rtc 服务接口，通过内部转换层先接入新模
- 物化策略先支持最小集合，再逐步扩展
- 关键流状态机变更必须有可重复的生命周期测试兜

## 10. 完成定义

满足以下条件时，本 step 完成

- stream rtc 已成为独立的一等能
- 生命周期、恢复和桥接规则已稳
- 与消息主链路、投影和通知系统已完成基本衔

## 11. 下一步准入条件

进入 Step 07 前必须确认：

- 协议、主链路和实时能力已经足够稳定，可以进入控制面治理和扩展能力治理阶段
## 12. 2026-04-08 As-Built 补充（一

- 已落地到真实代码的部分：
  - 新增 `adapters/rtc-volcengine`，提供默`rtc-volcengine` runtime adapter
  - `services/im-call-runtime/src/lib.rs`
    - `RtcRuntime` 已接入 `ProviderRegistry + RtcProviderPort`
    - 默认 `with_store(...)` 已装`rtc-volcengine`
    - `create_session(...)` 已真实调provider `create_session(...)`
    - `issue_participant_credential(...)` 已真实调provider `issue_participant_credential(...)`
    - `reject_session(...) / end_session(...)` 已真实调provider `close_session(...)`
  - `crates/im-domain-core/src/rtc.rs`
    - `RtcSession` 已增加通用 provider 元数据：
      - `provider_plugin_id`
      - `provider_session_id`
      - `access_endpoint`
      - `provider_region`
- 自动化验证：
  - `services/im-call-runtime/tests/rtc_runtime_persistence_test.rs`
    - 新增 provider-aware runtime 测试，覆`create_session / issue_participant_credential / provider_health_snapshot / close_session`
  - `services/im-call-runtime/tests/rtc_signal_flow_test.rs`
  - `services/sdkwork-im-cloud-gateway/tests/rtc_runtime_persistence_test.rs`
- 对本 step 当前状态的判断
  - `CP06-1` 已进入真实运行时闭环的第一阶段，不再只是契约与架构文档
  - 当前仍未完成的部分是对外 credential / callback / artifact surface，以`rtc-aliyun / rtc-tencent` adapter

## 12. 2026-04-08 As-Built 补充（二
- 已继续把 Step 06 / Wave B 从“默provider runtime adapter 已接入”推进到“外RTC provider surface 已闭环”：
  - `services/im-call-runtime/src/lib.rs`
    - 新增 `POST /im/v3/api/calls/sessions/{rtcSessionId}/credentials`
    - 新增 `GET /backend/v3/api/rtc/provider_health`
    - 直接复用已有 `RtcRuntime::issue_participant_credential(...)` `provider_health_snapshot(...)`
  - `services/sdkwork-im-cloud-gateway/src/node/rtc.rs`
    - 新增同名 handler，并沿用当前 conversation-bound access guard
  - `services/sdkwork-im-cloud-gateway/src/node/build.rs`
    - 暴露`im-call-runtime` 一致的两条 RTC provider surface
- 自动化验证已补齐
  - `services/im-call-runtime/tests/http_smoke_test.rs`
    - `test_issue_rtc_participant_credential_over_http`
    - `test_get_rtc_provider_health_over_http`
  - `services/sdkwork-im-cloud-gateway/tests/http_e2e_test.rs`
    - `test_local_minimal_profile_issues_rtc_participant_credential_over_http`
    - `test_local_minimal_profile_gets_rtc_provider_health_over_http`
- 对本 step 当前状态的判断
  - Step 06 / `06-B` 已从“provider runtime 内部闭环”推进到“session + credential + health 对外 surface 闭环”
  - 当前剩余缺口集中`callback / artifact` surface，以`rtc-aliyun / rtc-tencent` adapter
## 12. 2026-04-08 As-Built（Wave B callback / artifact

- `services/im-call-runtime/src/lib.rs`
  - 新增 `RtcRuntime::map_provider_callback(...)`
  - 新增 `RtcRuntime::recording_artifact(...)`
  - 新增 `POST /backend/v3/api/rtc/provider_callbacks`
  - 新增 `Retired recording artifact HTTP read; call artifacts are delivered as Drive-backed IM records`
- `services/sdkwork-im-cloud-gateway`
  - 镜像暴露同名 RTC provider surface
  - callback 保持 provider/integration 面，不引入厂DTO
  - artifact 继续session / conversation 边界约束
- 新增验证
  - `services/im-call-runtime/tests/http_smoke_test.rs`
    - `test_map_rtc_provider_callback_over_http`
    - `test_get_rtc_recording_artifact_over_http`
  - `services/sdkwork-im-cloud-gateway/tests/http_e2e_test.rs`
    - `test_local_minimal_profile_maps_rtc_provider_callback_over_http`
    - `test_local_minimal_profile_gets_rtc_recording_artifact_over_http`
- Step 06 / `06-B`
  - provider external surface 已从 `session + credential + health` 收口到 `callback + artifact`
  - 下一轮优`rtc-aliyun / rtc-tencent` adapter `ObjectStorageProvider` 录制产物归档接入
## 12. 2026-04-08 As-Built（RTC Aliyun / Tencent adapters

- 新增 crate
  - `adapters/rtc-aliyun`
  - `adapters/rtc-tencent`
- 两个 provider 均实现完`RtcProviderPort` contract
  - `session`
  - `credential`
  - `callback`
  - `health`
  - `recording artifact`
- `services/im-call-runtime/src/lib.rs`
  - `RtcRuntime::with_store(...)` 默认内建 provider map 已扩容到
    - `rtc-volcengine`
    - `rtc-aliyun`
    - `rtc-tencent`
- 新增验证
  - `adapters/rtc-aliyun/tests/adapter_contract_test.rs`
  - `adapters/rtc-tencent/tests/adapter_contract_test.rs`
  - `services/im-call-runtime/tests/rtc_runtime_persistence_test.rs`
    - `test_runtime_can_route_to_tenant_selected_builtin_rtc_providers`
- Step 06 / `06-B`
  - RTC provider external surface provider matrix 已形成最小闭环
  - 下一轮优`ObjectStorageProvider` 统一录制产物归档，以local profile provider registry 注入
