> Migrated from `docs/database-design.md` on 2026-06-24.
> Owner: SDKWork maintainers

## 概述

本文档描述 Sdkwork IM 即时通信系统的数据库设计，包括所有表结构、索引策略、ID 设计和数据生命周期管理。

## 设计原则

1. **雪花算法 ID**: 所有主键使用 BIGINT 类型的 Snowflake ID，全局唯一且趋势递增
2. **租户+组织双重隔离**: 所有表包含 `tenant_id` 和 `organization_id` 字段
3. **CQRS+ES 模式**: 命令端使用事件溯源，查询端使用投影表
4. **幂等写入**: 所有写入操作支持幂等性
5. **数据生命周期**: 所有表包含 `retention_until` 字段用于数据归档

## ID 设计

| ID 类型 | 数据类型 | 生成方式 | 说明 |
|---------|----------|----------|------|
| 主键 ID | BIGINT | Snowflake | 全局唯一，趋势递增 |
| 外键引用 | TEXT | IAM 提供 | 引用 iam_user.user_id |
| 会话 ID | TEXT | 业务生成 | 如 conv_{group_id} |
| 消息序号 | BIGINT | 会话内递增 | im_conversation_seq_counters |

### Snowflake ID 结构

```
+-----------------------------------------------------------------------+
| 0 | 41-bit timestamp (ms) | 10-bit node_id | 12-bit sequence |
+-----------------------------------------------------------------------+

- 时间戳: 41 位，支持约 69 年
- 节点 ID: 10 位，支持 1024 个节点
- 序列号: 12 位，每毫秒支持 4096 个 ID
```

## 表清单

### 第一部分：社交关系表（7 张）

#### 1. im_friend_requests - 好友请求表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| request_id | BIGINT | 请求 ID（Snowflake） |
| requester_user_id | TEXT | 请求者用户 ID |
| target_user_id | TEXT | 目标用户 ID |
| request_message | TEXT | 请求消息 |
| status | TEXT | 状态：pending/accepted/declined/canceled/expired |
| expired_at | TIMESTAMPTZ | 过期时间 |
| created_at | TIMESTAMPTZ | 创建时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |

**索引:**
- `pk_im_friend_requests`: (tenant_id, organization_id, request_id)
- `idx_im_friend_requests_requester`: (tenant_id, organization_id, requester_user_id, status, created_at DESC)
- `idx_im_friend_requests_target`: (tenant_id, organization_id, target_user_id, status, created_at DESC)

#### 2. im_friendships - 好友关系表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| friendship_id | BIGINT | 好友关系 ID（Snowflake） |
| user_low_id | TEXT | 规范化用户 ID（较小） |
| user_high_id | TEXT | 规范化用户 ID（较大） |
| initiator_user_id | TEXT | 发起者用户 ID |
| status | TEXT | 状态：active/removed |
| established_at | TIMESTAMPTZ | 建立时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |

**索引:**
- `pk_im_friendships`: (tenant_id, organization_id, friendship_id)
- `idx_im_friendships_user_low`: (tenant_id, organization_id, user_low_id, status, established_at DESC)
- `idx_im_friendships_user_high`: (tenant_id, organization_id, user_high_id, status, established_at DESC)

#### 3. im_user_blocks - 用户屏蔽表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| block_id | BIGINT | 屏蔽 ID（Snowflake） |
| blocker_user_id | TEXT | 屏蔽者用户 ID |
| blocked_user_id | TEXT | 被屏蔽者用户 ID |
| scope | TEXT | 作用域：all/friendship/direct_chat |
| direct_chat_id | BIGINT | 单聊会话 ID |
| reason | TEXT | 屏蔽原因 |
| expires_at | TIMESTAMPTZ | 过期时间 |
| created_at | TIMESTAMPTZ | 创建时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |

