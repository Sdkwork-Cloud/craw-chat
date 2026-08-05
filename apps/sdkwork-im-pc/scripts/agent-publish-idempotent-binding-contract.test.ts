import assert from 'node:assert/strict';
import type { SdkworkAgentsAppClient } from '@sdkwork/agents-pc-core/sdk/agentsAppSdkClient';
import type * as AgentServiceModule from '@sdkwork/agents-pc-agents';

type AgentServiceExports = typeof AgentServiceModule;
type RecordLike = Record<string, unknown>;

async function loadAgentServiceModule(): Promise<AgentServiceExports> {
  const loaded = (await import('@sdkwork/agents-pc-agents')) as Partial<AgentServiceExports> & {
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

const calls: Array<{
  body?: RecordLike;
  id?: string;
  operation: string;
  params?: RecordLike;
}> = [];

const fakeClient = {
  ai: {
    agents: {
      deployments: {
        async create(id: string, body: RecordLike, params: RecordLike) {
          calls.push({ body, id, operation: 'ai.agents.deployments.create', params });
          return {
            data: {
              agentId: id,
              deploymentId: body.deploymentId,
              status: 'active',
            },
          };
        },
      },
      providerBindings: {
        async create(id: string, body: RecordLike, params: RecordLike) {
          calls.push({ body, id, operation: 'ai.agents.providerBindings.create', params });
          throw {
            body: {
              code: 'conflict',
              detail: 'The requested provider binding cannot be created again.',
              errorCategory: 'business',
              status: 409,
              title: 'Conflict',
              type: 'about:blank',
            },
            status: 409,
          };
        },
      },
    },
  },
} as unknown as SdkworkAgentsAppClient;

const { createSdkworkAgentService } = await loadAgentServiceModule();
const agentService = createSdkworkAgentService(() => fakeClient);

await agentService.publishAgent('agent.pc.idempotent.publish');

const bindingCall = calls.find((call) => call.operation === 'ai.agents.providerBindings.create');
assert.equal(bindingCall?.id, 'agent.pc.idempotent.publish');
assert.equal(bindingCall?.body?.bindingId, 'binding.manifest.default');

const deploymentCall = calls.find((call) => call.operation === 'ai.agents.deployments.create');
assert.equal(
  deploymentCall?.id,
  'agent.pc.idempotent.publish',
  'publishAgent must continue by creating a deployment when the default provider binding already exists',
);
assert.equal(deploymentCall?.body?.bindingId, 'binding.manifest.default');
assert.ok(deploymentCall);

console.log('sdkwork im pc agent publish idempotent binding contract passed.');
