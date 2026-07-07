import assert from 'node:assert/strict';
import type { SdkworkAgentAppClient } from '@sdkwork/im-pc-core';
import {
  createSdkworkAgentService,
} from '@sdkwork/agents-pc-agents';

const agentSdkCalls: Array<{
  body?: Record<string, unknown>;
  id?: string;
  operation: string;
  params?: Record<string, unknown>;
}> = [];

const AGENT_UI_CONFIG_CONSTRAINT_PREFIX = 'sdkwork.agent.pc.config:';

function standardManifest(agentId: string, displayName: string, description: string, author = 'sdkwork-agents-pc') {
  return {
    agent_id: agentId,
    description,
    display_name: displayName,
    domain: 'intelligence',
    event_families: ['agent.lifecycle'],
    manifest_type: 'agent',
    name: agentId,
    optional_capabilities: [
      { capability_id: 'tool.invoke' },
      { capability_id: 'knowledge.read' },
      { capability_id: 'memory.query' },
    ],
    owner: { name: author },
    required_capabilities: [{ capability_id: 'model.chat' }],
    schema_version: '1.0.0',
    status: 'active',
    version: '0.1.0',
  };
}

function uiConfigConstraint(config: Record<string, unknown>): string {
  return `${AGENT_UI_CONFIG_CONSTRAINT_PREFIX}${JSON.stringify(config)}`;
}

function defaultCodeTaskIntent(config: Record<string, unknown>, prompt: string): Record<string, unknown> {
  return {
    constraints: [
      `agent.type=${config.type ?? 'normal'}`,
      uiConfigConstraint(config),
    ],
    contextPaths: Array.isArray(config.knowledgeBaseIds) ? config.knowledgeBaseIds : [],
    prompt,
  };
}

