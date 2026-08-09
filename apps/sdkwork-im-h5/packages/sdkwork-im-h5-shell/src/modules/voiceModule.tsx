import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "VoiceSummaryApp" | "AIVoiceSynthPage";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/im-h5-ai-voice");
    return { default: mod[name] };
  });
}

const VoiceSummaryApp = lazyComponent("VoiceSummaryApp");
const AIVoiceSynthPage = lazyComponent("AIVoiceSynthPage");

export const voiceModule: ImH5CapabilityModule = {
  id: "voice",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceVoiceSummary, render: () => <VoiceSummaryApp /> },
    { ...IM_H5_ROUTE_DEFINITIONS.aiVoiceSynth, render: () => <AIVoiceSynthPage /> },
  ],
};
