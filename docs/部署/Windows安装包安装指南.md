# Windows .msi 安装指南（standalone 服务端）

原生安装包面向 Windows x64/arm64（arm64 经 x64 模拟运行 WinSW 包装器），按
[RUNTIME_DIRECTORY_SPEC](../../../sdkwork-specs/RUNTIME_DIRECTORY_SPEC.md) §4.4
与 [PACKAGING_SPEC](../../../sdkwork-specs/PACKAGING_SPEC.md) §5.5 的目录规范安装，
并通过固定校验和的 WinSW 包装器注册 Windows 服务 `sdkwork-chat`。

## 1. 获取安装包

```powershell
# 从发布渠道下载，例如：
Invoke-WebRequest https://cdn.sdkwork.com/apps/chat/STABLE/<version>/sdkwork-im-windows-x64-standalone-server-msi-<version>.msi
# 校验（发布契约要求 SHA-256 不可变校验和）
Get-FileHash sdkwork-im-windows-x64-standalone-server-msi-<version>.msi -Algorithm SHA256
```

构建方式（开发者/CI，需 Windows host + WiX）：

```powershell
pnpm release:build:prod -- --target server          # 构建 windows-msvc 二进制与 web 产物
pnpm release:stage -- --package-id windows-x64-standalone-server-zip --version <version>
pnpm install:native:build -- --package-id windows-x64-standalone-server-msi --version <version>
```

## 2. 安装

```powershell
msiexec /i sdkwork-im-windows-x64-standalone-server-msi-<version>.msi
# 静默安装：
msiexec /i sdkwork-im-windows-x64-standalone-server-msi-<version>.msi /qn
```

MSI 是 perMachine（`Scope="perMachine"`），需要管理员权限；固定 UpgradeCode 保证
同产品升级（MajorUpgrade）会替换旧版本。安装完成即注册并启动
`sdkwork-chat` 服务（WinSW 包装器读取同目录 `sdkwork-chat-service.xml`）。

## 3. 目录规范（安装后）

| 角色 | 路径 |
|---|---|
| 私有运行资产（二进制/服务包装器） | `%ProgramFiles%\sdkwork\chat\bin`、`...\service\windows` |
| 共享只读资产（web） | `%ProgramFiles%\sdkwork\chat\web` |
| 文档 | `%ProgramFiles%\sdkwork\chat\doc\INSTALL.md` |
| 安装清单 | `%ProgramFiles%\sdkwork\chat\install-manifest.json` |
| 运行时配置 | `%ProgramData%\sdkwork\chat\`（`server.env`、`*.example`） |
| 持久数据 | `%ProgramData%\sdkwork\chat\Data` |
| 日志 | `%ProgramData%\sdkwork\chat\Logs` |
| 缓存 | `%ProgramData%\sdkwork\chat\Cache` |
| 运行时态 | `%ProgramData%\sdkwork\chat\Run` |
| 服务 | SCM 服务名 `sdkwork-chat` |

## 4. 配置 PostgreSQL

```powershell
# 编辑 %ProgramData%\sdkwork\chat\server.env 与 %ProgramData%\sdkwork\chat\server.env
notepad "$env:ProgramData\sdkwork\chat\server.env"
```

占位值替换为真实 PostgreSQL 连接（数据库名/用户名必须匹配统一的
`SDKWORK_DATABASE_*` 工作区身份；密码放入 `server.env` 引用的 secret 文件或
环境变量，切勿写入配置模板）：

```text
SDKWORK_IM_DEPLOYMENT_PROFILE=standalone
SDKWORK_IM_RUNTIME_TARGET=server
SDKWORK_IM_ENVIRONMENT=production
SDKWORK_IM_PROFILE_ID=standalone.production
SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND=0.0.0.0:18079
SDKWORK_DATABASE_ENGINE=postgresql
SDKWORK_DATABASE_HOST=<postgres-host>
SDKWORK_DATABASE_PORT=5432
SDKWORK_DATABASE_NAME=sdkwork_ai_prod
SDKWORK_DATABASE_SCHEMA=sdkwork_ai_prod
SDKWORK_DATABASE_USERNAME=sdkwork_ai_prod
SDKWORK_DATABASE_PASSWORD=<在受限的 server.env 中配置>
SDKWORK_DATABASE_SSL_MODE=require
SDKWORK_DATABASE_AUTO_MIGRATE=true
```

## 5. 服务管理

```powershell
sc query sdkwork-chat          # 状态
sc stop sdkwork-chat           # 停止
sc start sdkwork-chat          # 启动
Get-Content "$env:ProgramData\sdkwork\chat\Logs\*.log" -Tail 100 -Wait   # 日志
# 服务包装器配置文件：
#   %ProgramFiles%\sdkwork\chat\service\windows\sdkwork-chat-service.xml
```

## 6. 验证

```powershell
curl.exe -fsS http://127.0.0.1:18079/healthz   # {"status":"ok"}
curl.exe -fsS http://127.0.0.1:18079/readyz
```

## 7. 升级与卸载

```powershell
# 升级：直接安装新版本 MSI（MajorUpgrade 替换，服务停止/重启由安装器处理）
msiexec /i sdkwork-im-windows-x64-standalone-server-msi-<new-version>.msi /qn
# 卸载：停止并移除 sdkwork-chat 服务（数据与配置保留在 %ProgramData%）
msiexec /x sdkwork-im-windows-x64-standalone-server-msi-<version>.msi /qn
```
