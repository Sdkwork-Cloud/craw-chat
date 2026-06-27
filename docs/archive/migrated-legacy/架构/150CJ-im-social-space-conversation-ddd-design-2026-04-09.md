# 150CJ - IM 社交关系 / 空间治理 / 会话分层 DDD 设计

## 1. 目标

`sdkwork-im` 增加面向长期演进的好友关系、群组、频道、空间治理数据模型，并与现有 `conversation-runtime / session-gateway / control-plane-api / projection-service` 形成清晰边界。

本设计要同时满足四个要求。

1. 支持 C 端社IM、企业协作、社区运营三类场景，而不是只为单一聊天场景建模。
2. `conversation/member` 退回“消息容器与运行态投影”角色，不再承担所有社交真值。
3. 命名必须业务友好，符DDD 统一语言，避`account` 这类高歧义词。
4. 数据库既能支撑大群、高并发消息，也能支撑持续迭代和跨服务扩展。

## 2. 最终结。

最终推荐采`关系+ 空间+ 会话域` 三层主模型，并把治理能力作为横切域独立出来：

- `关系域` 负责 `user / actor / friendship / direct_chat / block` 等社会关系真值。
- `空间域` 负责 `space / chat_group / chat_channel / member / role / acl` 等组织与群治理真值。
- `会话域` 负责 `conversation / message / read_cursor / runtime fanout`，只做消息承载与运行时投影。
- `治理域` 负责 `invitation / membership_request / ban_record / mute_setting / report_ticket / audit_event / outbox_event`。

这意味着。

- `friendship` 不是 `conversation` 的副产物，而是独立聚合。
- `chat_group` 的成员真值不再依`conversation_member`。
- `chat_channel` 默认走“继承访+ 覆盖规则”，而不是为每个频道复制整张成员表。
- 联系人列表、群名册、最近互动、未读角标优先做投影，不把它们当核心真值表。

## 3. 统一语言

| 术语 | 含义 | 设计说明 |
| --- | --- | --- |
| `user` | 真实自然人用| 只在“好友、联系人、实名主体”语义下使用 |
| `actor` | 任意可参IM 的主| 可取 `user / bot / agent / system / external_guest / device_proxy` |
| `member` | actor 在某个容器中的成员关| `member` 是关系，不是人本|
| `space` | 工作区、社区、组织容| 上层治理容器，可选承载多个群 |
| `chat_group` | 群组 | 支持临时群、社区群、组织群 |
| `chat_channel` | 群内频道 | 文本、公告、话题、语音等 |
| `friendship` | 好友关系 | 只建模人和人的双向关|
| `direct_chat` | 双人直聊业务对象 | 可由好友、陌生私聊、系统触发产|
| `conversation` | 消息容器 | 运行态概念，不承担好群成员真|
| `invitation` | 邀| 适用space / group / channel |
| `membership_request` | 入群/入空间申| 适用于需要审批的容器 |
| `ban_record` | 封禁/禁言记录 | 治理真值，不混入成员表 |
| `mute_setting` | 用户侧免打扰设置 | 偏好设置，不代表治理处罚 |
| `audit_event` | 审计事件 | 审批、踢人、禁言、权限变更都要落审计 |
| `outbox_event` | 跨服务发布事| 驱动投影、搜索、通知、风控同|

明确不推荐把核心实体命名`account`。在支付、钱包、账务语境里，`account` 通常表达账户，不适合当前 IM 领域主语。这里冻结为。

- 人用 `user`
- 参与者用 `actor`
- 容器内关系用 `member`

## 4. 领域边界

