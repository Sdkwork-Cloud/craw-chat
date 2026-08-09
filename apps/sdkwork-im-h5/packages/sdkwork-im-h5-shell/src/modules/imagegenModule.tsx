import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "AIImagePage";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/im-h5-ai-image");
    return { default: mod[name] };
  });
}

const AIImagePage = lazyComponent("AIImagePage");

export const imagegenModule: ImH5CapabilityModule = {
  id: "imagegen",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.aiImage, render: () => <AIImagePage /> },
  ],
};
