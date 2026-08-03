## 目标

将 sdkwork-im 现有一份**私有**的设备自适应(PC/H5)开发引导脚本，升级为 `@sdkwork/app-topology`(即 `sdkwork-app dev` 框架)的**原生通用能力**：仓库只需在 `topology.spec.json` 声明，`pnpm dev` 启动时框架自动拉起 PC/H5 renderer + UA 自适应入口(含互相 fallback)；sdkwork-specs 补齐标准与校验器，保证 sdkwork-space 下所有 pc+h5 应用可统一采纳。sdkwork-im 作为参考落地。

当前状态：sdkwork-im 已有完整实现(`scripts/dev/run-sdkwork-im-adaptive-web-dev.mjs` + `scripts/lib/im-web-client-routing.mjs`，入口 :3801 / PC :4176 / H5 :4178，UA 检测 + Vary + fallback + API/WS 代理)，但它是 IM 专用、框架不感知；`@sdkwork/app-topology` 的 plan-v5 只校验 `browserDeliveries` 不实现；sdkwork-specs 仅有生产 nginx 侧标准(SDKWORK_DEPLOY_SPEC §8 Adaptive Web)，开发侧标准缺失。~22 个兄弟仓库有 pc+h5 但均无此能力(已全仓检索确认)。

## 1. sdkwork-app-topology(通用框架核心)

**1.1 新增 `tools/topology/lib/adaptive-web.mjs`**(全通用、零仓库私有字符串)：
- `detectWebDeviceClass({userAgent, secChUaMobile, overrides, tablet})` → `mobile|desktop`，检测顺序对齐 SDKWORK_DEPLOY_SPEC §8：overrides → `Sec-CH-UA-Mobile: ?1` → 移动 UA 正则(§8 的 `Mobile|Android|iPhone|iPod|webOS|BlackBerry|IEMobile|Opera Mini|MicroMessenger|HuaweiBrowser|HarmonyOS|UCBrowser|Quark`) → 默认 desktop；iPad 默认 pc、可声明覆盖为 h5
- `webClientFallbackOrder(deviceClass, clientArchitectures)` → mobile `[h5,pc]` / desktop `[pc,h5]`
- `resolveAvailableWebClient(...)`、`matchCanonicalApiPath(pathname)`(泛化 IM 的 `isCanonicalImApiPath`)
- `createAdaptiveWebServer({delivery, renderers, apiTarget})` — 移植泛化 IM 服务：UA 路由、`Vary: user-agent`、首选→备用 renderer fallback(GET/HEAD 且备用就绪)、API 路径与 WS upgrade 代理到 apiTarget、vite 依赖缓存 410、502 文案
- `spawnWebRenderer`/`waitForWebRenderer` — 经 `spawnLifecycleCommand` 启动(纳入开发会话管理)、按架构 UA 探测就绪(120s 超时)
- `startAdaptiveWebDelivery({runtime, plan, delivery, env})` — 编排：按 `delivery.renderers` 声明启动各 renderer(环境 = profile env + host/port(portEnv/defaultPort) + `surface.clientHttpEnv/clientWebsocketEnv` ← delivery.browserVisibleOrigin + renderer 私有 env)，就绪后启动入口 server，返回可关闭句柄

**1.2 `tools/topology/lib/plan-v5.mjs`**：`resolveBrowserDeliveries` 解析可选 `delivery.renderers`(架构 → `{applicationRoot, command/args 或 script, defaultPort, portEnv, hostEnv, userAgent, env}`)，输出 `renderers[]` + `adaptive` 标记

**1.3 `tools/topology/lib/spec-v5.mjs`**：校验 `renderers` 形状(applicationRoot 安全相对路径、portEnv/hostEnv 大写 env key、架构 ⊆ delivery.clientArchitectures、去重)；被 adaptive delivery 覆盖的 client process 的 `script` 变为可选

**1.4 `specs/topology.schema.v5.json`**：browserDeliveries item 增加可选 `renderers` 属性

**1.5 `scripts/sdkwork-app.mjs` `runGenericDevelopment`**：识别 plan 中 adaptive delivery(dev-server-proxy + renderers)，跳过被覆盖 client process 的直接启动；health 检查通过后框架自行启动 renderers + 自适应入口(renderer 子进程纳入会话 childPids、信号时清理)；"无本地进程"守卫考虑 adaptive delivery；`_sdkwork:dev:*` 钩子保留为逃生通道

