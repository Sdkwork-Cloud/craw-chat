# 部署文档

Topology v2 是唯一的部署标准。权威来源：

- `specs/topology.spec.json`
- `configs/topology/*.env`
- [docs/topology-greenfield.md](../topology-greenfield.md)

## 开发入口

```bash
pnpm install
pnpm dev              # standalone.unified-process.development
pnpm dev:browser      # PostgreSQL + standalone browser dev
pnpm dev:desktop      # PostgreSQL + standalone desktop dev
pnpm dev:server          # 仅服务端
```

默认 application ingress：`http://127.0.0.1:18079`

## 生产安装

- [源码部署](./源码部署.md)
- [server版本安装与初始化](./server版本安装与初始化.md)
- [server版本配置与PostgreSQL接入](./server版本配置与PostgreSQL接入.md)
- [server版本service托管标准](./server版本service托管标准.md)

## 数据库

- [PostgreSQL 配置索引](./postgresql-database-configuration.md)
- [Ubuntu与WSL-PostgreSQL初始化建库授权手册](./Ubuntu与WSL-PostgreSQL初始化建库授权手册.md)
- [开发环境PostgreSQL数据库配置教程](./开发环境PostgreSQL数据库配置教程.md)
- [线上环境PostgreSQL数据库配置教程](./线上环境PostgreSQL数据库配置教程.md)

## 验证与矩阵

- [CLI聊天验证与兼容矩阵](./CLI聊天验证与兼容矩阵.md)
- [兼容矩阵与SDK-CLI-operator验证索引](./兼容矩阵与SDK-CLI-operator验证索引.md)

## 性能与灾备

- [性能与灾备演练场景](./性能与灾备演练场景.md)
- Step 11 catalog：`tools/perf/step-11-scenario-catalog.json`

## Release

- [Release bundle 归档约定](../../artifacts/releases/README.md)

## 已退役

旧版 profile、compose 与本地 lifecycle 脚本已删除，详见 [topology-greenfield.md](../topology-greenfield.md)。

## 验证命令

与根 [README.md](../../README.md) 一致。完整标准门禁：

```bash
pnpm verify
```

部署与数据库专项：

```bash
pnpm test:deployment-docs-encoding
pnpm test:governed-docs-encoding
pnpm test:postgresql-ubuntu-wsl-guide
pnpm test:postgresql-pnpm-db-command
pnpm test:database-framework-standard
pnpm test:database-naming-standard
```

拓扑、运行时与商用门禁抽样：

```bash
pnpm test:topology-baggage
pnpm test:runtime-standard
pnpm test:workflow-commercial-gates
pnpm test:sdkwork-im-pc-dev-command
cargo test -p sdkwork-im-cli --test chat_cli_contract_test
node sdks/test/verify-im-v3-sdk-family-contract.test.mjs
node scripts/dev/sdkwork-im-rtc-signaling-boundary.test.mjs
```
