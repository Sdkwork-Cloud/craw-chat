> Migrated from `docs/部署/Ubuntu与WSL-PostgreSQL初始化建库授权手册.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Ubuntu 与 WSL PostgreSQL 初始化建库授权手册

本文用于在 Ubuntu 或 WSL Ubuntu 中为 SDKWork Chat 准备本地 PostgreSQL。当前应用 app code 为 `chat`，开发数据库使用 `sdkwork_ai_dev`，生产示例使用 `sdkwork`。

desktop 开发编排默认使用 PostgreSQL：`pnpm dev:desktop 默认使用 PostgreSQL`。安装后的 desktop runtime 本地数据默认保留 SQLite，文件位于 `~/.sdkwork/chat/data/chat.sqlite`。本文的 PostgreSQL profile 用于 `pnpm dev`、`pnpm dev:browser`、`pnpm dev:desktop`、server 集成测试和数据库迁移验证。

## 1. 安装 PostgreSQL

```bash
sudo apt update
sudo apt install -y postgresql postgresql-contrib
sudo systemctl enable --now postgresql
sudo systemctl status postgresql
```

Redis 本地开发基线：

```bash
sudo apt install -y redis-server
sudo service redis-server start
redis-cli ping
```

`redis-cli ping` 应返回：

```text
PONG
```

进入管理控制台：

```bash
sudo -u postgres psql
```

## 2. 创建本地开发库、schema 和账号

```sql
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'sdkwork_ai_dev') THEN
    CREATE ROLE sdkwork_ai_dev LOGIN PASSWORD 'sdkworkdev123';
  ELSE
    ALTER ROLE sdkwork_ai_dev WITH LOGIN PASSWORD 'sdkworkdev123';
  END IF;
END
$$;

SELECT 'CREATE DATABASE sdkwork_ai_dev OWNER sdkwork_ai_dev'
WHERE NOT EXISTS (
  SELECT 1 FROM pg_database WHERE datname = 'sdkwork_ai_dev'
)\gexec

\connect sdkwork_ai_dev

CREATE SCHEMA IF NOT EXISTS sdkwork_ai_dev AUTHORIZATION sdkwork_ai_dev;

GRANT CONNECT ON DATABASE sdkwork_ai_dev TO sdkwork_ai_dev;
GRANT TEMPORARY ON DATABASE sdkwork_ai_dev TO sdkwork_ai_dev;
GRANT USAGE, CREATE ON SCHEMA sdkwork_ai_dev TO sdkwork_ai_dev;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA sdkwork_ai_dev TO sdkwork_ai_dev;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA sdkwork_ai_dev TO sdkwork_ai_dev;
GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA sdkwork_ai_dev TO sdkwork_ai_dev;

ALTER DEFAULT PRIVILEGES IN SCHEMA sdkwork_ai_dev
  GRANT ALL PRIVILEGES ON TABLES TO sdkwork_ai_dev;
ALTER DEFAULT PRIVILEGES IN SCHEMA sdkwork_ai_dev
  GRANT ALL PRIVILEGES ON SEQUENCES TO sdkwork_ai_dev;
ALTER DEFAULT PRIVILEGES IN SCHEMA sdkwork_ai_dev
  GRANT ALL PRIVILEGES ON FUNCTIONS TO sdkwork_ai_dev;

ALTER ROLE sdkwork_ai_dev SET search_path TO sdkwork_ai_dev, public;
```

## 3. 验证本地连接

```bash
PGPASSWORD=sdkworkdev123 \
psql -h 127.0.0.1 -p 5432 -U sdkwork_ai_dev -d sdkwork_ai_dev \
  -c "SELECT current_database(), current_user, current_schema();"
```

预期输出包含：

```text
SELECT current_database(), current_user, current_schema()
current_database = sdkwork_ai_dev
current_user = sdkwork_ai_dev
current_schema = sdkwork_ai_dev
```

Windows 跑应用，PostgreSQL 跑在 WSL Ubuntu 时，Windows 应用不应该使用 WSL 内部的 Unix socket。Windows PowerShell 访问 WSL PostgreSQL 时必须走 TCP：

```powershell
Test-NetConnection 127.0.0.1 -Port 5432
wsl hostname -I
```

## 4. 配置 .env.postgres

仓库根目录复制模板：

```powershell
Copy-Item .env.postgres.example .env.postgres
```

标准字段：

```env
SDKWORK_IM_DEPLOYMENT_MODE=server
SDKWORK_IM_DATABASE_ENGINE=postgresql
SDKWORK_IM_DATABASE_HOST=127.0.0.1
SDKWORK_IM_DATABASE_PORT=5432
SDKWORK_CLAW_DATABASE_NAME=sdkwork_ai_dev
SDKWORK_CLAW_DATABASE_SCHEMA=sdkwork_ai_dev
SDKWORK_CLAW_DATABASE_USERNAME=sdkwork_ai_dev
SDKWORK_CLAW_DATABASE_PASSWORD=sdkworkdev123
SDKWORK_IM_DATABASE_SSL_MODE=disable
SDKWORK_IM_DATABASE_MAX_CONNECTIONS=10

