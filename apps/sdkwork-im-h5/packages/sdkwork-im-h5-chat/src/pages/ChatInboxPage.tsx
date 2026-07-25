import { useEffect, useState } from 'react';
import { subscribeInboxLiveRefresh } from '../services/chatRealtimeService';
import type { ConversationMessage } from '../services/chatConversationService';

export interface ChatInboxPageProps {
  onSelectConversation?: (conversationId: string) => void;
}

export function ChatInboxPage({ onSelectConversation }: ChatInboxPageProps) {
  const [conversations, setConversations] = useState<ConversationMessage[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Subscribe to live inbox refresh events so the conversation list
    // re-fetches whenever an inbox-scoped realtime event fires.
    const unsubscribe = subscribeInboxLiveRefresh(() => {
      setLoading(true);
      // TODO: fetch latest conversations and call setConversations
      void conversations;
      setLoading(false);
    });

    return () => {
      unsubscribe();
    };
  }, [conversations]);

  return (
    <div className="chat-inbox-page">
      {loading ? (
        <div className="chat-inbox-loading">Loading conversations...</div>
      ) : (
        <div className="chat-inbox-list">
          {/* Conversation list items rendered here */}
        </div>
      )}
      {onSelectConversation ? null : null}
    </div>
  );
}

export default ChatInboxPage;
