import { configureAgentService, configureKnowledgeSelectionAdapter } from '@sdkwork/agents-pc-agents';

import { getAgentAppSdkClientWithSession } from './agentAppSdkClient';

export function bootstrapAgentsPcForIm(): void {
  configureAgentService(() => getAgentAppSdkClientWithSession() as never);
  configureKnowledgeSelectionAdapter({
    async getBases() {
      const { knowledgeSelectionService } = await import('@sdkwork/knowledgebase-pc-knowledge');
      const bases = await knowledgeSelectionService.getBases();
      return bases.map((base) => ({
        id: String(base.id ?? ''),
        name: String(base.name ?? ''),
        description: typeof base.description === 'string' ? base.description : undefined,
        type: base.type === 'personal' || base.type === 'team' || base.type === 'all'
          ? base.type
          : undefined,
        updatedAt: typeof base.updatedAt === 'number'
          ? new Date(base.updatedAt).toISOString()
          : typeof base.updatedAt === 'string'
            ? base.updatedAt
            : undefined,
        documentCount: typeof base.count === 'number' ? base.count : undefined,
        count: typeof base.count === 'number' ? base.count : undefined,
        logo: typeof base.logo === 'string' ? base.logo : undefined,
      }));
    },
  } as Parameters<typeof configureKnowledgeSelectionAdapter>[0]);
}

export function resolveAgentsPcEmbedUrl(): string {
  const configured = import.meta.env.VITE_SDKWORK_IM_PC_AGENTS_EMBED_URL;
  if (typeof configured === 'string' && configured.trim().length > 0) {
    return configured.trim();
  }
  return 'http://127.0.0.1:5195';
}

export type AgentsPcEmbedMode = 'package' | 'iframe';

export function resolveAgentsPcEmbedMode(): AgentsPcEmbedMode {
  const configured = import.meta.env.VITE_SDKWORK_IM_PC_AGENTS_EMBED_MODE;
  if (configured === 'iframe') {
    return 'iframe';
  }
  return 'package';
}
