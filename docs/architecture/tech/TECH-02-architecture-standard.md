> Migrated from `docs/架构/02-架构标准与总体设计.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 架构标准与总体设计

## 1. 文档定位

本文档是 `sdkwork-im` 的基础总体设计文档，用于把 `130-149` 中冻结的推荐架构落到“统一标准语言”上，作为模块划分、协议设计、存储设计和实现计划的公共前提。

本文档与以下总纲保持一致：

- `130-连接优先的AI时代即时通讯架构蓝图-2026-04-06`
- `131-连接管理与分层弹性扩容架构设2026-04-06`
- `132-存储架构与自主演进路线设2026-04-06`
- `133-代码结构治理与crate拆分标准-2026-04-06`
- `134-AI-Agent-IoT统一实时通信模型设计-2026-04-06`
- `135-行业对标与终局能力矩阵-2026-04-06`
- `136-关键业务链路与跨Plane时序设计-2026-04-06`
- `137-部署拓扑与容量规划设2026-04-06`
- `138-高可用与灾备恢复设计-2026-04-06`
- `139-权限能力模型与协议演进设2026-04-06`
- `140-可观测性与SLO治理设计-2026-04-06`
- `141-数据生命周期与归档成本治理设2026-04-06`
- `142-控制面与配置治理设计-2026-04-06`
- `143-统一协议总纲与分层设2026-04-06`
- `144-CCP传输绑定与握手协商设2026-04-06`
- `145-CCP数据协议与版本兼容安全设2026-04-06`
- `146-CCP协议注册表与多端SDK兼容治理设计-2026-04-06`
- `147-CCP到Crate与接口模块落地映射设2026-04-06`
- `148-CCP控制面注册表与协议发布治理设2026-04-06`
- `149-多Cell多Region协议升级与灾备兼容设2026-04-06`

## 2. 系统定位

`sdkwork-im` 不是单纯的聊天后端，而是一套面AI 时代和智能硬件时代的实时通信基础设施。其目标是同时承载：

- 消费级即时消息体。
- 企业协作与知识沉淀
- AI / Agent 流式交互
- AIoT 设备接入与控。
- 高连接密度和独立扩容
- 自主可控的商业化路线

系统终局形态定义为。

`连接优先、消息稳定、流式原生、协作可挂载、AI 可对话、设备可接入、组件可替换的下一代即时通讯平台`

## 3. 总体架构模型

### 3.1 六个 plane

系统采用六个 plane 作为主架构骨架：

- `Link Plane`
- `Route Plane`
- `Messaging Plane`
- `Stream / AI Plane`
- `Projection Plane`
- `Storage Plane`

### 3.2 两个横切 plane

系统采用两个横切治理层：

- `Control Plane`
- `Ops Plane`

### 3.3 四层代码分层

为了让设计能稳定落到代码结构中，工程内部采用四层职责模型。

- `Contracts`：协议、DTO、事envelope、稳定边。
- `Domain`：领域模型、规则、状态机、不变量
- `Application / Runtime`：用例编排、连接运行时、后台任务、恢复流。
- `Adapters / Services`：HTTP/WS/SSE/MQTT 接口、存储适配、启动与部署入口

## 4. 总体拓扑

```text
Client / Bot / Agent / Device
  -> Link Plane
  -> Route Plane
  -> Messaging Plane / Stream-AI Plane / RTC Signaling
  -> Storage Plane
  -> Projection Plane

Control Plane
  -> Tenant / Identity / Policy / Quota / Capability / Routing Strategy

Ops Plane
  -> Observability / Diagnostics / Backup / Restore / Drain / Upgrade
