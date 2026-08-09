import React from "react";
import { Bot } from "lucide-react";

import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";
import { TabSolidBot } from "../navigation/solidTabIcons";

type ComponentName = "AgentList" | "AgentSearch" | "AgentCreate" | "AddFriend" | "Scan" | "OrganizationList" | "OrganizationDetail" | "AddressBook" | "NewFriends";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/im-h5-contacts");
    return { default: mod[name] };
  });
}

const AgentList = lazyComponent("AgentList");
const AgentSearch = lazyComponent("AgentSearch");
const AgentCreate = lazyComponent("AgentCreate");
const AddFriend = lazyComponent("AddFriend");
const Scan = lazyComponent("Scan");
const OrganizationList = lazyComponent("OrganizationList");
const OrganizationDetail = lazyComponent("OrganizationDetail");
const AddressBook = lazyComponent("AddressBook");
const NewFriends = lazyComponent("NewFriends");

export const contactsModule: ImH5CapabilityModule = {
  id: "contacts",
  navigation: [
    { id: "agents", moduleId: "contacts", path: "/agents", labelKey: "common.tabs.agents", icon: Bot, activeIcon: TabSolidBot },
  ],
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.agentsList, render: () => <AgentList /> },
    { ...IM_H5_ROUTE_DEFINITIONS.agentsSearch, render: () => <AgentSearch /> },
    { ...IM_H5_ROUTE_DEFINITIONS.agentsCreate, render: () => <AgentCreate /> },
    { ...IM_H5_ROUTE_DEFINITIONS.agentsEdit, render: () => <AgentCreate /> },
    { ...IM_H5_ROUTE_DEFINITIONS.contactsIndex, render: () => <AddressBook /> },
    { ...IM_H5_ROUTE_DEFINITIONS.contactsAddFriend, render: () => <AddFriend /> },
    { ...IM_H5_ROUTE_DEFINITIONS.contactsScan, render: () => <Scan /> },
    { ...IM_H5_ROUTE_DEFINITIONS.contactsFriendRequests, render: () => <NewFriends /> },
    { ...IM_H5_ROUTE_DEFINITIONS.contactsOrg, render: () => <OrganizationList /> },
    { ...IM_H5_ROUTE_DEFINITIONS.contactsOrgDetail, render: () => <OrganizationDetail /> },
  ],
};