const fakeAgentClient = {
  ai: {
    agents: {
      async list(params: Record<string, unknown>) {
        agentSdkCalls.push({ operation: 'ai.agents.list', params });
        const items = [
          {
            agentId: 'agent.market.code',
            displayName: 'Code Reviewer',
            description: 'Reviews pull requests with project context.',
            defaultCodeTaskIntent: defaultCodeTaskIntent({
              avatar: 'https://cdn.example.test/code.png',
              author: 'Platform',
              categoryId: 'tech',
              color: 'bg-green-500',
              iconName: 'Code',
              type: 'normal',
              users: '12k',
              welcomeMessage: 'Ready to review.',
            }, 'Reviews pull requests with project context.'),
            manifest: standardManifest(
              'agent.market.code',
              'Code Reviewer',
              'Reviews pull requests with project context.',
              'Platform',
            ),
            status: 'active',
            tags: ['tech'],
            visibility: 'public',
          },
          {
            agentId: 'agent.my.docs',
            displayName: 'Docs Assistant',
            description: 'Drafts internal docs.',
            defaultCodeTaskIntent: defaultCodeTaskIntent({
              avatar: 'https://cdn.example.test/docs.png',
              author: 'Me',
              categoryId: 'writing',
              color: 'bg-blue-500',
              iconName: 'FileText',
              type: 'normal',
              users: '1',
            }, 'Drafts internal docs.'),
            manifest: standardManifest('agent.my.docs', 'Docs Assistant', 'Drafts internal docs.', 'Me'),
            status: 'draft',
            tags: ['writing'],
            visibility: 'private',
          },
        ];
        const scopedItems = params.scope === 'mine'
          ? items.filter((item) => item.visibility === 'private')
          : params.scope === 'market'
            ? items.filter((item) => item.visibility === 'public')
            : items;
        return {
          data: {
            items: scopedItems,
            pageInfo: {
              hasMore: false,
              mode: 'offset',
              page: 1,
              pageSize: params.pageSize,
              totalItems: String(scopedItems.length),
              totalPages: 1,
            },
          },
        };
      },
      async retrieve(id: string, params: Record<string, unknown>) {
        agentSdkCalls.push({ id, operation: 'ai.agents.retrieve', params });
        return {
          data: {
            agentId: id,
            displayName: 'Docs Assistant',
            description: 'Drafts internal docs.',
            defaultCodeTaskIntent: defaultCodeTaskIntent({
              avatar: 'https://cdn.example.test/docs.png',
              author: 'Me',
              categoryId: 'writing',
              color: 'bg-blue-500',
              iconName: 'FileText',
              type: 'normal',
              users: '1',
            }, 'Drafts internal docs.'),
            manifest: standardManifest(id, 'Docs Assistant', 'Drafts internal docs.', 'Me'),
            status: 'draft',
            tags: ['writing'],
            visibility: 'private',
          },
        };
      },
      async create(body: Record<string, unknown>, params: Record<string, unknown>) {
        agentSdkCalls.push({ body, operation: 'ai.agents.create', params });
        return {
          data: {
            agentId: body.agentId,
            code: body.code,
            defaultCodeTaskIntent: body.defaultCodeTaskIntent,
            displayName: body.displayName,
            description: body.description,
            manifest: body.manifest,
            status: 'draft',
            visibility: body.visibility,
            version: '1',
          },
        };
      },
      async update(id: string, body: Record<string, unknown>, params: Record<string, unknown>) {
        agentSdkCalls.push({ body, id, operation: 'ai.agents.update', params });
        return {
          data: {
            agentId: id,
            defaultCodeTaskIntent: body.defaultCodeTaskIntent,
            displayName: body.displayName ?? 'Updated Agent',
            description: body.description,
            manifest: body.manifest,
            status: 'draft',
            visibility: body.visibility ?? 'private',
            version: '2',
          },
        };
      },
      async delete(id: string, params: Record<string, unknown>) {
        agentSdkCalls.push({ id, operation: 'ai.agents.delete', params });
        return {
          data: {
            agentId: id,
            displayName: 'Deleted Agent',
            manifest: {},
            status: 'deleted',
            visibility: 'private',
            version: '3',
          },
        };
      },
      compositionSlots: {
        async list(id: string, params: Record<string, unknown>) {
          agentSdkCalls.push({ id, operation: 'ai.agents.compositionSlots.list', params });
          return {
            items: [],
            pageInfo: {
              hasMore: false,
              mode: 'offset',
              page: 1,
              pageSize: params.pageSize,
              totalItems: '0',
              totalPages: 1,
            },
          };
        },
        async create(id: string, body: Record<string, unknown>) {
          agentSdkCalls.push({ body, id, operation: 'ai.agents.compositionSlots.create' });
          return {
            data: {
              agentId: id,
              ...(body.data as Record<string, unknown> | undefined),
            },
          };
        },
        async delete(id: string, slotId: string) {
          agentSdkCalls.push({ id, operation: 'ai.agents.compositionSlots.delete', params: { slotId } });
          return { data: { accepted: true } };
        },
      },
      providerBindings: {
        async create(id: string, body: Record<string, unknown>, params: Record<string, unknown>) {
          agentSdkCalls.push({ body, id, operation: 'ai.agents.providerBindings.create', params });
          return {
            data: {
              active: Boolean(body.makeDefault),
              agentId: id,
              bindingId: body.bindingId,
              providerId: body.providerId,
              status: 'active',
            },
          };
        },
      },
      deployments: {
        async create(id: string, body: Record<string, unknown>, params: Record<string, unknown>) {
          agentSdkCalls.push({ body, id, operation: 'ai.agents.deployments.create', params });
          return {
            data: {
              agentId: id,
              deploymentId: `${id}:deployment`,
              status: 'active',
            },
          };
        },
      },
      previewResponses: {
        async create(id: string, body: Record<string, unknown>, params?: Record<string, unknown>) {
          agentSdkCalls.push({ body, id, operation: 'ai.agents.previewResponses.create', params });
          return {
            data: {
              agentId: id,
              completedAt: body.requestedAt,
              executionId: body.executionId,
              inputPayload: body.inputPayload ?? {},
              operation: 'preview_response',
              outputPayload: id === 'agent.no.output' ? {} : { reply: 'backend preview reply' },
              requestedAt: body.requestedAt,
              status: 'completed',
              tenantId: params?.tenantId ?? '100001',
            },
          };
        },
      },
      promptOptimizations: {
        async create(id: string, body: Record<string, unknown>, params?: Record<string, unknown>) {
          agentSdkCalls.push({ body, id, operation: 'ai.agents.promptOptimizations.create', params });
          return {
            data: {
              agentId: id,
              completedAt: body.requestedAt,
              executionId: body.executionId,
              inputPayload: body.inputPayload ?? {},
              operation: 'prompt_optimization',
              outputPayload: { optimizedPrompt: 'Answer user questions with verified backend context.' },
              requestedAt: body.requestedAt,
              status: 'completed',
              tenantId: params?.tenantId ?? '100001',
            },
          };
        },
      },
    },
  },
} as unknown as SdkworkAgentAppClient;

