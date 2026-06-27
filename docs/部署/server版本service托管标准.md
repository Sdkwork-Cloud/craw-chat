# Server 版本 Service 托管标准

`sdkwork-im-server` 向 operator 暴露一个正统 service 身份。应用目录、配置、日志和运行状态统一使用 app code `chat`。

## 跨平台目标

- Linux: `systemd`
- macOS: `launchd`
- Windows: Windows Service `SdkworkImServer`

统一托管启动命令：

```text
sdkwork-im-server --config <config-root>/chat.toml
```

## 标准路径

Linux：

- Install root: `/opt/sdkwork/chat`
- Config root: `/etc/sdkwork/chat`
- Config file: `/etc/sdkwork/chat/chat.toml`
- Env file: `/etc/sdkwork/chat/server.env`
- Data root: `/var/lib/sdkwork/chat`
- Log root: `/var/log/sdkwork/chat`
- Run root: `/run/sdkwork/chat`

Windows Service：

- Install root: `%ProgramFiles%/sdkwork/chat`
- Config root: `%ProgramData%/sdkwork/chat`
- Config file: `%ProgramData%/sdkwork/chat/chat.toml`
- Env file: `%ProgramData%/sdkwork/chat/server.env`
- Data root: `%ProgramData%/sdkwork/chat/Data`
- Log root: `%ProgramData%/sdkwork/chat/Logs`
- Run root: `%ProgramData%/sdkwork/chat/Run`

macOS service：

- Install root: `/usr/lib/sdkwork/chat`
- Config root: `/Library/Application Support/sdkwork/chat`
- Config file: `/Library/Application Support/sdkwork/chat/chat.toml`
- Env file: `/Library/Application Support/sdkwork/chat/server.env`
- Data root: `/Library/Application Support/sdkwork/chat/Data`
- Log root: `/Library/Logs/sdkwork/chat`
- Run root: `/Library/Application Support/sdkwork/chat/Run`

## Service templates

- systemd template: `deployments/systemd/sdkwork-im-server.service`
- launchd template: `deployments/launchd/com.sdkwork.SdkworkIm.server.plist`
- Windows Service wrapper template: `deployments/windows-service/SdkworkImServer.xml`
- launchd label: `com.sdkwork.SdkworkIm.server`
- Windows Service name: `SdkworkImServer`

`install-service-server` 在 `<config-root>/generated/` 下渲染实例化文件。

## Runtime contract

- Foreground start、systemd、launchd、Windows Service 必须共享同一 `chat.toml` 配置源。
- `SDKWORK_IM_CONFIG_FILE` 可覆盖配置路径。
- `SDKWORK_IM_LOG_DIR` 必须指向平台 log root。
- Server deployments default to PostgreSQL and Redis。
- Desktop deployments default to SQLite and Redis disabled。
- Browser-visible variables must not expose database URLs、Redis URLs 或 password files。

## Unified Gateway Contract

- 对外服务入口仍为 `sdkwork-im-cloud-gateway`。
- 标准 operator 端点：`/healthz`、`/readyz`、`/openapi.json`、`/openapi/index.json`、`/docs`。
- 上游服务 schema 代理：`/openapi/services/<service-id>.openapi.json`。
- 渲染文档：`/docs/services/<service-id>`。

## 相关文档

- [server版本安装与初始化.md](./server版本安装与初始化.md)
- [源码部署.md](./源码部署.md)
