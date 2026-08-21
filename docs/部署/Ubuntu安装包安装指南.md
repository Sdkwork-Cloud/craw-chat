# Ubuntu .deb 安装指南（standalone 服务端）

原生安装包面向 Ubuntu/Debian 发行版，将 standalone 服务端按
[RUNTIME_DIRECTORY_SPEC](../../../sdkwork-specs/RUNTIME_DIRECTORY_SPEC.md) §4.1
与 [PACKAGING_SPEC](../../../sdkwork-specs/PACKAGING_SPEC.md) §5.5 的目录规范安装，
并注册 `sdkwork-im.service` systemd 服务。

## 1. 获取安装包

```bash
# 从发布渠道下载，例如：
wget https://cdn.sdkwork.com/apps/chat/STABLE/<version>/sdkwork-im-linux-ubuntu-x64-standalone-server-deb-<version>.deb
# 校验（发布契约要求 SHA-256 不可变校验和）
sha256sum sdkwork-im-linux-ubuntu-x64-standalone-server-deb-<version>.deb
```

构建方式（开发者/自托管）：

```bash
# 1) 构建 Linux release 二进制与 web 产物
pnpm release:build:prod -- --target server
# 2) stage 归档输入（原生包复用同一 staging 布局）
pnpm release:stage -- --package-id linux-x64-standalone-server-tar-gz --version <version>
# 3) 构建 .deb
pnpm install:native:build -- --package-id linux-ubuntu-x64-standalone-server-deb --version <version>
# 4) 字节级校验
node scripts/release/validate-sdkwork-im-install-artifacts.mjs \
  --package-id linux-ubuntu-x64-standalone-server-deb \
  --artifact-path dist/release-packages/sdkwork-im-linux-ubuntu-x64-standalone-server-deb-<version>.deb \
  --version <version> --json
```

## 2. 安装

```bash
sudo apt install ./sdkwork-im-linux-ubuntu-x64-standalone-server-deb-<version>.deb
```

postinst 脚本完成（不启动服务）：

- 创建系统账号 `sdkwork`（`/usr/sbin/nologin`）
- 创建目录树：`/usr/lib/sdkwork/im`（只读资产）、`/etc/sdkwork/im`（配置，
  `0750 root:sdkwork`）、`/etc/sdkwork/database`（工作区数据库配置）、
  `/var/lib|/var/log|/var/cache|/run/sdkwork/im`（数据/日志/缓存/运行时态，
  `0750 sdkwork:sdkwork`）
- 生成 `/etc/sdkwork/im/server.env`（进程环境，无秘密）与
  `/etc/sdkwork/database/database.secret`（占位 `change-me`，启动前必须替换）
- `systemctl daemon-reload && systemctl enable sdkwork-im.service`

## 3. 配置 PostgreSQL

```bash
sudo editor /etc/sdkwork/im/server.env
sudo editor /etc/sdkwork/database/database.secret
```

`server.env` 使用规范环境键（ENVIRONMENT_SPEC §7.1/§7.3）。占位值
（`127.0.0.1`/`sdkwork_ai_prod`/`change-me`）在真实配置前会被启动拒绝：

```text
SDKWORK_IM_DEPLOYMENT_PROFILE=standalone
SDKWORK_IM_RUNTIME_TARGET=server
SDKWORK_IM_ENVIRONMENT=production
SDKWORK_IM_PROFILE_ID=standalone.production
SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND=0.0.0.0:18079
SDKWORK_CORS_ALLOWED_ORIGINS=http://<你的域名>,http://localhost
SDKWORK_DATABASE_HOST=<postgres-host>
SDKWORK_DATABASE_NAME=sdkwork_ai_prod
SDKWORK_DATABASE_USERNAME=sdkwork_ai_prod
SDKWORK_DATABASE_PASSWORD_FILE=/etc/sdkwork/database/database.secret
SDKWORK_DATABASE_SSL_MODE=require
SDKWORK_DATABASE_AUTO_MIGRATE=true
```

数据库初始化（首次，一次性）：

```bash
sudo -u postgres psql <<'SQL'
CREATE USER sdkwork_ai_prod WITH PASSWORD '<strong-password>';
CREATE DATABASE sdkwork_ai_prod OWNER sdkwork_ai_prod;
SQL
# 密码写入 secret 文件（0600/0640，root:sdkwork）
sudo editor /etc/sdkwork/database/database.secret
```

## 4. 启动与验证

```bash
sudo systemctl start sdkwork-im
sudo systemctl status sdkwork-im --no-pager
sudo journalctl -u sdkwork-im -f

curl -fsS http://127.0.0.1:18079/healthz   # {"status":"ok"}
curl -fsS http://127.0.0.1:18079/readyz
```

## 5. 目录规范（安装后）

| 角色 | 路径 | 属主/权限 |
|---|---|---|
| 私有运行资产（二进制） | `/usr/lib/sdkwork/im/bin` | root:root 0755 |
| 共享只读资产（web） | `/usr/share/sdkwork/im/web` | root:root 0755 |
| 文档 | `/usr/share/doc/sdkwork/im/INSTALL.md` | root:root 0644 |
| 安装清单 | `/usr/share/sdkwork/im/install-manifest.json` | root:root 0644 |
| 运行时配置 | `/etc/sdkwork/im/`（`server.env`、`*.example`） | root:sdkwork 0750/0640 |
| 工作区数据库配置 | `/etc/sdkwork/database/`（`database.secret`） | root:sdkwork 0750/0640 |
| 持久数据 | `/var/lib/sdkwork/im` | sdkwork:sdkwork 0750 |
| 日志 | `/var/log/sdkwork/im` | sdkwork:sdkwork 0750 |
| 缓存 | `/var/cache/sdkwork/im` | sdkwork:sdkwork 0750 |
| 运行时态 | `/run/sdkwork/im` | sdkwork:sdkwork 0750 |
| 服务单元 | `/usr/lib/systemd/system/sdkwork-im.service` | root:root 0644 |

## 6. nginx 反向代理（可选）

```nginx
server {
  listen 80;
  server_name im.example.com;
  location / {
    proxy_pass http://127.0.0.1:18079;
    proxy_set_header Host localhost:18079;
    proxy_set_header X-Forwarded-Host $host;
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";
  }
}
```

## 7. 升级与回滚

```bash
# 升级：apt 直接覆盖（MajorUpgrade 语义；先备份数据库）
sudo apt install ./sdkwork-im-linux-ubuntu-x64-standalone-server-deb-<new-version>.deb
# 回滚：安装旧版本 deb（数据/配置位于 /etc 与 /var/lib，apt 不删除）
sudo apt install ./sdkwork-im-linux-ubuntu-x64-standalone-server-deb-<old-version>.deb
```

## 8. 卸载

```bash
sudo apt remove sdkwork-chat
```

prerm 会停止并禁用 `sdkwork-im.service`。`/var/lib/sdkwork/im` 数据与
`/etc/sdkwork/im` 配置按 dpkg conffile 规则保留；`sudo apt purge sdkwork-chat`
清理配置文件（数据目录需手动删除）。
