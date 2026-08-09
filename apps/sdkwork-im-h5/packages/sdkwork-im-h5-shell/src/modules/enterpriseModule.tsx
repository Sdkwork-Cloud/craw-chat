import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "EnterpriseCenter" | "EnterpriseSearch" | "EnterpriseInvite" | "EnterpriseJoin" | "EnterprisePostJob" | "EnterprisePostSupply" | "EnterprisePostDemand" | "EnterpriseYellowPages" | "EnterpriseSite" | "EnterpriseRecruitment" | "EnterpriseProducts";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/im-h5-enterprise");
    return { default: mod[name] };
  });
}

const EnterpriseCenter = lazyComponent("EnterpriseCenter");
const EnterpriseSearch = lazyComponent("EnterpriseSearch");
const EnterpriseInvite = lazyComponent("EnterpriseInvite");
const EnterpriseJoin = lazyComponent("EnterpriseJoin");
const EnterprisePostJob = lazyComponent("EnterprisePostJob");
const EnterprisePostSupply = lazyComponent("EnterprisePostSupply");
const EnterprisePostDemand = lazyComponent("EnterprisePostDemand");
const EnterpriseYellowPages = lazyComponent("EnterpriseYellowPages");
const EnterpriseSite = lazyComponent("EnterpriseSite");
const EnterpriseRecruitment = lazyComponent("EnterpriseRecruitment");
const EnterpriseProducts = lazyComponent("EnterpriseProducts");

export const enterpriseModule: ImH5CapabilityModule = {
  id: "enterprise",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.enterpriseCenter, render: () => <EnterpriseCenter /> },
    { ...IM_H5_ROUTE_DEFINITIONS.enterpriseSearch, render: () => <EnterpriseSearch /> },
    { ...IM_H5_ROUTE_DEFINITIONS.enterpriseInvite, render: () => <EnterpriseInvite /> },
    { ...IM_H5_ROUTE_DEFINITIONS.enterpriseJoin, render: () => <EnterpriseJoin /> },
    { ...IM_H5_ROUTE_DEFINITIONS.enterprisePostJob, render: () => <EnterprisePostJob /> },
    { ...IM_H5_ROUTE_DEFINITIONS.enterprisePostSupply, render: () => <EnterprisePostSupply /> },
    { ...IM_H5_ROUTE_DEFINITIONS.enterprisePostDemand, render: () => <EnterprisePostDemand /> },
    { ...IM_H5_ROUTE_DEFINITIONS.enterpriseYellowPages, render: () => <EnterpriseYellowPages /> },
    { ...IM_H5_ROUTE_DEFINITIONS.enterpriseSite, render: () => <EnterpriseSite /> },
    { ...IM_H5_ROUTE_DEFINITIONS.enterpriseRecruitment, render: () => <EnterpriseRecruitment /> },
    { ...IM_H5_ROUTE_DEFINITIONS.enterpriseProducts, render: () => <EnterpriseProducts /> },
  ],
};
