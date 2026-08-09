import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "VipSubscriptionPage" | "TokenRechargePage";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/membership-mobile-react-subscription");
    return { default: mod[name] };
  });
}

const VipSubscriptionPage = lazyComponent("VipSubscriptionPage");
const TokenRechargePage = lazyComponent("TokenRechargePage");

export const membershipModule: ImH5CapabilityModule = {
  id: "membership",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.membershipVip, render: () => <VipSubscriptionPage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.membershipRecharge, render: () => <TokenRechargePage /> },
  ],
};