**1.6 `tools/topology/lib/index.mjs`** 导出新模块；新增 `tests/adaptive-web.test.mjs`(检测逻辑含 Sec-CH-UA-Mobile/iPad、fallback、API 路径；server 集成测试移植 IM 用例)、扩展 `topology-v5.test.mjs`(renderers 校验/解析)

## 2. sdkwork-specs(标准补齐 — 用户要求"假如sdkwork-specs有问题，应该更新")

**2.1 `APP_RUNTIME_TOPOLOGY_SPEC.md`** 新增 §8.2「Adaptive Browser Delivery」(开发侧标准)：声明契约(renderers)、共享检测契约(引用 SDKWORK_DEPLOY_SPEC §8)、fallback 语义、`Vary: user-agent`、canonical API 路径保留、同源 env 注入、单 renderer collapse 模式、访问地址规则
**2.2 `SDKWORK_DEPLOY_SPEC.md` §8**：补充开发侧镜像实现与共享检测契约引用
**2.3 `README.md`** §2/§3 相关行目的补充 adaptive browser delivery
**2.4 新增 `tools/check-adaptive-web-standard.mjs`**：声明 pc-web+h5 的仓库必须声明覆盖双架构的 adaptive browser delivery(renderers)，standalone.development 必须 dev-server-proxy，校验 env/域契约；接入 `sdkwork-app doctor`(与现有两个 checker 同机制)

## 3. sdkwork-im(参考落地)

**3.1 `specs/topology.spec.json`**：`im-adaptive-web` delivery 增加 `renderers`(pc-web → apps/sdkwork-im-pc，node `scripts/dev/run-vite-cli.mjs --host 127.0.0.1 --port {port} --strictPort`，defaultPort 4176，portEnv `SDKWORK_IM_PC_INTERNAL_DEV_PORT`；h5 → apps/sdkwork-im-h5，4178，`SDKWORK_IM_H5_INTERNAL_DEV_PORT`)；cloud.development 同步补 `browserDeliveries`；`im-browser` 进程去掉 `script`(保留 bindEnv/applicationRoot 供 accessEndpoints)
**3.2 `package.json`**：删除 `_sdkwork:client:browser:standalone|cloud` 及失效的 `_sdkwork:client:h5:*`(保留 apps/sdkwork-im-pc 的 `_sdkwork:client:browser` 供桌面开发)
**3.3 删除** `run-sdkwork-im-adaptive-web-dev.mjs`、`im-web-client-routing.mjs` 及两个对应测试(逻辑已归框架)
**3.4 更新 `sdkwork-im-web-domain-routing-standard.test.mjs`**：改断言新契约(声明式 delivery + renderers、:3801 域契约、无根 `_sdkwork:client:browser:*` 钩子)
**3.5 新增 `sdkwork-im-adaptive-web-topology.test.mjs`**：经 `@sdkwork/app-topology` runtime 解析 plan → adaptive delivery 存在、renderers/端口/入口 origin(3801)/apiTarget(standalone 18079、cloud api-dev.sdkwork.com)、UA 路由符合 §8 契约

## 4. 验证

- sdkwork-app-topology：`pnpm test`、`pnpm check`
- sdkwork-specs：checker 自测 + README/规范一致性
- sdkwork-im：受影响测试(`sdkwork-im-web-domain-routing-standard.test.mjs`、新 topology 测试、`sdkwork-im-pc-dev-command.test.mjs`)、`topology:validate`、`topology:plan`；最后 `pnpm dev` 实跑冒烟(gateway + 自适应入口，UA 探针验证 PC→pc / 手机→h5 / 关掉一个 renderer 验证 fallback)

## 注意点(刻意对齐)

- iPad UA 由 IM 现状(h5)改为规范 §8 默认(pc)，可通过声明 override 恢复 — 这是"对齐 sdkwork-specs"的明确结果
- 桌面开发路径(`im-pc-dev.mjs`、apps/sdkwork-im-pc 的 `_sdkwork:client:browser`)不受影响
- 其他仓库不强制本次迁移：标准 + doctor 校验器保证后续增量采纳(校验器对无 pc+h5 仓库空过)