SDKWORK_IM_REDIS_ENABLED=true
SDKWORK_IM_REDIS_HOST=127.0.0.1
SDKWORK_IM_REDIS_PORT=6379
SDKWORK_IM_REDIS_DATABASE=0
SDKWORK_IM_REDIS_KEY_PREFIX=chat
SDKWORK_IM_REDIS_TLS=false

SDKWORK_CLAW_DATABASE_ADMIN_HOST=127.0.0.1
SDKWORK_CLAW_DATABASE_ADMIN_PORT=5432
SDKWORK_CLAW_DATABASE_ADMIN_USERNAME=postgres
SDKWORK_CLAW_DATABASE_ADMIN_PASSWORD=postgres_admin_pass
SDKWORK_CLAW_DATABASE_ADMIN_DATABASE=postgres
SDKWORK_CLAW_DATABASE_ADMIN_SSL_MODE=disable
```

## 5. 使用 pnpm 初始化和迁移

查看计划：

```powershell
pnpm db:postgres:plan
```

初始化 role、database、schema、grant 和默认权限：

```powershell
pnpm db:postgres:init
```

应用迁移：

```powershell
pnpm db:postgres:migrate
```

规范 DDL 基线：

```text
database/ddl/baseline/postgres/0001_im_legacy_baseline.sql
```

## 6. 开发启动入口

```powershell
pnpm dev
pnpm dev:browser
pnpm dev:desktop
pnpm dev:browser:sqlite
pnpm dev:desktop:sqlite
```

显式 PostgreSQL desktop 诊断：

```powershell
pnpm dev:desktop:postgres
```

## 7. 允许局域网访问

默认 PostgreSQL 通常只监听本机。如果需要局域网机器访问，需要同时修改 `postgresql.conf`、`pg_hba.conf` 和系统防火墙。

在 `postgresql.conf` 中检查：

```conf
listen_addresses = '*'
```

在 `pg_hba.conf` 中添加受控网段，示例认证方式使用 `scram-sha-256`：

```conf
host    sdkwork_ai_dev    sdkwork_ai_dev    127.0.0.1/32           scram-sha-256
host    sdkwork_ai_dev    postgres          127.0.0.1/32           scram-sha-256
host    sdkwork_ai_dev    sdkwork_ai_dev    192.168.1.0/24         scram-sha-256
host    sdkwork_ai_dev    postgres          192.168.1.0/24         scram-sha-256
host    sdkwork_ai_dev    sdkwork_ai_dev    <WINDOWS_HOST_CIDR>    scram-sha-256
host    sdkwork_ai_dev    postgres          <WINDOWS_HOST_CIDR>    scram-sha-256
```

打开防火墙：

```bash
sudo ufw allow from 192.168.1.0/24 to any port 5432 proto tcp
sudo systemctl restart postgresql
```

WSL2 NAT 场景可能需要 Windows 端口转发：

```powershell
netsh interface portproxy add v4tov4 listenaddress=0.0.0.0 listenport=5432 connectaddress=<WSL_IP> connectport=5432
```

WSL mirrored 网络模式下，优先测试 `127.0.0.1`。如果无法访问，再使用 `wsl hostname -I` 获取 WSL IP。

## 8. 手工 smoke

```sql
CREATE TABLE IF NOT EXISTS sdkwork_ai_dev.__manual_smoke_check (
  id bigserial PRIMARY KEY,
  note text NOT NULL
);
INSERT INTO sdkwork_ai_dev.__manual_smoke_check(note) VALUES ('ok');
SELECT * FROM sdkwork_ai_dev.__manual_smoke_check ORDER BY id DESC LIMIT 1;
DROP TABLE sdkwork_ai_dev.__manual_smoke_check;
```

如果 `pnpm db:postgres:migrate` 报 `permission denied for schema sdkwork_ai_dev`，重新执行第 2 节中的 grant 与 `ALTER DEFAULT PRIVILEGES`。

## 9. 生产配置参考

生产 server 不使用 `.env.postgres`。使用 `/etc/sdkwork/chat/chat.toml`、`/etc/sdkwork/chat/server.env`、`/etc/sdkwork/chat/postgresql.yaml`、`/etc/sdkwork/chat/database.secret`。

`postgresql.yaml` 生产字段示例：

```yaml
provider: postgresql

connection:
  host: postgres.internal.example.com
  port: 5432
  database: sdkwork
  username: sdkwork
  passwordFile: /etc/sdkwork/chat/database.secret
  sslmode: require

schema:
  name: sdkwork
```

安全要求：

- 密码不要提交 Git。
- 线上环境使用 `/etc/sdkwork/chat/database.secret` 或平台密钥管理系统。
- Server 与 container 默认 PostgreSQL。
- Desktop 默认 SQLite，除非用户显式配置外部数据库。

