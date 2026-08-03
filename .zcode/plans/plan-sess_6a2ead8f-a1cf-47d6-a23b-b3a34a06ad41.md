# H5前后端对接联调实施方案（IM核心闭环）

## 目标与边界
- 联调范围（用户已确认）：**chat全能力（含会话内消息搜索）+ contacts（通讯录/好友申请/添加好友）**，即已设计UI且IM自有的表面
- 保持默认模块挂载不变（chat/notary/orders）；联调验证时用 `VITE_SDKWORK_IM_H5_MODULES=chat,contacts,notary,orders,drive` 开启
- **不改变任何页面UI设计与视觉效果**；占位页面（组织架构/Agent/频道/扫码/通话）保持fail-closed不动
- 模式：前端service类方法 → 调用SDK → 后端API；后端缺API/SDK缺方法 → 补后端 + 重新生成SDK；DB缺数据 → 补种子数据；反复迭代至端到端联调通过
- 严格遵循 sdkwork-specs（CODE_STYLE/NAMING/TYPESCRIPT_CODE/RUST_CODE/API_SPEC/SDK_SPEC/TEST_SPEC/APP_H5_ARCHITECTURE_SPEC 等，按progressive loading按需加载）

## 现状（已完成调研，事实依据）
- chat/contacts 大部分service已接真实SDK；**唯一明确缺口：ChatProfile页"会话内搜索"**——后端 `GET /im/v3/api/chat/messages/search`（services/sdkwork-comms-conversation-service/src/conversation_state/http.rs:336）已存在，路由crate的 paths.rs 也有 `MESSAGE_SEARCH` 常量，但 OpenAPI yaml（apis/open-api/im/sdkwork-im-im.openapi.yaml）与生成的SDK transport 均**不含该端点**（契约过期/物化遗漏）→ 需重新物化契约+重生成SDK
- 环境问题：① gateway启动失败——嵌入式knowledgebase App API compose时报 `Rust i64 与 SQL INT4 不兼容`（.standalone-dev.err.log:766），需诊断根因并修复（可能涉及IM侧迁移或sibling knowledgebase侧，按RTC/Group-KB边界最小修复）；② `.h5-vite.err.log` 显示生成的SDK transport dist缺失 → `pnpm sdk:ensure:im-generated-transport`
- 数据库：63张 im_* 表（迁移0001-0012齐全），但 **seeds 为空占位**（common/001_bootstrap.sql 仅 `SELECT 1;`），无任何演示数据

## 执行步骤

### 阶段0：环境基线修复
1. 诊断INT8/INT4根因：定位gateway compose时执行的知识库启动查询与对应表（psql只读检查表结构/列类型），对齐schema（优先IM侧迁移修复；若根因在sibling sdkwork-knowledgebase，报告并最小修复）
2. `pnpm sdk:ensure:im-generated-transport` 补齐SDK构建产物
3. 验证基线：`pnpm db:postgres:init` + `pnpm db:postgres:migrate` → 启动gateway → `/healthz` 通过 → H5 dev server可启动

### 阶段1：数据库种子数据（database/seeds/，幂等可重复执行）
1. 演示账号：按CLI文档约定（owner/Owner#2026、guest/Guest#2026）+ 若干同事账号，通过IAM注册API创建（幂等脚本；若IAM表与IM表同库则种子SQL可直接引用，执行时确认）
2. IM表演示数据种子SQL：im_conversations / im_conversation_members / im_conversation_messages / im_message_media_refs / im_friend_requests / im_friendships / im_user_profiles / im_user_settings / im_contact_tags 等，覆盖chat+contacts全部功能的数据能力（含搜索可命中的消息、收藏、好友申请待处理等），注册到 seed.manifest.json 并遵循 database/README.md 机制
3. 种子数据与表schema对齐（按 baseline DDL 的ID/类型/约束），`pnpm db:seed` 验证可重复执行

### 阶段2：消息搜索 契约→SDK→前端接线（本轮核心SDK补齐闭环）
1. 重新物化OpenAPI：`pnpm api:assembly:materialize`，确认 `/im/v3/api/chat/messages/search` 进入 yaml；若物化仍缺失 → 调查 sdkwork-im-openapi 生成链路（解析范围）并修复，或调整路由注册位置
2. 重新生成TS SDK：`node sdks/sdkwork-im-sdk/bin/generate-sdk.mjs --language typescript` + 构建transport
3. 扩展 imSdkClient/ChatSdkPort：暴露消息搜索方法
4. ChatService.searchChatHistory 真实实现（后端search_messages handler参数对齐：conversation_id + q + cursor分页，映射回 Message[]，遵循现有assertCursorPage模式）
5. 补充单元测试（沿用 node:test + mock SDK 现有模式，参照 ChatService.test.ts）

### 阶段3：chat/contacts 已接线能力端到端核验与收尾
- 逐页核验chat：ChatList/ChatDetail/ChatProfile/CreateGroupChat/GlobalSearch 的全部已接线方法（列表/详情/发送/媒体/收藏/置顶/免打扰/已读/全局搜索/建群），修复联调中发现的前后端契约不符问题（不改UI）
- 逐页核验contacts：AddressBook/AddFriend/NewFriends（搜索用户/好友申请/接受/拒绝/通讯录/发起单聊）
- ChatService其余未接线方法（clearChatHistory/joinOrCreateGroupChat等）UI未调用 → 保持现状，不新增后端

### 阶段4：端到端浏览器验证
- 启动完整standalone环境（PostgreSQL + gateway 18079 + H5 dev + 种子数据）
- 浏览器自动化（browser-use）：登录演示账号 → 走查 chat+contacts 完整用户流程 → 截图核验UI与设计一致（无UI改动）
- `pnpm typecheck` + 全部单元测试通过

### 阶段5：回归与报告
- `pnpm check`（含 db:contract:check、manifest校验等窄面检查）、typecheck、测试全绿
- 输出联调报告：各阶段验证命令与结果、契约/SDK/DB变更清单

## 风险与处理
- OpenAPI物化可能仍不覆盖services目录下的路由 → 执行时验证并修复生成链路
- INT8/INT4根因可能在sibling knowledgebase仓库 → 按知识库边界最小修复并报告
- 演示账号ID与IM种子数据关联方式 → 执行阶段0后确认IAM表同库情况再定（同库则SQL子查询引用，异库则脚本动态注入）
- 不提交git（除非用户另行要求）