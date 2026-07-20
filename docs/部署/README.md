# SDKWork IM 部署文档

Topology v5 是当前唯一运行拓扑标准。权威来源：

- `specs/topology.spec.json`
- `etc/sdkwork.deployment.config.json`
- `etc/topology/*.env`
- `deployments/deploy.yaml`（生产部署 manifest v2）
- [发布契约](../releases/README.md)

## 开发入口

```bash
pnpm install --frozen-lockfile
pnpm dev
pnpm dev:standalone
pnpm dev:cloud
pnpm stop
```

`pnpm dev` 与 `pnpm dev:standalone` 等价。`standalone.development` 启动一个应用
standalone gateway 和选定客户端；`cloud.development` 只启动选定客户端，并连接已部署的
`platform.api-gateway`。cloud development 不启动本地 gateway、API、数据库、Redis、
migration、seed 或部署态 worker。

standalone 默认 application ingress 是 `http://127.0.0.1:18079`。

## 生产安装

- [源码部署](./源码部署.md)
- [server 版本安装与初始化](./server版本安装与初始化.md)
- [server 版本配置与 PostgreSQL 接入](./server版本配置与PostgreSQL接入.md)
- [server 版本 service 托管标准](./server版本service托管标准.md)

生产 side effect 必须显式选择 profile、environment、artifact id、digest、artifact evidence、
approval 和 rollback target。本文档中的验证命令不会执行 apply 或 rollback。

## 数据库

- [PostgreSQL 配置索引](./postgresql-database-configuration.md)
- [Ubuntu 与 WSL PostgreSQL 初始化](./Ubuntu与WSL-PostgreSQL初始化建库授权手册.md)
- [开发环境 PostgreSQL 配置](./开发环境PostgreSQL数据库配置教程.md)
- [线上环境 PostgreSQL 配置](./线上环境PostgreSQL数据库配置教程.md)

## 验证与演练

- [CLI 聊天验证与兼容矩阵](./CLI聊天验证与兼容矩阵.md)
- [兼容矩阵与 SDK/CLI/operator 索引](./兼容矩阵与SDK-CLI-operator验证索引.md)
- [性能与灾备演练场景](./性能与灾备演练场景.md)

```bash
pnpm exec sdkwork-app doctor
pnpm deploy:validate:standalone
pnpm deploy:validate:cloud
pnpm test:workflow-commercial-gates
pnpm verify
```

历史设计和退役命令可以保留在明确标注的历史设计文档中，但不得作为当前运行或发布入口。
