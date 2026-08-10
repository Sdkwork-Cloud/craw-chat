## 我的声音 CRUD：数据库审计结论 + 实施计划

### 审计结论（回答"为什么不用 audioAssets"）
`audioAssets` 落点是 `voice_audio_artifact` 表 —— 它是**生成任务产物表**（kind: audio/transcript/translation/sfx/music/image/video，uk(task_id, artifact_index)），**无 user_id/tenant_id 列**（租户过滤靠 JOIN voice_generation_task），`q: scope:mine` 是纯客户端假约定。行业专业设计（ElevenLabs/讯飞智作/腾讯智影）中"我的声音"是**独立声音档案实体**（名称、描述、试听样本、克隆状态、voice 模型引用）。当前库无此实体 → **新增 `voice_profile` 表**承载「我的声音」CRUD。

### Part 1 — sdkwork-voice 数据库
- 新增 `voice_profile`：id, profile_no(uk), tenant_id, organization_id, user_id, name, description, kind(cloned/uploaded/preset), status(training/ready/failed/disabled), voice_id, provider_code, sample_media_json(Drive 引用), duration_seconds, created_at, updated_at, deleted, version；索引 (tenant_id, user_id, deleted, created_at)
- 落点：sqlx migration 源 `0001_voice_core.sql` → 重生成 baseline → `pnpm db:materialize:contract` 重生成 database/contract
- ⚠️ DDL 变更（Human Review 项，本计划即审批）

### Part 2 — sdkwork-voice 后端 app-api（前缀 /app/v3/api/voice/voice_profiles）
- 新增 5 操作：`voiceProfiles.list`（服务端分页 page/page_size/sort/q，按 tenant+user 过滤）/ `.retrieve` / `.create`（name/description/kind/sampleMedia）/ `.update`（name/description）/ `.delete`
- Rust：routes-voice-app-api（manifest.rs/routes.rs/handlers.rs）+ service.rs（5 处理器+dispatch+序列化）+ ports.rs（record+trait）+ generation-repository-sqlx/store.rs（SQL）
- 权限复用现成映射：list/retrieve→voice.tasks.read，create/update/delete→voice.tasks.write
- 音频文件上传走 Drive（H5 已有 uploadAudio 先例），库中存 Drive 引用

### Part 3 — SDK 重生成
- `pnpm api:materialize` → OpenAPI + sdkgen.yaml → sdkwork-sdk-generator/bin/sdkgen.js → `@sdkwork/voice-app-sdk` 新增 `voice.voiceProfiles.*`

### Part 4 — sdkwork-voice 新建 H5 包 `apps/sdkwork-voice-common/packages/sdkwork-voice-mobile-my-voices`
- 服务层 `myVoiceService.ts`（注入 client 端口，对齐 voice-pc-core/sdkPorts 模式；含 Drive 预览授权 URL）
- 页面：MyVoicesPage（卡片列表+预览播放+时长/创建时间+空态+骨架屏+服务端分页+长按 ActionSheet 编辑/删除）、CreateVoicePage（录音/上传→Drive 上传进度→试听→命名描述→成功态）、MyVoiceDetailPage（播放器/信息/重命名/删除确认 Dialog）
- i18n zh/en + 组件测试

### Part 5 — 集成 sdkwork-im-h5
1. pnpm-workspace.yaml 挂载新包；`@sdkwork/im-h5-ai-voice` 适配器 re-export（并更新其 component.spec.json）
2. im-h5-core/src/sdk/voiceAppSdkClient.ts（init/get/reset 单例，对齐 driveAppSdkClient.ts）+ bootstrap environment.ts 新增 voiceAppApiBaseUrl（VITE_SDKWORK_VOICE_APP_API_BASE_URL，回退 gateway）+ sdkClients.ts 注入
3. userModule.tsx 三个路由（/me/voices*）改指向新组件；移除 im-h5-user mock 三页（VoiceService 保留，VoiceSelectionPage 仍用）
4. 补齐 route catalog 缺失的 user.voices.* i18n keys

### Part 6 — 验证（逐层收窄）
- voice：cargo test --workspace、pnpm typecheck、check-api-envelope、check-pagination.mjs、db:validate + db:materialize:contract
- im-h5：pnpm typecheck、test:workspace、check-application-layering.mjs、check-app-sdk-consumer-imports.mjs、check-frontend-composition.mjs
- 环境可用则 dev 起服 + 浏览器截图验证

### Human Review 触发点（计划内显式列出）
新表 DDL、API authority 扩展（5 新操作）、生成 SDK 重生成、H5 路由替换 mock 页面