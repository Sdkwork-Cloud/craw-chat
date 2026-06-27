import React from "react";

import { resolveAgentsPcEmbedUrl } from "../config/agentsEmbed";

export interface AgentsPcEmbedSurfaceProps {
  title?: string;
}

export const AgentsPcEmbedSurface: React.FC<AgentsPcEmbedSurfaceProps> = ({
  title = "SDKWork Agents",
}) => {
  const embedUrl = resolveAgentsPcEmbedUrl();

  return (
    <div className="flex h-full min-h-0 flex-1 flex-col bg-[#141414]">
      <iframe
        title={title}
        src={embedUrl}
        className="h-full w-full flex-1 border-0"
        allow="clipboard-read; clipboard-write"
      />
    </div>
  );
};