#### 4. im_direct_chats - 单聊会话表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| direct_chat_id | BIGINT | 单聊 ID（Snowflake） |
| left_actor_kind | TEXT | 左侧参与者类型 |
| left_actor_id | TEXT | 左侧参与者 ID |
| right_actor_kind | TEXT | 右侧参与者类型 |
| right_actor_id | TEXT | 右侧参与者 ID |
| pair_hash | TEXT | 规范化后的哈希 |
| status | TEXT | 状态：active/archived/closed |
| conversation_id | TEXT | 关联的会话 ID |
| created_at | TIMESTAMPTZ | 创建时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |

#### 5. im_external_connections - 外部连接表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| connection_id | BIGINT | 连接 ID（Snowflake） |
| external_tenant_id | TEXT | 外部租户 ID |
| external_org_name | TEXT | 外部组织名称 |
| connection_kind | TEXT | 连接类型：shared_channel |
| status | TEXT | 状态：active/suspended/revoked |
| established_at | TIMESTAMPTZ | 建立时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |

#### 6. im_external_member_links - 外部成员链接表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| link_id | BIGINT | 链接 ID（Snowflake） |
| connection_id | BIGINT | 连接 ID |
| local_actor_kind | TEXT | 本地参与者类型 |
| local_actor_id | TEXT | 本地参与者 ID |
| external_member_id | TEXT | 外部成员 ID |
| external_display_name | TEXT | 外部显示名称 |
| status | TEXT | 状态：active/revoked |
| linked_at | TIMESTAMPTZ | 链接时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |

#### 7. im_shared_channel_policies - 共享频道策略表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| policy_id | BIGINT | 策略 ID（Snowflake） |
| connection_id | BIGINT | 连接 ID |
| channel_id | TEXT | 频道 ID |
| conversation_id | TEXT | 会话 ID |
| policy_version | BIGINT | 策略版本 |
| history_visibility | TEXT | 历史可见性：shared/isolated |
| status | TEXT | 状态：active/suspended |
| applied_at | TIMESTAMPTZ | 应用时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |

### 组织模型表

#### 8. im_spaces - 空间/组织表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| space_id | BIGINT | 空间 ID（Snowflake） |
| space_name | TEXT | 空间名称 |
| space_type | TEXT | 空间类型：organization/team/project/community |
| owner_user_id | TEXT | 所有者用户 ID |
| description | TEXT | 描述 |
| avatar_url | TEXT | 头像 URL |
| max_members | INTEGER | 最大成员数（默认 10000） |
| settings_json | JSONB | 设置 JSON |
| created_at | TIMESTAMPTZ | 创建时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |

#### 9. im_space_members - 空间成员表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| space_id | BIGINT | 空间 ID |
| user_id | TEXT | 用户 ID |
| role | TEXT | 角色：owner/admin/member/guest |
| nickname | TEXT | 昵称 |
| joined_at | TIMESTAMPTZ | 加入时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |

#### 10. im_chat_groups - 群组表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| group_id | BIGINT | 群组 ID（Snowflake） |
| space_id | BIGINT | 所属空间 ID（可选） |
| group_name | TEXT | 群组名称 |
| group_type | TEXT | 群组类型：normal/announcement/project/department |
| owner_user_id | TEXT | 所有者用户 ID |
| conversation_id | TEXT | 关联的会话 ID |
| max_members | INTEGER | 最大成员数（默认 500） |
| description | TEXT | 描述 |
| avatar_url | TEXT | 头像 URL |
| announcement | TEXT | 群公告 |
| settings_json | JSONB | 设置 JSON |
| created_at | TIMESTAMPTZ | 创建时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |

#### 11. im_group_members - 群组成员表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| group_id | BIGINT | 群组 ID |
| user_id | TEXT | 用户 ID |
| role | TEXT | 角色：owner/admin/member/muted |
| nickname | TEXT | 群内昵称 |
| mute_until | TIMESTAMPTZ | 禁言截止时间 |
| joined_at | TIMESTAMPTZ | 加入时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |

