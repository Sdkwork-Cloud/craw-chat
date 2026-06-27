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

function legacyIntent(): RecordLike {
  return {
    constraints: [
      'agent.type=normal',
      `${AGENT_UI_CONFIG_CONSTRAINT_PREFIX}${JSON.stringify({
        systemPrompt: 'Legacy system prompt should not return after explicit clear.',
        type: 'normal',
        welcomeMessage: 'Legacy welcome should not return after explicit clear.',
      })}`,
    ],
    contextPaths: [],
    prompt: 'Legacy system prompt should not return after explicit clear.',
  };
}

function makeAgentRecord(managementProfile: AgentManagementProfile): RecordLike {
  return {
    agentId: 'agent.pc.empty.text',
    displayName: 'Empty Text Agent',
    description: 'Agent with explicitly cleared text fields',
    defaultCodeTaskIntent: legacyIntent(),
    implementationKind: 'manifest-only',
    managementProfile,
    manifest: {
      description: 'Manifest prompt should not return after explicit clear.',
    },
    status: 'active',
    tags: ['tech'],
    visibility: 'private',
  };
}

const requests: {
  create?: { body: AgentRequestBody; params: RecordLike };
  update?: { id: string; body: AgentRequestBody; params: RecordLike };
} = {};

const emptyTextProfile = {
  systemPrompt: '',
  welcomeMessage: '',
} satisfies AgentManagementProfile;

const fakeClient = {
  ai: {
    agents: {
      async list(params: RecordLike) {
        assert.equal(params.page, 1);
        assert.equal(params.pageSize, 100);
        return {
          data: {
            items: [makeAgentRecord(emptyTextProfile)],
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
      async retrieve(id: string) {
        return {
          data: {
            ...makeAgentRecord(emptyTextProfile),
            agentId: id,
          },
        };
      },
      async update(id: string, body: AgentRequestBody, params: RecordLike) {
        requests.update = { id, body, params };
        return {
          data: {
            ...makeAgentRecord(body.managementProfile ?? {}),
            agentId: id,
            displayName: body.displayName ?? 'Updated Empty Text Agent',
          },
        };
      },
    },
  },
} as unknown as SdkworkAgentAppClient;

const { createSdkworkAgentService } = await loadAgentServiceModule();
const agentService = createSdkworkAgentService(() => fakeClient);

const [listedAgent] = await agentService.getAgents();
assert.ok(listedAgent, 'expected one listed agent');
assert.equal(listedAgent.systemPrompt, '');
assert.equal(listedAgent.welcomeMessage, '');

await agentService.createAgent({
  avatar: '',
  description: 'Created with explicitly cleared text fields',
  name: 'Created Empty Text Agent',
  systemPrompt: '',
  type: 'normal',
  welcomeMessage: '',
});

assert.equal(requests.create?.body.managementProfile?.systemPrompt, '');
assert.equal(requests.create?.body.managementProfile?.welcomeMessage, '');

await agentService.updateAgent('agent.pc.empty.text', {
  name: 'Renamed Empty Text Agent',
});

assert.equal(requests.update?.body.managementProfile?.systemPrompt, '');
assert.equal(requests.update?.body.managementProfile?.welcomeMessage, '');
assert.equal(
  (requests.update?.body.defaultCodeTaskIntent as RecordLike | undefined)?.prompt,
  '',
);

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

function hasNullishWelcomeFallback(): boolean {
  let found = false;
  const visit = (node: ts.Node): void => {
    if (
      ts.isVariableDeclaration(node) &&
      ts.isIdentifier(node.name) &&
      node.name.text === 'nextWelcomeMessage' &&
      node.initializer &&
      ts.isBinaryExpression(node.initializer) &&
      node.initializer.operatorToken.kind === ts.SyntaxKind.QuestionQuestionToken &&
      ts.isPropertyAccessExpression(node.initializer.left) &&
      ts.isIdentifier(node.initializer.left.expression) &&
      node.initializer.left.expression.text === 'agent' &&
      node.initializer.left.name.text === 'welcomeMessage'
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
  hasNullishWelcomeFallback(),
  'CreateAgentView must preserve an explicitly cleared welcomeMessage instead of restoring the default welcome text',
);

console.log('sdkwork im pc agent management profile empty text contract passed.');
