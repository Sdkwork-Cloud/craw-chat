export type AgentsPcEmbedMode = "package" | "iframe";

export function resolveAgentsPcEmbedMode(): AgentsPcEmbedMode {
  const configured = import.meta.env.VITE_SDKWORK_IM_PC_AGENTS_EMBED_MODE;
  if (configured === "iframe") {
    return "iframe";
  }
  return "package";
}

export function resolveAgentsPcEmbedUrl(): string {
  const configured = import.meta.env.VITE_SDKWORK_IM_PC_AGENTS_EMBED_URL;
  if (typeof configured === "string" && configured.trim().length > 0) {
    return configured.trim();
  }
  return "http://127.0.0.1:5195";
}
