import React, { useEffect, useMemo, useState } from 'react';
import {
  Boxes,
  CalendarDays,
  FileText,
  Image as ImageIcon,
  LayoutList,
  Link2,
  Loader2,
  MessageSquareText,
  Mic,
  Music,
  Search,
  X,
} from 'lucide-react';
import { AnimatePresence, motion } from 'motion/react';
import { useTranslation } from 'react-i18next';
import type { Chat, Message, User } from '@sdkwork/im-pc-types';
import { Avatar, cn } from '@sdkwork/im-pc-commons';
import { chatService } from '../services/ChatService';
import { contactService } from '../services/ContactService';
import {
  createChatHistorySenderProfileIndex,
  resolveChatHistoryMessageSender,
} from '../services/ChatHistoryMessageDisplayModel';
import {
  CHAT_HISTORY_SEARCH_TABS,
  filterChatHistoryMessages,
  getChatHistoryMessageResultKind,
  getChatHistorySearchPlaceholderKey,
  type ChatHistorySearchTabId,
} from '../services/ChatHistorySearchModel';
import { toast } from './Toast';

interface ChatHistoryModalProps {
  chat?: Chat;
  chatId?: string;
  chatName?: string;
  groupMemberProfiles?: readonly User[];
  isOpen: boolean;
  onClose: () => void;
  senderProfiles?: Record<string, User>;
}

const EMPTY_USER_LIST: readonly User[] = [];
const EMPTY_SENDER_PROFILES: Record<string, User> = {};

const TAB_ICON_BY_ID: Record<ChatHistorySearchTabId, React.ReactNode> = {
  all: <LayoutList size={13} />,
  apps: <Boxes size={13} />,
  files: <FileText size={13} />,
  links: <Link2 size={13} />,
  media: <ImageIcon size={13} />,
  messages: <MessageSquareText size={13} />,
  music: <Music size={13} />,
  voice: <Mic size={13} />,
};

