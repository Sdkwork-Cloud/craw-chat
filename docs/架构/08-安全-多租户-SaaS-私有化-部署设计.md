# 安全、多租户、SaaS 与私有化部署设计

## 1. 文档目标

本文档用于冻结`sdkwork-im` 在安全、多租户隔离、SaaS 形态和私有化部署上的统一设计，确保：

- 认证上下文成为权威身份来源
- 安全模型同时兼顾隐私聊天与协作场
- 多租户能够从连接层一直隔离到存储和运维层
- SaaS 与私有化共享同一套协议和架构骨架

## 2. 安全总原则

- 外部入口默认 `TLS 1.3`
- 内部服务通信推荐 `mTLS`
- 默认最小权限
- 默认 tenant-aware 鉴权与审计
- 高风险写操作必须具备幂等和回放保护
- `CCP` 权威字段边界必须前置到接入层
- 安全和多租户校验要尽量靠前进入接入层，而不是拖到业务末端

## 3. 身份与认证模型

### 3.1 外部身份

系统支持以下外部身份：

- 用户
- 设备
- Bot / 应用
- Agent
- 服务账号

### 3.2 内部身份

系统内部至少区分：

- service identity
- runtime identity
- node identity

### 3.3 权威上下文原则

- `tenantId` 必须来自已校验的 JWT、session 或 trusted identity context
- `actorId / actorKind / sessionId / clientRouteId` 必须来自认证上下文或可信绑定关系
- `sender / routeEpoch / serverTs` 必须来自服务端推导与路由状态
- 业务请求体不得覆盖这些权威字段
- gateway 负责把权威上下文转换为内部命令上下文与审计字段

### 3.4 协议安全边界

- 长连接握手必须遵循 `hello -> auth_bind -> session_resume` 的统一协议顺序
- 未完成认证绑定前，不允许进入普通业务帧阶段
- 客户端不得伪造`tenantId`、`actorId`、`sender`、`routeEpoch`
- 恢复链路必须校验 `resume token`、`sessionId`、`clientRouteId` `routeEpoch`
- 设备侧在高信任场景可追加 `device signature`，但不替代`TLS`

## 4. 双轨安全模型

### 4.1 隐私有

适用于：

- 私聊
- 敏感群
- 私域设备控制

支持有

- 可选端到端加密
- 设备绑定
- 密钥轮换
- 更严格的内容可见性边界

### 4.2 协作域

适用于：

- 企业群
- AI 工作台
- 知识协作
- Bot / Workflow / 审计场景

支持有

- 服务端检查
- 合规审计
- 管理治理
- AI 上下文可见和可挂载

## 5. 多租户隔离模型

### 5.1 身份隔离

- 每个请求、连接、事件和审计记录都必须显式绑定`tenant_id`
- 不允许从客户端业务 payload 直接信任租户字段

### 5.2 配额隔离

至少应支持：

- tenant connection quota
- tenant send TPS
- tenant stream throughput
- tenant media upload quota
- tenant automation quota
- tenant AI token / stream frame quota

### 5.3 调度隔离

- fair queue
- shuffle sharding
- noisy neighbor control
- hot tenant isolation lane
- link lane / route lane / storage lane 分层治理

### 5.4 数据隔离

- shared logical isolation
- dedicated cell
- dedicated storage lane
- tenant-scoped backup and recovery

### 5.5 故障隔离

- projection failure isolation
- automation worker isolation
- Agent / IoT sidecar isolation
- tenant lane throttling

## 6. 目标 SaaS 部署设计

### 6.1 Shared Cell

适用于大多数租户

- 多租户共享基础设施
- 强配额和隔离治理
- 高资源利用率
- 支持热点租户 lane 隔离

### 6.2 Dedicated Cell

适用于大租户或高合规租户

- 专属 cell
- 专属 route lane / link lane / storage lane
- 更高 SLA 与更强审计策

### 6.3 Region / Cell 原则

- 不做无限大的全局单集
- 一cell 是故障域、部署域、扩展域和运维域
- 仅region 写入必须受控，不允许同一会话region 多主

## 7. 目标私有化部署设计

### 7.1 Standard

- 使用 `PostgreSQL + Redis + Object Storage`
- 可接入客IAM / KMS / 对象存储
- 支持本地升级、回滚和备份

### 7.2 Restricted

- 受限联网
- 可导入离线依赖
- 可禁用外部Webhook / Push / 第三方集

### 7.3 Air-Gapped

- 离线镜像
- 离线许可
- 离线升级包
- 离线诊断工具
- 离线配置初始化脚本

## 8. 安装与启动原则

- profile 驱动装配
- 配置模板
- 支持最小闭环安
- 支持滚动升级
- 支持安全下线、扩容和节点排空
- `bin/` 下脚本需支持安装、初始化配置、启动、停止、重启和状态检查

## 9. 备份与恢复

必须支持有

- metadata snapshot
- message log replay
- object storage backup
- projection rebuild
- tenant-scoped recovery
- route hot state rebuild

恢复顺序建议：

1. 恢复 metadata
2. 恢复 message log / stream checkpoint
3. 重建 projection
4. 恢复 route presence 热状态
5. 核对对象存储引用一致性

## 10. 运维与诊断

至少应支持：

- health check
- lag diagnostics
- route ownership view
- rebalance progress
- drain status
- diagnostic bundle export
- per-tenant quota / throttle 视图
- connection / resume / reconnect 指标

当前最小实现已暴露基础运维入口，后续需要按 plane 进一步细化

## 11. 结论

`sdkwork-im` 的安全和部署设计必须以“认证上下文权威性 + 双轨安全模型 + 多租户全链路隔离 + cell 化部署”为前提。只有这样，系统才能同时满足消费级消息体验、企业协作能力、AI 可治理和私有化落地的要求。

## 2026-04-09 增补：当前安全落地与部署口径

### A. 当前安全真相

- 当前仓库真实已收口的安全核心是：认证上下文权威化、`tenant_id` 服务端推导、`sender/actor` 服务端生成、连接恢复链路校验、provider 治理面鉴权限
- 当前最接近已落地运行形态的`sdkwork-im-server` 与现有服务拆分组合，不是完整 SaaS Shared/Dedicated 或完整私有化产品包
- 因此，6 节描述的是目标部署标准，不应被解读为当前所有部署形态已完整产品化

### B. 当前部署真相

- 当前真实工程形态已经支持本地最小闭环、provider 注入、控制面与运维面基础 surface
- Shared Cell、Dedicated Cell、Restricted、Air-Gapped 仍主要是标准设计与演进目标：
- 文档必须区分“当前可运行 profile”与“目标交付 profile”

### C. 文档口径规则

- 写当前应用信息时，优先描述已存在服务、认证边界、ops/control-plane 已验证能力
- 仅SaaS 或私有化章节时，必须显式标注为目标部署标准
- 不允许把未来合规、隔离、灾备能力写成当前已完成事实现
