import { MessageCircle } from "lucide-react";
import { Navigate, useParams } from "react-router";

import { ChatDetail, ChatList, ChatLifecycle, ChatProfile, CreateGroupChat, VideoCall, VoiceCall } from "@sdkwork/im-h5-chat";

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
  return <ChatDetail />;
}

export const chatModule: ImH5CapabilityModule = {
  id: "chat",
  lifecycle: ChatLifecycle,
  navigation: [
    { id: "chat", moduleId: "chat", path: "/", labelKey: "common.tabs.chat", icon: MessageCircle },
  ],
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.chatInbox, render: () => <ChatList /> },
    { ...IM_H5_ROUTE_DEFINITIONS.chatConversation, render: () => <ConversationRoute /> },
    { ...IM_H5_ROUTE_DEFINITIONS.chatProfile, render: () => <ChatProfile /> },
    { ...IM_H5_ROUTE_DEFINITIONS.chatCreateGroup, render: () => <CreateGroupChat /> },
    { ...IM_H5_ROUTE_DEFINITIONS.chatVoiceCall, render: () => <VoiceCall /> },
    { ...IM_H5_ROUTE_DEFINITIONS.chatVideoCall, render: () => <VideoCall /> },
  ],
};
