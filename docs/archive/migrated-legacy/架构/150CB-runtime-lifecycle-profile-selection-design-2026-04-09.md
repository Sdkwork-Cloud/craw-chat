# Runtime Lifecycle Profile Selection Design

## Decision

- `init/install/start/stop/restart` 必须与 runtime ops 共享同一 profile 解析器，而不是各自硬编码 `standalone.split-services.development`。

## State Model

- profile set: `standalone.split-services.development | standalone.split-services.development`
- config priority:
  - `standalone.split-services.development`: `.runtime/standalone.split-services.development/config/standalone.split-services.development.env` -> `.runtime/standalone.split-services.development/config/standalone.split-services.development.env`
  - `standalone.split-services.development`: `.runtime/standalone.split-services.development/config/standalone.split-services.development.env`
- runtime-dir fallback: 若 profile config 未显式覆盖，则回退到 `.runtime/standalone.split-services.development`

## Contract

- PowerShell / CMD：`-ProfileName <standalone.split-services.development|standalone.split-services.development>`
- Bash：`--profile <standalone.split-services.development|standalone.split-services.development>`
- `init-config-local.*` 为选定 profile 写入主 config 文件。
- `install/start/stop/restart` 必须按选定 profile 解析 config 与 runtime-dir。
- `retired-lifecycle-restart.*` 必须把 profile 继续传给 `stop/start`。

## Boundary

- 当前设计只统一 profile 入口，不声明 `standalone.split-services.development` 已拥有独立 runtime topology。
- 原生 Bash 执行态仍需单独验证。
