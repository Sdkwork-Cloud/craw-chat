## sdkwork-im 原生安装包打包能力（商业化就绪）

参考 sdkwork-cloudrouter 的 native installer 体系，为 sdkwork-im 补齐 Ubuntu/Windows 原生安装包能力，覆盖 standalone 服务端与桌面客户端，严格落 RUNTIME_DIRECTORY_SPEC §4/§6、PACKAGING_SPEC §5.2/§5.3/§5.5、DEPLOYMENT_SPEC §5 的目录与内容规范。

### 现状（已具备，不再重复建设）
- 归档包管线已存在：18 包矩阵（browser/server/desktop×平台×架构）、stage/package/validate/sign/sbom/evidence、commercial-readiness 门禁
- 应用码 `chat` 已对齐：安装根 `/opt/sdkwork/chat`、`%ProgramFiles%/sdkwork/chat`；systemd/WinSW/launchd 模板、config 模板齐备
- 桌面端 Tauri `bundle.targets: "all"` 已产出原生安装器，collect 脚本已收集

### 实施步骤

**1. 原生安装包构建器（新建 `scripts/release/build-sdkwork-im-native-installer.mjs`，镜像 cloudrouter）**
- `.deb`（Ubuntu 服务端）：手写 ar+ustar（跨平台确定性输出，mtime=0）；control（`Depends: libssl3, ca-certificates` 等，不捆绑发行版库）、`postinst`（创建 sdkwork 用户/组、目录树 `/etc|/var/lib|/var/log|/var/cache|/run/sdkwork/chat`（0750 root:sdkwork）、写 `/etc/sdkwork/chat/server.env` 占位（无秘密）、daemon-reload+enable 不启动）、`prerm`（stop+disable）
- 数据映射（PACKAGING_SPEC §5.5）：bin→`/usr/lib/sdkwork/chat/bin`（+/usr/bin 链接）、web→`/usr/share/sdkwork/chat/web`、INSTALL.md→`/usr/share/doc/sdkwork/chat`、install-manifest→`/usr/share/sdkwork/chat`、unit→`/usr/lib/systemd/system/sdkwork-chat.service`、config 模板→`/etc/sdkwork/chat/*.example`
- systemd unit 构建时生成：`sdkwork-chat.service`、`EnvironmentFile=/etc/sdkwork/chat/server.env`、`User=sdkwork`、StateDirectory/加固
- `.msi`（Windows 服务端）：WiX CLI 生成 .wxs（ProgramFiles64Folder+CommonAppDataFolder 树、固定 `WINDOWS_UPGRADE_CODE`、MajorUpgrade、EmbedCab、perMachine）；服务经 WinSW.exe（固定版本+sha256 下载）以 `ServiceInstall` 注册 `sdkwork-chat`
- `.pkg`（macOS 服务端）：pkgbuild 生成（仅 macOS host/CI）
- `native-install-layout.v1` 清单注入 install-manifest；快照缓存；SOURCE_DATE_EPOCH 确定性

**2. 矩阵/清单/工作流扩展**
- `plan-sdkwork-im-install-packages.mjs` 新增原生包 id：`linux-ubuntu-{x64,arm64}-standalone-server-deb`、`windows-{x64,arm64}-standalone-server-msi`、`macos-{x64,arm64}-standalone-server-pkg`、桌面端 `linux-ubuntu-*-standalone-desktop-{deb,appimage}`、`windows-*-standalone-desktop-{msi,exe}`（GITHUB_WORKFLOW_SPEC §5 命名：Linux 原生带发行版段）
- `stage-sdkwork-im-release-package.mjs`：原生包输入（deb 版 unit、WinSW xml、env 模板）
- `sdkwork.app.config.json` 新增条目（保持 enabled:false / releaseBuildDeferred:true，本次不发布）；`sdkwork.workflow.json` 新增原生 targets（ubuntu-latest / windows-latest 等 runner，复用现有 lifecycle 阶段）
- pnpm 脚本（PNPM_SCRIPT_SPEC 合规）：`install:native:build`、`install:native:check`、`release:native:package`、`release:native:package:check`

**3. 验证门禁扩展**：`validate-sdkwork-im-install-artifacts.mjs` 增加 .deb（ar+tar 字节级解析、必含条目、mode/owner）、.msi 结构校验；接入 `check-package-content-standard.mjs` 内容规范检查

**4. 文档**：`docs/部署/` 新增《Ubuntu deb 安装指南》《Windows MSI 安装指南》（安装/升级/卸载/回滚、目录规范引用）、增强生成的 INSTALL.md

**5. 端到端本机验证**
- Ubuntu（WSL 主证据）：构建 deb → 停止 docker IM 组合（数据卷保留）→ `apt install ./sdkwork-chat-*.deb` → 校验目录/权限/unit → 配 server.env 指向现有 docker postgres → `systemctl start sdkwork-chat` → healthz 200 + testapidocker 域名登录验证 → 卸载清理验证
- Windows：检查 wix/msvc toolchain（缺失则 `dotnet tool install -g wix`）→ 构建 MSI → 字节级校验（不本机安装）
- 桌面端：管线接入 + 校验脚本测试（Tauri 完整构建由 CI 承载）

**6. 记录项（不修改源码，仅记录）**：tauri identifier `com.sdkwork.chatpc` 与 app.config `com.sdkwork.chat.desktop` 不一致（后续统一）；`SDKWORK_IM_*` 与 ENVIRONMENT_SPEC §4 `SDKWORK_CHAT_*` 的既有差异不迁移；桌面端本地网关监督属产品特性，不在本次范围

### 边界
- 不触发 GitHub 发布、不改 enabled 状态、不提交签名密钥
- 不改变现有归档包/容器打包行为；网关源码零改动
- 桌面 Tauri 构建不在本机执行（耗时长），仅接入与校验

### 关键风险
- MSI 本地构建依赖 wix + windows-msvc 目标工具链（实施时检查，不可用则记录并留 CI 承载）
- deb 安装测试短暂接管 18079（docker 组合可随时恢复，数据卷保留）