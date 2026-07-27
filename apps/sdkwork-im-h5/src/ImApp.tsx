import { useCallback, useMemo, type ReactNode } from 'react';
import { Navigate, Route, Routes, useLocation, useNavigate } from 'react-router-dom';
import { ChatConversationPage } from '@sdkwork/im-h5-chat';

export interface ImAppProps {
  children?: ReactNode;
}

export interface ParsedConversationRoute {
  conversationId: string;
  conversationPath: string;
}

export const IM_APP_HOME_PATH = '/';

export function parseConversationRoute(pathname: string): ParsedConversationRoute | null {
  if (!pathname || typeof pathname !== 'string') {
    return null;
  }

  const trimmed = pathname.trim();
  const match = /^\/chat\/([^/?#]+)/u.exec(trimmed);
  if (!match) {
    return null;
  }

  const conversationId = decodeURIComponent(match[1] ?? '');
  if (!conversationId) {
    return null;
  }

  return {
    conversationId,
    conversationPath: `/chat/${conversationId}`,
  };
}

export function ImApp({ children }: ImAppProps) {
  const location = useLocation();
  const navigate = useNavigate();

  const parsed = useMemo(() => parseConversationRoute(location.pathname), [location.pathname]);

  const handleOpenConversation = useCallback((conversationId: string) => {
    navigate(`/chat/${conversationId}`);
  }, [navigate]);

  if (parsed && parsed.conversationId) {
    return (
      <Routes>
        <Route
          path="/chat/:conversationId"
          element={<ChatConversationPage conversationId={parsed.conversationId} />}
        />
      </Routes>
    );
  }

  return (
    <Routes>
      <Route path="/" element={<Navigate to={IM_APP_HOME_PATH} replace />} />
      <Route
        path="/chat/:conversationId"
        element={
          <ConversationRouteRenderer onOpenConversation={handleOpenConversation} />
        }
      />
      {children}
    </Routes>
  );
}

function ConversationRouteRenderer({
  onOpenConversation,
}: {
  onOpenConversation: (conversationId: string) => void;
}) {
  const location = useLocation();
  const parsed = parseConversationRoute(location.pathname);

  if (!parsed) {
    return <Navigate to={IM_APP_HOME_PATH} replace />;
  }

  return (
    <ChatConversationPage
      conversationId={parsed.conversationId}
      key={parsed.conversationId}
    />
  );
}

export default ImApp;
