import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';
import ts from 'typescript';
import type { SdkworkAgentAppClient } from '@sdkwork/agents-pc-core/sdk/agentsAppSdkClient';
import type { AgentManagementProfile } from '@sdkwork/agents-app-sdk';
import type * as AgentServiceModule from '../../../sdkwork-agents/apps/sdkwork-agents-pc/packages/sdkwork-agents-pc-agents/src/services/AgentService.ts';

type AgentServiceExports = typeof AgentServiceModule;
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

function makeAgentRecord(managementProfile: AgentManagementProfile): RecordLike {
  return {
    id: '1002',
    agentId: 'agent.pc.empty.arrays',
    tenantId: '0',
    organizationId: '0',
    ownerUserId: '0',
    code: 'agent.pc.empty.arrays',
    displayName: 'Empty Array Agent',
    description: 'Agent with cleared capabilities',
    manifest: {},
    defaultCodeTaskIntent: {
      contextPaths: ['knowledge.base.legacy'],
      constraints: [],
    },
    implementationKind: 'manifest-only',
    implementationProviderId: null,
    managementProfile,
    status: 'active',
    tags: ['tech'],
    version: '4',
    visibility: 'private',
    createdAt: '2026-06-01T00:00:00Z',
    updatedAt: '2026-06-01T00:10:00Z',
    deletedAt: null,
  };
}

const requests: {
  create?: { body: AgentRequestBody; params: RecordLike };
} = {};

const emptyArrayProfile = {
  knowledgeBaseIds: [],
  skillIds: [],
  suggestedPrompts: [],
  toolIds: [],
  voiceIds: [],
} satisfies AgentManagementProfile;

const fakeClient = {
  ai: {
    agents: {
      async list(params: RecordLike) {
        assert.equal(params.page, 1);
        assert.equal(params.pageSize, 100);
        return {
          data: {
            items: [makeAgentRecord(emptyArrayProfile)],
            pageInfo: {
              page: 1,
              pageSize: 100,
              totalItems: '1',
              totalPages: 1,
            },
          },
        };
      },
      async create(body: AgentRequestBody, params: RecordLike) {
        requests.create = { body, params };
        return {
          data: makeAgentRecord(body.managementProfile ?? {}),
        };
      },
    },
  },
} as unknown as SdkworkAgentAppClient;

const { createSdkworkAgentService } = await loadAgentServiceModule();
const agentService = createSdkworkAgentService(() => fakeClient);

const [listedAgent] = await agentService.getAgents();
assert.ok(listedAgent, 'expected one listed agent');
assert.deepEqual(listedAgent.knowledgeBaseIds, []);
assert.deepEqual(listedAgent.skillIds, []);
assert.deepEqual(listedAgent.suggestedPrompts, []);
assert.deepEqual(listedAgent.toolIds, []);
assert.deepEqual(listedAgent.voiceIds, []);

await agentService.createAgent({
  avatar: '',
  description: 'Created with empty arrays',
  knowledgeBaseIds: [],
  name: 'Created Empty Array Agent',
  skillIds: [],
  suggestedPrompts: [],
  toolIds: [],
  type: 'normal',
  voiceIds: [],
});

assert.deepEqual(requests.create?.body.managementProfile?.knowledgeBaseIds, []);
assert.deepEqual(requests.create?.body.managementProfile?.skillIds, []);
assert.deepEqual(requests.create?.body.managementProfile?.suggestedPrompts, []);
assert.deepEqual(requests.create?.body.managementProfile?.toolIds, []);
assert.deepEqual(requests.create?.body.managementProfile?.voiceIds, []);

const createAgentViewText = readFileSync(
  '../../../sdkwork-agents/apps/sdkwork-agents-pc/packages/sdkwork-agents-pc-agents/src/pages/CreateAgentView.tsx',
  'utf8',
);
const createAgentViewSource = ts.createSourceFile(
  'CreateAgentView.tsx',
  createAgentViewText,
  ts.ScriptTarget.Latest,
  true,
  ts.ScriptKind.TSX,
);

function hasNullishOnlySetterFallback(setterName: string, agentProperty: string): boolean {
  let found = false;
  const visit = (node: ts.Node): void => {
    if (
      ts.isCallExpression(node) &&
      ts.isIdentifier(node.expression) &&
      node.expression.text === setterName &&
      node.arguments[0] &&
      ts.isBinaryExpression(node.arguments[0]) &&
      node.arguments[0].operatorToken.kind === ts.SyntaxKind.QuestionQuestionToken &&
      ts.isPropertyAccessExpression(node.arguments[0].left) &&
      ts.isIdentifier(node.arguments[0].left.expression) &&
      node.arguments[0].left.expression.text === 'agent' &&
      node.arguments[0].left.name.text === agentProperty
    ) {
      found = true;
      return;
    }
    ts.forEachChild(node, visit);
  };
  visit(createAgentViewSource);
  return found;
}

assert.ok(
  hasNullishOnlySetterFallback('setSelectedVoiceIds', 'voiceIds'),
  'CreateAgentView must preserve explicit empty voiceIds instead of restoring default voice',
);
assert.ok(
  hasNullishOnlySetterFallback('setSelectedKnowledgeIds', 'knowledgeBaseIds'),
  'CreateAgentView must preserve explicit empty knowledgeBaseIds',
);
assert.ok(
  hasNullishOnlySetterFallback('setSelectedToolIds', 'toolIds'),
  'CreateAgentView must preserve explicit empty toolIds',
);
assert.ok(
  hasNullishOnlySetterFallback('setSelectedSkillIds', 'skillIds'),
  'CreateAgentView must preserve explicit empty skillIds',
);

console.log('sdkwork im pc agent management profile empty array contract passed.');
