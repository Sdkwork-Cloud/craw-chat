import assert from 'node:assert/strict';
import { pathToFileURL } from 'node:url';
import type { SdkworkAgentAppClient } from '@sdkwork/agents-pc-core/sdk/agentsAppSdkClient';
import type { AgentManagementProfile } from '@sdkwork/agents-app-sdk';
import type * as AgentServiceModule from '../../../sdkwork-agents/apps/sdkwork-agents-pc/packages/sdkwork-agents-pc-agents/src/services/AgentService.ts';

type AgentServiceExports = typeof AgentServiceModule;
type RecordLike = Record<string, unknown>;

const AGENT_UI_CONFIG_CONSTRAINT_PREFIX = 'sdkwork.agent.pc.config:';

async function loadAgentServiceModule(): Promise<AgentServiceExports> {
  const moduleUrl = pathToFileURL(
    '../../../sdkwork-agents/apps/sdkwork-agents-pc/packages/sdkwork-agents-pc-agents/src/services/AgentService.ts',
  ).href;
  const loaded = (await import(moduleUrl)) as Partial<AgentServiceExports> & {
    default?: Partial<AgentServiceExports>;
  };
  const createSdkworkAgentService =
    loaded.createSdkworkAgentService ?? loaded.default?.createSdkworkAgentService;
  const parseAgentCatalogSnapshot =
    loaded.parseAgentCatalogSnapshot ?? loaded.default?.parseAgentCatalogSnapshot;
  assert.equal(typeof createSdkworkAgentService, 'function');
  assert.equal(typeof parseAgentCatalogSnapshot, 'function');
  return {
    ...loaded.default,
    ...loaded,
    createSdkworkAgentService,
    parseAgentCatalogSnapshot,
  } as AgentServiceExports;
}

function legacyIndependentIntent(): RecordLike {
  return {
    constraints: [
      'agent.type=independent',
      `${AGENT_UI_CONFIG_CONSTRAINT_PREFIX}${JSON.stringify({
        knowledgeBaseIds: ['knowledge.base.legacy'],
        systemPrompt: 'Preserve the independent agent prompt.',
        type: 'independent',
      })}`,
    ],
    contextPaths: ['knowledge.base.legacy'],
    prompt: 'Preserve the independent agent prompt.',
  };
}

const partialManagementProfile = {
  avatar: 'robot',
  welcomeMessage: 'Welcome from partial profile.',
} satisfies AgentManagementProfile;

function makePartialProfileAgentRecord(): RecordLike {
  return {
    agentId: 'agent.pc.partial.profile',
    displayName: 'Partial Profile Agent',
    description: 'Keeps legacy type when profile is partial',
    defaultCodeTaskIntent: legacyIndependentIntent(),
    implementationKind: 'manifest-only',
    managementProfile: partialManagementProfile,
    manifest: {},
    status: 'active',
    tags: ['tech'],
    visibility: 'private',
  };
}

const fakeClient = {
  ai: {
    agents: {
      async list(params: RecordLike) {
        assert.equal(params.page, 1);
        assert.equal(params.pageSize, 100);
        return {
          data: {
            items: [makePartialProfileAgentRecord()],
            pageInfo: {
              page: 1,
              pageSize: 100,
              totalItems: '1',
              totalPages: 1,
            },
          },
        };
      },
    },
  },
} as unknown as SdkworkAgentAppClient;

const { createSdkworkAgentService, parseAgentCatalogSnapshot } = await loadAgentServiceModule();

const agentService = createSdkworkAgentService(() => fakeClient);
const [listedAgent] = await agentService.getAgents();

assert.ok(listedAgent, 'expected one listed agent');
assert.equal(
  listedAgent.type,
  'independent',
  'partial managementProfile must not overwrite the legacy independent agent type',
);
assert.equal(listedAgent.avatar, 'robot');
assert.equal(listedAgent.welcomeMessage, 'Welcome from partial profile.');
assert.equal(listedAgent.systemPrompt, 'Preserve the independent agent prompt.');
assert.deepEqual(listedAgent.knowledgeBaseIds, ['knowledge.base.legacy']);

const [snapshotAgent] = parseAgentCatalogSnapshot(
  {
    agents: [
      {
        id: 'agent.catalog.partial.profile',
        name: 'Catalog Partial Profile Agent',
        description: 'Catalog keeps legacy type when profile is partial',
        defaultCodeTaskIntent: legacyIndependentIntent(),
        managementProfile: partialManagementProfile,
        scope: 'market',
      },
    ],
  },
  'market',
);

assert.ok(snapshotAgent, 'expected one catalog snapshot agent');
assert.equal(
  snapshotAgent.type,
  'independent',
  'catalog partial managementProfile must not overwrite the legacy independent agent type',
);
assert.equal(snapshotAgent.avatar, 'robot');
assert.equal(snapshotAgent.welcomeMessage, 'Welcome from partial profile.');
assert.equal(snapshotAgent.systemPrompt, 'Preserve the independent agent prompt.');
assert.deepEqual(snapshotAgent.knowledgeBaseIds, ['knowledge.base.legacy']);

console.log('sdkwork im pc partial agent management profile type contract passed.');
