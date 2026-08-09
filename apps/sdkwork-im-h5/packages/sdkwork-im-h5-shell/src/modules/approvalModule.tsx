import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "ApprovalApp" | "CreateApproval" | "ApprovalDetail";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/im-h5-approval");
    return { default: mod[name] };
  });
}

const ApprovalApp = lazyComponent("ApprovalApp");
const CreateApproval = lazyComponent("CreateApproval");
const ApprovalDetail = lazyComponent("ApprovalDetail");

export const approvalModule: ImH5CapabilityModule = {
  id: "approval",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceApproval, render: () => <ApprovalApp /> },
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceApprovalCreate, render: () => <CreateApproval /> },
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceApprovalDetail, render: () => <ApprovalDetail /> },
  ],
};
