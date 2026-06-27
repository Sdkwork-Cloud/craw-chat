import assert from 'node:assert/strict';
import { pathToFileURL } from 'node:url';
import type { SdkworkAgentAppClient } from '@sdkwork/agents-pc-core/sdk/agentsAppSdkClient';
import type { AgentManagementProfile } from '@sdkwork/agents-app-sdk';
import type * as AgentServiceModule from '../../../sdkwork-agents/apps/sdkwork-agents-pc/packages/sdkwork-agents-pc-agents/src/services/AgentService.ts';

type AgentServiceExports = typeof AgentServiceModule;
type AgentConfig = AgentServiceModule.AgentConfig;
type RecordLike = Record<string, unknown>;
type AgentRequestBody = RecordLike & {
  defaultCodeTaskIntent?: {
    constraints?: unknown[];
    contextPaths?: unknown[];
  };
  managementProfile?: AgentManagementProfile | null;
};

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
  assert.equal(typeof createSdkworkAgentService, 'function');
  return {
    ...loaded.default,
    ...loaded,
    createSdkworkAgentService,
  } as AgentServiceExports;
}

function makeAgentRecord(overrides: RecordLike = {}): RecordLike {
  const agentId = String(overrides.agentId ?? 'agent.pc.array.normalization');
  return {
    id: '1004',
    agentId,
    tenantId: '0',
    organizationId: '0',
    ownerUserId: '0',
    code: agentId,
    displayName: String(overrides.displayName ?? 'Array Normalization Agent'),
    description: String(overrides.description ?? 'Agent with normalized array settings'),
    manifest: {},
    defaultCodeTaskIntent: overrides.defaultCodeTaskIntent ?? {
      constraints: [],
      contextPaths: [],
    },
    implementationKind: 'manifest-only',
    implementationProviderId: null,
    managementProfile: overrides.managementProfile ?? {
      knowledgeBaseIds: ['knowledge.base.current'],
      suggestedPrompts: ['Current prompt'],
      type: 'normal',
    },
    status: 'active',
    tags: ['tech'],
    version: '1',
    visibility: 'private',
    createdAt: '2026-06-01T00:00:00Z',
    updatedAt: '2026-06-01T00:10:00Z',
    deletedAt: null,
    ...overrides,
  };
}

function extractUiConfig(body: AgentRequestBody): RecordLike {
  const encoded = body.defaultCodeTaskIntent?.constraints?.find(
    (item): item is string =>
      typeof item === 'string' && item.startsWith(AGENT_UI_CONFIG_CONSTRAINT_PREFIX),
  );
  assert.ok(encoded, 'defaultCodeTaskIntent must include encoded PC agent UI config');
  const parsed = JSON.parse(encoded.slice(AGENT_UI_CONFIG_CONSTRAINT_PREFIX.length)) as unknown;
  assert.equal(typeof parsed, 'object');
  assert.notEqual(parsed, null);
  return parsed as RecordLike;
}

const requests: {
  create?: { body: AgentRequestBody; params: RecordLike };
  update?: { id: string; body: AgentRequestBody; params: RecordLike };
} = {};

const fakeClient = {
  ai: {
    agents: {
      async create(body: AgentRequestBody, params: RecordLike) {
        requests.create = { body, params };
        return {
          data: makeAgentRecord({
            agentId: body.agentId,
            defaultCodeTaskIntent: body.defaultCodeTaskIntent,
            displayName: body.displayName,
            description: body.description,
            managementProfile: body.managementProfile,
          }),
        };
      },
      async retrieve(id: string) {
        return {
          data: makeAgentRecord({
            agentId: id,
          }),
        };
      },
      async update(id: string, body: AgentRequestBody, params: RecordLike) {
        requests.update = { id, body, params };
        return {
          data: makeAgentRecord({
            agentId: id,
            defaultCodeTaskIntent: body.defaultCodeTaskIntent,
            displayName: body.displayName,
            description: body.description,
            managementProfile: body.managementProfile,
          }),
        };
      },
    },
  },
} as unknown as SdkworkAgentAppClient;

const { createSdkworkAgentService } = await loadAgentServiceModule();
const agentService = createSdkworkAgentService(() => fakeClient);

const dirtyArrayConfig = {
  avatar: '',
  description: 'Created with dirty array settings',
  knowledgeBaseIds: [' knowledge.base.docs ', '', '   ', 'knowledge.base.api'],
  name: 'Created Array Normalization Agent',
  suggestedPrompts: [' Summarize docs ', '', '   ', 'Plan release'],
  type: 'normal',
} satisfies AgentConfig;

await agentService.createAgent(dirtyArrayConfig);

assert.deepEqual(requests.create?.body.managementProfile?.knowledgeBaseIds, [
  'knowledge.base.docs',
  'knowledge.base.api',
]);
assert.deepEqual(requests.create?.body.managementProfile?.suggestedPrompts, [
  'Summarize docs',
  'Plan release',
]);
assert.deepEqual(requests.create?.body.defaultCodeTaskIntent?.contextPaths, [
  'knowledge.base.docs',
  'knowledge.base.api',
]);
assert.deepEqual(extractUiConfig(requests.create?.body ?? {}).knowledgeBaseIds, [
  'knowledge.base.docs',
  'knowledge.base.api',
]);
assert.deepEqual(extractUiConfig(requests.create?.body ?? {}).suggestedPrompts, [
  'Summarize docs',
  'Plan release',
]);

await agentService.updateAgent('agent.pc.array.normalization', {
  knowledgeBaseIds: [' knowledge.base.update ', ' ', 'knowledge.base.release'],
  suggestedPrompts: [' Check release notes ', '', 'Prepare rollout'],
} satisfies Partial<AgentConfig>);

assert.deepEqual(requests.update?.body.managementProfile?.knowledgeBaseIds, [
  'knowledge.base.update',
  'knowledge.base.release',
]);
assert.deepEqual(requests.update?.body.managementProfile?.suggestedPrompts, [
  'Check release notes',
  'Prepare rollout',
]);
assert.deepEqual(requests.update?.body.defaultCodeTaskIntent?.contextPaths, [
  'knowledge.base.update',
  'knowledge.base.release',
]);
assert.deepEqual(extractUiConfig(requests.update?.body ?? {}).knowledgeBaseIds, [
  'knowledge.base.update',
  'knowledge.base.release',
]);
assert.deepEqual(extractUiConfig(requests.update?.body ?? {}).suggestedPrompts, [
  'Check release notes',
  'Prepare rollout',
]);

console.log('sdkwork im pc agent management profile array normalization contract passed.');