export const ChatHistoryModal: React.FC<ChatHistoryModalProps> = ({
  chat,
  chatId,
  chatName,
  groupMemberProfiles = EMPTY_USER_LIST,
  isOpen,
  onClose,
  senderProfiles = EMPTY_SENDER_PROFILES,
}) => {
  const { t } = useTranslation();
  const resolvedChatId = chat?.id ?? chatId ?? '';
  const resolvedChatName = chat?.name ?? chatName;
  const [activeTab, setActiveTab] = useState<ChatHistorySearchTabId>('all');
  const [messages, setMessages] = useState<Message[]>([]);
  const [contactProfiles, setContactProfiles] = useState<User[]>([]);
  const [resolvedSenderProfiles, setResolvedSenderProfiles] = useState<User[]>([]);
  const [query, setQuery] = useState('');
  const [selectedDate, setSelectedDate] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const currentUser = useMemo(() => contactService.getCurrentUser(), []);

  useEffect(() => {
    if (!isOpen) {
      return undefined;
    }

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        onClose();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, onClose]);

  useEffect(() => {
    if (!isOpen) {
      return;
    }

    setActiveTab('all');
    setQuery('');
    setSelectedDate('');
    setResolvedSenderProfiles([]);
  }, [isOpen, resolvedChatId]);

  useEffect(() => {
    if (!isOpen || !resolvedChatId) {
      return undefined;
    }

    let isStale = false;
    setIsLoading(true);
    chatService.getMessages(resolvedChatId)
      .then((nextMessages) => {
        if (!isStale) {
          setMessages(nextMessages);
        }
      })
      .catch(() => {
        if (!isStale) {
          setMessages([]);
          toast(t('chat.historySearch.toast.loadFailed'), 'error');
        }
      })
      .finally(() => {
        if (!isStale) {
          setIsLoading(false);
        }
      });

    return () => {
      isStale = true;
    };
  }, [isOpen, resolvedChatId, t]);

  useEffect(() => {
    if (!isOpen) {
      return undefined;
    }

    let isStale = false;
    contactService.getContacts()
      .then((contacts) => {
        if (!isStale) {
          setContactProfiles(contacts);
        }
      })
      .catch(() => {
        if (!isStale) {
          setContactProfiles([]);
        }
      });

    return () => {
      isStale = true;
    };
  }, [isOpen]);

  const senderProfileIndex = useMemo(() => (
    createChatHistorySenderProfileIndex(
      [...contactProfiles, ...groupMemberProfiles, ...resolvedSenderProfiles],
      senderProfiles,
    )
  ), [contactProfiles, groupMemberProfiles, resolvedSenderProfiles, senderProfiles]);

  useEffect(() => {
    if (!isOpen || messages.length === 0) {
      return undefined;
    }

    const missingSenderIds = Array.from(new Set(
      messages
        .map((message) => message.senderId)
        .filter((senderId) => (
          senderId !== 'me'
          && senderId !== currentUser.id
          && senderId !== currentUser.chatId
          && senderId !== 'system'
          && !senderProfileIndex[senderId]
          && !(chat?.type === 'single')
        )),
    ));

    if (missingSenderIds.length === 0) {
      return undefined;
    }

    let isStale = false;
    void Promise.all(missingSenderIds.map((senderId) => contactService.getUserById(senderId)))
      .then((users) => {
        if (isStale) {
          return;
        }
        const nextUsers = users.filter((user): user is User => Boolean(user));
        if (nextUsers.length === 0) {
          return;
        }
        setResolvedSenderProfiles((previousProfiles) => {
          const byId = new Map(previousProfiles.map((profile) => [profile.id, profile]));
          for (const profile of nextUsers) {
            byId.set(profile.id, profile);
          }
          return Array.from(byId.values());
        });
      })
      .catch(() => undefined);

    return () => {
      isStale = true;
    };
  }, [chat?.type, currentUser.chatId, currentUser.id, isOpen, messages, senderProfileIndex]);

  const filteredMessages = useMemo(() => (
    filterChatHistoryMessages(messages, {
      activeTab,
      date: selectedDate,
      query,
    })
  ), [activeTab, messages, query, selectedDate]);

  const tabCounts = useMemo(() => (
    CHAT_HISTORY_SEARCH_TABS.reduce((counts, tab) => {
      counts[tab.id] = filterChatHistoryMessages(messages, {
        activeTab: tab.id,
        date: selectedDate,
        query: '',
      }).length;
      return counts;
    }, {} as Record<ChatHistorySearchTabId, number>)
  ), [messages, selectedDate]);

  const activePlaceholder = t(getChatHistorySearchPlaceholderKey(activeTab));
  const resultCountText = t('chat.historySearch.resultCount', { count: filteredMessages.length });
  const fallbackMemberName = t('chat.fallback.memberName');

  return (
    <AnimatePresence>
      {isOpen && (
        <motion.div
          animate={{ opacity: 1 }}
          className="fixed inset-0 z-[100] flex items-center justify-center bg-black/55 backdrop-blur-sm"
          exit={{ opacity: 0 }}
          initial={{ opacity: 0 }}
          onClick={onClose}
          transition={{ duration: 0.15 }}
        >
          <motion.section
            animate={{ opacity: 1, scale: 1, y: 0 }}
            aria-labelledby="chat-history-search-title"
            aria-modal="true"
            className="flex h-[720px] max-h-[calc(100vh-40px)] w-[920px] max-w-[calc(100vw-32px)] flex-col overflow-hidden rounded-xl bg-[#242529] shadow-[0_24px_80px_rgba(0,0,0,0.46)]"
            exit={{ opacity: 0, scale: 0.96, y: 12 }}
            initial={{ opacity: 0, scale: 0.96, y: 12 }}
            onClick={(event) => event.stopPropagation()}
            role="dialog"
            transition={{ duration: 0.18, ease: 'easeOut' }}
          >
            <header className="flex shrink-0 items-center justify-between bg-[#2b2c30] px-5 py-4">
              <div className="min-w-0">
                <h2 className="truncate text-base font-semibold text-gray-100" id="chat-history-search-title">
                  {t('chat.historySearch.title')}
                </h2>
                <div className="mt-1 flex items-center gap-2 text-xs text-gray-400">
                  <span className="truncate">{resolvedChatName || t('chat.historySearch.currentChat')}</span>
                  <span className="h-1 w-1 rounded-full bg-gray-600" />
                  <span>{resultCountText}</span>
                </div>
              </div>
              <button
                aria-label={t('chat.historySearch.actions.close')}
                className="flex h-8 w-8 items-center justify-center rounded-lg text-gray-400 transition-colors hover:bg-white/10 hover:text-gray-100"
                onClick={onClose}
                type="button"
              >
                <X size={18} />
              </button>
            </header>

            <div className="shrink-0 bg-[#242529] px-5 py-4 shadow-[inset_0_1px_0_rgba(255,255,255,0.03)]">
              <div className="flex flex-col gap-3 lg:flex-row">
                <label className="relative min-w-0 flex-1">
                  <Search className="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" size={17} />
                  <input
                    aria-label={activePlaceholder}
                    autoFocus
                    className="h-10 w-full rounded-lg bg-[#17181b] pl-10 pr-10 text-sm text-gray-100 outline-none transition-shadow placeholder:text-gray-500 focus:ring-2 focus:ring-indigo-400/45"
                    onChange={(event) => setQuery(event.target.value)}
                    placeholder={activePlaceholder}
                    type="search"
                    value={query}
                  />
                  {query && (
                    <button
                      aria-label={t('chat.historySearch.actions.clearSearch')}
                      className="absolute right-2 top-1/2 flex h-6 w-6 -translate-y-1/2 items-center justify-center rounded text-gray-500 transition-colors hover:bg-white/10 hover:text-gray-200"
                      onClick={() => setQuery('')}
                      type="button"
                    >
                      <X size={14} />
                    </button>
                  )}
                </label>
                <label className="flex h-10 min-w-[190px] items-center gap-2 rounded-lg bg-[#17181b] px-3 text-sm text-gray-300 transition-shadow focus-within:ring-2 focus-within:ring-indigo-400/45">
                  <CalendarDays className="text-gray-500" size={16} />
                  <span className="sr-only">{t('chat.historySearch.dateAria')}</span>
                  <input
                    aria-label={t('chat.historySearch.dateAria')}
                    className="min-w-0 flex-1 bg-transparent text-sm text-gray-200 outline-none [color-scheme:dark]"
                    onChange={(event) => setSelectedDate(event.target.value)}
                    type="date"
                    value={selectedDate}
                  />
                  {selectedDate && (
                    <button
                      aria-label={t('chat.historySearch.actions.clearDate')}
                      className="flex h-6 w-6 items-center justify-center rounded text-gray-500 transition-colors hover:bg-white/10 hover:text-gray-200"
                      onClick={() => setSelectedDate('')}
                      type="button"
                    >
                      <X size={13} />
                    </button>
                  )}
                </label>
              </div>

              <div
                aria-label={t('chat.historySearch.tabsAria')}
                className="mt-4 flex items-center gap-2 overflow-x-auto pb-1 custom-scrollbar"
                role="tablist"
              >
                {CHAT_HISTORY_SEARCH_TABS.map((tab) => {
                  const isActive = activeTab === tab.id;
                  return (
                    <button
                      aria-selected={isActive}
                      className={cn(
                        'flex h-7 shrink-0 items-center gap-1.5 rounded-md px-2 text-xs font-medium transition-colors',
                        isActive
                          ? 'bg-indigo-500/18 text-indigo-100 shadow-[inset_0_0_0_1px_rgba(129,140,248,0.16)]'
                          : 'text-gray-400 hover:bg-white/[0.06] hover:text-gray-200',
                      )}
                      key={tab.id}
                      onClick={() => setActiveTab(tab.id)}
                      role="tab"
                      type="button"
                    >
                      {TAB_ICON_BY_ID[tab.id]}
                      <span>{t(tab.labelKey)}</span>
                      <span className={cn(
                        'ml-0.5 text-[11px]',
                        isActive ? 'text-indigo-100/80' : 'text-gray-500',
                      )}
                      >
                        {tabCounts[tab.id] ?? 0}
                      </span>
                    </button>
                  );
                })}
              </div>
            </div>

            <div className="min-h-0 flex-1 overflow-y-auto bg-[#202124] py-3 custom-scrollbar">
              {isLoading ? (
                <div className="flex h-full flex-col items-center justify-center text-sm text-gray-400">
                  <Loader2 className="mb-3 animate-spin text-indigo-300" size={30} />
                  {t('chat.historySearch.state.loading')}
                </div>
              ) : filteredMessages.length === 0 ? (
                <div className="flex h-full flex-col items-center justify-center px-8 text-center">
                  <Search className="mb-4 text-gray-600" size={36} />
                  <div className="text-sm font-medium text-gray-300">{t('chat.historySearch.state.emptyTitle')}</div>
                  <div className="mt-2 max-w-[360px] text-sm leading-6 text-gray-500">
                    {t('chat.historySearch.state.emptyDescription')}
                  </div>
                </div>
              ) : (
                <div className="flex flex-col gap-1" role="list">
                  {filteredMessages.map((message) => {
                    const sender = resolveChatHistoryMessageSender({
                      chat,
                      currentUser,
                      fallbackMemberName,
                      message,
                      senderProfiles: senderProfileIndex,
                    });
                    return (
                      <ChatHistoryMessageResult
                        key={message.id}
                        message={message}
                        resultKindLabel={t(`chat.historySearch.type.${getChatHistoryMessageResultKind(message)}`)}
                        sender={sender}
                        timestamp={formatTimestamp(message.timestamp)}
                      />
                    );
                  })}
                </div>
              )}
            </div>
          </motion.section>
        </motion.div>
      )}
    </AnimatePresence>
  );
};

