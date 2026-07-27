import { useCallback, useEffect, useState } from 'react';
import { useNavigate } from 'react-router';
import type {
  ConversationInboxPage as ConversationInboxPageView,
} from '@sdkwork/im-sdk';
import { subscribeInboxLiveRefresh } from '../services/chatRealtimeService';

interface ChatInboxPageProps {
  onOpenConversation?: (conversationId: string) => void;
}

export function ChatInboxPage({ onOpenConversation }: ChatInboxPageProps) {
  const navigate = useNavigate();
  const [inbox, setInbox] = useState<ConversationInboxPageView | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  const refreshInbox = useCallback(async () => {
    setIsLoading(true);
    try {
      // Inbox list refresh placeholder; production code wires the IM SDK list call here.
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    void refreshInbox();
    const unsubscribe = subscribeInboxLiveRefresh(() => {
      void refreshInbox();
    });
    return () => {
      unsubscribe();
    };
  }, [refreshInbox]);

  const handleOpenConversation = useCallback((conversationId: string) => {
    if (onOpenConversation) {
      onOpenConversation(conversationId);
      return;
    }
    navigate(`/chat/${conversationId}`);
  }, [navigate, onOpenConversation]);

  return (
    <div className="sdkwork-im-h5-chat-inbox">
      <header className="sdkwork-im-h5-chat-inbox-header">
        <h1>Conversations</h1>
      </header>
      <ul className="sdkwork-im-h5-chat-inbox-list">
        {inbox?.items?.length
          ? inbox.items.map((item) => (
              <li key={item.conversationId}>
                <button
                  type="button"
                  onClick={() => handleOpenConversation(item.conversationId)}
                >
                  <span>{item.title ?? item.conversationId}</span>
                </button>
              </li>
            ))
          : !isLoading && <li className="sdkwork-im-h5-chat-inbox-empty">No conversations</li>}
      </ul>
    </div>
  );
}

export default ChatInboxPage;