| 限界上下| 核心实体 | 负责什| 不负责什| 建议落位 |
| --- | --- | --- | --- | --- |
| `Directory / Identity` | `user`, `actor` | 主体身份、主体类型、外部映| 好友关系、群治理 | `crates/im-domain-core` |
| `Social` | `friend_request`, `friendship`, `user_block`, `direct_chat` | 好友、拉黑、直聊建| 群成员、消息存| `crates/im-domain-core` + `services/control-plane-api` |
| `Space` | `space`, `space_role`, `space_member` | 工作社区治理 | 消息内容 | `crates/im-domain-core` + `services/control-plane-api` |
| `Group` | `chat_group`, `group_role`, `group_member`, `chat_channel`, `channel_access_rule` | 群、频道、权限覆| 消息正文 | `crates/im-domain-core` + `services/control-plane-api` |
| `Conversation` | `conversation`, `message`, `read_cursor`, `conversation_member` | 消息容器、消息流、游标、运行态投| 好友真值、群成员真| `services/conversation-runtime` |
| `Governance` | `invitation`, `membership_request`, `ban_record`, `mute_setting`, `report_ticket`, `audit_event`, `outbox_event` | 审批、封禁、审计、跨服务事件 | 长期联系人列| `services/control-plane-api` + `services/audit-service` |
| `Projection` | `contact_projection`, `group_roster_projection`, `channel_badge_projection` | 联系人、名册、角标、搜索视| 真值裁| `services/projection-service` |

## 5. 聚合边界与不变量

### 5.1 `user` / `actor`

- 一`user` 可以映射一个或多个 `actor`，例如真人用户本体、AI 代理、外部访客代理。
- 好友关系只在 `user` 维度建立，不建议`actor` 维度建立“好友”，否则会把 bot/device/system 也拖入社交关系。
- 消息发送、群成员、频道参与统一使用 `actor_id`，这样同一套会话域可以兼容 AI / Agent / 系统消息。

### 5.2 `friendship`

- `friendship` 是一个无序二元关系，只允许一条当前生效记录。
- 数据库侧必须通过 `user_low_id + user_high_id` 建唯一约束，避免双写成两行。
- `friend_request` 只负责申请流程，接受后生成或激`friendship`。
- `direct_chat` `friendship` 解耦。好友接受后可以自动创建直聊，也可以懒创建。

### 5.3 `space`

- `space` 是治理容器，不等于消息会话。
- `space` 负责组织边界、角色、政策、入口权限、成员生命周期。
- `space_member` 应建模为“一人一条成员关系聚合”，不能把全体成员塞`space` 单一大聚合。

### 5.4 `chat_group`

- `chat_group` 负责群元数据、入群策略、统计字段、默认频道等。
- `group_member` 仍然是一人一条成员关系聚合，不把整个成员列表作为单个聚合内对象集合。
- 大群场景下，群资料修改与成员进出必须能独立事务提交。

### 5.5 `chat_channel`

- `chat_channel` 默认继承 `chat_group` `space` 的访问边界。
- 只对例外情况`channel_access_rule` 覆盖规则。
- 除非是显式订阅型频道，否则不要默认建立海`channel_member` 真值表。
- 用户侧“已加入/隐藏/置顶/免打扰”才`channel_subscription`。

### 5.6 `conversation`

- `conversation` 是消息容器，不是好友或群治理真值。
- `conversation.business_type + business_id` 指向 `direct_chat` `chat_channel`。
- `conversation_member` 允许保留，但必须定义为运行态投影或缓存，不可回写为群成员真值。

### 5.7 `governance`

- 审批、邀请、禁言、封禁、举报必须独立建模。
- 不允许把“禁言中”“等待审批中”直接塞`group_member.status` 然后丢失历史。
- 所有关键治理动作都应写`audit_event`，跨服务同步通过 `outbox_event` 驱动。

## 6. 数据库总原。

1. 每张核心表都`tenant_id`，并让热查询索引`tenant_id` 开头。
2. 主键推荐统一使用可排ID，例`BIGINT snowflake_id`；对API 可序列化为字符串。
3. 统一采用 `current_state + domain_event + outbox_event + projection` 四层组合。
4. 核心真值表保留 `status / version / created_at / updated_at / deleted_at`，支持幂等和审计。
5. 高频写表和历史表分离，避免消息热写拖垮治理查询。
6. `contact_projection`、`conversation_member`、未读角标、最近联系人都属于可重建视图。
7. 大群权限优先走角色继承和覆盖规则，避免无意义的成员复制。

## 7. 必须新增的数据模。

### 7.1 Directory / Identity