#### 12. im_chat_channels - 频道表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| channel_id | BIGINT | 频道 ID（Snowflake） |
| space_id | BIGINT | 所属空间 ID |
| channel_name | TEXT | 频道名称 |
| channel_type | TEXT | 频道类型：text/voice/announcement/forum |
| description | TEXT | 描述 |
| conversation_id | TEXT | 关联的会话 ID |
| position | INTEGER | 排序位置 |
| is_nsfw | BOOLEAN | 是否 NSFW |
| is_pinned | BOOLEAN | 是否置顶 |
| topic | TEXT | 频道话题 |
| settings_json | JSONB | 设置 JSON |
| created_at | TIMESTAMPTZ | 创建时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |

#### 13. im_channel_access_rules - 频道访问规则表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| rule_id | BIGINT | 规则 ID（Snowflake） |
| channel_id | BIGINT | 频道 ID |
| rule_type | TEXT | 规则类型：allow/deny |
| principal_kind | TEXT | 主体类型：user/role/group |
| principal_id | TEXT | 主体 ID |
| permission | TEXT | 权限：view/send/manage |
| created_at | TIMESTAMPTZ | 创建时间 |

### 消息互动表

#### 14. im_message_reactions - 消息 Reaction 表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| conversation_id | TEXT | 会话 ID |
| message_id | BIGINT | 消息 ID |
| user_id | TEXT | 用户 ID |
| reaction_type | TEXT | Reaction 类型（emoji） |
| created_at | TIMESTAMPTZ | 创建时间 |

#### 15. im_message_pins - 消息 Pin 表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| conversation_id | TEXT | 会话 ID |
| message_id | BIGINT | 消息 ID |
| pinned_by_user_id | TEXT | Pin 者用户 ID |
| pin_reason | TEXT | Pin 原因 |
| pinned_at | TIMESTAMPTZ | Pin 时间 |

#### 16. im_threads - Thread 表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| thread_id | BIGINT | Thread ID（Snowflake） |
| conversation_id | TEXT | 会话 ID |
| root_message_id | BIGINT | 根消息 ID |
| thread_title | TEXT | Thread 标题 |
| reply_count | INTEGER | 回复数量 |
| last_reply_at | TIMESTAMPTZ | 最后回复时间 |
| last_reply_user_id | TEXT | 最后回复用户 ID |
| created_at | TIMESTAMPTZ | 创建时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |

#### 17. im_thread_subscriptions - Thread 订阅表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| thread_id | BIGINT | Thread ID |
| user_id | TEXT | 用户 ID |
| last_read_seq | BIGINT | 最后已读序号 |
| notification_level | TEXT | 通知级别：all/mentions/none |
| subscribed_at | TIMESTAMPTZ | 订阅时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |

### 用户扩展表

#### 18. im_user_profiles - IM 用户资料扩展表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| user_id | TEXT | 用户 ID |
| im_nickname | TEXT | IM 专属昵称 |
| im_avatar_url | TEXT | IM 专属头像 |
| im_status_message | TEXT | 状态消息 |
| im_notification_prefs | JSONB | 通知偏好 |
| im_mute_settings | JSONB | 免打扰设置 |
| im_privacy_settings | JSONB | 隐私设置 |
| im_online_status | TEXT | 在线状态：online/away/busy/invisible/offline |
| last_active_at | TIMESTAMPTZ | 最后活跃时间 |
| created_at | TIMESTAMPTZ | 创建时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |

#### 19. im_user_settings - 用户设置表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| user_id | TEXT | 用户 ID |
| setting_key | TEXT | 设置键 |
| setting_value | JSONB | 设置值 |
| updated_at | TIMESTAMPTZ | 更新时间 |

