import assert from 'node:assert/strict';
import { pathToFileURL } from 'node:url';
import type * as AgentServiceModule from '../../../sdkwork-agents/apps/sdkwork-agents-pc/packages/sdkwork-agents-pc-agents/src/services/AgentService.ts';

type AgentServiceExports = typeof AgentServiceModule;

async function loadAgentServiceModule(): Promise<AgentServiceExports> {
  const moduleUrl = pathToFileURL(
    '../../../sdkwork-agents/apps/sdkwork-agents-pc/packages/sdkwork-agents-pc-agents/src/services/AgentService.ts',
  ).href;
  const loaded = (await import(moduleUrl)) as Partial<AgentServiceExports> & {
    default?: Partial<AgentServiceExports>;
  };
  const parseAgentCatalogSnapshot =
    loaded.parseAgentCatalogSnapshot ?? loaded.default?.parseAgentCatalogSnapshot;
  assert.equal(typeof parseAgentCatalogSnapshot, 'function');
  return {
    ...loaded.default,
    ...loaded,
    parseAgentCatalogSnapshot,
  } as AgentServiceExports;
}

const { parseAgentCatalogSnapshot } = await loadAgentServiceModule();

const [marketAgent] = parseAgentCatalogSnapshot(
  {
    agents: [
      {
        id: 'agent.catalog.managed',
        name: 'Catalog Managed Agent',
        description: 'Agent catalog snapshot contract',
        scope: 'market',
        managementProfile: {
          author: 'SDKWork Agent Studio',
          avatar: 'robot',
          categoryId: 'tech',
          color: 'bg-purple-500',
          debugMode: false,
          iconName: 'Terminal',
          jsonMode: true,
          knowledgeBaseIds: ['knowledge.base.catalog.manual'],
          memoryEnabled: false,
          model: 'model.google.gemini-1.5-pro',
          skillIds: ['skill.multi-agent'],
          suggestedPrompts: ['Explain the catalog', 'Plan the next action'],
          systemPrompt: 'Act as a catalog managed agent.',
          temperature: 0.2,
          toolIds: ['tool.mcp-github'],
          type: 'independent',
          users: '88',
          voiceIds: ['voice.voice-1'],
          welcomeMessage: 'Welcome from the catalog profile.',
        },
      },
    ],
  },
  'market',
);

assert.ok(marketAgent, 'expected one market agent from the catalog snapshot');
assert.equal(marketAgent.id, 'agent.catalog.managed');
assert.equal(marketAgent.name, 'Catalog Managed Agent');
assert.equal(marketAgent.description, 'Agent catalog snapshot contract');
assert.equal(marketAgent.author, 'SDKWork Agent Studio');
assert.equal(marketAgent.avatar, 'robot');
assert.equal(marketAgent.categoryId, 'tech');
assert.equal(marketAgent.color, 'bg-purple-500');
assert.equal(marketAgent.debugMode, false);
assert.equal(marketAgent.iconName, 'Terminal');
assert.equal(marketAgent.jsonMode, true);
assert.deepEqual(marketAgent.knowledgeBaseIds, ['knowledge.base.catalog.manual']);
assert.equal(marketAgent.memoryEnabled, false);
assert.equal(marketAgent.model, 'Gemini 1.5 Pro');
assert.deepEqual(marketAgent.skillIds, ['multi-agent']);
assert.deepEqual(marketAgent.suggestedPrompts, ['Explain the catalog', 'Plan the next action']);
assert.equal(marketAgent.systemPrompt, 'Act as a catalog managed agent.');
assert.equal(marketAgent.temperature, 0.2);
assert.deepEqual(marketAgent.toolIds, ['mcp-github']);
assert.equal(marketAgent.type, 'independent');
assert.equal(marketAgent.users, '88');
assert.deepEqual(marketAgent.voiceIds, ['voice-1']);
assert.equal(marketAgent.welcomeMessage, 'Welcome from the catalog profile.');

console.log('sdkwork im pc agent catalog snapshot management profile contract passed.');
