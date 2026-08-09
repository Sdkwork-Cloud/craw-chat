import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "AIWritingPage";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/im-h5-ai-writing");
    return { default: mod[name] };
  });
}

const AIWritingPage = lazyComponent("AIWritingPage");

export const writingModule: ImH5CapabilityModule = {
  id: "writing",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.aiWriting, render: () => <AIWritingPage /> },
  ],
};
