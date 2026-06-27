> Migrated from `docs/部署/server版本配置与PostgreSQL接入.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Server 版本配置与 PostgreSQL 接入

`sdkwork-im-server` 是 SDKWork Chat 的 server 安装入口。当前应用 identity 为 `chat`，发布包名为 `sdkwork-chat`，对外路径为 `/sdkwork/chat`。

Server 与 container 默认使用 PostgreSQL。Desktop 本地用户数据默认使用 SQLite，文件位于 `~/.sdkwork/chat/data/chat.sqlite`。

完整的线上 PostgreSQL 配置教程见 [线上环境PostgreSQL数据库配置教程](./线上环境PostgreSQL数据库配置教程.md)。本文只补充 server 侧 `postgresql.yaml` 外部配置文件契约。

## 标准路径

Linux server/service/container：

```text
/etc/sdkwork/chat/chat.toml
/etc/sdkwork/chat/server.env
/etc/sdkwork/chat/postgresql.yaml
/etc/sdkwork/chat/database.secret
/var/lib/sdkwork/chat
/var/log/sdkwork/chat
/run/sdkwork/chat
```

Windows Service 使用 `%ProgramFiles%/sdkwork/chat` 与 `%ProgramData%/sdkwork/chat`。

## postgresql.yaml 外部配置契约

`postgresql.yaml` 是 server 侧 PostgreSQL 安装模式的外部配置文件。它冻结连接密钥引用 `passwordFile`、schema 生命周期字段 `migrationMode`，以及生命周期模式 `verify-only`、`bootstrap-schema`、`create-db-and-schema`。

```yaml
provider: postgresql

connection:
  host: postgres.internal.example.com
  port: 5432
  database: sdkwork
  username: sdkwork
  passwordFile: /etc/sdkwork/chat/database.secret
  sslmode: require
  applicationName: sdkwork-chat-server
  connectTimeoutSeconds: 10

schema:
  name: sdkwork
  provisioningMode: none
  migrationMode: apply
  expectedVersion: latest

pool:
  minConnections: 5
  maxConnections: 20
  idleTimeoutSeconds: 300
  maxLifetimeSeconds: 1800
```

## 生命周期模式

- `verify-only`：数据库、账号、schema 已存在，只校验配置与连通性
- `bootstrap-schema`：PostgreSQL 已存在，应用账号可写，初始化 schema
- `create-db-and-schema`：管理员权限下创建数据库与 schema

`init-storage-server` 使用 `/etc/sdkwork/chat/postgresql.yaml` 与平台等价路径；密码来自 `database.secret`，不得写入 Git 或安装包。

## server.env 最小字段

生产 server 环境变量前缀为 `SDKWORK_IM_*`：

```env
SDKWORK_IM_DEPLOYMENT_MODE=server
SDKWORK_IM_CONFIG_FILE=/etc/sdkwork/chat/chat.toml
SDKWORK_IM_DATA_DIR=/var/lib/sdkwork/chat
SDKWORK_IM_LOG_DIR=/var/log/sdkwork/chat
SDKWORK_IM_RUN_DIR=/run/sdkwork/chat
SDKWORK_IM_DATABASE_ENGINE=postgresql
SDKWORK_IM_DATABASE_HOST=postgres.internal.example.com
SDKWORK_IM_DATABASE_PORT=5432
SDKWORK_CLAW_DATABASE_NAME=sdkwork_ai_prod
SDKWORK_CLAW_DATABASE_SCHEMA=sdkwork_ai_prod
SDKWORK_CLAW_DATABASE_USERNAME=sdkwork_ai_prod
SDKWORK_IM_DATABASE_PASSWORD_FILE=/etc/sdkwork/chat/database.secret
SDKWORK_IM_DATABASE_SSL_MODE=require
```

`DATABASE_PROVIDER` 与 `DATABASE_SSLMODE` 不是标准名称。请使用 `SDKWORK_IM_DATABASE_ENGINE` 与 `SDKWORK_IM_DATABASE_SSL_MODE`。

## 数据库迁移

Server 发布环境通过 `database/` 生命周期模块与 `sdkwork-database-cli` 执行 schema bootstrap，不再直接引用 `deployments/database/` 遗留目录。规范 DDL 基线为 `database/ddl/baseline/postgres/0001_im_legacy_baseline.sql`。