| 表名 | 作用 | 关键字段 | 核心索引 |
| --- | --- | --- | --- |
| `im_user` | 自然人主| `user_id`, `tenant_id`, `status`, `display_name`, `avatar_url` | `uk(tenant_id, user_id)` |
| `im_actor` | 统一参与者主| `actor_id`, `tenant_id`, `actor_type`, `user_id`, `external_ref`, `status` | `uk(tenant_id, actor_type, external_ref)`、`idx(tenant_id, user_id)` |

### 7.2 Social

| 表名 | 作用 | 关键字段 | 核心索引 |
| --- | --- | --- | --- |
| `im_friend_request` | 好友申请 | `request_id`, `requester_user_id`, `target_user_id`, `status`, `request_message`, `expired_at` | `idx(tenant_id, target_user_id, status, created_at desc)`、`idx(tenant_id, requester_user_id, status, created_at desc)` |
| `im_friendship` | 好友真| `friendship_id`, `user_low_id`, `user_high_id`, `initiator_user_id`, `status`, `established_at` | `uk(tenant_id, user_low_id, user_high_id)` |
| `im_friendship_event` | 好友关系历史 | `event_id`, `friendship_id`, `event_type`, `operator_user_id`, `payload_json` | `idx(friendship_id, occurred_at desc)` |
| `im_user_block` | 拉黑关系 | `block_id`, `blocker_user_id`, `blocked_user_id`, `scope`, `status`, `expires_at` | `uk(tenant_id, blocker_user_id, blocked_user_id, scope)` |
| `im_direct_chat` | 直聊业务对象 | `direct_chat_id`, `left_actor_id`, `right_actor_id`, `pair_hash`, `status`, `conversation_id` | `uk(tenant_id, pair_hash)` |
| `im_contact_projection` | 联系人投影视| `owner_user_id`, `contact_type`, `target_id`, `relationship_state`, `last_interaction_at` | `idx(tenant_id, owner_user_id, sort_key)` |

### 7.3 Space

| 表名 | 作用 | 关键字段 | 核心索引 |
| --- | --- | --- | --- |
| `im_space` | 工作社区容器 | `space_id`, `space_type`, `visibility`, `owner_actor_id`, `name`, `join_policy`, `status` | `idx(tenant_id, status, created_at desc)` |
| `im_space_role` | 空间角色 | `role_id`, `space_id`, `role_key`, `role_name`, `priority`, `permission_bits` | `uk(space_id, role_key)` |
| `im_space_member` | 空间成员真| `member_id`, `space_id`, `actor_id`, `membership_status`, `join_source`, `joined_at`, `left_at` | `uk(space_id, actor_id)`、`idx(actor_id, membership_status, updated_at desc)` |
| `im_space_member_role` | 空间成员角色分配 | `member_role_id`, `space_id`, `member_id`, `role_id` | `uk(member_id, role_id)` |
| `im_space_member_event` | 空间成员历史 | `event_id`, `space_id`, `actor_id`, `event_type`, `operator_actor_id` | `idx(space_id, occurred_at desc)` |

### 7.4 Group / Channel

| 表名 | 作用 | 关键字段 | 核心索引 |
| --- | --- | --- | --- |
| `im_chat_group` | 群组真| `group_id`, `space_id`, `group_kind`, `owner_actor_id`, `name`, `join_policy`, `member_limit`, `default_channel_id`, `status` | `idx(tenant_id, space_id, status)` |
| `im_group_role` | 群角| `role_id`, `group_id`, `role_key`, `priority`, `permission_bits` | `uk(group_id, role_key)` |
| `im_group_member` | 群成员真| `member_id`, `group_id`, `actor_id`, `membership_status`, `nickname_in_group`, `joined_at`, `left_at` | `uk(group_id, actor_id)`、`idx(actor_id, membership_status, updated_at desc)` |
| `im_group_member_role` | 群成员角色分| `member_role_id`, `group_id`, `member_id`, `role_id` | `uk(member_id, role_id)` |
| `im_group_member_event` | 群成员历| `event_id`, `group_id`, `actor_id`, `event_type`, `operator_actor_id` | `idx(group_id, occurred_at desc)` |
| `im_chat_channel` | 群内频道 | `channel_id`, `group_id`, `channel_type`, `name`, `topic`, `visibility`, `default_auto_join`, `conversation_id`, `status` | `idx(group_id, sort_order)` |
| `im_channel_access_rule` | 频道访问覆盖规则 | `rule_id`, `channel_id`, `subject_type`, `subject_id`, `effect`, `permission_bits`, `priority` | `idx(channel_id, priority)`、`idx(channel_id, subject_type, subject_id)` |
| `im_channel_subscription` | 频道用户偏好/订阅 | `subscription_id`, `channel_id`, `actor_id`, `subscription_state`, `notification_level`, `pinned_at` | `uk(channel_id, actor_id)` |