async function main(): Promise<void> {
  const service = createSdkworkAgentService(() => fakeAgentClient);

  const marketAgents = (await service.listAgentsPage({ scope: 'market', pageSize: 100 })).items;
  assert.deepEqual(
    marketAgents.map((agent) => ({
      author: agent.author,
      avatar: agent.avatar,
      categoryId: agent.categoryId,
      color: agent.color,
      description: agent.description,
      iconName: agent.iconName,
      id: agent.id,
      name: agent.name,
      type: agent.type,
      users: agent.users,
      welcomeMessage: agent.welcomeMessage,
    })),
    [
      {
        author: 'Platform',
        avatar: 'https://cdn.example.test/code.png',
        categoryId: 'tech',
        color: 'bg-green-500',
        description: 'Reviews pull requests with project context.',
        iconName: 'Code',
        id: 'agent.market.code',
        name: 'Code Reviewer',
        type: 'normal',
        users: '12k',
        welcomeMessage: 'Ready to review.',
      },
    ],
    'market agent catalog must be parsed from sdkwork-agent-app-sdk public agent records without local mock entries',
  );

  const myAgents = (await service.listAgentsPage({ scope: 'mine', pageSize: 100 })).items;
  assert.deepEqual(
    myAgents.map((agent) => ({
      author: agent.author,
      avatar: agent.avatar,
      categoryId: agent.categoryId,
      color: agent.color,
      description: agent.description,
      iconName: agent.iconName,
      id: agent.id,
      name: agent.name,
      type: agent.type,
      users: agent.users,
    })),
    [
      {
        author: 'Me',
        avatar: 'https://cdn.example.test/docs.png',
        categoryId: 'writing',
        color: 'bg-blue-500',
        description: 'Drafts internal docs.',
        iconName: 'FileText',
        id: 'agent.my.docs',
        name: 'Docs Assistant',
        type: 'normal',
        users: '1',
      },
    ],
    'my agent catalog must be parsed from sdkwork-agent-app-sdk owned agent records without local mock entries',
  );

  const createdAgent = await service.createAgent({
    avatar: 'https://cdn.example.test/new.png',
    categoryId: 'developer',
    description: 'new agent',
    knowledgeBaseIds: ['kb_docs'],
    name: 'New Agent',
    systemPrompt: 'Use project context.',
    type: 'normal',
    welcomeMessage: 'Ready.',
  });
  assert.equal(createdAgent.name, 'New Agent');
  assert.match(createdAgent.id ?? '', /^agent\.pc\.new-agent\.[a-z0-9]{12}$/u);

  const updatedAgent = await service.updateAgent('agent.my.docs', {
    description: 'updated agent',
    name: 'Updated Agent',
    type: 'independent',
  });
  assert.equal(updatedAgent.name, 'Updated Agent');
  assert.equal(updatedAgent.type, 'independent');

  await service.publishAgent('agent.my.docs');
  await service.deleteAgent('agent.my.docs');

  assert.deepEqual(
    agentSdkCalls.map((call) => call.operation),
    [
      'ai.agents.list',
      'ai.agents.list',
      'ai.agents.create',
      'ai.agents.compositionSlots.list',
      'ai.agents.compositionSlots.create',
      'ai.agents.retrieve',
      'ai.agents.update',
      'ai.agents.compositionSlots.list',
      'ai.agents.providerBindings.create',
      'ai.agents.delete',
    ],
    'agent catalog read/write operations must use sdkwork-agent-app-sdk ai.agents methods',
  );
  assert.deepEqual(
    agentSdkCalls[2]?.body,
    {
      agentId: createdAgent.id,
      code: createdAgent.id,
      defaultCodeTaskIntent: {
        constraints: [
          'agent.type=normal',
          uiConfigConstraint({
            author: undefined,
            avatar: 'https://cdn.example.test/new.png',
            categoryId: 'developer',
            color: undefined,
            debugMode: undefined,
            iconName: undefined,
            knowledgeBaseIds: ['kb_docs'],
            jsonMode: undefined,
            memoryEnabled: undefined,
            model: undefined,
            skillIds: undefined,
            suggestedPrompts: undefined,
            systemPrompt: 'Use project context.',
            temperature: undefined,
            toolIds: undefined,
            type: 'normal',
            users: undefined,
            voiceIds: undefined,
            welcomeMessage: 'Ready.',
          }),
        ],
        contextPaths: ['kb_docs'],
        prompt: 'Use project context.',
      },
      description: 'new agent',
      displayName: 'New Agent',
      implementationKind: 'manifest-only',
      implementationProviderId: null,
      managementProfile: {
        author: undefined,
        avatar: 'https://cdn.example.test/new.png',
        categoryId: 'developer',
        color: undefined,
        debugMode: undefined,
        iconName: undefined,
        knowledgeBaseIds: ['kb_docs'],
        jsonMode: undefined,
        memoryEnabled: undefined,
        model: undefined,
        skillIds: undefined,
        suggestedPrompts: undefined,
        systemPrompt: 'Use project context.',
        temperature: undefined,
        toolIds: undefined,
        type: 'normal',
        users: undefined,
        voiceIds: undefined,
        welcomeMessage: 'Ready.',
      },
      manifest: {
        ...standardManifest(createdAgent.id ?? '', 'New Agent', 'Use project context.'),
      },
      requestedAt: agentSdkCalls[2]?.body?.requestedAt,
      tags: ['developer'],
      visibility: 'private',
    },
    'agent creation must map the PC product model into the standardized agent app SDK DTO',
  );
  assert.deepEqual(
    agentSdkCalls[3],
    {
      id: createdAgent.id,
      operation: 'ai.agents.compositionSlots.list',
      params: { page: 1, pageSize: 20 },
    },
    'agent creation must inspect current composition slots before adding desired knowledge bindings',
  );
  assert.deepEqual(
    {
      data: (agentSdkCalls[4]?.body?.data as Record<string, unknown> | undefined),
      id: agentSdkCalls[4]?.id,
      operation: agentSdkCalls[4]?.operation,
    },
    {
      data: {
        tenantId: '100001',
        organizationId: '0',
        slotId: 'slot.kb.space.kb_docs',
        slotKind: 'knowledge',
        targetModule: 'knowledgebase',
        targetRef: 'kb.space.kb_docs',
        priority: '1',
        enabled: true,
        policyJson: '{}',
      },
      id: createdAgent.id,
      operation: 'ai.agents.compositionSlots.create',
    },
    'agent creation must persist knowledge-base composition slots through the agent SDK',
  );
  assert.deepEqual(
    agentSdkCalls[5],
    {
      id: 'agent.my.docs',
      operation: 'ai.agents.retrieve',
      params: undefined,
    },
    'agent update must retrieve the current record before merging partial management-profile edits',
  );
  assert.deepEqual(
    agentSdkCalls[6],
    {
      body: {
        defaultCodeTaskIntent: {
          constraints: [
            'agent.type=independent',
            uiConfigConstraint({
              author: 'Me',
              avatar: 'https://cdn.example.test/docs.png',
              categoryId: 'writing',
              color: 'bg-blue-500',
              debugMode: undefined,
              iconName: 'FileText',
              knowledgeBaseIds: undefined,
              jsonMode: undefined,
              memoryEnabled: undefined,
              model: undefined,
              skillIds: undefined,
              suggestedPrompts: undefined,
              systemPrompt: 'Drafts internal docs.',
              temperature: undefined,
              toolIds: undefined,
              type: 'independent',
              users: '1',
              voiceIds: undefined,
              welcomeMessage: undefined,
            }),
          ],
          contextPaths: [],
          prompt: 'Drafts internal docs.',
        },
        description: 'updated agent',
        displayName: 'Updated Agent',
        managementProfile: {
          author: 'Me',
          avatar: 'https://cdn.example.test/docs.png',
          categoryId: 'writing',
          color: 'bg-blue-500',
          debugMode: undefined,
          iconName: 'FileText',
          knowledgeBaseIds: undefined,
          jsonMode: undefined,
          memoryEnabled: undefined,
          model: undefined,
          skillIds: undefined,
          suggestedPrompts: undefined,
          systemPrompt: 'Drafts internal docs.',
          temperature: undefined,
          toolIds: undefined,
          type: 'independent',
          users: '1',
          voiceIds: undefined,
          welcomeMessage: undefined,
        },
        manifest: standardManifest('agent.my.docs', 'Updated Agent', 'Drafts internal docs.', 'Me'),
        requestedAt: agentSdkCalls[6]?.body?.requestedAt,
        tags: ['writing'],
      },
      id: 'agent.my.docs',
      operation: 'ai.agents.update',
      params: undefined,
    },
    'agent update must persist standard manifest and PC product metadata through sdkwork-agent-app-sdk',
  );
  assert.deepEqual(
    agentSdkCalls[7],
    {
      id: 'agent.my.docs',
      operation: 'ai.agents.compositionSlots.list',
      params: { page: 1, pageSize: 20 },
    },
    'agent update must diff composition slots through the agent SDK after saving metadata',
  );
  assert.deepEqual(
    agentSdkCalls[8],
    {
      body: {
        bindingId: 'binding.manifest.default',
        capabilities: ['model.chat', 'tool.invoke'],
        configurationProfileId: 'profile.agent.manifest.default',
        implementationKind: 'manifest-only',
        makeDefault: true,
        providerId: 'provider.agent.manifest',
        requestedAt: agentSdkCalls[8]?.body?.requestedAt,
      },
      id: 'agent.my.docs',
      operation: 'ai.agents.providerBindings.create',
      params: undefined,
    },
    'agent publish must prepare a standard default provider binding before deployment',
  );

  const preview = await service.requestPreviewResponse({
    config: {
      avatar: 'https://cdn.example.test/assistant.png',
      description: 'Replies with backend agent runtime results.',
      id: 'agent.my.docs',
      name: 'Docs Assistant',
      systemPrompt: 'Use concise internal documentation style.',
      type: 'normal',
    },
    content: 'summarize this release note',
    debugMode: true,
    memoryEnabled: false,
    model: 'sdkwork-agent-runtime',
    temperature: 0.4,
  });
  assert.equal(preview.content, 'backend preview reply');
  assert.match(preview.executionId, /^execution\.pc\.agent\.preview\./u);

  await assert.rejects(
    () =>
      service.requestPreviewResponse({
        config: {
          avatar: '',
          description: 'No generated reply in backend output.',
          id: 'agent.no.output',
          name: 'No Output Agent',
          type: 'normal',
        },
        content: 'this should not be faked locally',
      }),
    /did not return a preview response/,
    'agent preview must fail closed instead of synthesizing a local response when backend output has no reply',
  );

  const optimized = await service.optimizePrompt({
    config: {
      avatar: '',
      description: 'Optimizes prompts.',
      id: 'agent.my.docs',
      name: 'Docs Assistant',
      type: 'normal',
    },
    prompt: 'answer questions',
  });
  assert.equal(optimized.optimizedPrompt, 'Answer user questions with verified backend context.');
  assert.match(optimized.executionId, /^execution\.pc\.agent\.prompt\./u);

  assert.deepEqual(
    agentSdkCalls.slice(10).map((call) => ({
      agentId: call.id,
      content: call.body?.content,
      debugMode: call.body?.debugMode,
      executionIdPrefix: typeof call.body?.executionId === 'string'
        ? call.body.executionId.replace(/^execution\.pc\.agent\.(preview|prompt)\..+$/u, 'execution.pc.agent.$1.*')
        : undefined,
      inputPayload: call.body?.inputPayload,
      memoryEnabled: call.body?.memoryEnabled,
      model: call.body?.model,
      operation: call.operation,
      params: call.params,
      prompt: call.body?.prompt,
      temperature: call.body?.temperature,
    })),
    [
      {
        agentId: 'agent.my.docs',
        content: 'summarize this release note',
        debugMode: true,
        executionIdPrefix: 'execution.pc.agent.preview.*',
        inputPayload: {
          agent: {
            avatar: 'https://cdn.example.test/assistant.png',
            categoryId: undefined,
            debugMode: undefined,
            description: 'Replies with backend agent runtime results.',
            id: 'agent.my.docs',
            jsonMode: undefined,
            knowledgeBaseIds: undefined,
            memoryEnabled: undefined,
            model: undefined,
            name: 'Docs Assistant',
            skillIds: undefined,
            suggestedPrompts: undefined,
            systemPrompt: 'Use concise internal documentation style.',
            temperature: undefined,
            toolIds: undefined,
            type: 'normal',
            voiceIds: undefined,
            welcomeMessage: undefined,
          },
          content: 'summarize this release note',
          debugMode: true,
          memoryEnabled: false,
          model: 'model.sdkwork-agent-runtime',
          temperature: 0.4,
        },
        memoryEnabled: false,
        model: 'model.sdkwork-agent-runtime',
        operation: 'ai.agents.previewResponses.create',
        params: undefined,
        prompt: undefined,
        temperature: 0.4,
      },
      {
        agentId: 'agent.no.output',
        content: 'this should not be faked locally',
        debugMode: false,
        executionIdPrefix: 'execution.pc.agent.preview.*',
        inputPayload: {
          agent: {
            avatar: '',
            categoryId: undefined,
            debugMode: undefined,
            description: 'No generated reply in backend output.',
            id: 'agent.no.output',
            jsonMode: undefined,
            knowledgeBaseIds: undefined,
            memoryEnabled: undefined,
            model: undefined,
            name: 'No Output Agent',
            skillIds: undefined,
            suggestedPrompts: undefined,
            systemPrompt: undefined,
            temperature: undefined,
            toolIds: undefined,
            type: 'normal',
            voiceIds: undefined,
            welcomeMessage: undefined,
          },
          content: 'this should not be faked locally',
          debugMode: false,
          memoryEnabled: false,
          model: undefined,
          temperature: undefined,
        },
        memoryEnabled: false,
        model: undefined,
        operation: 'ai.agents.previewResponses.create',
        params: undefined,
        prompt: undefined,
        temperature: undefined,
      },
      {
        agentId: 'agent.my.docs',
        content: undefined,
        debugMode: undefined,
        executionIdPrefix: 'execution.pc.agent.prompt.*',
        inputPayload: {
          agent: {
            avatar: '',
            categoryId: undefined,
            debugMode: undefined,
            description: 'Optimizes prompts.',
            id: 'agent.my.docs',
            jsonMode: undefined,
            knowledgeBaseIds: undefined,
            memoryEnabled: undefined,
            model: undefined,
            name: 'Docs Assistant',
            skillIds: undefined,
            suggestedPrompts: undefined,
            systemPrompt: undefined,
            temperature: undefined,
            toolIds: undefined,
            type: 'normal',
            voiceIds: undefined,
            welcomeMessage: undefined,
          },
          prompt: 'answer questions',
        },
        memoryEnabled: undefined,
        model: undefined,
        operation: 'ai.agents.promptOptimizations.create',
        params: undefined,
        prompt: 'answer questions',
        temperature: undefined,
      },
    ],
    'agent preview and prompt optimization must use sdkwork-agent-app-sdk runtime execution endpoints',
  );

  console.log('sdkwork-im-pc agent catalog real-logic contract passed');
}

void main();
