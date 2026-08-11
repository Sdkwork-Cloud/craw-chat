import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "CommunityList" | "CreateCommunity" | "CommunityDetail" | "CommunityProfile" | "CommunityGroupManagement" | "CreateCommunityGroup" | "CommunityEditField" | "CommunityEditImage" | "CommunityEditTabs" | "CommunityMembers" | "CommunityQRCode" | "CommunityPostCreate" | "CommunityGroupQRs" | "CircleCashierBridge";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/community-mobile-react-community");
    return { default: mod[name] };
  });
}

const CommunityList = lazyComponent("CommunityList");
const CreateCommunity = lazyComponent("CreateCommunity");
const CommunityDetail = lazyComponent("CommunityDetail");
const CommunityProfile = lazyComponent("CommunityProfile");
const CommunityGroupManagement = lazyComponent("CommunityGroupManagement");
const CreateCommunityGroup = lazyComponent("CreateCommunityGroup");
const CommunityEditField = lazyComponent("CommunityEditField");
const CommunityEditImage = lazyComponent("CommunityEditImage");
const CommunityEditTabs = lazyComponent("CommunityEditTabs");
const CommunityMembers = lazyComponent("CommunityMembers");
const CommunityQRCode = lazyComponent("CommunityQRCode");
const CommunityPostCreate = lazyComponent("CommunityPostCreate");
const CommunityGroupQRs = lazyComponent("CommunityGroupQRs");
const CircleCashierBridge = lazyComponent("CircleCashierBridge");

export const communityModule: ImH5CapabilityModule = {
  id: "community",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.communityList, render: () => <CommunityList /> },
    { ...IM_H5_ROUTE_DEFINITIONS.communityCreate, render: () => <CreateCommunity /> },
    { ...IM_H5_ROUTE_DEFINITIONS.communityDetail, render: () => <CommunityDetail /> },
    { ...IM_H5_ROUTE_DEFINITIONS.communityProfile, render: () => <CommunityProfile /> },
    { ...IM_H5_ROUTE_DEFINITIONS.communityProfileGroups, render: () => <CommunityGroupManagement /> },
    { ...IM_H5_ROUTE_DEFINITIONS.communityProfileGroupsEdit, render: () => <CreateCommunityGroup /> },
    { ...IM_H5_ROUTE_DEFINITIONS.communityProfileEdit, render: () => <CommunityEditField /> },
    { ...IM_H5_ROUTE_DEFINITIONS.communityProfileImage, render: () => <CommunityEditImage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.communityProfileTabs, render: () => <CommunityEditTabs /> },
    { ...IM_H5_ROUTE_DEFINITIONS.communityProfileMembers, render: () => <CommunityMembers /> },
    { ...IM_H5_ROUTE_DEFINITIONS.communityProfileQrCode, render: () => <CommunityQRCode /> },
    { ...IM_H5_ROUTE_DEFINITIONS.communityPostCreate, render: () => <CommunityPostCreate /> },
    { ...IM_H5_ROUTE_DEFINITIONS.communityGroupsCreate, render: () => <CreateCommunityGroup /> },
    { ...IM_H5_ROUTE_DEFINITIONS.communityGroupQrs, render: () => <CommunityGroupQRs /> },
    { ...IM_H5_ROUTE_DEFINITIONS.communityCashier,
      render: () => (
        <CircleCashierBridge
          orderDetailPath={IM_H5_ROUTE_DEFINITIONS.ordersDetail.path}
          orderCenterPath={IM_H5_ROUTE_DEFINITIONS.ordersCenter.path}
        />
      ),
    },
  ],
};
