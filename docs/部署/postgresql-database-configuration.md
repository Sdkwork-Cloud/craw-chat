# PostgreSQL Database Configuration

This page is the stable index for SDKWork Chat database configuration.

Canonical application identity:

- App code: `chat`
- Product/package namespace: `sdkwork-chat`
- Public SDKWork route: `/sdkwork/chat`
- Standard environment prefix: `SDKWORK_IM_*`
- Linux server config: `/etc/sdkwork/chat/chat.toml`
- Linux PostgreSQL password file: `/etc/sdkwork/chat/database.secret`
- Linux PostgreSQL helper config: `/etc/sdkwork/chat/postgresql.yaml`
- Desktop SQLite file: `~/.sdkwork/chat/data/chat.sqlite`

Server, container, browser development, and desktop development orchestration
default to PostgreSQL. Installed desktop deployments and desktop local runtime
user data keep SQLite by default.

## Environment-specific guides

- [Ubuntu与WSL-PostgreSQL初始化建库授权手册](./Ubuntu与WSL-PostgreSQL初始化建库授权手册.md)
  - Copy-paste workflow for Ubuntu and WSL Ubuntu.
  - Covers PostgreSQL installation, `sdkwork_ai_dev` database/schema/user creation, grants, and migration SQL execution.
  - Covers `listen_addresses`, `pg_hba.conf`, UFW, WSL2 NAT, mirrored networking, and `netsh interface portproxy`.
  - Supports repository-owned `pnpm db:postgres:plan`, `pnpm db:postgres:init`, and `pnpm db:postgres:migrate` commands after `.env.postgres` is configured.

- [开发环境PostgreSQL数据库配置教程](./开发环境PostgreSQL数据库配置教程.md)
  - Local developer PostgreSQL workflow.
  - Uses `.env.postgres` copied from `.env.postgres.example`.
  - Uses split `SDKWORK_IM_DATABASE_*` host/engine/ssl fields plus unified `SDKWORK_CLAW_DATABASE_*` workspace identity (`NAME`, `SCHEMA`, `USERNAME`, `PASSWORD`), and `SDKWORK_CLAW_DATABASE_ADMIN_PASSWORD` for bootstrap.
  - `pnpm dev` and `pnpm dev:browser` use PostgreSQL for integrated browser/server development.
  - `pnpm dev:desktop` uses PostgreSQL for standalone desktop development orchestration.
  - Installed desktop runtime local user data remains SQLite.
  - `pnpm dev:browser:sqlite` and `pnpm dev:desktop:sqlite` are explicit SQLite development entries.

- [线上环境PostgreSQL数据库配置教程](./线上环境PostgreSQL数据库配置教程.md)
  - Production and private deployment workflow.
  - Uses `/etc/sdkwork/chat/chat.toml`, `/etc/sdkwork/chat/server.env`, `/etc/sdkwork/chat/postgresql.yaml`, and `/etc/sdkwork/chat/database.secret`.
  - Keeps passwords out of Git, package archives, process command lines, and logs.
  - Uses service lifecycle scripts or the target service manager, including Windows Service, not local dev commands.

## Standard rules

Do not reuse one configuration file for both local development and production.

Use these standard names for new config and docs:

```env
SDKWORK_IM_DATABASE_ENGINE=postgresql
SDKWORK_IM_DATABASE_HOST=127.0.0.1
SDKWORK_IM_DATABASE_PORT=5432
SDKWORK_CLAW_DATABASE_NAME=sdkwork_ai_dev
SDKWORK_CLAW_DATABASE_SCHEMA=sdkwork_ai_dev
SDKWORK_CLAW_DATABASE_USERNAME=sdkwork_ai_dev
SDKWORK_CLAW_DATABASE_PASSWORD=sdkworkdev123
SDKWORK_IM_DATABASE_SSL_MODE=disable
SDKWORK_IM_DATABASE_MAX_CONNECTIONS=10
```

`SDKWORK_IM_DATABASE_PROVIDER` and `SDKWORK_IM_DATABASE_SSLMODE` are not standard names. Use `SDKWORK_IM_DATABASE_ENGINE` and `SDKWORK_IM_DATABASE_SSL_MODE`.
