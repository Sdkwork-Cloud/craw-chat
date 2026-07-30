# 线上环境 PostgreSQL 数据库配置教程

本文说明 SDKWork Chat server 在线上环境如何配置 PostgreSQL。生产环境不得使用仓库根目录的 `.env.postgres`，不得把数据库密码提交到 Git、安装包、进程命令行或日志。

## 应用与数据库标识

- app code: `chat`
- public route: `/sdkwork/chat`
- package name: `sdkwork-chat`
- workspace database: `sdkwork_ai_prod`
- workspace schema: `sdkwork_ai_prod`
- runtime username: `sdkwork_ai_prod`

同一 SDKWork 部署中的模块共用上述 database 和 schema。模块只拥有自己的表、索引、约束、seed 和 migration，不创建私有 database 或 schema。

## 1. 运行时目录

Linux server/service/container 使用：

```text
/opt/sdkwork/chat                 # archive install root
/etc/sdkwork/chat                 # config and secret root
/var/lib/sdkwork/chat             # durable data
/var/log/sdkwork/chat             # file logs
/run/sdkwork/chat                 # runtime state
```

核心配置文件：

```text
/etc/sdkwork/chat/
  chat.toml
  server.env
  postgresql.yaml
  database.secret
  redis.secret
```

Windows Service 使用 `%ProgramFiles%/sdkwork/chat` 与 `%ProgramData%/sdkwork/chat`。macOS service 使用 `/usr/lib/sdkwork/chat`、`/Library/Application Support/sdkwork/chat` 和 `/Library/Logs/sdkwork/chat`。

## 2. 创建工作区数据库与账号

以下 SQL 由 DBA 或部署管理员在受控窗口执行：

```sql
CREATE ROLE sdkwork_ai_prod LOGIN PASSWORD 'replace-with-generated-password';
CREATE DATABASE sdkwork_ai_prod OWNER sdkwork_ai_prod;
\connect sdkwork_ai_prod

CREATE SCHEMA IF NOT EXISTS sdkwork_ai_prod AUTHORIZATION sdkwork_ai_prod;
GRANT CONNECT ON DATABASE sdkwork_ai_prod TO sdkwork_ai_prod;
GRANT USAGE, CREATE ON SCHEMA sdkwork_ai_prod TO sdkwork_ai_prod;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA sdkwork_ai_prod TO sdkwork_ai_prod;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA sdkwork_ai_prod TO sdkwork_ai_prod;

ALTER DEFAULT PRIVILEGES IN SCHEMA sdkwork_ai_prod
  GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO sdkwork_ai_prod;
ALTER DEFAULT PRIVILEGES IN SCHEMA sdkwork_ai_prod
  GRANT USAGE, SELECT, UPDATE ON SEQUENCES TO sdkwork_ai_prod;
ALTER ROLE sdkwork_ai_prod SET search_path TO sdkwork_ai_prod, public;
```

生产运行账号不使用 PostgreSQL 超级用户。数据库所有者、migrator 和运行账号需要分离时，由部署平台按最小权限创建角色，但 database、schema 和 runtime username 的 workspace identity 不得改成应用或模块名。

## 3. chat.toml

`chat.toml` 是 server 运行时主配置入口：

```toml
[runtime]
environment = "production"
deployment_profile = "standalone"
runtime_target = "server"
app_code = "chat"

[server]
bind_address = "0.0.0.0:18079"
trust_forwarded_headers = true

[paths]
config_directory = "/etc/sdkwork/chat"
data_directory = "/var/lib/sdkwork/chat"
log_directory = "/var/log/sdkwork/chat"
runtime_directory = "/run/sdkwork/chat"

[database]
engine = "postgresql"
host = "postgres.internal.example.com"
port = 5432
database = "sdkwork_ai_prod"
schema = "sdkwork_ai_prod"
username = "sdkwork_ai_prod"
password_file = "/etc/sdkwork/chat/database.secret"
ssl_mode = "require"
max_connections = 20
```

## 4. server.env

数据库字段只使用 `SDKWORK_DATABASE_*`。`SDKWORK_IM_*` 仅用于 IM 应用自身配置。

