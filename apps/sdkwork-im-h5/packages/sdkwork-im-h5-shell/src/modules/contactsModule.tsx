import React from "react";
import { Bot } from "lucide-react";
import { useNavigate } from "react-router";

import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";
import { TabSolidBot } from "../navigation/solidTabIcons";
import { showToast } from "@sdkwork/im-h5-commons";
import type { AgentConfig } from "@sdkwork/agents-h5-agents";

type MarketplaceComponentName = "AgentMarketplaceMobileView" | "AgentMarketplaceSearchView";
type ContactsComponentName = "AddFriend" | "Scan" | "OrganizationList" | "OrganizationDetail" | "AddressBook" | "NewFriends";

function lazyMarketplaceComponent(name: MarketplaceComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/agents-h5-agents");
    return { default: mod[name] };
  });
}

function lazyContactsComponent(name: ContactsComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/im-h5-contacts");
    return { default: mod[name] };
  });
}

const GroupChatListPage = React.lazy(async () => {
  const mod = await import("@sdkwork/im-h5-chat");
  return { default: mod.GroupChatListPage };
});

const AgentMarketplaceMobileView = lazyMarketplaceComponent("AgentMarketplaceMobileView");
const AgentMarketplaceSearchView = lazyMarketplaceComponent("AgentMarketplaceSearchView");
const AddFriend = lazyContactsComponent("AddFriend");
const Scan = lazyContactsComponent("Scan");
const OrganizationList = lazyContactsComponent("OrganizationList");
const OrganizationDetail = lazyContactsComponent("OrganizationDetail");
const AddressBook = lazyContactsComponent("AddressBook");
const NewFriends = lazyContactsComponent("NewFriends");

/** Start an agent session: navigate to the agents chat route with display info. */
function useStartAgentChat() {
  const navigate = useNavigate();
  return (agent: AgentConfig) => {
    navigate(`/agent/chat/${encodeURIComponent(agent.id ?? "")}`, {
      state: {
        agent: {
          name: agent.name,
          welcomeMessage: agent.welcomeMessage,
        },
      },
    });
  };
}

function AgentMarketplaceRoute() {
  const navigate = useNavigate();
  const startChat = useStartAgentChat();
  return (
    <AgentMarketplaceMobileView
      onStartChat={startChat}
      onCreateAgent={() => navigate("/agent/create")}
      onSearch={() => navigate("/agent-search")}
      notify={(message) => showToast(message)}
    />
  );
}

function AgentMarketplaceSearchRoute() {
  const navigate = useNavigate();
  const startChat = useStartAgentChat();
  return (
    <AgentMarketplaceSearchView
      onStartChat={startChat}
      onBack={() => navigate(-1)}
      notify={(message) => showToast(message)}
    />
  );
}

export const contactsModule: ImH5CapabilityModule = {
  id: "contacts",
  navigation: [
    { id: "agents", moduleId: "contacts", path: "/agents", labelKey: "common.tabs.agents", icon: Bot, activeIcon: TabSolidBot },
  ],
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.agentsList, render: () => <AgentMarketplaceRoute /> },
    { ...IM_H5_ROUTE_DEFINITIONS.agentsSearch, render: () => <AgentMarketplaceSearchRoute /> },
    { ...IM_H5_ROUTE_DEFINITIONS.contactsIndex, render: () => <AddressBook /> },
    { ...IM_H5_ROUTE_DEFINITIONS.contactsAddFriend, render: () => <AddFriend /> },
    { ...IM_H5_ROUTE_DEFINITIONS.contactsScan, render: () => <Scan /> },
    { ...IM_H5_ROUTE_DEFINITIONS.contactsFriendRequests, render: () => <NewFriends /> },
    { ...IM_H5_ROUTE_DEFINITIONS.contactsGroupChats, render: () => <GroupChatListPage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.contactsOrg, render: () => <OrganizationList /> },
    { ...IM_H5_ROUTE_DEFINITIONS.contactsOrgDetail, render: () => <OrganizationDetail /> },
  ],
};
