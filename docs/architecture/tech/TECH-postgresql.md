> Migrated from `docs/部署/开发环境PostgreSQL数据库配置教程.md` on 2026-06-24.
> Owner: SDKWork maintainers

本文说明 SDKWork Chat 在本地开发环境中如何显式使用 PostgreSQL。

- app code: `chat`
- 发布访问根路径: `/sdkwork/chat`
- 环境变量前缀: `SDKWORK_IM_*`（连接字段可与统一工作区 `SDKWORK_CLAW_DATABASE_*` 别名并存）
- desktop 安装运行时本地数据默认数据库: SQLite
- desktop SQLite 路径: `~/.sdkwork/chat/data/chat.sqlite`

## 1. 本地启动入口

浏览器集成开发默认使用 PostgreSQL + standalone：

```powershell
pnpm dev
pnpm dev:browser
pnpm dev:browser:postgres
```

desktop 开发编排默认使用 PostgreSQL + standalone：

```powershell
pnpm dev:desktop
pnpm dev:desktop:postgres
```

`pnpm dev:desktop 默认使用 PostgreSQL`。安装后的 desktop runtime 本地用户数据仍默认使用 SQLite，数据库文件写入 `~/.sdkwork/chat/data/chat.sqlite`，Windows 等价路径为 `%USERPROFILE%\.sdkwork\chat\data\chat.sqlite`。

显式 SQLite 开发入口保留：

```powershell
pnpm dev:browser:sqlite
pnpm dev:desktop:sqlite
```

## 2. 配置文件位置

开发 PostgreSQL 使用仓库根目录 `.env.postgres`。仓库只提交模板 `.env.postgres.example`，本机真实配置不提交 Git。

创建本机配置：

```powershell
Copy-Item .env.postgres.example .env.postgres
```

## 3. 标准 PostgreSQL 配置

`.env.postgres` 使用拆分字段维护数据库连接，不把 host、database、username、password 混在一条长 URL 中。

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

SDKWORK_CLAW_DATABASE_ADMIN_HOST=127.0.0.1
SDKWORK_CLAW_DATABASE_ADMIN_PORT=5432
SDKWORK_CLAW_DATABASE_ADMIN_USERNAME=postgres
SDKWORK_CLAW_DATABASE_ADMIN_PASSWORD=postgres_admin_pass
SDKWORK_CLAW_DATABASE_ADMIN_DATABASE=postgres
SDKWORK_CLAW_DATABASE_ADMIN_SSL_MODE=disable
```

脚本会组装并桥接给当前 Rust 运行时所需的数据库 URL：

```text
postgresql://sdkwork_ai_dev:sdkworkdev123@127.0.0.1:5432/sdkwork_ai_dev?sslmode=disable
```

`DATABASE_PROVIDER` 与 `DATABASE_SSLMODE` 不是标准名称。新配置必须使用 `DATABASE_ENGINE` 与 `DATABASE_SSL_MODE`。

## 4. 初始化和迁移数据库

执行前确认本机已安装 PostgreSQL 客户端工具，命令行可以访问 `psql`：

```powershell
psql --version
```

查看计划，不连接或修改数据库：

```powershell
pnpm db:postgres:plan
```

初始化数据库、应用账号、schema、授权和默认权限：

```powershell
pnpm db:postgres:init
```

执行仓库 PostgreSQL 生命周期迁移（通过 `sdkwork-database-cli bootstrap`）：

```powershell
pnpm db:postgres:migrate
```

规范 DDL 基线位于：

```text
database/ddl/baseline/postgres/0001_im_baseline.sql
```

`pnpm db:postgres:migrate` 委托 `database/` 生命周期模块与 `sdkwork-database-cli bootstrap` 执行迁移，不再直接引用 `deployments/database/` 遗留目录。

脚本通过 `PGPASSWORD` 把密码传给 `psql`，不会把密码拼进命令行参数。`pnpm db:postgres:plan` 输出中的密码会被替换为 `***`。

## 5. Windows 应用访问 WSL Ubuntu PostgreSQL

如果 SDKWork Chat 的 `pnpm` 在 Windows PowerShell 中运行，而 PostgreSQL 安装在 WSL Ubuntu 中，`.env.postgres` 的 host 必须填写 Windows 能访问到的 TCP 地址。优先使用：

```env
SDKWORK_IM_DATABASE_HOST=127.0.0.1
```

验证端口：

```powershell
Test-NetConnection 127.0.0.1 -Port 5432
```

如果需要使用 WSL IP：

```powershell
wsl hostname -I
```

然后把 `SDKWORK_IM_DATABASE_HOST` 改成对应 IP。Windows 应用不应该使用 WSL 内部的 Unix socket。

## 6. 手工连接验证

```powershell
psql "host=127.0.0.1 port=5432 dbname=sdkwork_ai_dev user=sdkwork_ai_dev password=sdkworkdev123 sslmode=disable"
```

成功后可检查当前库、用户和 schema：

```sql
SELECT current_database(), current_user, current_schema();
```

## 7. 与 production 的边界

`.env.postgres` 仅用于本地开发和集成测试。server 发布包使用 `/etc/sdkwork/chat/chat.toml`、`/etc/sdkwork/chat/server.env`、`/etc/sdkwork/chat/postgresql.yaml` 与 `/etc/sdkwork/chat/database.secret`。desktop 发布包和安装运行时本地用户数据不依赖 PostgreSQL，默认 SQLite 数据文件为 `~/.sdkwork/chat/data/chat.sqlite`。

