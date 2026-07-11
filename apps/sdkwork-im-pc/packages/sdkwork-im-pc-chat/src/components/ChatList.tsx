import React, { useMemo, useRef, useState } from 'react';
import { useVirtualizer } from '@tanstack/react-virtual';
import { useTranslation } from 'react-i18next';
import { Chat } from '@sdkwork/im-pc-types';
import { Avatar } from '@sdkwork/im-pc-commons';
import { cn } from '@sdkwork/im-pc-commons';
import { ContextMenu, ContextMenuItem } from './ContextMenu';
import { Pin, BellOff, Trash2, CheckCircle, MessageCircle } from 'lucide-react';
import { toast } from './Toast';
import { chatService } from '../services/ChatService';
import { groupService } from '../services/GroupService';
import { ConfirmModal } from './ConfirmModal';

const RTC_CALL_DESCRIPTOR_PREFIX = 'rtc-call:';
const CHAT_LIST_ROW_HEIGHT = 64;
const CHAT_LIST_OVERSCAN = 8;

interface ChatListProps {
  chats: Chat[];
  activeChatId?: string;
  onChatSelect: (chat: Chat) => void;
  onChatsChange?: () => void;
  onChatDeleted?: (chatId: string) => void;
  searchQuery?: string;
  hasMoreChats?: boolean;
  loadingMoreChats?: boolean;
  onLoadMoreChats?: () => void;
}

interface RtcCallDescriptor {
  actorId?: string;
  initiatorId?: string;
  mode?: string;
  receiverId?: string;
  state?: string;
}

type TranslationFunction = (key: string, options?: Record<string, unknown>) => string;

function parseJsonRecord(value: unknown): Record<string, unknown> | undefined {
  if (value && typeof value === 'object' && !Array.isArray(value)) {
    return value as Record<string, unknown>;
  }
  if (typeof value !== 'string' || value.trim().length === 0) {
    return undefined;
  }
  try {
    const parsed: unknown = JSON.parse(value);
    return parsed && typeof parsed === 'object' && !Array.isArray(parsed)
      ? parsed as Record<string, unknown>
      : undefined;
  } catch {
    return undefined;
  }
}

