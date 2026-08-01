import React from "react";

import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type DriveComponentName = "CloudDriveApp" | "CloudDriveShareClaimPage";

function lazyDriveComponent(name: DriveComponentName) {
  return React.lazy(async () => {
    const driveModule = await import("@sdkwork/im-h5-cloud-drive");
    return { default: driveModule[name] };
  });
}

const CloudDriveApp = lazyDriveComponent("CloudDriveApp");
const CloudDriveShareClaimPage = lazyDriveComponent("CloudDriveShareClaimPage");

export const driveModule: ImH5CapabilityModule = {
  id: "drive",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.driveWorkspace, render: () => <CloudDriveApp /> },
    {
      ...IM_H5_ROUTE_DEFINITIONS.driveShareClaim,
      render: () => <CloudDriveShareClaimPage />,
    },
  ],
};
