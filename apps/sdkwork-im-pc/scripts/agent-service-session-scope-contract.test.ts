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

const SESSION_STORAGE_KEY = 'sdkwork-im-pc:session:v1';

class MemoryStorage implements Storage {
  private readonly values = new Map<string, string>();

  get length(): number {
    return this.values.size;
  }

  clear(): void {
    this.values.clear();
  }

  getItem(key: string): string | null {
    return this.values.get(key) ?? null;
  }

  key(index: number): string | null {
    return Array.from(this.values.keys())[index] ?? null;
  }

  removeItem(key: string): void {
    this.values.delete(key);
  }

  setItem(key: string, value: string): void {
    this.values.set(key, value);
  }
}

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

function installSession(): void {
  const localStorage = new MemoryStorage();
  const windowLike = {
    dispatchEvent() {
      return true;
    },
    localStorage,
  };
  Object.defineProperty(globalThis, 'window', {
    configurable: true,
    value: windowLike,
  });
  localStorage.setItem(
    SESSION_STORAGE_KEY,
    JSON.stringify({
      accessToken: 'access.token',
      authToken: 'auth.token',
      context: {
        appId: 'sdkwork-im-pc',
        authLevel: 'password',
        deploymentMode: 'saas',
        environment: 'dev',
        organizationId: 'organization.real.10',
        sessionId: 'session.real.1',
        tenantId: 'tenant.real.1',
        userId: 'user.real.100',
      },
      user: {
        id: 'user.real.100',
      },
    }),
  );
}

function makeAgentRecord(overrides: RecordLike = {}): RecordLike {
  const agentId = String(overrides.agentId ?? 'agent.pc.session.scope');
  return {
    id: '1005',
    agentId,
    tenantId: 'tenant.real.1',
    organizationId: 'organization.real.10',
    ownerUserId: 'user.real.100',
    code: agentId,
    displayName: String(overrides.displayName ?? 'Session Scope Agent'),
    description: String(overrides.description ?? 'Agent scoped to the current session'),
    manifest: {},
    defaultCodeTaskIntent: {
      constraints: [],
      contextPaths: [],
    },
    implementationKind: 'manifest-only',
    implementationProviderId: null,
    managementProfile: overrides.managementProfile ?? {
      type: 'normal',
    },
    status: 'active',
    tags: ['tech'],
    version: '1',
    visibility: overrides.visibility ?? 'private',
    createdAt: '2026-06-01T00:00:00Z',
    updatedAt: '2026-06-01T00:10:00Z',
    deletedAt: null,
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
      async list(params?: RecordLike) {
        calls.push({ operation: 'ai.agents.list', params });
        return {
          data: {
            items: [makeAgentRecord()],
            pageInfo: {
              page: 1,
              pageSize: 100,
              totalItems: '1',
              totalPages: 1,
            },
          },
        };
      },
      async create(body: AgentRequestBody, params?: RecordLike) {
        calls.push({ body, operation: 'ai.agents.create', params });
        return {
          data: makeAgentRecord({
            agentId: body.agentId,
            displayName: body.displayName,
            description: body.description,
            managementProfile: body.managementProfile,
          }),
        };
      },
      async retrieve(id: string, params?: RecordLike) {
        calls.push({ id, operation: 'ai.agents.retrieve', params });
        return {
          data: makeAgentRecord({ agentId: id }),
        };
      },
      async update(id: string, body: AgentRequestBody, params?: RecordLike) {
        calls.push({ body, id, operation: 'ai.agents.update', params });
        return {
          data: makeAgentRecord({
            agentId: id,
            displayName: body.displayName,
            description: body.description,
            managementProfile: body.managementProfile,
          }),
        };
      },
      async delete(id: string, params?: RecordLike) {
        calls.push({ id, operation: 'ai.agents.delete', params });
        return {
          data: makeAgentRecord({ agentId: id }),
        };
      },
      deployments: {
        async create(id: string, body: AgentRequestBody, params?: RecordLike) {
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
      previewResponses: {
        async create(id: string, body: AgentRequestBody, params?: RecordLike) {
          calls.push({ body, id, operation: 'ai.agents.previewResponses.create', params });
          return {
            data: {
              agentId: id,
              executionId: body.executionId,
              outputPayload: { reply: 'session scoped preview' },
            },
          };
        },
      },
      promptOptimizations: {
        async create(id: string, body: AgentRequestBody, params?: RecordLike) {
          calls.push({ body, id, operation: 'ai.agents.promptOptimizations.create', params });
          return {
            data: {
              agentId: id,
              executionId: body.executionId,
              outputPayload: { optimizedPrompt: 'Use current session scope.' },
            },
          };
        },
      },
      providerBindings: {
        async create(id: string, body: AgentRequestBody, params?: RecordLike) {
          calls.push({ body, id, operation: 'ai.agents.providerBindings.create', params });
          return {
            data: {
              active: true,
              agentId: id,
              bindingId: body.bindingId,
              status: 'active',
            },
          };
        },
      },
    },
  },
} as unknown as SdkworkAgentAppClient;

installSession();

const { createSdkworkAgentService } = await loadAgentServiceModule();
const agentService = createSdkworkAgentService(() => fakeClient);

await agentService.getAgents();
await agentService.getMarketAgents();
await agentService.createAgent({
  avatar: '',
  description: 'Created in current session',
  name: 'Current Session Agent',
  type: 'normal',
} satisfies AgentConfig);
await agentService.updateAgent('agent.pc.session.scope', {
  description: 'Updated in current session',
} satisfies Partial<AgentConfig>);
await agentService.publishAgent('agent.pc.session.scope');
await agentService.requestPreviewResponse({
  config: {
    avatar: '',
    description: 'Preview in current session',
    id: 'agent.pc.session.scope',
    name: 'Current Session Agent',
    type: 'normal',
  },
  content: 'test',
});
await agentService.optimizePrompt({
  config: {
    avatar: '',
    description: 'Optimize in current session',
    id: 'agent.pc.session.scope',
    name: 'Current Session Agent',
    type: 'normal',
  },
  prompt: 'answer',
});
await agentService.deleteAgent('agent.pc.session.scope');

const listCalls = calls.filter((call) => call.operation === 'ai.agents.list');
assert.equal(listCalls.length, 2);
for (const call of listCalls) {
  assert.deepEqual(call.params, {
    page: 1,
    pageSize: 100,
  });
}

const createCall = calls.find((call) => call.operation === 'ai.agents.create');
assert.equal(createCall?.params, undefined);

for (const operation of [
  'ai.agents.retrieve',
  'ai.agents.update',
  'ai.agents.providerBindings.create',
  'ai.agents.deployments.create',
  'ai.agents.previewResponses.create',
  'ai.agents.promptOptimizations.create',
  'ai.agents.delete',
]) {
  const operationCalls = calls.filter((call) => call.operation === operation);
  assert.ok(operationCalls.length > 0, `expected ${operation} to be called`);
  for (const call of operationCalls) {
    assert.equal(call.params, undefined, `${operation} must not receive frontend-derived scope`);
  }
}

const forbiddenScopeKeys = ['organizationId', 'ownerUserId', 'tenantId'] as const;
for (const call of calls) {
  for (const key of forbiddenScopeKeys) {
    assert.equal(
      Object.hasOwn(call.params ?? {}, key),
      false,
      `${call.operation} params must not include ${key}`,
    );
    assert.equal(
      Object.hasOwn(call.body ?? {}, key),
      false,
      `${call.operation} body must not include ${key}`,
    );
  }
}

console.log('sdkwork im pc agent service session-derived scope contract passed.');
