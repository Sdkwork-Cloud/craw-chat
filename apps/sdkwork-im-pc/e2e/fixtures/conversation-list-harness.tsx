import React, { useState } from 'react';
import { createRoot } from 'react-dom/client';
import { I18nextProvider } from 'react-i18next';
import type { Chat } from '@sdkwork/im-pc-types';

import { ChatList } from '../../packages/sdkwork-im-pc-chat/src/components/ChatList';
import i18n from '../../packages/sdkwork-im-pc-chat/src/i18n';
import '../../src/index.css';

declare global {
  interface Window {
    __chatListHarnessLoadMoreCalls: number;
  }
}

const conversationCount = Number.parseInt(
  new URLSearchParams(window.location.search).get('count') ?? '10000',
  10,
);
const referenceTime = Date.UTC(2026, 0, 1);
const chats: Chat[] = Array.from({ length: conversationCount }, (_, ordinal) => ({
  id: `conversation-${ordinal}`,
  name: `Conversation ${ordinal.toString().padStart(5, '0')}`,
  type: 'single',
  unreadCount: ordinal % 17 === 0 ? 1 : 0,
  updatedAt: referenceTime - ordinal * 1_000,
}));
window.__chatListHarnessLoadMoreCalls = 0;

function ConversationListHarness() {
  const [activeChatId, setActiveChatId] = useState<string>();

  return (
    <I18nextProvider i18n={i18n}>
      <main className="flex h-screen min-h-0 bg-[#181818] text-white">
        <ChatList
          chats={chats}
          activeChatId={activeChatId}
          onChatSelect={(chat) => setActiveChatId(chat.id)}
          hasMoreChats
          onLoadMoreChats={() => {
            window.__chatListHarnessLoadMoreCalls += 1;
          }}
        />
      </main>
    </I18nextProvider>
  );
}

void i18n.changeLanguage('en-US').then(() => {
  const root = document.getElementById('root');
  if (!root) {
    throw new Error('Conversation list harness root is unavailable.');
  }
  createRoot(root).render(<ConversationListHarness />);
});
