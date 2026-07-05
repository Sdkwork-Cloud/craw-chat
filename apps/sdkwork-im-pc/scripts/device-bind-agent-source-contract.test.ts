import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const sourceText = readFileSync(
  './packages/sdkwork-im-pc-devices/src/components/BindAgentModal.tsx',
  'utf8',
);

assert.match(
  sourceText,
  /async function loadBindAgentCatalog\(\): Promise<AgentConfig\[\]>/u,
  'BindAgentModal must load agents through a dedicated paginated catalog helper',
);
assert.match(
  sourceText,
  /agentService\.listAgentsPage\([\s\S]*scope:\s*'mine'/u,
  'BindAgentModal must include my private agents so device binding works before marketplace publication',
);
assert.match(
  sourceText,
  /agentService\.listAgentsPage\([\s\S]*scope:\s*'market'/u,
  'BindAgentModal must keep marketplace agents available for device binding',
);
assert.match(
  sourceText,
  /mergeUniqueAgents\(minePage\.items, marketPage\.items\)/u,
  'BindAgentModal must merge and dedupe mine and marketplace pages into the selectable agent list',
);
assert.doesNotMatch(
  sourceText,
  /getAgents|getMarketAgents/u,
  'BindAgentModal must not call removed AgentService list helpers',
);

console.log('sdkwork im pc device bind agent source contract passed.');
