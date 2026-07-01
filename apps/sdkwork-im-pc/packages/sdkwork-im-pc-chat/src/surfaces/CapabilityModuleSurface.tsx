import React from "react";
import type { TFunction } from "i18next";
import {
  isShellCapabilityModule,
  LazyCapabilityModuleRenderer,
  resolveWorkspaceAppTab,
} from "@sdkwork/im-pc-shell";
import { WorkspaceView } from "@sdkwork/im-pc-workspace";
import type { Chat, User } from "@sdkwork/im-pc-types";

import { AgentView, type Agent, CreateAgentView } from "@sdkwork/agents-pc-agents";
import { resolveAgentsPcEmbedMode } from "../config/agentsEmbed";
import { ContactsView } from "../pages/ContactsView";
import { FavoritesView } from "../pages/FavoritesView";
import type { CallType } from "../components/CallOverlay";
import { AgentsPcEmbedSurface } from "./AgentsPcEmbedSurface";

export interface CapabilityModuleSurfaceProps {
  activeTab: string;
  searchQuery: string;
  editAgentId?: string;
  pendingCommunityId: string | null;
  t: TFunction;
  onTabChange: (tab: string) => void;
  onEditAgentIdChange: (agentId?: string) => void;
  onOpenCreateAgentModal: () => void;
  onPendingCommunityHandled: () => void;
  onOpenAddFriend: () => void;
  onStartAgentChat: (agent: Agent) => Promise<void>;
  onEnterpriseStartChat: (enterpriseId: string, enterpriseName: string) => Promise<void>;
  onEnterpriseCall: (enterpriseId: string, enterpriseName: string) => void;
  onContactSendMessage: (user: User) => Promise<void>;
  onContactStartCall: (type: CallType, user: User) => void;
  onOpenGroup: (group: Chat) => Promise<void>;
  showToast: (message: string, kind?: "success" | "error" | "info") => void;
}

export const CapabilityModuleSurface: React.FC<CapabilityModuleSurfaceProps> = ({
  activeTab,
  searchQuery,
  editAgentId,
  pendingCommunityId,
  t,
  onTabChange,
  onEditAgentIdChange,
  onOpenCreateAgentModal,
  onPendingCommunityHandled,
  onOpenAddFriend,
  onStartAgentChat,
  onEnterpriseStartChat,
  onEnterpriseCall,
  onContactSendMessage,
  onContactStartCall,
  onOpenGroup,
  showToast,
}) => {
  const renderShellModule = (
    ModuleComponent: React.LazyExoticComponent<React.ComponentType<any>>,
  ) => {
    switch (activeTab) {
      case "orders":
        return <ModuleComponent />;
      case "shop":
        return <ModuleComponent onNavigateToOrders={() => onTabChange("orders")} />;
      case "notary":
      case "mail":
      case "drive":
      case "calendar":
      case "approval":
      case "report":
      case "attendance":
      case "knowledge":
      case "course":
      case "videogen":
      case "imagegen":
      case "voicegen":
      case "musicgen":
      case "writing":
        return <ModuleComponent />;
      case "voice":
        return (
          <ModuleComponent
            onSelectVoice={(voice: { name: string }) => {
              showToast(t("chat.toast.voiceLoading", { name: voice.name }), "success");
            }}
            onCreateVoice={() => {
              showToast(t("chat.toast.voiceCloneSoon"), "success");
            }}
          />
        );
      case "enterprise":
        return <ModuleComponent onStartChat={onEnterpriseStartChat} onCall={onEnterpriseCall} />;
      case "devices":
        return (
          <ModuleComponent
            onEditAgent={(id: string) => {
              onEditAgentIdChange(id);
              onTabChange("create-agent");
            }}
          />
        );
      case "community":
        return (
          <ModuleComponent
            initialCommunityId={pendingCommunityId ?? undefined}
            onInitialCommunityHandled={onPendingCommunityHandled}
          />
        );
      default:
        return <ModuleComponent />;
    }
  };

  if (isShellCapabilityModule(activeTab)) {
    return (
      <LazyCapabilityModuleRenderer
        activeTab={activeTab}
        renderModule={(_moduleId, ModuleComponent) => renderShellModule(ModuleComponent)}
      />
    );
  }

  switch (activeTab) {
    case "agent":
      if (resolveAgentsPcEmbedMode() === "iframe") {
        return <AgentsPcEmbedSurface />;
      }
      return (
        <AgentView
          onStartChat={onStartAgentChat}
          onCreateAgent={() => {
            onEditAgentIdChange(undefined);
            onOpenCreateAgentModal();
          }}
          onEditAgent={(id) => {
            onEditAgentIdChange(id);
            onTabChange("create-agent");
          }}
        />
      );
    case "create-agent":
      if (resolveAgentsPcEmbedMode() === "iframe") {
        return <AgentsPcEmbedSurface title="SDKWork Agents — Create" />;
      }
      return (
        <CreateAgentView
          onBack={() => {
            onTabChange("agent");
            onEditAgentIdChange(undefined);
          }}
          initialAgentId={editAgentId}
        />
      );
    case "workspace":
      return (
        <WorkspaceView
          onAppSelect={(appId) => {
            const tab = resolveWorkspaceAppTab(appId);
            if (tab) {
              onTabChange(tab);
            } else {
              showToast(t("chat.toast.workspaceAppUnavailable", { appId }), "error");
            }
          }}
        />
      );
    case "contacts":
      return (
        <ContactsView
          searchQuery={searchQuery}
          onSendMessage={onContactSendMessage}
          onStartCall={onContactStartCall}
          onAddFriend={onOpenAddFriend}
          onAppSelect={(appId) => {
            if (appId === "mail") onTabChange("mail");
          }}
          onOpenGroup={onOpenGroup}
        />
      );
    case "favorites":
      return <FavoritesView searchQuery={searchQuery} />;
    default:
      return (
        <div className="flex-1 flex items-center justify-center bg-[#1e1e1e]">
          <div className="text-gray-500 text-xl capitalize">{activeTab} Content</div>
        </div>
      );
  }
};
