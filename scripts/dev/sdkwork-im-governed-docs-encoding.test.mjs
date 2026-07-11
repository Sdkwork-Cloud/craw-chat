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

const currentMessageHistoryDocs = [
  'docs/sites/architecture/module-map.md',
  'docs/architecture/tech/TECH-module-map.md',
  'docs/product/prd/PRD.md',
  'docs/product/compliance/SLA_SLO.md',
  'docs/operations/DISASTER_RECOVERY.md',
  'docs/sites/api-reference/im/media.md',
  'docs/architecture/tech/TECH-media.md',
  'docs/architecture/tech/PAGINATION-DEBT-REGISTER.md',
  'docs/architecture/tech/TECH-changelog.md',
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

for (const relativePath of currentMessageHistoryDocs) {
  const absolutePath = path.join(repoRoot, relativePath);
  assert.ok(fs.existsSync(absolutePath), `current IM message-history doc must exist: ${relativePath}`);
  const source = fs.readFileSync(absolutePath, 'utf8');
  assert.doesNotMatch(
    source,
    /\uFFFD|[鈥搂鈮鈹]/u,
    `${relativePath} must not contain UTF-8 replacement characters or mojibake from mis-decoded UTF-8`,
  );
}

const moduleMapDocs = [
  'docs/sites/architecture/module-map.md',
  'docs/architecture/tech/TECH-module-map.md',
];
for (const relativePath of moduleMapDocs) {
  const source = fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
  assert.doesNotMatch(
    source,
    /projection-service[^\n|]*\|[^\n]*timeline/iu,
    `${relativePath} must not describe projection-service as the public message-history timeline owner`,
  );
}

const prdSource = fs.readFileSync(path.join(repoRoot, 'docs/product/prd/PRD.md'), 'utf8');
assert.doesNotMatch(
  prdSource,
  /virtualized timeline|WebSocket timeline sync/iu,
  'PRD client delivery matrix must describe message history windows and message sync, not timeline sync',
);
assert.match(
  prdSource,
  /virtualized message history window/iu,
  'PRD client delivery matrix must mention the bounded message history window',
);

const slaSource = fs.readFileSync(path.join(repoRoot, 'docs/product/compliance/SLA_SLO.md'), 'utf8');
assert.doesNotMatch(
  slaSource,
  /Fetch timeline/iu,
  'SLA/SLO API latency table must name message history fetches, not timeline fetches',
);

const disasterRecoverySource = fs.readFileSync(path.join(repoRoot, 'docs/operations/DISASTER_RECOVERY.md'), 'utf8');
assert.doesNotMatch(
  disasterRecoverySource,
  /inbox \+ timeline|Fetch inbox \+ timeline/iu,
  'disaster recovery gates must verify inbox plus message history, not inbox plus timeline',
);
assert.match(
  disasterRecoverySource,
  /Timeline of detection, decision, execution, verification/u,
  'disaster recovery PIR text may keep incident timeline wording',
);

for (const relativePath of [
  'docs/sites/api-reference/im/media.md',
  'docs/architecture/tech/TECH-media.md',
]) {
  const source = fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
  assert.doesNotMatch(
    source,
    /Message timelines/iu,
    `${relativePath} must describe message history entries, not message timelines`,
  );
}

const paginationDebtSource = fs.readFileSync(
  path.join(repoRoot, 'docs/architecture/tech/PAGINATION-DEBT-REGISTER.md'),
  'utf8',
);
for (const forbidden of [
  /timeline seq fallback/iu,
  /reappearing in timeline/iu,
  /inbox\/contacts\/timeline/iu,
  /ChatService` timeline\/members/iu,
]) {
  assert.doesNotMatch(
    paginationDebtSource,
    forbidden,
    'pagination debt register must use message-history wording for current message history paths',
  );
}

const techChangelogSource = fs.readFileSync(
  path.join(repoRoot, 'docs/architecture/tech/TECH-changelog.md'),
  'utf8',
);
assert.doesNotMatch(
  techChangelogSource,
  /timeline seq fallback/iu,
  'TECH changelog must use message-history wording for current message history paths',
);

process.stdout.write('sdkwork-im governed docs encoding standard passed\n');