```

这意味着系统不是“一个网+ 一个消息服务”的简单形态，而是按压力面拆开的可扩展体系。

## 5. 核心设计标准

### 5.1 连接优先标准

- 连接层必须是独立 plane，而不是普API 附属模块。
- 链路握手、鉴权、心跳、背压、重连、恢复、出站队列必须统一抽象。
- 连接层必须支持与业务层解耦的弹性扩容。

### 5.2 消息标准

- 同一 `conversation_id` 内严格有序。
- 不追求跨会话全局顺序。
- 会话写路径采`single writer per scope` 原则。
- 命令和事件分离，提交成功先于任何投影和副作用。

### 5.3 流标。

- 流是原生能力，不AI 专用能力。
- 流支`open / delta / patch / checkpoint / finalize / abort` 生命周期。
- 流既可短暂存在，也可持久化并最终物化为消息、卡片、文件或知识条目。
- 流可承载 LLM token、任务进度、音频转写、设telemetry、结构化 patch。

### 5.4 主体标准

系统内的实时主体统一为：

- `user`
- `agent`
- `device`
- `bot`
- `system`

所有主体的发言、事件和行为统一通过 `actor / sender` 模型表达，不再依赖单一 `senderId` 字段。

### 5.5 协作标准

系统必须支持以下协作锚点。

- `topic`
- `thread`
- `card`
- `document reference`
- `task reference`
- `workflow reference`
- `AI summary`
- `knowledge context anchor`

### 5.6 文件资源标准

- 文件生命周期统一`sdkwork-drive` 负责。
- 消息通过 `ContentPart.drive` (`DriveReference`) 引用文件，并携带 `MediaResource` 使用快照。
- IM 不拥有上传会话、版本、存储事实、权限校验或访问 URL 签发。

### 5.7 RTC 标准

- RTC 只进入信令层。
- 媒体面独立部署，不塞IM 内核。
- 录制、转写、摘要和附件作为消息体系的产物回流。

### 5.8 多租户标。

- 所有持久化对象、事件、审计记录都必须显式携带 `tenant_id`。
- 租户权威信息只能来自认证上下文和服务端治理，不来自业务请求体。
- 隔离必须覆盖身份、配额、调度、数据、缓存、故障与运维域。

### 5.9 可插拔标。

- 核心语义固定，基础设施实现可替换。
- 所有外部依赖通过端口接入，不允许具体驱动穿透到领域层。
- 自有知识产权应集中在协议、事件、消息、流、路由和存储抽象，而非一开始重造所有基础设施。

## 6. Durable Truth Query Truth

### 6.1 Durable Truth

以下状态变化属于权威真相，必须先写入持久层。

- 会话创建与成员治。
- 消息发送、编辑、撤回
- 已读推进
- 流开启、checkpoint、完成、中。
- RTC 关键生命周期
- 媒体资源关键绑定事件
- Agent 关键事件IoT 控制关键事件

### 6.2 Query Truth

以下视图由投影构建，可重建，不作为最终真相源。

- `timeline`
- `inbox`
- `conversation summary`
- `read cursor view`
- `client-route event window`
- `stream summary`
- `notification view`
- `audit export view`

## 7. Cell 化与扩展原则

- 不做无限大的单一全局集群。
- 每个 `Cell` 是故障域、扩展单元、部署单元与运维单元。
- `SaaS` 形态采用共cell 与独cell 并存。
- 私有化默认单 cell 起步，但协议和模块边界与 SaaS 保持一致。

## 8. 所有权与一致性原。

- 路由与会话归属采`epoch + fencing` 防止陈旧连接写入。
- 节点下线采用 `graceful drain`，不允许粗暴摘除。
- 会话内部严格单主写，多节点之间通过显式归属和迁移完成切换。

## 9. 设计红线

- 不允许缓存成为消息真相源。
- 不允许副作用阻塞消息提交主路径。
- 不允许控制面强事务进入热消息路径。
- 不允许把 RTC 媒体面塞IM 内核。
- 不允许把 AI 场景硬编码为普通消息补丁。
- 不允许把设备接入做成旁路外挂系统。
- 不允许通过巨型 `lib.rs` 或少数大文件承载系统复杂度。

## 10. 结论

`sdkwork-im` 的总体设计已经明确收敛为“六plane + 两个横切治理+ 一套统一主体与流模型”的形态。后续所有模块、接口、存储和实现工作都必须以这一形态为前提推进。

## 2026-04-09 增补：现状、目标态与文档口径

### A. 当前实现真相

- 当前 workspace 真实成员`Cargo.toml` 为准，主要由 `im-*`、`sdkwork-im-contract-*`、`sdkwork-im-ccp-*`、`services/*`、`adapters/*` 组成。
- 当前 durable truth 仍以 `conversation / message / stream / rtc / route / projection / provider policy` 为主。
- 好友、空间、群组、频道、线程、外部协作尚未在仓库中形成完整独立的 durable truth 主模型。

### B. 目标态真。

- 社交真相：`friend_request / friendship / user_block / direct_chat`
- 空间真相：`space / space_member / space_role`
- 组织真相：`chat_group / group_member / group_role / chat_channel / channel_access_rule`
- 会话真相：`conversation / conversation_member / message / read_cursor`
- 治理真相：`invitation / membership_request / ban_record / mute_setting / audit_event / outbox_event`
- 扩展真相：`thread / thread_subscription / external_connection / external_member_link / message_reaction / pin_record / history_visibility_policy / retention_policy`

### C. 硬性架构约。

- `user` 表示自然人；`actor` 表示任意参与者；`member` 只表示参与者在容器内的关系。
- `space` 是治理容器，不是超大自动加入会话。
- `chat_channel` 使用继承权限加覆盖规则，不以 `conversation_member` 直接表达组织权限。
- `thread` 是一等模型，不是 reply 字段补丁。
- `conversation` 只承载消息运行时、读写顺序与 fanout，不承担好友、群组、空间真相。
- 外部共享必须显式建模，不允许把外部协作伪装成普通成员。

### D. 文档使用规则

- 写“当前实现”时，以 `152CJ-current-architecture-as-built-alignment-2026-04-09.md` 与当workspace 为准。
- 写“目标架构”时，以 `150CJ-im-social-space-conversation-ddd-design-2026-04-09.md` `151CJ-im-benchmark-model-alignment-2026-04-09.md` 为准。
- 若当前实现与目标态不一致，必须显式分成 `Current State / Target State / Deferred`，禁止混写。

