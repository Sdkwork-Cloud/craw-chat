import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "AIVideoPage";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/im-h5-ai-video");
    return { default: mod[name] };
  });
}

const AIVideoPage = lazyComponent("AIVideoPage");

export const videogenModule: ImH5CapabilityModule = {
  id: "videogen",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.aiVideo, render: () => <AIVideoPage /> },
  ],
};