### 7.5 Governance

| 表名 | 作用 | 关键字段 | 核心索引 |
| --- | --- | --- | --- |
| `im_invitation` | 邀| `invitation_id`, `target_type`, `target_id`, `inviter_actor_id`, `invitee_type`, `invitee_value`, `status`, `expires_at` | `idx(target_type, target_id, status, created_at desc)` |
| `im_membership_request` | 入群/入空间申| `request_id`, `target_type`, `target_id`, `applicant_actor_id`, `status`, `reviewer_actor_id`, `decided_at` | `idx(target_type, target_id, status, created_at desc)` |
| `im_ban_record` | 封禁/禁言记录 | `ban_id`, `target_scope_type`, `target_scope_id`, `subject_actor_id`, `ban_type`, `status`, `starts_at`, `expires_at` | `idx(target_scope_type, target_scope_id, subject_actor_id, status)` |
| `im_mute_setting` | 用户侧免打扰 | `setting_id`, `owner_actor_id`, `target_type`, `target_id`, `mute_scope`, `expires_at` | `uk(owner_actor_id, target_type, target_id, mute_scope)` |
| `im_report_ticket` | 举报工单 | `report_id`, `reporter_actor_id`, `target_type`, `target_id`, `reason_code`, `status` | `idx(status, created_at desc)` |
| `im_audit_event` | 审计日志 | `audit_id`, `domain`, `aggregate_type`, `aggregate_id`, `action_type`, `operator_actor_id`, `payload_json` | `idx(domain, aggregate_type, aggregate_id, created_at desc)` |
| `im_outbox_event` | Outbox 事件 | `event_id`, `aggregate_type`, `aggregate_id`, `event_type`, `payload_json`, `publish_status`, `available_at` | `idx(publish_status, available_at)` |

### 7.6 Conversation Runtime

| 表名 | 作用 | 关键字段 | 核心索引 |
| --- | --- | --- | --- |
| `im_conversation` | 消息容器 | `conversation_id`, `business_type`, `business_id`, `scenario`, `status`, `last_message_id`, `last_message_at` | `uk(tenant_id, business_type, business_id)` |
| `im_conversation_member` | 会话成员运行态投| `member_id`, `conversation_id`, `actor_id`, `member_state`, `source_version`, `last_synced_at` | `uk(conversation_id, actor_id)` |
| `im_message` | 消息主表 | `message_id`, `conversation_id`, `seq`, `sender_actor_id`, `message_type`, `body_json`, `sent_at`, `edited_at`, `recalled_at` | `uk(conversation_id, seq)`、`idx(conversation_id, sent_at desc)` |
| `im_message_attachment` | 消息附件 | `attachment_id`, `message_id`, `object_key`, `mime_type`, `size_bytes` | `idx(message_id)` |
| `im_read_cursor` | 阅读游标 | `cursor_id`, `conversation_id`, `actor_id`, `last_read_seq`, `last_read_message_id`, `updated_at` | `uk(conversation_id, actor_id)` |

## 8. 关键关系链路

### 8.1 好友接受

1. 创建或更`im_friendship`
2. 写入 `im_friendship_event`
3. 根据产品策略懒创建或立即创建 `im_direct_chat`
4. `im_direct_chat` 对应一`im_conversation`
5. `projection-service` 刷新 `im_contact_projection`

### 8.2 入群审批

1. 先写 `im_membership_request`
2. 审批通过后写 `im_group_member`
3. 同时`im_group_member_event`
4. 根据频道继承规则生成或刷`im_conversation_member`
5. 若有欢迎消息，由 `conversation-runtime` 进入消息。

### 8.3 频道权限判断

1. 读取 `group_member / group_member_role`
2. 叠加 `space_member / space_role`
3. 应用 `im_channel_access_rule`
4. 生成运行`conversation_member` 投影
5. `conversation-runtime` 做最终消息级写入校验

