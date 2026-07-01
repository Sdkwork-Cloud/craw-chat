import React, { useMemo, useState } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { useTranslation } from 'react-i18next';
import { Chat, Message, User } from '@sdkwork/im-pc-types';
import { MessageList } from './MessageList';
import { MessageInput } from './MessageInput';
import { chatService } from '../services/ChatService';
import { SYSTEM_ASSISTANT_AGENT, systemAssistantService } from '../services/SystemAssistantService';
import { toast } from './Toast';
import { ChatHistoryModal } from './ChatHistoryModal';

interface ChatWindowProps {
  chat: Chat;
  messageSearchQuery?: string;
  onOpenGroupInvite?: (groupId: string) => Promise<void>;
}

export const ChatWindow: React.FC<ChatWindowProps> = ({ chat, messageSearchQuery = '', onOpenGroupInvite }) => {
  const { t } = useTranslation();
  const [refreshKey, setRefreshKey] = useState(0);
  const [replyingTo, setReplyingTo] = useState<Message['replyTo'] | undefined>();
  const [editingMessage, setEditingMessage] = useState<{ id: string; content: string } | null>(null);
  const [isHistoryOpen, setIsHistoryOpen] = useState(false);
  const [isTyping, setIsTyping] = useState(false);
  const isSystemAssistantChat = systemAssistantService.isSystemAssistantChat(chat);
  const assistantSenderProfiles = useMemo<Record<string, User>>(() => (
    isSystemAssistantChat
      ? {
          [SYSTEM_ASSISTANT_AGENT.id]: {
            avatar: SYSTEM_ASSISTANT_AGENT.avatar,
            id: SYSTEM_ASSISTANT_AGENT.id,
            name: t('chat.systemAssistant.displayName'),
            status: 'online',
          },
        }
      : ({} as Record<string, User>)
  ), [isSystemAssistantChat, t]);
  const assistantWelcomeMessages = useMemo<Message[]>(() => (
    isSystemAssistantChat
      ? [
          {
            chatId: chat.id,
            content: t('chat.systemAssistant.welcomeMessage'),
            id: `${chat.id}:system-assistant-welcome`,
            senderId: SYSTEM_ASSISTANT_AGENT.id,
            timestamp: Math.max(0, chat.updatedAt - 1),
            type: 'text',
          },
        ]
      : []
  ), [chat.id, chat.updatedAt, isSystemAssistantChat, t]);
  const agentSenderProfiles = useMemo<Record<string, User>>(() => (
    !isSystemAssistantChat && chat.welcomeMessage
      ? {
          [chat.id]: {
            avatar: chat.avatar,
            id: chat.id,
            name: chat.name,
            status: 'online',
          },
        }
      : {}
  ), [chat.avatar, chat.id, chat.name, chat.welcomeMessage, isSystemAssistantChat]);
  const agentWelcomeMessages = useMemo<Message[]>(() => (
    !isSystemAssistantChat && chat.welcomeMessage
      ? [
          {
            chatId: chat.id,
            content: chat.welcomeMessage,
            id: `${chat.id}:agent-welcome`,
            senderId: chat.id,
            timestamp: Math.max(0, chat.updatedAt - 1),
            type: 'text',
          },
        ]
      : []
  ), [chat.id, chat.updatedAt, chat.welcomeMessage, isSystemAssistantChat]);
  const displaySenderProfiles = isSystemAssistantChat ? assistantSenderProfiles : agentSenderProfiles;
  const displayWelcomeMessages = isSystemAssistantChat ? assistantWelcomeMessages : agentWelcomeMessages;

  const handleSend = async (content: string, type: Message['type'] = 'text', extraInfo?: Partial<Message>) => {
    try {
      await chatService.sendMessage(chat.id, content, type, replyingTo, extraInfo);
      setReplyingTo(undefined);
      setRefreshKey(prev => prev + 1);
    } catch (error) {
      toast(t('chat.window.toast.sendFailed'), 'error');
    }
  };

  const handleEditSubmit = async (messageId: string, text: string) => {
    try {
      await chatService.editMessage(chat.id, messageId, text);
      setEditingMessage(null);
      setRefreshKey(prev => prev + 1);
    } catch (error) {
      toast(t('chat.window.toast.editFailed'), 'error');
    }
  };

  return (
    <div className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0 min-h-0 relative">
      {/* Messages */}
      <MessageList
        chatId={chat.id}
        fallbackMessages={displayWelcomeMessages}
        refreshKey={refreshKey}
        searchQuery={messageSearchQuery}
        senderProfiles={displaySenderProfiles}
        onReply={(msg, senderName) => setReplyingTo({ id: msg.id, senderName, content: msg.content })}
        onEdit={(msg) => setEditingMessage({ id: msg.id, content: msg.content })}
        onOpenGroupInvite={onOpenGroupInvite}
      />

      {/* Typing Indicator */}
      <div className="relative w-full z-10 pointer-events-none">
        <AnimatePresence>
          {isTyping && (
            <motion.div
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, scale: 0.95 }}
              className="absolute bottom-4 left-8 flex items-center gap-2 bg-[#2b2b2d] px-4 py-2 rounded-2xl rounded-tl-sm shadow-sm max-w-max pointer-events-auto"
            >
              <div className="flex gap-1.5 items-center justify-center h-4">
                 <div className="w-1.5 h-1.5 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0ms' }} />
                 <div className="w-1.5 h-1.5 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '150ms' }} />
                 <div className="w-1.5 h-1.5 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '300ms' }} />
              </div>
              <span className="text-xs text-gray-400 ml-1">{t('chat.window.typing')}</span>
            </motion.div>
          )}
        </AnimatePresence>
      </div>

      {/* Input Area */}
      <MessageInput
        onSend={handleSend}
        placeholder={isSystemAssistantChat ? t('chat.systemAssistant.inputPlaceholder') : t('chat.window.inputPlaceholder')}
        replyingTo={replyingTo}
        isTyping={isTyping}
        editingMessage={editingMessage}
        onEditSubmit={handleEditSubmit}
        onCancelEdit={() => setEditingMessage(null)}
        onStop={() => {
           setIsTyping(false);
        }}
        onCancelReply={() => setReplyingTo(undefined)}
        onHistoryClick={() => setIsHistoryOpen(true)}
      />

      <ChatHistoryModal
        chat={chat}
        isOpen={isHistoryOpen}
        onClose={() => setIsHistoryOpen(false)}
        chatId={chat.id}
        chatName={chat.name}
        senderProfiles={displaySenderProfiles}
      />
    </div>
  );
};
