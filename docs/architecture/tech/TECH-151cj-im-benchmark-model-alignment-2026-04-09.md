> Migrated from `docs/架构/151CJ-im-benchmark-model-alignment-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 151CJ - IM 对标产品模型抽象与架构收。

## 1. 目标

把行业头IM / 协作产品中最稳定、最值得复用的结构抽象出来，作为 `sdkwork-im` 后续群组、好友、空间、频道、线程和治理模型的收敛标准。

本文件不复制竞品功能清单，只提炼稳定结构。

## 2. 对标样本

| 产品 | 官方参| 稳定结构 |
| --- | --- | --- |
| Slack | https://slack.com/help/articles/360017938993-What-is-a-channel | `workspace -> channel -> thread`，频道分 `public/private`，公开频道可搜索和加入，私有频道需显式添加 |
| Slack | https://slack.com/help/articles/201314026-Permissions-by-role-in-Slack-Permissions-by-role-in-Slack | `owner/admin/member/guest` 分层，频道治理权限与工作区角色分离，外部协作guest 不是同一语义 |
| Slack | https://slack.com/help/articles/6705229084563-Slack-Connect--Manage-your-organization’s-connections’s-connections | 跨组织协作是显式连接模型，不应伪装成普通群成员 |
| Microsoft Teams | https://support.microsoft.com/en-us/office/standard-private-or-shared-channels-in-microsoft-teams-de3e20b0-7494-439c-b7e5-75899ebe6a0e | `team -> channel`，频道分 `standard/private/shared`，标准频道继承团队可见性，私有/共享频道是例外容|
| Microsoft Teams | https://learn.microsoft.com/en-us/microsoftteams/shared-channels | 外部协作和跨团队协作`shared channel`，不是直接复制整支团队的成员|
| Discord | https://support.discord.com/hc/en-us/articles/206029707-Setting-Up-Permissions-FAQ | `server -> category -> channel` 权限继承，频道可与分类同步或覆盖 |
| Discord | https://support.discord.com/hc/en-us/articles/206141927-How-is-the-permission-hierarchy-structured | 权限计算走“角色基+ 频道覆盖 + 成员特例|
| Discord | https://support.discord.com/hc/en-us/articles/4403205878423-Threads-FAQ | 线程是频道内子会话，不等于新频道；线程可有独立加入和通知语义 |
| Matrix | https://matrix.org/docs/communities/getting-started/ | `space -> room`，加Space 不等于自动加入全room，公开 Space 可以包含私有 room |
| Matrix | https://spec.matrix.org/v1.1/client-server-api/ | 成员状态至少区`invite / join / ban / knock`，治理状态应durable truth |
| Matrix | https://spec.matrix.org/latest/rooms/ | room version 独立演进，协治理规则升级不能靠隐式热更新 |

## 3. 归纳后的结构标准

### 3.1 顶层容器

- `space` 对应 Slack workspace、Teams team、Discord server、Matrix space。
- 加入 `space` 不等于自动加入所`chat_group / chat_channel`。
- `space` 负责成员目录、默认策略、外部协作边界和全局角色。

### 3.2 群与频道

- `chat_group` 负责稳定成员名册和治理。
- `chat_channel` 是群内业务分区，不应默认复制整张群成员表。
- 默认权限模型应为。
  - 上层角色授权
  - 频道继承
  - 覆盖规则
  - 个别成员特例

这比“每个频道都存一份成员真值”更接近 Discord / Teams / Matrix 的成熟做法。

### 3.3 线程

- `thread` 必须是一级模型，而不是仅靠消息回复字段拼出来。
- 线程属于 `channel/message` 下级对象，不升级成新`group`。
- 线程至少应支持：
  - 创建。
  - 来源消息
  - 参与订阅。
  - 独立未读与通知级别
  - 关闭/归档状。

### 3.4 直聊与频道分。

- `direct_chat` 应独立于 `chat_channel`。
- 好友关系、陌生私聊、跨组织 DM 都不应复用群/频道成员模型。
- `conversation` 只承载消息，不承担“好友是否成立”的裁决。

### 3.5 外部协作

- guest、external member、shared channel、cross-tenant link 不是一回事。
- 对标 Slack Connect Teams shared channels，外部协作应是显式模型。
- 推荐最少引入：
  - `external_connection`
  - `external_member_link`
  - `shared_channel_policy`

### 3.6 治理状。

- 对标 Matrix membership Discord role/permission 体系，审批、禁言、封禁、踢出、历史可见性必须是 durable truth。
- 这些状态不能只挂在 `conversation_member` 或通知投影上。

### 3.7 版本与策。

- 对标 Matrix room versions，治理规则和协议能力要支持显式版本演进。
- 推荐`space/group/channel/conversation` 上保留：
  - `policy_version`
  - `capability_flags`
  - `history_visibility`
  - `retention_policy_ref`

## 4. `sdkwork-im` 的强制落地规。

1. `space` 是顶层治理容器，`chat_group` 是稳定成员容器，`chat_channel` 是业务分区，`thread` 是频道内子会话。
2. `friendship`、`group_member`、`space_member`、`ban_record`、`membership_request` 必须durable truth。
3. `conversation_member` 只能是运行态投影，不再充当好友真值。
4. 频道权限默认走“继+ overwrite”，不做海量重复成员复制。
5. 外部协作必须显式建模，不能把外部协作者直接混进普member 语义。
6. 线程必须独立建模，否则消息、通知、未读、权限都会长期失真。

## 5. 推荐补充模型

| 模型 | 必要| 说明 |
| --- | --- | --- |
| `im_chat_thread` | | 对标 Slack/Discord/Matrix threads |
| `im_thread_subscription` | | 线程参与、未读、通知级别 |
| `im_external_connection` | | 对标 Slack Connect / Teams shared channel |
| `im_external_member_link` | | 外部成员映射与来源组织追|
| `im_message_reaction` | | 行业通用协作能力 |
| `im_pin_record` | | 频道知识锚点 |
| `im_history_visibility_policy` | | 对标 Matrix 房间历史可见|
| `im_retention_policy` | | 对标企业协作和合规治|
| `im_member_directory_projection` | | 空间/群成员目录读模型 |

## 6. 结论

行业头部产品的共同点不是“功能很多”，而是它们在以下几点上高度一致：

- 顶层容器、群/频道、线程、直聊严格分。
- 权限走角色继承和覆盖
- 外部协作单独建模
- 治理状态是 durable truth
- 消息容器不承担全部社会关系真。

`sdkwork-im` 后续文档和实现应以这组结构标准为准，而不是继续在 `conversation/member` 上堆更多语义。

