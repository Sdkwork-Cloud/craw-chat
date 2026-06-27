> Migrated from `docs/部署/server版本安装与初始化.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Server 版本安装与初始化

`sdkwork-im-server` 是 SDKWork Chat 的正统 server 安装入口。当前应用 app code 为 `chat`，发布包名是 `sdkwork-chat`，对外路径是 `/sdkwork/chat`。

统一启动入口：

```text
sdkwork-im-server --config <config-root>/chat.toml
```

## 命令面

- `install-server`
- `init-config-server`
- `init-storage-server`
- `verify-server`
- `plan-release-server`
- `install-service-server`
- `start-server`
- `stop-server`
- `restart-server`
- `status-server`

## 首次安装流程

1. 运行 `install-server`
2. 运行 `init-config-server`
3. 配置 PostgreSQL 与 Redis
4. 运行 `init-storage-server`
5. 运行 `verify-server`
6. 运行 `install-service-server`
7. 运行 `start-server`

PowerShell 示例：

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\install-server.ps1 -InstanceName default
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\init-config-server.ps1 -InstanceName default
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\init-storage-server.ps1 -InstanceName default -Mode verify-only
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\verify-server.ps1 -InstanceName default
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\install-service-server.ps1 -InstanceName default
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\start-server.ps1 -InstanceName default -Release
```

Bash 示例：

```bash
bash bin/install-server.sh --instance default
bash bin/init-config-server.sh --instance default
bash bin/init-storage-server.sh --instance default --mode verify-only
bash bin/verify-server.sh --instance default
bash bin/install-service-server.sh --instance default
bash bin/start-server.sh --instance default --release
```

## 标准路径矩阵

Linux archive：

- Install root: `/opt/sdkwork/chat`
- Config root: `/etc/sdkwork/chat`
- Config file: `/etc/sdkwork/chat/chat.toml`
- Env file: `/etc/sdkwork/chat/server.env`
- PostgreSQL helper config: `/etc/sdkwork/chat/postgresql.yaml`
- PostgreSQL password file: `/etc/sdkwork/chat/database.secret`
- Data root: `/var/lib/sdkwork/chat`
- Log root: `/var/log/sdkwork/chat`
- Run root: `/run/sdkwork/chat`

macOS service 与 Windows Service 路径矩阵见 [server版本service托管标准.md](./server版本service托管标准.md)。

Desktop user data 与 server data 分离。Desktop 默认 SQLite 位于 `~/.sdkwork/chat/data/chat.sqlite`。

## Release payload contract

Server archives must include `bin/sdkwork-im-server`、`config/*.example`、lifecycle scripts、service templates、`web/sdkwork-chat-pc/dist`、`INSTALL.md`、`install-manifest.json`。

Packages must not include secrets、`.env*`、本地 SQLite、generated runtime state、`node_modules` 或 Git metadata。

## Database contract

Server release packages default to PostgreSQL。Desktop packages default to SQLite at `~/.sdkwork/chat/data/chat.sqlite`。

数据库 schema 由 `database/` 生命周期模块管理，规范基线为 `database/ddl/baseline/postgres/0001_im_legacy_baseline.sql`。

## 相关文档

- [server版本配置与PostgreSQL接入.md](./server版本配置与PostgreSQL接入.md)
- [线上环境PostgreSQL数据库配置教程.md](./线上环境PostgreSQL数据库配置教程.md)

