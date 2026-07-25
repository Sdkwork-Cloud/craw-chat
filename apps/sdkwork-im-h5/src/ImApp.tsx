import { lazy, Suspense } from 'react';
import { Route, Routes } from 'react-router-dom';
import { ChatConversationPage } from '@sdkwork/im-h5-chat';

const ChatInboxPage = lazy(() =>
  import('@sdkwork/im-h5-chat').then((module) => ({ default: module.ChatInboxPage })),
);

export const IM_APP_HOME_PATH = '/';

export interface ParsedConversationRoute {
  conversationId: string;
  messageId?: string;
}

export function parseConversationRoute(
  pathname: string,
): ParsedConversationRoute | null {
  const match = pathname.match(/^\/chat\/([^/]+)(?:\/message\/([^/]+))?$/u);
  if (!match) {
    return null;
  }
  return {
    conversationId: match[1],
    ...(match[2] ? { messageId: match[2] } : {}),
  };
}

export default function ImApp() {
  return (
    <Suspense fallback={null}>
      <Routes>
        <Route path={IM_APP_HOME_PATH} element={<ChatInboxPage />} />
        <Route path="/chat/:conversationId" element={<ChatConversationPage />} />
        <Route path="/chat/:conversationId/message/:messageId" element={<ChatConversationPage />} />
      </Routes>
    </Suspense>
  );
}
