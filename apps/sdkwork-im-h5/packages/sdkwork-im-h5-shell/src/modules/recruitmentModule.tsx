import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "RecruitmentApp" | "CreateJob" | "CandidateDetail";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/im-h5-recruitment");
    return { default: mod[name] };
  });
}

const RecruitmentApp = lazyComponent("RecruitmentApp");
const CreateJob = lazyComponent("CreateJob");
const CandidateDetail = lazyComponent("CandidateDetail");

export const recruitmentModule: ImH5CapabilityModule = {
  id: "recruitment",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceRecruitment, render: () => <RecruitmentApp /> },
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceRecruitmentCreate, render: () => <CreateJob /> },
    { ...IM_H5_ROUTE_DEFINITIONS.workspaceRecruitmentDetail, render: () => <CandidateDetail /> },
  ],
};
