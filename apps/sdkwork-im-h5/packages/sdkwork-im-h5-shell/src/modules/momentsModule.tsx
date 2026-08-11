import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

const MomentsPage = React.lazy(async () => {
  const mod = await import("@sdkwork/im-h5-moments");
  return { default: mod.MomentsPage };
});

export const momentsModule: ImH5CapabilityModule = {
  id: "moments",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.momentsDiscover, render: () => <MomentsPage /> },
  ],
};
