#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

const governedDocs = [
  'docs/架构/09-实施计划.md',
  'docs/架构/README.md',
  'docs/架构/06-Gateway-API-与协议设计.md',
  'docs/架构/08-安全-多租户-SaaS-私有化-部署设计.md',
  'docs/架构/131-连接管理与分层弹性扩容架构设计-2026-04-06.md',
  'docs/架构/133-代码结构治理与crate拆分标准-2026-04-06.md',
  'docs/架构/140-可观测性与SLO治理设计-2026-04-06.md',
  'docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md',
  'docs/架构/144-CCP传输绑定与握手协商设计-2026-04-06.md',
  'docs/架构/152CJ-Loop54补充-2026-04-11.md',
  'docs/架构/27-外部认证与Trusted-Identity边界标准-2026-04-05.md',
  'docs/架构/29-剩余独立服务公网认证收口与Public-Builder补齐-2026-04-05.md',
  'docs/架构/30-审计与运维接口最小权限标准-2026-04-05.md',
  'docs/架构/48-公网上行Bearer必须进行签名校验标准-2026-04-05.md',
  'docs/架构/124-local-chat-cli-multi-terminal-validation-standard-2026-04-06.md',
  'docs/架构/125-local-chat-window-launcher-standard-2026-04-06.md',
  'docs/架构/126-windows-visible-chat-gui-validation-standard-2026-04-06.md',
  'docs/架构/127-chat-cli-direct-binary-wrapper-standard-2026-04-06.md',
  'docs/架构/129-chat-window-gui-utf8-cli-json-standard-2026-04-06.md',
  'docs/架构/sdkwork-im-rtc-complete-integration-guide.md',
  'docs/architecture/decisions/README.md',
  'docs/architecture/decisions/ADR-20260615-crate-naming-alignment.md',
  'docs/architecture/decisions/ADR-20260615-craw-chat-to-sdkwork-im-rebrand.md',
  'database/README.md',
  'docs/部署/README.md',
  'specs/README.md',
];

for (const relativePath of governedDocs) {
  const absolutePath = path.join(repoRoot, relativePath);
  assert.ok(fs.existsSync(absolutePath), `governed doc must exist: ${relativePath}`);
  const source = fs.readFileSync(absolutePath, 'utf8');
  assert.doesNotMatch(
    source,
    /\uFFFD/u,
    `${relativePath} must not contain UTF-8 replacement characters (encoding corruption)`,
  );
  assert.doesNotMatch(
    source,
    /å…¼å®¹|ç›®çš„|æœ¬é¡µ/u,
    `${relativePath} must not contain mojibake from mis-decoded UTF-8`,
  );
}

process.stdout.write('sdkwork-im governed docs encoding standard passed\n');
