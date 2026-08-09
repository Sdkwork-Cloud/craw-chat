import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "AIMusicPage";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/im-h5-ai-music");
    return { default: mod[name] };
  });
}

const AIMusicPage = lazyComponent("AIMusicPage");

export const musicgenModule: ImH5CapabilityModule = {
  id: "musicgen",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.aiMusic, render: () => <AIMusicPage /> },
  ],
};
