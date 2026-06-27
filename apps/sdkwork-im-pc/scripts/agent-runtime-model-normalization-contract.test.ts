import assert from 'node:assert/strict';
import { pathToFileURL } from 'node:url';
import type { SdkworkAgentAppClient } from '@sdkwork/agents-pc-core/sdk/agentsAppSdkClient';
import type * as AgentServiceModule from '../../../sdkwork-agents/apps/sdkwork-agents-pc/packages/sdkwork-agents-pc-agents/src/services/AgentService.ts';

type AgentServiceExports = typeof AgentServiceModule;
type AgentConfig = AgentServiceModule.AgentConfig;
type RecordLike = Record<string, unknown>;

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

const calls: Array<{
  body?: RecordLike;
  id?: string;
  operation: string;
  params?: RecordLike;
}> = [];

const fakeClient = {
  ai: {
    agents: {
      previewResponses: {
        async create(id: string, body: RecordLike, params: RecordLike) {
          calls.push({ body, id, operation: 'ai.agents.previewResponses.create', params });
          return {
            data: {
              agentId: id,
              executionId: body.executionId,
              outputPayload: { reply: 'normalized model preview' },
            },
          };
        },
      },
      promptOptimizations: {
        async create(id: string, body: RecordLike, params: RecordLike) {
          calls.push({ body, id, operation: 'ai.agents.promptOptimizations.create', params });
          return {
            data: {
              agentId: id,
              executionId: body.executionId,
              outputPayload: { optimizedPrompt: 'Use normalized model ids.' },
            },
          };
        },
      },
    },
  },
} as unknown as SdkworkAgentAppClient;

const { createSdkworkAgentService } = await loadAgentServiceModule();
const agentService = createSdkworkAgentService(() => fakeClient);

const config = {
  avatar: '',
  description: 'Runtime model normalization',
  id: 'agent.pc.runtime.model',
  model: 'GPT-4o',
  name: 'Runtime Model Agent',
  temperature: 0.4,
  type: 'normal',
} satisfies AgentConfig;

await agentService.requestPreviewResponse({
  config,
  content: 'Preview with UI model name',
  model: 'GPT-4o',
  temperature: 0.4,
});

await agentService.optimizePrompt({
  config,
  prompt: 'Optimize with UI model name',
});

const previewCall = calls.find((call) => call.operation === 'ai.agents.previewResponses.create');
assert.equal(previewCall?.body?.model, 'model.openai.gpt-4o');
assert.equal(
  (previewCall?.body?.inputPayload as RecordLike | undefined)?.model,
  'model.openai.gpt-4o',
);
assert.equal(
  ((previewCall?.body?.inputPayload as RecordLike | undefined)?.agent as RecordLike | undefined)?.model,
  'model.openai.gpt-4o',
);

const optimizeCall = calls.find((call) => call.operation === 'ai.agents.promptOptimizations.create');
assert.equal(
  ((optimizeCall?.body?.inputPayload as RecordLike | undefined)?.agent as RecordLike | undefined)?.model,
  'model.openai.gpt-4o',
);

console.log('sdkwork im pc agent runtime model normalization contract passed.');
