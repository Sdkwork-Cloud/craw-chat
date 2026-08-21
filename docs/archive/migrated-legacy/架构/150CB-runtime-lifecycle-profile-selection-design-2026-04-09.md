# Runtime Lifecycle Profile Selection Design

## Decision

- `init/install/start/stop/restart` 必须与 runtime ops 共享同一 profile 解析器，而不是各自硬编码 `standalone.development`。

## State Model

- profile set: `standalone.development | standalone.development`
- config priority:
  - `standalone.development`: `.runtime/standalone.development/config/standalone.development.env` -> `.runtime/standalone.development/config/standalone.development.env`
  - `standalone.development`: `.runtime/standalone.development/config/standalone.development.env`
- runtime-dir fallback: 若 profile config 未显式覆盖，则回退到 `.runtime/standalone.development`

## Contract

- PowerShell / CMD：`-ProfileName <standalone.development|standalone.development>`
- Bash：`--profile <standalone.development|standalone.development>`
- `init-config-local.*` 为选定 profile 写入主 config 文件。
- `install/start/stop/restart` 必须按选定 profile 解析 config 与 runtime-dir。
- `retired-lifecycle-restart.*` 必须把 profile 继续传给 `stop/start`。

## Boundary

- 当前设计只统一 profile 入口，不声明 `standalone.development` 已拥有独立 runtime topology。
- 原生 Bash 执行态仍需单独验证。
