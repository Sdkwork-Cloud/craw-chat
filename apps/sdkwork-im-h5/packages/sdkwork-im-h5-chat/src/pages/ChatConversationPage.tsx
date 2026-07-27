import { useCallback, useEffect, useState } from 'react';
import type {
  ConversationMessageListResponse,
  ImDecodedMessage,
} from '@sdkwork/im-sdk';
import { listMessages, postText } from '../services/chatConversationService';
import {
  subscribeConversationLiveMessages,
} from '../services/chatRealtimeService';

interface ChatConversationPageProps {
  conversationId: string;
}

export function ChatConversationPage({ conversationId }: ChatConversationPageProps) {
  const [messages, setMessages] = useState<ImDecodedMessage[]>([]);
  const [draft, setDraft] = useState('');
  const [isSending, setIsSending] = useState(false);

  const fetchConversationMessages = useCallback(async () => {
    if (!conversationId) {
      return;
    }
    try {
      const response: ConversationMessageListResponse = await listMessages(conversationId);
      setMessages(response.items ?? []);
    } catch {
      // ignore fetch errors in H5 placeholder
    }
  }, [conversationId]);

  const sendConversationText = useCallback(async (text: string) => {
    const trimmed = text.trim();
    if (!trimmed || !conversationId || isSending) {
      return;
    }
    setIsSending(true);
    try {
      await postText(conversationId, trimmed);
      setDraft('');
      await fetchConversationMessages();
    } finally {
      setIsSending(false);
    }
  }, [conversationId, fetchConversationMessages, isSending]);

  useEffect(() => {
    void fetchConversationMessages();
    const unsubscribe = subscribeConversationLiveMessages(conversationId, () => {
      void fetchConversationMessages();
    });
    return () => {
      unsubscribe();
    };
  }, [conversationId, fetchConversationMessages]);

  return (
    <div className="sdkwork-im-h5-chat-conversation">
      <header className="sdkwork-im-h5-chat-conversation-header">
        <h1>Conversation</h1>
      </header>
      <ul className="sdkwork-im-h5-chat-conversation-messages">
        {messages.map((message) => (
          <li key={message.messageId}>{message.text ?? JSON.stringify(message)}</li>
        ))}
      </ul>
      <form
        className="sdkwork-im-h5-chat-conversation-composer"
        onSubmit={(event) => {
          event.preventDefault();
          void sendConversationText(draft);
        }}
      >
        <input
          type="text"
          value={draft}
          onChange={(event) => setDraft(event.currentTarget.value)}
          placeholder="Send a message"
        />
        <button type="submit" disabled={isSending}>Send</button>
      </form>
    </div>
  );
}

export default ChatConversationPage;
