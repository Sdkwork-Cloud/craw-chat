import { AppLayout } from "@sdkwork/im-h5-shell";
import { ChatConversationPage, ChatInboxPage } from "@sdkwork/im-h5-chat";

import { IM_APP_HOME_PATH } from "./constants/appRoutes";

interface ImAppProps {
  route: string;
}

function parseConversationRoute(route: string): { conversationId: string } | null {
  const match = route.match(/^\/chat\/conversations\/([^/]+)$/u);
  if (!match?.[1]) {
    return null;
  }
  return { conversationId: decodeURIComponent(match[1]) };
}

export function ImApp({ route }: ImAppProps) {
  const activePath = route.startsWith("/chat") ? route : IM_APP_HOME_PATH;
  const conversationRoute = parseConversationRoute(route);

  const renderRoute = () => {
    if (conversationRoute) {
      return <ChatConversationPage conversationId={conversationRoute.conversationId} />;
    }

    if (route === "/chat/inbox" || route === IM_APP_HOME_PATH) {
      return <ChatInboxPage />;
    }

    return (
      <div>
        <h2>Page Not Found</h2>
        <p>Unknown IM route: {route}</p>
        <a href="#/chat/inbox">Go to Inbox</a>
      </div>
    );
  };

  return <AppLayout activePath={activePath}>{renderRoute()}</AppLayout>;
}
