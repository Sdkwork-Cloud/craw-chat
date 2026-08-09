import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "ReportApp" | "CreateReport" | "ReportDetail";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/im-h5-report");
    return { default: mod[name] };
  });
}

const ReportApp = lazyComponent("ReportApp");
const CreateReport = lazyComponent("CreateReport");
const ReportDetail = lazyComponent("ReportDetail");

export const reportModule: ImH5CapabilityModule = {
  id: "report",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceReport, render: () => <ReportApp /> },
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceReportCreate, render: () => <CreateReport /> },
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceReportDetail, render: () => <ReportDetail /> },
  ],
};
