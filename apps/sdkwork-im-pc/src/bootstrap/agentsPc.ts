import { configureAgentService, configureKnowledgeSelectionAdapter } from "@sdkwork/agents-pc-agents";
import { knowledgeSelectionService as imKnowledgeSelectionService } from "@sdkwork/knowledgebase-pc-knowledge";
import { getAgentAppSdkClientWithSession } from "@sdkwork/im-pc-core/sdk/agentAppSdkClient";

export function bootstrapImAgentsPcIntegration(): void {
  configureAgentService(() => getAgentAppSdkClientWithSession() as never);
  configureKnowledgeSelectionAdapter({
    async getBases() {
      const bases = await imKnowledgeSelectionService.getBases();
      return bases.map((base) => ({
        id: base.id,
        name: base.name,
        description: base.description,
        type: base.type,
        updatedAt: typeof base.updatedAt === "number"
          ? new Date(base.updatedAt).toISOString()
          : base.updatedAt,
        documentCount: base.count,
        count: base.count,
        logo: base.logo,
      }));
    },
  });
}

export function resolveAgentsPcEmbedUrl(): string {
  const configured = import.meta.env.VITE_SDKWORK_IM_PC_AGENTS_EMBED_URL;
  if (typeof configured === "string" && configured.trim().length > 0) {
    return configured.trim();
  }
  return "http://127.0.0.1:5195";
}

export type AgentsPcEmbedMode = "package" | "iframe";

export function resolveAgentsPcEmbedMode(): AgentsPcEmbedMode {
  const configured = import.meta.env.VITE_SDKWORK_IM_PC_AGENTS_EMBED_MODE;
  if (configured === "iframe") {
    return "iframe";
  }
  return "package";
}
