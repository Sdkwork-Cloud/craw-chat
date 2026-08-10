# H5「我的 → 知识库」CRUD 实现与集成方案

## 背景（探索结论）
- H5 入口/路由/模块/i18n 壳已全部就绪（`MeFeaturesSection` → `/workspace/knowledge`，routeCatalog 5 条路由，knowledgeModule 懒加载 5 个组件），但 5 个页面是 `CapabilityUnavailablePage` 占位、`KnowledgeBaseService` 全 throw。
- 真实页面实现在 sibling 仓库 `sdkwork-knowledgebase/apps/sdkwork-knowledgebase-common/packages/sdkwork-knowledgebase-mobile-react-knowledge`（组件层已预置：KnowledgeBaseCard/BatchActionBar/HeaderFilter/EmptyState/DocumentCard）。
- `@sdkwork/knowledgebase-app-sdk` 生成客户端已具备：spaces create/retrieve/update/delete + members/browser；documents list/create/retrieve/update/delete + content(只读)；ingests.create（携带 payloadMarkdown 的异步内容写入通道）。
- 约束（已与你确认）：① 无终端用户列表接口 → **本地注册表 + 并行服务端校验同步**（PC 同构方案）；② 无归档写入端点 → **移除归档 UI，保留删除/重命名**（服务端返回 archived 状态时如实展示）。

## Part A：sdkwork-knowledgebase 仓库（页面与服务真实实现）
1. **`src/services/KnowledgeBaseService.ts` 重写**
   - Runtime 注入模式（镜像 drive 的 `CloudDriveRuntime`）：`KnowledgeBaseRuntime { client: SdkworkKnowledgebaseAppClient; resolveScopeKey? }` + `configureKnowledgeBaseRuntime()` / `resetKnowledgeBaseRuntime()`，未注入时保持 fail-closed（`KnowledgeBaseCapabilityUnavailableError`）。
   - 本地注册表（镜像 PC `knowledgebaseSpaceRegistry`）：key `sdkwork.knowledgebase.spaces.v1.h5.{scopeKey}`，条目 `{ spaceId, icon?, color?, createdAt, lastOpenedAt? }`，localStorage 安全读写。
   - 视图模型映射：server `KnowledgeSpace` ↔ mobile `KnowledgeBase`（icon/color 来自注册表、isArchived=status==='archived'）。
   - 方法：`getKnowledgeBases()`（注册表→并行 `spaces.retrieve` 同步：成功更新名称/状态、404/已删除剪枝、网络错误保留本地条目）；`getKnowledgeBase`；`createKnowledgeBase`（create+注册表 upsert）；`updateKnowledgeBase`（PATCH name/description）；`deleteKnowledgeBase(s)`（delete+注册表移除）；`getDocumentsByKbId`/`getAllDocuments`（documents.list 分页）；`getDocument`（retrieve+content 只读）；`createDocument`（documents.create + ingests.create 携带 markdown，crypto.randomUUID 幂等键）；`updateDocument`（仅元数据 title）；`deleteDocument`。移除 archive/unarchive 方法。
2. **5 个页面真实实现**（Tailwind + @sdkwork/ui-mobile-react PageLayout/Dialog/ActionSheet/Toast 等既有组件，参考 MyAgentsPage/FavoritesPage 模式）
   - `KnowledgeBaseApp`（列表 `/workspace/knowledge`）：加载/错误/空态；搜索+筛选（all/newest/oldest/recently_updated 客户端排序）；卡片菜单 ActionSheet（打开/重命名/删除）；重命名 Dialog；删除确认 Dialog；批量选择+批量删除（BatchActionBar）；新建按钮。
   - `CreateKnowledgeBase`（`/workspace/knowledge/create`）：name 必填 + description + 预设 emoji 图标与颜色选择（本地元数据入注册表）→ 创建后返回列表刷新。
   - `KnowledgeBaseDocumentList`（知识库详情 `/workspace/knowledge/:id`）：KB 头部（名称/描述+菜单：重命名/删除）；文档列表分页+客户端搜索；文档卡片菜单（打开/删除）；「新建文档」→ doc/create；删除知识库确认后回列表。
   - `CreateDocument`（`/workspace/knowledge/:id/doc/create`）：title 必填 + markdown 内容 → create+ingest，提示内容异步同步。
   - `KnowledgeBaseDetail`（文档详情 `/workspace/knowledge/:kbId/doc/:id`）：标题+内容渲染（contentState=draft 显示"内容同步中"）；重命名标题、删除文档；不存在/失败态。
3. **组件微调**（对齐服务端能力 + 全量 i18n）：HeaderFilter 移除"已归档"chip、BatchActionBar 移除归档/取消归档按钮、EmptyKnowledgeBaseState 移除归档分支、DocumentCard 对 content/category/author 缺失优雅降级；硬编码中文文案全部收口为 i18n key。
4. **package.json**：+ `@sdkwork/knowledgebase-app-sdk: workspace:*`。
5. **测试**：重写 `KnowledgeBaseService.test.ts`（保留 fail-closed 用例；新增 stub client 成功路径：CRUD 映射、注册表剪枝、分页、ingest 调用）。

## Part B：sdkwork-im 仓库（集成进 sdkwork-im-h5）
1. **SDK client 装配**（全程镜像 drive/order 既有模式，禁止裸 HTTP）：
   - `apps/sdkwork-im-h5/package.json` + `@sdkwork/knowledgebase-app-sdk: workspace:*`；`vite.config.ts` + `@sdkwork/knowledgebase-app-sdk` alias → sibling src。
   - 新增 `packages/sdkwork-im-h5-core/src/sdk/knowledgebaseAppSdkClient.ts`（baseUrl 解析 `SDKWORK_KNOWLEDGEBASE_APP_API_BASE_URL`/`VITE_*`，默认 `/`；create/init/get/reset），`sdk/index.ts` 导出。
   - `src/bootstrap/environment.ts` + `knowledgebaseAppApiBaseUrl`；`src/bootstrap/sdkClients.ts` init+composition+reset；`src/bootstrap/index.ts` `configureKnowledgeBaseRuntime({ client, resolveScopeKey })`（scopeKey 取当前用户 id，作注册表隔离键）+ resetH5Bootstrap 重置。
2. **i18n**：`sdkwork-im-h5-commons/src/locales/{zh,en}/knowledge.json` 补齐路由 titleKey（create_title/detail_title/doc_create/doc_detail）与全部新增文案。
3. **集成契约测试**：`apps/sdkwork-im-h5/scripts/knowledgebase-app-sdk-integration-contract.test.mjs`（镜像 drive 版）+ 根 package.json `test:h5-knowledgebase-app-sdk-integration`。

## Part C：验证
- knowledgebase 仓库：运行 `KnowledgeBaseService.test.ts`（node:test）。
- sdkwork-im：`pnpm install`（更新 lock）、`pnpm test:h5-knowledgebase-app-sdk-integration`、H5 typecheck（tsc）。

## 边界与说明
- 不修改生成 SDK / 契约 / 认证 / 部署配置；不碰 orders.json 等无关改动。
- 已知 API 限制如实呈现：列表仅含本设备打开/创建过的知识库；文档内容写入为异步 ingest 管道；文档内容编辑无写入端点，本次仅支持元数据（标题）编辑。