## 9. 性能与扩展策。

### 9.1 热表拆分

- `im_message`、`im_audit_event`、`im_outbox_event` 应优先按时间或哈希分区。
- `im_group_member`、`im_space_member` 只保留当前态，历史进入 `*_event` 表。
- 群统计字段例`member_count / channel_count / active_member_count` 放在根表，允许异步校正。

### 9.2 大群与超大群

- `group_member` `(tenant_id, group_id)` 局部聚簇。
- `message` 查询永远`(conversation_id, seq)` `(conversation_id, sent_at)`，避免全局时间线扫描。
- `conversation_member` 可以`conversation_id hash` 做分桶，必要时只保留在线窗口缓存。

### 9.3 可重建投。

以下模型允许删除后重建：

- `im_contact_projection`
- `im_conversation_member`
- 角标、未读、最近联系人、群名册摘要

以下模型不可视为可丢弃缓存：

- `im_friendship`
- `im_space_member`
- `im_group_member`
- `im_channel_access_rule`
- `im_invitation`
- `im_membership_request`
- `im_ban_record`

### 9.4 事件一致。

- 所有跨域副作用统一`im_outbox_event` 发出。
- 联系人、搜索、通知、风控不直接读写真值表。
- 任何“由会话推导好友”或“由会话成员倒推出群成员”的反向建模都应禁止。

## 10. 与现有仓库的落地关系

| 仓库模块 | 推荐职责 |
| --- | --- |
| `crates/im-domain-core` | 新增 `social / space / group / governance` 核心领域对象 |
| `crates/im-domain-events` | 承载好友、成员、治理事件定|
| `crates/im-platform-contracts` | 提供仓储接口、查询契约、投影契|
| `services/conversation-runtime` | 保持消息、游标、会话投影运行态；不再承载好友/群成员真|
| `services/session-gateway` | 会话连接、presence、realtime fanout；消费投影结|
| `services/control-plane-api` | 管理端治理、审批、角色、封禁、策略下|
| `services/projection-service` | 联系人、名册、频道订阅、角标等投影 |
| `services/audit-service` | 审计存储、检索、追|

## 11. 迁移原则

1. 不直接推翻现`conversation-runtime`，而是先为其补`business_type / business_id` 映射。
2. 现有成员逻辑逐步降级`conversation_member` 投影。
3. 新的好友、群、空间、治理真值应先落独立表，再由投影驱动会话侧。
4. 旧逻辑若继续把 `conversation_member` 当真值，应在实施阶段显式标记为待淘汰缺口。

## 12. 结论

这套设计比“直接在现有 `conversation / member` 上继续堆字段”更稳健，原因是。

- 它把社会关系、组织治理、消息运行态彻底分开了。
- 它能同时兼容 `user` / `actor`，为 AI / Agent / 系统参与者保留扩展位。
- 它对大群性能更友好，因为成员真值、频道权限、消息热写不再硬绑在一个聚合里。
- 它符DDD，统一语言稳定，后续代码结构、数据库结构API 边界都能持续收敛。

## 2026-04-09 Addendum

### A. 行业对标收敛

补充对标见：

- `docs/架构/151CJ-im-benchmark-model-alignment-2026-04-09.md`

Slack、Teams、Discord、Matrix 提炼后，本设计新增三条硬规则。

1. `space` 不等于自动加入全`group/channel` 的超级会话。
2. `chat_channel` 必须优先走“继+ 覆盖”权限模型。
3. `thread` 必须是一级模型，不能只靠消息回复字段硬拼。

### B. 推荐补充模型

为避免后续再返工，建议把以下模型提前纳入标准层：

| 模型 | 作用 |
| --- | --- |
| `im_chat_thread` | 频道内线程主模型 |
| `im_thread_subscription` | 线程参与与通知 |
| `im_external_connection` | 跨租跨组织协作关|
| `im_external_member_link` | 外部成员映射 |
| `im_message_reaction` | 反应表情 |
| `im_pin_record` | 置顶/知识锚点 |
| `im_history_visibility_policy` | 历史可见性策|
| `im_retention_policy` | 留存与合规策|