function ChatHistoryMessageResult({
  message,
  resultKindLabel,
  sender,
  timestamp,
}: {
  message: Message;
  resultKindLabel: string;
  sender: ReturnType<typeof resolveChatHistoryMessageSender>;
  timestamp: string;
}): React.ReactElement {
  return (
    <article className="group flex px-5 py-3 transition-colors hover:bg-white/[0.035]" role="listitem">
      <div className="flex min-w-0 flex-1 gap-3">
        <Avatar
          alt={sender.name}
          className="mt-1 h-9 w-9 rounded bg-[#2b2b2d] text-[12px] shadow-[0_0_0_1px_rgba(255,255,255,0.05)]"
          fallback={sender.name.slice(0, 1)}
          src={sender.avatar}
        />
        <div className="min-w-0 flex-1">
          <div className="mb-1 flex min-w-0 items-center gap-2">
            <span className="truncate text-[12px] font-medium text-gray-300">{sender.name}</span>
            <span className="shrink-0 text-[11px] text-gray-500">{timestamp}</span>
            <span className="shrink-0 rounded bg-white/[0.06] px-1.5 py-0.5 text-[10px] text-gray-500">{resultKindLabel}</span>
          </div>
          {message.replyTo && (
            <div className="mb-1.5 max-w-[520px] truncate rounded bg-white/[0.055] px-3 py-1.5 text-[12px] text-gray-400">
              <span className="mr-1 font-medium text-gray-300">{message.replyTo.senderName}:</span>
              {message.replyTo.content}
            </div>
          )}
          <div className="min-w-0 text-[14px] font-medium leading-6 text-gray-100">
            {renderHistoryMessagePlainText(message)}
          </div>
        </div>
      </div>
    </article>
  );
}

function renderHistoryMessagePlainText(message: Message): string {
  switch (message.type) {
    case 'text':
      return message.content;
    case 'image':
      return message.fileName || message.desc || message.content || 'Image';
    case 'video':
      return message.fileName || message.desc || message.content || 'Video';
    case 'voice':
      return message.duration ? `Voice message ${message.duration}''` : (message.content || 'Voice message');
    case 'link':
      return message.fileName || message.desc || message.content || 'Link';
    case 'applet':
      return message.fileName || message.desc || message.content || 'Mini app';
    case 'card':
      return message.fileName || message.desc || message.content || 'Contact card';
    case 'file':
      return [message.fileName || message.content || 'File', message.fileSize].filter(Boolean).join(' · ');
    case 'music':
      return message.fileName || message.desc || message.content || 'Music';
    default:
      return message.content;
  }
}

function formatTimestamp(timestamp: number): string {
  const date = new Date(timestamp);
  if (!Number.isFinite(date.getTime())) {
    return '';
  }
  return new Intl.DateTimeFormat(undefined, {
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    month: 'short',
  }).format(date);
}
