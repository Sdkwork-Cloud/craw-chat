import { useCallback, useEffect, useRef, useState } from 'react';
import { Loader2, RefreshCw } from 'lucide-react';
import { MAX_LIST_PAGE_SIZE } from '@sdkwork/utils';
import type {
  ConversationMessageEntry,
  ConversationMessageListResponse,
} from '@sdkwork/im-h5-core/sdk';
import {
  listMessages,
  markConversationRead,
  postText,
} from '../services/chatConversationService';
import {
  subscribeConversationLiveMessages,
} from '../services/chatRealtimeService';

interface ChatConversationPageProps {
  conversationId: string;
}

const MESSAGE_PAGE_SIZE = 50;

export function ChatConversationPage({ conversationId }: ChatConversationPageProps) {
  const [messages, setMessages] = useState<ConversationMessageEntry[]>([]);
  const [nextCursor, setNextCursor] = useState<string | undefined>();
  const [draft, setDraft] = useState('');
  const [isLoading, setIsLoading] = useState(true);
  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const [isSending, setIsSending] = useState(false);
  const [error, setError] = useState(false);
  const requestGeneration = useRef(0);

  const fetchConversationMessages = useCallback(async (cursor?: string) => {
    if (!conversationId) {
      return;
    }

    const generation = ++requestGeneration.current;
    if (cursor) {
      setIsLoadingMore(true);
    } else {
      setIsLoading(true);
      setError(false);
    }
    try {
      const response: ConversationMessageListResponse = await listMessages(conversationId, {
        params: {
          pageSize: Math.min(MESSAGE_PAGE_SIZE, MAX_LIST_PAGE_SIZE),
          ...(cursor ? { cursor } : {}),
        },
      });
      assertMessageCursorPage(response);
      if (generation !== requestGeneration.current) {
        return;
      }
      setMessages((previous) => mergeMessages(cursor ? previous : [], response.items));
      setNextCursor(
        response.pageInfo.hasMore && response.pageInfo.nextCursor
          ? response.pageInfo.nextCursor
          : undefined,
      );
      if (!cursor) {
        void markConversationRead(conversationId, response.highWatermark).catch(() => undefined);
      }
    } catch {
      if (generation === requestGeneration.current) {
        setError(true);
      }
    } finally {
      if (generation === requestGeneration.current) {
        setIsLoading(false);
        setIsLoadingMore(false);
      }
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
      requestGeneration.current += 1;
      unsubscribe();
    };
  }, [conversationId, fetchConversationMessages]);

  return (
    <div className="sdkwork-im-h5-chat-conversation">
      <header className="sdkwork-im-h5-chat-conversation-header">
        <h1>Conversation</h1>
        <button
          type="button"
          aria-label="Refresh messages"
          onClick={() => void fetchConversationMessages()}
        >
          <RefreshCw aria-hidden="true" />
        </button>
      </header>
      {isLoading && <Loader2 aria-label="Loading messages" />}
      {!isLoading && error && (
        <button type="button" onClick={() => void fetchConversationMessages()}>
          Retry
        </button>
      )}
      <ul className="sdkwork-im-h5-chat-conversation-messages">
        {messages.map((message) => (
          <li key={message.messageId}>{message.body.text ?? message.summary ?? message.messageType}</li>
        ))}
      </ul>
      {!isLoading && !error && nextCursor && messages.length < MAX_LIST_PAGE_SIZE && (
        <button
          type="button"
          disabled={isLoadingMore}
          onClick={() => void fetchConversationMessages(nextCursor)}
        >
          {isLoadingMore ? 'Loading...' : 'Load older messages'}
        </button>
      )}
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

function assertMessageCursorPage(response: ConversationMessageListResponse): void {
  if (response.pageInfo.mode !== 'cursor') {
    throw new Error('IM message history must use cursor pagination');
  }
  if (response.pageInfo.hasMore && !response.pageInfo.nextCursor) {
    throw new Error('IM message history returned hasMore without nextCursor');
  }
}

function mergeMessages(
  previous: readonly ConversationMessageEntry[],
  incoming: readonly ConversationMessageEntry[],
): ConversationMessageEntry[] {
  const merged: ConversationMessageEntry[] = [];
  const messageIds = new Set<string>();
  for (const message of [...previous, ...incoming]) {
    if (merged.length >= MAX_LIST_PAGE_SIZE || messageIds.has(message.messageId)) {
      continue;
    }
    messageIds.add(message.messageId);
    merged.push(message);
  }
  return merged;
}

export default ChatConversationPage;
