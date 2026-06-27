import assert from 'node:assert/strict';
import { pathToFileURL } from 'node:url';
import type { SdkworkAgentAppClient } from '@sdkwork/agents-pc-core/sdk/agentsAppSdkClient';
import type { AgentManagementProfile } from '@sdkwork/agents-app-sdk';
import type * as AgentServiceModule from '../../../sdkwork-agents/apps/sdkwork-agents-pc/packages/sdkwork-agents-pc-agents/src/services/AgentService.ts';

type AgentServiceExports = typeof AgentServiceModule;
type AgentConfig = AgentServiceModule.AgentConfig;
type RecordLike = Record<string, unknown>;
type AgentRequestBody = RecordLike & {
  managementProfile?: AgentManagementProfile | null;
};

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
  const agentId = String(overrides.agentId ?? 'agent.pc.model.boundary');
  return {
    agentId,
    code: agentId,
    createdAt: '2026-06-01T00:00:00Z',
    deletedAt: null,
    description: String(overrides.description ?? 'Model normalization boundary'),
    displayName: String(overrides.displayName ?? 'Model Boundary Agent'),
    implementationKind: 'manifest-only',
    implementationProviderId: null,
    managementProfile: overrides.managementProfile ?? {},
    manifest: {},
    organizationId: '0',
    ownerUserId: '0',
    status: 'active',
    tags: [],
    tenantId: '0',
    updatedAt: '2026-06-01T00:10:00Z',
    version: '1',
    visibility: 'private',
    ...overrides,
  };
}

const calls: Array<{
  body?: AgentRequestBody;
  id?: string;
  operation: string;
  params?: RecordLike;
}> = [];

const fakeClient = {
  ai: {
    agents: {
      async create(body: AgentRequestBody, params: RecordLike) {
        calls.push({ body, operation: 'ai.agents.create', params });
        return {
          data: makeAgentRecord({
            agentId: body.agentId,
            displayName: body.displayName,
            managementProfile: body.managementProfile,
          }),
        };
      },
      previewResponses: {
        async create(id: string, body: AgentRequestBody, params: RecordLike) {
          calls.push({ body, id, operation: 'ai.agents.previewResponses.create', params });
          return {
            data: {
              agentId: id,
              executionId: body.executionId,
              outputPayload: { reply: 'runtime model boundary' },
            },
          };
        },
      },
    },
  },
} as unknown as SdkworkAgentAppClient;

const { createSdkworkAgentService } = await loadAgentServiceModule();
const agentService = createSdkworkAgentService(() => fakeClient);

await agentService.createAgent({
  avatar: '',
  description: 'Custom model should be stored as a standard management id',
  model: 'My Custom Runtime',
  name: 'Custom Model Agent',
  type: 'normal',
} satisfies AgentConfig);

await agentService.requestPreviewResponse({
  config: {
    avatar: '',
    description: 'Runtime model id should pass through',
    id: 'agent.pc.model.boundary',
    model: 'sdkwork-agent-runtime',
    name: 'Runtime Model Agent',
    type: 'normal',
  },
  content: 'test runtime model',
  model: 'sdkwork-agent-runtime',
});

const createCall = calls.find((call) => call.operation === 'ai.agents.create');
assert.equal(
  createCall?.body?.managementProfile?.model,
  'model.my-custom-runtime',
  'management profile persistence must normalize unknown UI/custom model labels into standard model ids',
);

const previewCall = calls.find((call) => call.operation === 'ai.agents.previewResponses.create');
assert.equal(
  previewCall?.body?.model,
  'sdkwork-agent-runtime',
  'runtime execution must preserve unknown backend/runtime model ids instead of adding model. prefix',
);
assert.equal(
  ((previewCall?.body?.inputPayload as RecordLike | undefined)?.agent as RecordLike | undefined)?.model,
  'sdkwork-agent-runtime',
);

console.log('sdkwork im pc agent model normalization boundary contract passed.');
