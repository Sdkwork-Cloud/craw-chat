# sdkwork-manager 域名迁移与部署标准落地（admin.sdkwork.com）

## 一、标准注册（sdkwork-specs，2 项）

1. **`APP_RUNTIME_TOPOLOGY_NAMING.md` §9.2 注册 `sdkwork-manager` 行**（版本 5.5）：
   ```
   | sdkwork-manager | admin.sdkwork.com / admin-dev.sdkwork.com / admin-test.sdkwork.com / admin-staging.sdkwork.com | api.sdkwork.com / api-dev/api-test/api-staging.sdkwork.com |
   ```
2. **§9.2 Rules 声明**（admin 主域角色首次先例）：`admin` 作为独立产品主域角色（区别于 `*-admin.sdkwork.com` 辅助面形态）；`sdkwork-manager` 绑定 `admin` 角色主机、`applicationCode = manager`（引用 §9.1 显式注册优先条款）；crate 名 / `SDKWORK_MANAGER_*` 环境键前缀 / `/etc/sdkwork/manager` 运行时目录保持跟随 applicationCode（birdcoder 同款）

## 二、sdkwork-manager 落地（9 项）

1. **`specs/topology.spec.json`**：`cloudPublicHosts` → `admin.sdkwork.com` + environments 变体（admin-dev/-test/-staging + api-dev/-test/-staging）
2. **profileRoot 迁移**：`etc/deployments/` → `etc/topology/`（git mv 8 个 env + topology profileFiles 同步 + deployment.config 路径同步 + workflow 引用检查）
3. **域名迁移 manager-* → admin-***（5 处配置面）：
   - `etc/topology/cloud.{development,test,staging,production}.env`（PUBLIC_HTTP_URL + 3 组 CORS_ALLOWED_ORIGINS + VITE 键）
   - `etc/sdkwork.deployment.config.json`（四环境 applicationOrigin）
   - `deployments/deploy.yaml`（cloud.production expose domain）
   - `apps/sdkwork-manager-pc/.env.production.example`
   - `docs/runbooks/LAUNCH_READINESS.md:188`
4. **standalone.staging.env**：`manager.standalone.staging.invalid` 占位折叠为 `http://127.0.0.1:18092`（与 standalone.test/production 一致）
5. **deploy.yaml**：新增 `cloud.test`（admin-test.sdkwork.com）与 `cloud.staging`（admin-staging.sdkwork.com）profile（nginx driver 同构）；standalone.production 保持（expose 空，host-service driver）
6. **workflow.json**：新增 `deployments` 段（standalone-production-server → linux-x64-standalone-server-tar-gz + cloud-production-web → web-universal-cloud-browser-zip）+ `lifecycle.deploy` 步骤；框架 schema 校验通过
7. **域名路由标准测试新增**：`test:web-domain-routing-standard`——断言四环境 admin-xxx 应用面矩阵 + api-xxx 平台面、standalone 折叠无云域名、deploy.yaml expose ∈ 注册域集、无 manager.sdkwork.com 残留、无 .invalid 占位；挂载 package.json
8. **materialize 产物**：`pnpm workflow:materialize-client-env` 重新生成（pc 8 个 .env）
9. **既有测试失败修复**（2 个）：
   - `manager-payment-database-bootstrap-standard.test.mjs:40`：cloud profile 的 `SDKWORK_DATABASE_SEED_LOCALE` 断言过时（env 已含 zh-CN）→ 同步断言
   - `manager-architecture.test.mjs:18-20`：`tsconfig.base.json` 的 `@sdkwork/utils` paths 断言过时 → 同步断言

## 三、明确不动（报告中注明）

- `applicationCode = manager`：crate 名（sdkwork-api-manager-*）、env 键前缀（SDKWORK_MANAGER_*）、运行时目录（/etc/sdkwork/manager）不动（§9.1 显式注册条款 + §9.2 Rules 提供合法性）
- `api.*` 平台网关共享面（api.sdkwork.com / api-dev|-test|-staging）不动
- deploy.yaml standalone.production 的 expose 为空（host-service driver，管理台不暴露独立域名）保持
- 工作区无未提交改动（干净）

## 四、验证

- sdkwork-manager：`test:web-domain-routing-standard`（新增）、topology:validate、deploy:validate（cloud.test/staging/production + standalone）、check:client-env（重新 materialize）、check:pnpm-script-standard、check:agent-workflow-standard、workflow schema validate、`test:node`（2 个既有失败修复后全绿）
- sdkwork-specs：check-deploy-standard 回归（20/20）
- 回归：IM/cloudrouter/KB/birdcoder/drive/appstore 的 deploy:validate 与域名路由测试不受影响

## 五、注意点

- profileRoot 迁移（etc/deployments → etc/topology）涉及 git mv + topology profileFiles + deployment.config + workflow 引用四处同步
- admin 角色是独立产品主域首次先例，规范 Rules 声明需明确与 `*-admin.sdkwork.com` 辅助面形态的区分
- 未提交 commit，改动按仓库分别呈现