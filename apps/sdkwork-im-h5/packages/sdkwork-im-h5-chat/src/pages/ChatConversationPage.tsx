import { useEffect, useState } from 'react';
import {
  fetchConversationMessages,
  sendConversationText,
  type ConversationMessage,
} from '../services/chatConversationService';
import { subscribeConversationLiveMessages } from '../services/chatRealtimeService';

export interface ChatConversationPageProps {
  conversationId: string;
}

export function ChatConversationPage({ conversationId }: ChatConversationPageProps) {
  const [messages, setMessages] = useState<ConversationMessage[]>([]);
  const [input, setInput] = useState('');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let cancelled = false;

    async function loadMessages() {
      setLoading(true);
      try {
        const response = await fetchConversationMessages(conversationId);
        if (!cancelled) {
          setMessages(response.messages);
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    }

    void loadMessages();

    // Subscribe to live messages for this conversation so newly posted
    // messages are appended without polling.
    const unsubscribe = subscribeConversationLiveMessages(conversationId, (message) => {
      if (!cancelled) {
        setMessages((prev) => [...prev, message as unknown as ConversationMessage]);
      }
    });

    return () => {
      cancelled = true;
      unsubscribe();
    };
  }, [conversationId]);

  async function handleSend() {
    if (!input.trim()) return;
    const text = input.trim();
    setInput('');
    await sendConversationText(conversationId, text);
  }

  return (
    <div className="chat-conversation-page">
      <div className="chat-conversation-messages">
        {loading ? (
          <div className="chat-conversation-loading">Loading messages...</div>
        ) : (
          messages.map((message) => (
            <div key={message.id} className="chat-conversation-message">
              {message.text ?? ''}
            </div>
          ))
        )}
      </div>
      <div className="chat-conversation-input">
        <input
          type="text"
          value={input}
          onChange={(event) => setInput(event.target.value)}
          placeholder="Type a message..."
        />
        <button type="button" onClick={() => void handleSend()}>
          Send
        </button>
      </div>
    </div>
  );
}

export default ChatConversationPage;