#### 20. im_conversation_settings - 会话设置表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| conversation_id | TEXT | 会话 ID |
| user_id | TEXT | 用户 ID |
| is_muted | BOOLEAN | 是否免打扰 |
| mute_until | TIMESTAMPTZ | 免打扰截止时间 |
| is_pinned | BOOLEAN | 是否置顶 |
| is_archived | BOOLEAN | 是否归档 |
| is_blocked | BOOLEAN | 是否屏蔽 |
| notification_level | TEXT | 通知级别：all/mentions/none |
| custom_name | TEXT | 自定义名称 |
| settings_json | JSONB | 设置 JSON |
| updated_at | TIMESTAMPTZ | 更新时间 |

### 搜索索引

#### 消息全文搜索

在 `im_conversation_messages` 表上创建：
- `search_vector` tsvector 列
- GIN 索引 `idx_im_messages_search`
- 触发器 `im_messages_search_trigger` 自动更新搜索向量

### 邀请和封禁表

#### 24. im_invitations - 邀请表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| invitation_id | BIGINT | 邀请 ID（Snowflake） |
| inviter_user_id | TEXT | 邀请者用户 ID |
| invitee_user_id | TEXT | 被邀请者用户 ID |
| invitee_email | TEXT | 被邀请者邮箱 |
| invitee_phone | TEXT | 被邀请者手机 |
| target_type | TEXT | 目标类型：space/group/channel |
| target_id | BIGINT | 目标 ID |
| role | TEXT | 角色 |
| status | TEXT | 状态：pending/accepted/declined/expired/canceled |
| message | TEXT | 邀请消息 |
| expires_at | TIMESTAMPTZ | 过期时间 |
| accepted_at | TIMESTAMPTZ | 接受时间 |
| created_at | TIMESTAMPTZ | 创建时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |

#### 25. im_ban_records - 封禁记录表

| 字段 | 类型 | 说明 |
|------|------|------|
| tenant_id | TEXT | 租户 ID |
| organization_id | TEXT | 组织 ID |
| ban_id | BIGINT | 封禁 ID（Snowflake） |
| target_type | TEXT | 目标类型：space/group/channel |
| target_id | BIGINT | 目标 ID |
| banned_user_id | TEXT | 被封禁用户 ID |
| banned_by_user_id | TEXT | 封禁者用户 ID |
| reason | TEXT | 封禁原因 |
| expires_at | TIMESTAMPTZ | 过期时间 |
| unbanned_at | TIMESTAMPTZ | 解封时间 |
| unbanned_by_user_id | TEXT | 解封者用户 ID |
| created_at | TIMESTAMPTZ | 创建时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |

## 表关系图

```
im_spaces (1) ──< im_space_members (N)
    │
    ├──< im_chat_groups (N)
    │       │
    │       └──< im_group_members (N)
    │
    └──< im_chat_channels (N)
            │
            └──< im_channel_access_rules (N)

im_conversation_messages (1) ──< im_message_reactions (N)
                         │
                         ├──< im_message_pins (N)
                         │
                         └──< im_threads (N)
                                 │
                                 └──< im_thread_subscriptions (N)

iam_user (1) ──< im_user_profiles (1)
         │
         ├──< im_user_settings (N)
         │
         └──< im_conversation_settings (N)

im_invitations: 邀请关系
im_ban_records: 封禁关系
```

## 数据生命周期

| 数据类型 | 保留策略 | 归档方式 |
|---------|---------|---------|
| 消息 | 2 年 | 按月分区归档到 S3 |
| 事件日志 | 7 年 | 按月分区归档到 S3 |
| 实时事件 | 30 天 | 自动过期删除 |
| 投影数据 | 随真值同步 | 从事件重建 |
| 用户资料 | 永久 | 软删除 |

## 索引策略

1. **主键索引**: 所有表使用复合主键 (tenant_id, organization_id, ...)
2. **查询索引**: 针对常见查询模式创建覆盖索引
3. **时间索引**: 对时间序列数据使用 BRIN 索引
4. **全文索引**: 使用 GIN 索引支持消息搜索
5. **部分索引**: 对软删除数据使用部分索引

