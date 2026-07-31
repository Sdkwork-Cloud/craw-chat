import { MessageCircle } from "lucide-react";
import { Navigate, useParams } from "react-router";

import { ChatConversationPage, ChatInboxPage, ChatLifecycle } from "@sdkwork/im-h5-chat";

import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

export interface ParsedConversationRoute {
  conversationId: string;
  conversationPath: string;
}

export const IM_APP_HOME_PATH = "/";

export function parseConversationRoute(pathname: string): ParsedConversationRoute | null {
  const match = /^\/chat\/([^/?#]+)/u.exec(pathname.trim());
  if (!match?.[1]) {
    return null;
  }
  const conversationId = decodeURIComponent(match[1]);
  return conversationId
    ? { conversationId, conversationPath: "/chat/" + conversationId }
    : null;
}

function ConversationRoute() {
  const { conversationId } = useParams();
  if (!conversationId) {
    return <Navigate to={IM_APP_HOME_PATH} replace />;
  }
  return <ChatConversationPage conversationId={conversationId} />;
}

export const chatModule: ImH5CapabilityModule = {
  id: "chat",
  lifecycle: ChatLifecycle,
  navigation: [
    { id: "chat", moduleId: "chat", path: "/", labelKey: "common.tabs.chat", icon: MessageCircle },
  ],
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.chatInbox, render: () => <ChatInboxPage /> },
    { ...IM_H5_ROUTE_DEFINITIONS.chatConversation, render: () => <ConversationRoute /> },
  ],
};