```env
SDKWORK_IM_DEPLOYMENT_PROFILE=standalone
SDKWORK_IM_RUNTIME_TARGET=server
SDKWORK_IM_CONFIG_FILE=/etc/sdkwork/chat/chat.toml
SDKWORK_IM_DATA_DIR=/var/lib/sdkwork/chat
SDKWORK_IM_LOG_DIR=/var/log/sdkwork/chat
SDKWORK_IM_RUN_DIR=/run/sdkwork/chat

SDKWORK_DATABASE_ENGINE=postgresql
SDKWORK_DATABASE_HOST=postgres.internal.example.com
SDKWORK_DATABASE_PORT=5432
SDKWORK_DATABASE_NAME=sdkwork_ai_prod
SDKWORK_DATABASE_SCHEMA=sdkwork_ai_prod
SDKWORK_DATABASE_USERNAME=sdkwork_ai_prod
SDKWORK_DATABASE_PASSWORD_FILE=/etc/sdkwork/chat/database.secret
SDKWORK_DATABASE_SSL_MODE=require
SDKWORK_DATABASE_MAX_CONNECTIONS=20
SDKWORK_DATABASE_MIN_CONNECTIONS=5
SDKWORK_DATABASE_ACQUIRE_TIMEOUT=10
SDKWORK_DATABASE_IDLE_TIMEOUT=300
SDKWORK_DATABASE_MAX_LIFETIME=1800

# 仅在托管平台必须提供完整 DSN 时使用显式覆盖：
# SDKWORK_DATABASE_URL=postgresql://sdkwork_ai_prod@postgres.internal.example.com:5432/sdkwork_ai_prod?sslmode=require
```

启动器必须拒绝任何应用或模块前缀的数据库键，不做双读或运行时桥接。

## 5. postgresql.yaml

`postgresql.yaml` 供安装和运维自动化读取结构化连接配置：

```yaml
provider: postgresql

connection:
  host: postgres.internal.example.com
  port: 5432
  database: sdkwork_ai_prod
  username: sdkwork_ai_prod
  passwordFile: /etc/sdkwork/chat/database.secret
  sslmode: require
  applicationName: sdkwork-chat-server
  connectTimeoutSeconds: 10

schema:
  name: sdkwork_ai_prod
  provisioningMode: none
  migrationMode: apply
  expectedVersion: latest

pool:
  minConnections: 5
  maxConnections: 20
  idleTimeoutSeconds: 300
  maxLifetimeSeconds: 1800
```

## 6. 密钥文件

`database.secret` 只包含密码本身，不含键名、引号或 URL：

```text
replace-with-generated-password
```

Linux 权限示例：

```bash
sudo chown root:sdkwork /etc/sdkwork/chat/database.secret
sudo chmod 0640 /etc/sdkwork/chat/database.secret
```

## 7. 连通性与迁移验证

在受控终端中验证连接，不把密码写进历史记录：

```bash
PGPASSWORD="$(cat /etc/sdkwork/chat/database.secret)" \
  psql "host=postgres.internal.example.com port=5432 dbname=sdkwork_ai_prod user=sdkwork_ai_prod sslmode=require" \
  -c 'SHOW search_path;'
```

`search_path` 必须以 `sdkwork_ai_prod` 开头。发布环境通过 `database/` 生命周期模块和 `sdkwork-database-cli` 执行 migration，不直接运行未登记的 SQL 文件。

Windows Service 应通过服务账号可读的受限 secret 文件或操作系统 secret provider 注入密码，并使用安装包内的 `init-config-server.ps1`、`init-storage-server.ps1` 和 `verify-server.ps1` 完成初始化与验证。

## 8. 备份与恢复

- 在 migration 前完成可恢复备份并记录恢复点。
- 备份必须覆盖 `sdkwork_ai_prod` database、角色授权和 schema 元数据。
- 恢复演练必须在隔离环境验证 migration version、表数量、关键约束和租户隔离查询。
- 禁止把生产 dump、密码或真实用户数据放入仓库和安装包。