function pickString(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function readRtcCallDescriptor(chat: Chat): RtcCallDescriptor | undefined {
  const message = chat.lastMessage;
  if (message?.type !== 'video_call' || !message.desc?.startsWith(RTC_CALL_DESCRIPTOR_PREFIX)) {
    return undefined;
  }

  try {
    const parsed = parseJsonRecord(decodeURIComponent(message.desc.slice(RTC_CALL_DESCRIPTOR_PREFIX.length)));
    if (!parsed) {
      return undefined;
    }

    return {
      actorId: pickString(parsed.actorId),
      initiatorId: pickString(parsed.initiatorId),
      mode: pickString(parsed.mode),
      receiverId: pickString(parsed.receiverId),
      state: pickString(parsed.state),
    };
  } catch {
    return undefined;
  }
}

function formatRtcCallMode(value: string | undefined, translate: TranslationFunction): string {
  return value && /video/iu.test(value)
    ? translate('chat.header.videoCall')
    : translate('chat.header.voiceCall');
}

function replaceRtcPreviewParticipantId(content: string, participantId: string | undefined, displayName: string): string {
  if (!participantId || participantId === displayName) {
    return content;
  }
  return content.split(participantId).join(displayName);
}

function formatRtcCallPreviewContent(
  chat: Chat,
  descriptor: RtcCallDescriptor,
  translate: TranslationFunction,
): string {
  const mode = formatRtcCallMode(descriptor.mode, translate);
  switch (descriptor.state) {
    case 'accepted':
      return translate('chat.list.callPreview.accepted', { mode });
    case 'rejected':
      return translate('chat.list.callPreview.rejected', { mode });
    case 'ended':
      return translate('chat.list.callPreview.ended', { mode });
    case 'started':
      return translate('chat.list.callPreview.started', {
        mode,
        name: chat.name,
      });
    default:
      return [descriptor.actorId, descriptor.initiatorId, descriptor.receiverId]
        .reduce<string>(
          (preview, participantId) => (
            participantId
              ? replaceRtcPreviewParticipantId(preview, participantId, chat.name)
              : preview
          ),
          chat.lastMessage?.content ?? mode,
        );
  }
}

function formatChatListLastMessage(chat: Chat, translate: TranslationFunction): string | undefined {
  const content = chat.lastMessage?.content;
  if (typeof content !== 'string') {
    return content;
  }

  const descriptor = readRtcCallDescriptor(chat);
  if (!descriptor) {
    return content;
  }

  return formatRtcCallPreviewContent(chat, descriptor, translate);
}

export const ChatList: React.FC<ChatListProps> = ({
  chats,
  activeChatId,
  onChatSelect,
  onChatsChange,
  onChatDeleted,
  searchQuery = '',
  hasMoreChats = false,
  loadingMoreChats = false,
  onLoadMoreChats,
}) => {
  const { i18n, t } = useTranslation();
  const [contextMenu, setContextMenu] = useState<{ x: number; y: number; chat: Chat } | null>(null);
  const [pendingDeleteChat, setPendingDeleteChat] = useState<Chat | null>(null);
  const listContainerRef = useRef<HTMLDivElement>(null);

  const handleListScroll = () => {
    const element = listContainerRef.current;
    if (!element || !hasMoreChats || loadingMoreChats || !onLoadMoreChats) {
      return;
    }
    const remaining = element.scrollHeight - element.scrollTop - element.clientHeight;
    if (remaining < 120) {
      onLoadMoreChats();
    }
  };

  const handleContextMenu = (event: React.MouseEvent, chat: Chat) => {
    event.preventDefault();
    setContextMenu({ x: event.clientX, y: event.clientY, chat });
  };

  const handleDeleteChat = async (chat: Chat) => {
    try {
      if (chat.type === 'group') {
        await groupService.deleteGroup(chat.id);
      } else {
        await chatService.deleteChat(chat.id);
      }
      onChatDeleted?.(chat.id);
      onChatsChange?.();
      toast(
        t(chat.type === 'group' ? 'chat.rightPanel.toast.groupLeft' : 'chat.list.toast.deleted'),
        'success',
      );
    } catch {
      toast(t('chat.list.toast.operationFailed'), 'error');
    }
  };

  const getContextMenuItems = (): ContextMenuItem[] => {
    if (!contextMenu) return [];
    const chat = contextMenu.chat;
    const isPinned = !!chat.isPinned;
    const isUnread = chat.unreadCount > 0 || !!chat.isMarkedUnread;
    const isMuted = !!chat.isMuted;

    return [
      {
        id: 'pin',
        label: t(isPinned ? 'chat.list.context.unpin' : 'chat.list.context.pin'),
        icon: <Pin size={14} className={isPinned ? 'rotate-45' : ''} />,
        onClick: async () => {
          try {
            await chatService.pinChat(chat.id, !isPinned);
            onChatsChange?.();
            toast(t(isPinned ? 'chat.list.toast.unpinned' : 'chat.list.toast.pinned'), 'success');
          } catch {
            toast(t('chat.list.toast.operationFailed'), 'error');
          }
        },
      },
      {
        id: 'read',
        label: t(isUnread ? 'chat.list.context.markRead' : 'chat.list.context.markUnread'),
        icon: isUnread ? <CheckCircle size={14} /> : <MessageCircle size={14} />,
        onClick: async () => {
          try {
            if (isUnread) {
              await chatService.markAsRead(chat.id);
            } else {
              await chatService.markAsUnread(chat.id);
            }
            onChatsChange?.();
            toast(t(isUnread ? 'chat.list.toast.markedRead' : 'chat.list.toast.markedUnread'), 'success');
          } catch {
            toast(t('chat.list.toast.operationFailed'), 'error');
          }
        },
      },
      {
        id: 'mute',
        label: t(isMuted ? 'chat.list.context.unmute' : 'chat.list.context.mute'),
        icon: <BellOff size={14} />,
        onClick: async () => {
          try {
            await chatService.muteChat(chat.id, !isMuted);
            onChatsChange?.();
            toast(t(isMuted ? 'chat.list.toast.unmuted' : 'chat.list.toast.muted'), 'success');
          } catch {
            toast(t('chat.list.toast.operationFailed'), 'error');
          }
        },
      },
      { id: 'div1', label: '', divider: true, onClick: () => {} },
      {
        id: 'delete',
        label: t('chat.list.context.delete'),
        icon: <Trash2 size={14} />,
        danger: true,
        onClick: () => {
          setPendingDeleteChat(chat);
        },
      },
    ];
  };

  const formatTime = (timestamp: number) => {
    const date = new Date(timestamp);
    const now = new Date();

    if (date.toDateString() === now.toDateString()) {
      return `${date.getHours().toString().padStart(2, '0')}:${date
        .getMinutes()
        .toString()
        .padStart(2, '0')}`;
    }

    const yesterday = new Date(now);
    yesterday.setDate(yesterday.getDate() - 1);
    if (date.toDateString() === yesterday.toDateString()) {
      return t('chat.list.time.yesterday');
    }

    try {
      return new Intl.DateTimeFormat(i18n.language || 'zh-CN', { weekday: 'short' }).format(date);
    } catch {
      return new Intl.DateTimeFormat('zh-CN', { weekday: 'short' }).format(date);
    }
  };

  const sortedChats = useMemo(() => {
    return [...chats]
      .filter((chat) => {
        if (!searchQuery.trim()) return true;
        const query = searchQuery.toLowerCase();
        const lastMessagePreview = formatChatListLastMessage(chat, t);
        return (
          chat.name.toLowerCase().includes(query)
          || (
            typeof lastMessagePreview === 'string'
            && lastMessagePreview.toLowerCase().includes(query)
          )
        );
      })
      .sort((a, b) => {
        const aPinned = !!a.isPinned;
        const bPinned = !!b.isPinned;
        if (aPinned && !bPinned) return -1;
        if (!aPinned && bPinned) return 1;
        return b.updatedAt - a.updatedAt;
      });
  }, [chats, searchQuery, t]);

  const chatVirtualizer = useVirtualizer({
    count: sortedChats.length,
    getScrollElement: () => listContainerRef.current,
    estimateSize: () => CHAT_LIST_ROW_HEIGHT,
    overscan: CHAT_LIST_OVERSCAN,
    getItemKey: (index) => sortedChats[index]?.id ?? index,
  });

  const focusVirtualRow = (index: number) => {
    const boundedIndex = Math.min(Math.max(index, 0), sortedChats.length - 1);
    chatVirtualizer.scrollToIndex(boundedIndex, { align: 'auto' });
    let remainingAttempts = 5;
    const focusWhenMounted = () => {
      const row = listContainerRef.current?.querySelector<HTMLButtonElement>(
        `[data-conversation-index="${boundedIndex}"]`,
      );
      if (row) {
        row.focus();
        return;
      }
      remainingAttempts -= 1;
      if (remainingAttempts > 0) {
        requestAnimationFrame(focusWhenMounted);
      }
    };
    requestAnimationFrame(focusWhenMounted);
  };

  const handleRowKeyDown = (
    event: React.KeyboardEvent<HTMLButtonElement>,
    index: number,
  ) => {
    const visiblePageSize = Math.max(
      1,
      Math.floor((listContainerRef.current?.clientHeight ?? CHAT_LIST_ROW_HEIGHT) / CHAT_LIST_ROW_HEIGHT),
    );
    const targetIndex = (() => {
      switch (event.key) {
        case 'ArrowDown': return index + 1;
        case 'ArrowUp': return index - 1;
        case 'Home': return 0;
        case 'End': return sortedChats.length - 1;
        case 'PageDown': return index + visiblePageSize;
        case 'PageUp': return index - visiblePageSize;
        default: return undefined;
      }
    })();
    if (targetIndex === undefined) {
      return;
    }
    event.preventDefault();
    focusVirtualRow(targetIndex);
  };

  return (
    <div className="flex w-[280px] shrink-0 flex-col bg-[#202020] border-r border-white/5 min-h-0">
      <div
        ref={listContainerRef}
        data-testid="chat-conversation-list"
        className="flex-1 overflow-y-auto custom-scrollbar relative"
        onScroll={handleListScroll}
      >
        {sortedChats.length === 0 ? (
          <div className="absolute inset-0 flex flex-col items-center justify-center text-center p-6">
            <div className="w-16 h-16 rounded-full bg-white/5 flex items-center justify-center mb-3">
              <MessageCircle size={28} className="text-gray-500" />
            </div>
            <p className="text-sm font-medium text-gray-400 mb-1">
              {searchQuery ? t('chat.list.empty.noMatches') : t('chat.list.empty.noConversations')}
            </p>
            <p className="text-[12px] text-gray-500">
              {searchQuery ? t('chat.list.empty.searchHint') : t('chat.list.empty.startHint')}
            </p>
          </div>
        ) : (
          <ul
            className="relative m-0 w-full list-none p-0"
            style={{ height: `${chatVirtualizer.getTotalSize()}px` }}
          >
            {chatVirtualizer.getVirtualItems().map((virtualRow) => {
              const chat = sortedChats[virtualRow.index];
              const isPinned = !!chat.isPinned;
              const isUnread = chat.unreadCount > 0 || !!chat.isMarkedUnread;
              const isMuted = !!chat.isMuted;
              const unreadCount = chat.unreadCount || 1;
              const openConversationLabel = t('chat.list.item.openConversation', { name: chat.name });
              const unreadLabel = t('chat.list.item.unreadCount', { count: unreadCount });
              const mutedLabel = t('chat.list.item.muted');
              const lastMessagePreview = formatChatListLastMessage(chat, t);

              return (
                <li
                  key={chat.id}
                  aria-posinset={virtualRow.index + 1}
                  aria-setsize={sortedChats.length}
                  className="absolute left-0 top-0 w-full"
                  style={{
                    height: `${virtualRow.size}px`,
                    transform: `translateY(${virtualRow.start}px)`,
                  }}
                >
                  <button
                    type="button"
                    data-conversation-index={virtualRow.index}
                    aria-label={openConversationLabel}
                    aria-current={activeChatId === chat.id ? 'true' : undefined}
                    title={openConversationLabel}
                    onClick={() => {
                      onChatSelect(chat);
                    }}
                    onContextMenu={(event) => handleContextMenu(event, chat)}
                    onKeyDown={(event) => handleRowKeyDown(event, virtualRow.index)}
                    className={cn(
                      'flex h-full w-full items-center border-0 bg-transparent px-3 py-3 text-left cursor-pointer transition-colors hover:bg-white/5',
                      activeChatId === chat.id && 'bg-white/10 hover:bg-white/10',
                      isPinned && activeChatId !== chat.id && 'bg-[#2b2b2d] hover:bg-[#323234]',
                    )}
                  >
                    <div className="relative shrink-0 mr-3">
                      <Avatar src={chat.avatar} alt={chat.name} className="w-[40px] h-[40px] rounded bg-[#2b2b2d] text-white font-bold" />
                      {isUnread && (
                        <div className={cn(
                          'absolute -top-1 -right-1 rounded-full border-2 border-[#202020]',
                          isMuted
                            ? 'w-2.5 h-2.5 bg-red-500'
                            : 'px-1.5 min-w-[18px] h-[18px] bg-red-500 text-white text-[10px] font-bold flex items-center justify-center',
                        )}
                          aria-label={unreadLabel}
                          title={unreadLabel}
                        >
                          {!isMuted && unreadCount}
                        </div>
                      )}
                    </div>
                    <div className="flex-1 min-w-0 flex flex-col justify-center">
                      <div className="flex items-center justify-between mb-1">
                        <div className="flex items-center gap-1 min-w-0">
                          <span className="text-[14px] text-gray-200 truncate">{chat.name}</span>
                          {isMuted && <BellOff size={12} className="text-gray-500 shrink-0" aria-label={mutedLabel} />}
                        </div>
                        <span className="text-[12px] text-gray-500 shrink-0 ml-2">{formatTime(chat.updatedAt)}</span>
                      </div>
                      <div className="text-[12px] text-gray-500 truncate">
                        {lastMessagePreview}
                      </div>
                    </div>
                  </button>
                </li>
              );
            })}
          </ul>
        )}
        {hasMoreChats && onLoadMoreChats ? (
          <div className="px-4 py-3 border-t border-white/5">
            <button
              type="button"
              className="w-full text-xs text-gray-400 hover:text-gray-200 disabled:opacity-50"
              disabled={loadingMoreChats}
              onClick={onLoadMoreChats}
            >
              {loadingMoreChats ? t('chat.list.loadingMore') : t('chat.list.loadMore')}
            </button>
          </div>
        ) : null}
      </div>
      {contextMenu && (
        <ContextMenu
          x={contextMenu.x}
          y={contextMenu.y}
          items={getContextMenuItems()}
          onClose={() => setContextMenu(null)}
        />
      )}
      <ConfirmModal
        isOpen={Boolean(pendingDeleteChat)}
        title={t(
          pendingDeleteChat?.type === 'group'
            ? 'chat.list.confirm.leaveGroupTitle'
            : 'chat.list.confirm.deleteTitle',
        )}
        message={t(
          pendingDeleteChat?.type === 'group'
            ? 'chat.list.confirm.leaveGroupMessage'
            : 'chat.list.confirm.deleteMessage',
          { name: pendingDeleteChat?.name ?? '' },
        )}
        danger
        onCancel={() => setPendingDeleteChat(null)}
        onConfirm={() => {
          const chat = pendingDeleteChat;
          setPendingDeleteChat(null);
          if (chat) {
            void handleDeleteChat(chat);
          }
        }}
      />
    </div>
  );
};
