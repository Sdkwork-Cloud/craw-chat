import React, { useState, useEffect, useRef, useCallback } from 'react';
import { useVirtualizer } from '@tanstack/react-virtual';
import { useTranslation } from 'react-i18next';
import { contactService } from '../services/ContactService';
import { chatService } from '../services/ChatService';
import { favoriteService } from '../services/FavoriteService';
import { parseGroupInviteDescriptor } from '../services/GroupService';
import { Avatar, MediaViewer, formatMessageTime } from '@sdkwork/im-pc-commons';
import { cn } from '@sdkwork/im-pc-commons';
import type { Message, User } from '@sdkwork/im-pc-types';
import { ContextMenu, ContextMenuItem } from './ContextMenu';
import { Copy, Reply, Forward, CheckSquare, Trash2, X, Check, Play, FileText, LayoutTemplate, Volume2, Video, Phone, Download, Smile, Star, Pencil, RotateCcw } from 'lucide-react';
import { toast } from './Toast';
import { ForwardModal } from './ForwardModal';
import { TextMessageItem, ImageMessageItem, VideoMessageItem, VoiceMessageItem, VideoCallMessageItem, LinkMessageItem, AppletMessageItem, CardMessageItem, FileMessageItem, MusicMessageItem } from './MessageItems';

interface MessageListProps {
  chatId: string;
  fallbackMessages?: Message[];
  refreshKey?: number;
  searchQuery?: string;
  senderProfiles?: Record<string, User>;
  onReply?: (msg: Message, senderName: string) => void;
  onEdit?: (msg: Message) => void;
  onOpenGroupInvite?: (groupId: string) => Promise<void>;
}

const EMPTY_MESSAGES: Message[] = [];
const EMPTY_SENDER_PROFILES: Record<string, User> = {};
const RTC_CALL_DESCRIPTOR_PREFIX = 'rtc-call:';

type RtcCallDisplayState = 'accepted' | 'ended' | 'rejected' | 'started' | 'syncing';

interface RtcCallDescriptor {
  actorId?: string;
  initiatorId?: string;
  mode?: string;
  receiverId?: string;
  signalType: string;
  state: RtcCallDisplayState;
}

function mergeDisplayMessages(messages: Message[], fallbackMessages: Message[]): Message[] {
  if (fallbackMessages.length === 0) {
    return messages;
  }

  const messageIds = new Set(messages.map((message) => message.id));
  return [
    ...fallbackMessages.filter((message) => !messageIds.has(message.id)),
    ...messages,
  ].sort((left, right) => left.timestamp - right.timestamp);
}

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

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
    }
  }
  return undefined;
}

function readRtcCallDescriptor(message: Message): RtcCallDescriptor | undefined {
  if (message.type !== 'video_call' || !message.desc?.startsWith(RTC_CALL_DESCRIPTOR_PREFIX)) {
    return undefined;
  }

  try {
    const parsed = parseJsonRecord(decodeURIComponent(message.desc.slice(RTC_CALL_DESCRIPTOR_PREFIX.length)));
    const state = pickString(parsed?.state);
    const signalType = pickString(parsed?.signalType) ?? 'rtc.signal';
    if (
      state === 'accepted'
      || state === 'ended'
      || state === 'rejected'
      || state === 'started'
      || state === 'syncing'
    ) {
      return {
        actorId: pickString(parsed?.actorId),
        initiatorId: pickString(parsed?.initiatorId),
        mode: pickString(parsed?.mode),
        receiverId: pickString(parsed?.receiverId),
        signalType,
        state,
      };
    }
  } catch {
    return undefined;
  }

  return undefined;
}

function collectRtcParticipantIds(messages: Message[]): string[] {
  const participantIds = new Set<string>();
  for (const message of messages) {
    const descriptor = readRtcCallDescriptor(message);
    if (!descriptor) {
      continue;
    }
    for (const participantId of [descriptor.actorId, descriptor.initiatorId, descriptor.receiverId]) {
      if (participantId) {
        participantIds.add(participantId);
      }
    }
  }
  return Array.from(participantIds);
}

function isVideoRtcMode(value: string | undefined): boolean {
  return Boolean(value && /video/iu.test(value));
}

type MessageTranslate = (key: string, options?: Record<string, unknown>) => string;

function formatRtcCallMode(value: string | undefined, t: MessageTranslate): string {
  return isVideoRtcMode(value) ? t('chat.messageList.rtcCall.videoMode') : t('chat.messageList.rtcCall.voiceMode');
}

function replaceParticipantId(content: string, participantId: string | undefined, displayName: string): string {
  if (!participantId || participantId === displayName) {
    return content;
  }
  return content.split(participantId).join(displayName);
}

function formatVideoCallMessageContent(
  message: Message,
  resolveDisplayName: (participantId: string | undefined, fallback: string) => string,
  t: MessageTranslate,
): string {
  const descriptor = readRtcCallDescriptor(message);
  if (!descriptor) {
    return message.content;
  }

  const mode = formatRtcCallMode(descriptor.mode, t);
  const initiator = resolveDisplayName(descriptor.initiatorId ?? message.senderId, t('chat.messageList.rtcCall.initiatorFallback'));
  const receiver = descriptor.receiverId ? resolveDisplayName(descriptor.receiverId, descriptor.receiverId) : undefined;
  const actor = resolveDisplayName(descriptor.actorId, t('chat.messageList.rtcCall.actorFallback'));
  const callSubject = receiver
    ? t('chat.messageList.rtcCall.subjectWithReceiver', { initiator, receiver, mode })
    : t('chat.messageList.rtcCall.subjectWithoutReceiver', { initiator, mode });

  switch (descriptor.state) {
    case 'accepted':
      return t('chat.messageList.rtcCall.accepted', { callSubject, actor });
    case 'rejected':
      return t('chat.messageList.rtcCall.rejected', { callSubject, actor });
    case 'ended':
      return t('chat.messageList.rtcCall.ended', { callSubject, actor });
    case 'started':
      return receiver
        ? t('chat.messageList.rtcCall.startedWithReceiver', { initiator, receiver, mode })
        : t('chat.messageList.rtcCall.startedWithoutReceiver', { initiator, mode });
    case 'syncing':
    default:
      return t('chat.messageList.rtcCall.syncing', { callSubject });
  }
}

function formatVideoCallMessage(
  message: Message,
  resolveDisplayName: (participantId: string | undefined, fallback: string) => string,
  t: MessageTranslate,
): Message {
  if (message.type !== 'video_call') {
    return message;
  }
  const descriptor = readRtcCallDescriptor(message);
  if (descriptor) {
    return {
      ...message,
      content: formatVideoCallMessageContent(message, resolveDisplayName, t),
    };
  }

  let content = message.content;
  for (const participantId of [message.senderId]) {
    content = replaceParticipantId(content, participantId, resolveDisplayName(participantId, participantId));
  }
  return { ...message, content };
}

function isCurrentUserMessage(message: Message, currentUser: User | null): boolean {
  if (!currentUser) {
    return false;
  }
  return message.senderId === currentUser.id || message.senderId === currentUser.chatId;
}

export const MessageList: React.FC<MessageListProps> = ({
  chatId,
  fallbackMessages = EMPTY_MESSAGES,
  refreshKey = 0,
  searchQuery = '',
  senderProfiles = EMPTY_SENDER_PROFILES,
  onReply,
  onEdit,
  onOpenGroupInvite,
}) => {
  const { t } = useTranslation();
  const [messages, setMessages] = useState<Message[]>([]);
  const [usersMap, setUsersMap] = useState<Record<string, User>>({});
  const [currentUser, setCurrentUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const [contextMenu, setContextMenu] = useState<{x: number, y: number, msg: Message} | null>(null);
  const [isMultiSelect, setIsMultiSelect] = useState(false);
  const [selectedIds, setSelectedIds] = useState<Set<string>>(new Set());
  const [forwardMessages, setForwardMessages] = useState<Message[]>([]);
  const [isForwardModalOpen, setIsForwardModalOpen] = useState(false);
  const scrollParentRef = useRef<HTMLDivElement>(null);
  const shouldStickToBottomRef = useRef(true);
  const loadingOlderRef = useRef(false);
  const [loadingOlder, setLoadingOlder] = useState(false);
  const [highlightedMessageId, setHighlightedMessageId] = useState<string | null>(null);

  const [viewerState, setViewerState] = useState({ isOpen: false, currentIndex: 0 });
  const fallbackMessageIds = React.useMemo(
    () => new Set(fallbackMessages.map((message) => message.id)),
    [fallbackMessages],
  );

  const filteredMessages = React.useMemo(() => {
    return messages.filter(msg => {
      if (!searchQuery.trim()) return true;
      return msg.content?.toLowerCase().includes(searchQuery.toLowerCase()) || msg.fileName?.toLowerCase().includes(searchQuery.toLowerCase());
    });
  }, [messages, searchQuery]);

  const virtualizer = useVirtualizer({
    count: filteredMessages.length,
    getScrollElement: () => scrollParentRef.current,
    estimateSize: () => 112,
    overscan: 12,
  });

  const scrollToBottom = useCallback((force = false) => {
    if (filteredMessages.length === 0) {
      return;
    }
    if (!force && !shouldStickToBottomRef.current) {
      return;
    }
    virtualizer.scrollToIndex(filteredMessages.length - 1, { align: 'end' });
  }, [filteredMessages.length, virtualizer]);

  const loadOlderMessages = useCallback(async () => {
    if (loadingOlderRef.current || !chatService.hasMoreMessages(chatId)) {
      return;
    }
    loadingOlderRef.current = true;
    setLoadingOlder(true);
    try {
      const element = scrollParentRef.current;
      const previousHeight = element?.scrollHeight ?? 0;
      const olderMessages = await chatService.loadMoreMessages(chatId);
      if (olderMessages.length === 0) {
        return;
      }
      setMessages((previous) => {
        const existingIds = new Set(previous.map((message) => message.id));
        const mergedOlder = olderMessages.filter((message) => !existingIds.has(message.id));
        return [...mergedOlder, ...previous];
      });
      requestAnimationFrame(() => {
        const scrollElement = scrollParentRef.current;
        if (scrollElement) {
          scrollElement.scrollTop = scrollElement.scrollHeight - previousHeight;
        }
      });
    } catch {
      toast(t('chat.messageList.toast.loadFailed'), 'error');
    } finally {
      loadingOlderRef.current = false;
      setLoadingOlder(false);
    }
  }, [chatId, t]);

  const handleScroll = useCallback(() => {
    const element = scrollParentRef.current;
    if (!element) {
      return;
    }
    const distanceFromBottom = element.scrollHeight - element.scrollTop - element.clientHeight;
    shouldStickToBottomRef.current = distanceFromBottom < 120;
    if (element.scrollTop < 80) {
      void loadOlderMessages();
    }
  }, [loadOlderMessages]);

  const scrollToMessage = useCallback((messageId: string) => {
    const index = filteredMessages.findIndex((message) => message.id === messageId);
    if (index < 0) {
      return;
    }
    virtualizer.scrollToIndex(index, { align: 'center' });
    setHighlightedMessageId(messageId);
    setTimeout(() => setHighlightedMessageId(null), 2000);
  }, [filteredMessages, virtualizer]);

  useEffect(() => {
    setCurrentUser(contactService.getCurrentUser());
    contactService.getContacts()
      .then(users => {
        const map: Record<string, User> = {};
        users.forEach(u => {
          map[u.id] = u;
          if (u.chatId) {
            map[u.chatId] = u;
          }
        });
        setUsersMap(map);
      })
      .catch(() => {
        setUsersMap({});
      });
  }, []);

  useEffect(() => {
    let isMounted = true;
    const loadMessages = async () => {
      setLoading(true);
      try {
        const data = await chatService.getMessages(chatId);
        if (!isMounted) {
          return;
        }
        setMessages(mergeDisplayMessages(data, fallbackMessages));
        shouldStickToBottomRef.current = true;
        setTimeout(() => scrollToBottom(true), 50);
      } catch {
        if (isMounted) {
          setMessages(fallbackMessages);
          toast(t('chat.messageList.toast.loadFailed'), 'error');
        }
      } finally {
        if (isMounted) {
          setLoading(false);
        }
      }
    };
    void loadMessages();
    return () => {
      isMounted = false;
    };
  }, [chatId, fallbackMessages, refreshKey, scrollToBottom]);

  useEffect(() => {
    scrollToBottom(false);
  }, [messages, scrollToBottom]);

  useEffect(() => {
    const unsubscribe = chatService.subscribeMessages(chatId, (message) => {
      setMessages(prev => {
        const byId = new Map(prev.map(item => [item.id, item]));
        byId.set(message.id, { ...byId.get(message.id), ...message });
        return mergeDisplayMessages(
          Array.from(byId.values()).sort((left, right) => left.timestamp - right.timestamp),
          fallbackMessages,
        );
      });
    });

    return () => {
      unsubscribe();
    };
  }, [chatId, fallbackMessages]);

  useEffect(() => {
    let isMounted = true;
    const missingParticipantIds = collectRtcParticipantIds(messages)
      .filter((participantId) => {
        if (currentUser && (participantId === currentUser.id || participantId === currentUser.chatId)) {
          return false;
        }
        return !usersMap[participantId] && !senderProfiles[participantId];
      });

    if (missingParticipantIds.length === 0) {
      return () => {
        isMounted = false;
      };
    }

    void Promise.all(missingParticipantIds.map((participantId) => contactService.getUserById(participantId)))
      .then((users) => {
        if (!isMounted) {
          return;
        }
        const resolvedUsers = users.filter((user): user is User => Boolean(user));
        if (resolvedUsers.length === 0) {
          return;
        }
        setUsersMap((previousUsersMap) => {
          const nextUsersMap = { ...previousUsersMap };
          for (const user of resolvedUsers) {
            nextUsersMap[user.id] = user;
            if (user.chatId) {
              nextUsersMap[user.chatId] = user;
            }
          }
          return nextUsersMap;
        });
      })
      .catch(() => undefined);

    return () => {
      isMounted = false;
    };
  }, [currentUser, messages, senderProfiles, usersMap]);

  const handleContextMenu = (e: React.MouseEvent, msg: Message) => {
    e.preventDefault();
    if (isMultiSelect) return;
    setContextMenu({ x: e.clientX, y: e.clientY, msg });
  };

  const handleDelete = async (idsToDelete: Set<string>) => {
    try {
      await Promise.all(Array.from(idsToDelete).map((messageId) => chatService.deleteMessage(chatId, messageId)));
      setMessages(prev => prev.filter(msg => !idsToDelete.has(msg.id)));
      toast(
        idsToDelete.size > 1
          ? t('chat.messageList.toast.deleteManySuccess', { count: idsToDelete.size })
          : t('chat.messageList.toast.deleteSuccess'),
        'success',
      );
      setIsMultiSelect(false);
      setSelectedIds(new Set());
    } catch {
      toast(t('chat.messageList.toast.deleteFailed'), 'error');
    }
  };

  const handleRecall = async (messageId: string) => {
    try {
      await chatService.recallMessage(chatId, messageId);
      setMessages(prev => prev.map(msg => msg.id === messageId
        ? { ...msg, isRecalled: true, content: '', reactions: [] }
        : msg));
      toast(t('chat.messageList.toast.recallSuccess'), 'success');
    } catch {
      toast(t('chat.messageList.toast.recallFailed'), 'error');
    }
  };

  const handleForward = (messagesToForward: Message[]) => {
    setForwardMessages(messagesToForward);
    setIsForwardModalOpen(true);
    setIsMultiSelect(false);
    setSelectedIds(new Set());
  };

  const handleGroupInviteClick = async (msg: Message) => {
    const descriptor = parseGroupInviteDescriptor(msg);
    if (!descriptor || !onOpenGroupInvite) {
      return;
    }
    try {
      await onOpenGroupInvite(descriptor.groupId);
    } catch {
      toast(t('chat.messageList.toast.openGroupFailed'), 'error');
    }
  };

  const getContextMenuItems = (): ContextMenuItem[] => {
    if (!contextMenu) return [];
    const sender = usersMap[contextMenu.msg.senderId];
    const isFallbackMessage = fallbackMessageIds.has(contextMenu.msg.id);
    const copyItem: ContextMenuItem = {
      id: 'copy',
      label: t('chat.messageList.contextMenu.copy'),
      icon: <Copy size={14} />,
      onClick: () => {
        navigator.clipboard.writeText(contextMenu.msg.content);
        toast(t('chat.messageList.toast.copySuccess'), 'success');
      },
    };
    if (isFallbackMessage) {
      return [copyItem];
    }
    const unknownUser = t('chat.messageList.unknownUser');
    const isOwnMessage = isCurrentUserMessage(contextMenu.msg, currentUser);
    const isRecalled = Boolean(contextMenu.msg.isRecalled);
    const isTextMessage = contextMenu.msg.type === 'text';
    const ownMessageItems: ContextMenuItem[] = isOwnMessage && !isRecalled
      ? [
          { id: 'edit', label: t('chat.messageList.contextMenu.edit'), icon: <Pencil size={14} />, onClick: () => { if (onEdit && isTextMessage) onEdit(contextMenu.msg); } },
          { id: 'recall', label: t('chat.messageList.contextMenu.recall'), icon: <RotateCcw size={14} />, onClick: () => { void handleRecall(contextMenu.msg.id); } },
        ].filter(item => !(item.id === 'edit' && (!isTextMessage || !onEdit)))
      : [];
    return [
      copyItem,
      ...ownMessageItems,
      { id: 'reply', label: t('chat.messageList.contextMenu.reply'), icon: <Reply size={14} />, onClick: () => { if (onReply) onReply(contextMenu.msg, sender?.name || unknownUser); } },
      { id: 'reaction', label: t('chat.messageList.contextMenu.reaction'), icon: <Smile size={14} />, onClick: () => { void handleReaction(contextMenu.msg.id, '👍'); } },
      { id: 'forward', label: t('chat.messageList.contextMenu.forward'), icon: <Forward size={14} />, onClick: () => handleForward([contextMenu.msg]) },
      { id: 'favorite', label: t('chat.messageList.contextMenu.favorite'), icon: <Star size={14} />, onClick: async () => {
          try {
            await favoriteService.addFavorite({
               type: contextMenu.msg.type === 'link' || contextMenu.msg.type === 'music' ? 'link' : contextMenu.msg.type === 'image' || contextMenu.msg.type === 'video' ? 'image' : contextMenu.msg.type === 'file' ? 'file' : 'chat',
               title: contextMenu.msg.fileName || contextMenu.msg.content.substring(0, 20),
               content: contextMenu.msg.content,
               conversationId: contextMenu.msg.chatId ?? chatId,
               messageId: contextMenu.msg.id,
               source: sender?.name || unknownUser
            });
            toast(t('chat.messageList.toast.favoriteSuccess'), 'success');
          } catch {
            toast(t('chat.messageList.toast.favoriteFailed'), 'error');
          }
      } },
      { id: 'select', label: t('chat.messageList.contextMenu.multiSelect'), icon: <CheckSquare size={14} />, onClick: () => { setIsMultiSelect(true); setSelectedIds(new Set([contextMenu.msg.id])); } },
      { id: 'div1', label: '', divider: true, onClick: () => {} },
      { id: 'delete', label: t('chat.messageList.contextMenu.delete'), icon: <Trash2 size={14} />, danger: true, onClick: () => handleDelete(new Set([contextMenu.msg.id])) },
    ];
  };

  const toggleSelect = (id: string) => {
    const next = new Set(selectedIds);
    if (next.has(id)) next.delete(id);
    else next.add(id);
    setSelectedIds(next);
  };

  const mediaMessages = messages.filter(m => m.type === 'image' || m.type === 'video');
  const mediaItems = mediaMessages.map(m => ({
    id: m.id,
    type: m.type as 'image' | 'video',
    src: m.content || '',
    name: m.fileName || (m.type === 'image' ? t('chat.messageList.media.image') : t('chat.messageList.media.video'))
  }));

  const handleMediaClick = (msg: Message) => {
    const index = mediaMessages.findIndex(m => m.id === msg.id);
    if (index !== -1) {
      setViewerState({ isOpen: true, currentIndex: index });
    }
  };

  const handleReaction = async (messageId: string, emoji: string) => {
    const msg = messages.find(m => m.id === messageId);
    if (!msg) return;
    
    try {
      const reaction = msg.reactions?.find(r => r.emoji === emoji);
      if (reaction && reaction.hasReacted) {
        await chatService.removeReaction(chatId, messageId, emoji);
      } else {
        await chatService.addReaction(chatId, messageId, emoji);
      }
    } catch {
      toast(t('chat.messageList.toast.reactionFailed'), 'error');
      return;
    }
    
    // Optimistic update
    setMessages(prev => prev.map(m => {
      if (m.id !== messageId) return m;
      const nextReactions = [...(m.reactions || [])];
      const rIdx = nextReactions.findIndex(r => r.emoji === emoji);
      if (rIdx >= 0) {
        if (nextReactions[rIdx].hasReacted) {
           nextReactions[rIdx].count--;
           nextReactions[rIdx].hasReacted = false;
           if (nextReactions[rIdx].count <= 0) nextReactions.splice(rIdx, 1);
        } else {
           nextReactions[rIdx].count++;
           nextReactions[rIdx].hasReacted = true;
        }
      } else {
        nextReactions.push({ emoji, count: 1, hasReacted: true });
      }
      return { ...m, reactions: nextReactions };
    }));
  };

  const resolveDisplayName = useCallback((participantId: string | undefined, fallback: string) => {
    if (!participantId) {
      return fallback;
    }
    if (currentUser && (participantId === currentUser.id || participantId === currentUser.chatId)) {
      return currentUser.name;
    }
    return senderProfiles[participantId]?.name
      ?? usersMap[participantId]?.name
      ?? fallback;
  }, [currentUser, senderProfiles, usersMap]);

  return (
    <div
      ref={scrollParentRef}
      onScroll={handleScroll}
      className="flex-1 min-h-0 overflow-y-auto p-6 flex flex-col bg-[#1e1e1e] custom-scrollbar relative"
    >
      {loading && <div className="text-center text-[12px] text-gray-500 my-4">{t('chat.messageList.loading')}</div>}
      {loadingOlder && (
        <div className="text-center text-[12px] text-gray-500 my-2" role="status">
          {t('chat.messageList.loadingOlder', { defaultValue: 'Loading earlier messages…' })}
        </div>
      )}
      {!loading && filteredMessages.length > 0 && (
        <div className="text-center text-[12px] text-gray-500 my-4">{formatMessageTime(filteredMessages[0].timestamp)}</div>
      )}

      <div
        className="relative w-full"
        style={{ height: filteredMessages.length > 0 ? `${virtualizer.getTotalSize()}px` : undefined }}
      >
        {virtualizer.getVirtualItems().map((virtualRow) => {
          const msg = filteredMessages[virtualRow.index];
          const index = virtualRow.index;
          const isMe = isCurrentUserMessage(msg, currentUser);
          const sender = isMe ? currentUser : (
            senderProfiles[msg.senderId] ?? usersMap[msg.senderId]
          );
          const showTime = index === 0 || msg.timestamp - filteredMessages[index - 1].timestamp > 1000 * 60 * 5;
          const displayMessage = formatVideoCallMessage(msg, resolveDisplayName, t);
          const isHighlighted = highlightedMessageId === msg.id;

          return (
            <div
              key={msg.id}
              data-index={virtualRow.index}
              ref={virtualizer.measureElement}
              className="absolute top-0 left-0 w-full"
              style={{ transform: `translateY(${virtualRow.start}px)` }}
            >
              {showTime && index > 0 && (
                <div className="text-center text-[12px] text-gray-500 my-4">{formatMessageTime(msg.timestamp)}</div>
              )}
              <div
                id={`msg-${msg.id}`}
                className={cn(
                  'flex mb-6 group relative rounded-lg transition-all duration-300',
                  isMe ? 'flex-row-reverse' : 'flex-row',
                  isMultiSelect && 'hover:bg-white/5 cursor-pointer p-2 -mx-2',
                  isHighlighted && 'ring-2 ring-[#00b42a] ring-opacity-50 bg-white/5',
                )}
                onClick={() => isMultiSelect && toggleSelect(msg.id)}
                onContextMenu={(e) => handleContextMenu(e, msg)}
              >
                {isMultiSelect && (
                  <div className={cn('flex items-center justify-center w-8 shrink-0', isMe ? 'ml-2' : 'mr-2')}>
                    <div className={cn('w-5 h-5 rounded-full border flex items-center justify-center transition-colors', selectedIds.has(msg.id) ? 'bg-[#00b42a] border-[#00b42a]' : 'border-gray-500')}>
                      {selectedIds.has(msg.id) && <Check size={12} className="text-white" />}
                    </div>
                  </div>
                )}
                <Avatar src={sender?.avatar} alt={sender?.name} className={cn('w-[36px] h-[36px] rounded shrink-0 bg-[#2b2b2d] text-white text-[12px] mt-1', isMe ? 'ml-3' : 'mr-3')} />
                <div className={cn('flex flex-col flex-1 min-w-0', isMe ? 'items-end' : 'items-start')}>
                  {!isMe && sender?.name && (
                    <div className="flex items-center gap-2 mb-1 px-1">
                      <span className="text-[12px] text-gray-400 font-medium">{sender.name}</span>
                    </div>
                  )}
                  {msg.replyTo && (
                    <div
                      onClick={(e) => {
                        e.stopPropagation();
                        scrollToMessage(msg.replyTo!.id);
                      }}
                      className={cn('mb-1.5 px-3 py-1.5 bg-white/5 border-gray-500 rounded text-[12px] text-gray-400 max-w-full truncate cursor-pointer hover:bg-white/10 transition-colors', isMe ? 'border-r-2' : 'border-l-2')}
                    >
                      <span className="font-medium mr-1">{msg.replyTo.senderName}:</span>
                      {msg.replyTo.content}
                    </div>
                  )}
                  <div className="relative flex items-center">
                    <div>
                      {msg.isRecalled ? (
                        <span className="text-[13px] text-gray-500 italic select-none">
                          {t('chat.messageList.recalledPlaceholder')}
                        </span>
                      ) : (
                        <>
                          {msg.type === 'text' && <TextMessageItem msg={msg} isMe={isMe} />}
                          {msg.type === 'image' && <ImageMessageItem msg={msg} isMe={isMe} onMediaClick={handleMediaClick} />}
                          {msg.type === 'video' && <VideoMessageItem msg={msg} isMe={isMe} onMediaClick={handleMediaClick} />}
                          {msg.type === 'voice' && <VoiceMessageItem msg={msg} isMe={isMe} />}
                          {msg.type === 'video_call' && <VideoCallMessageItem msg={displayMessage} isMe={isMe} />}
                          {msg.type === 'link' && <LinkMessageItem msg={msg} isMe={isMe} />}
                          {msg.type === 'applet' && <AppletMessageItem msg={msg} isMe={isMe} />}
                          {msg.type === 'card' && <CardMessageItem
                            msg={msg}
                            isMe={isMe}
                            onClick={parseGroupInviteDescriptor(msg) ? () => {
                              void handleGroupInviteClick(msg);
                            } : undefined}
                          />}
                          {msg.type === 'file' && <FileMessageItem msg={msg} isMe={isMe} />}
                          {msg.type === 'music' && <MusicMessageItem msg={msg} isMe={isMe} allMessages={messages} />}
                          {msg.isEdited && (
                            <span className="ml-1 text-[11px] text-gray-500 select-none align-bottom">
                              {t('chat.messageList.editedIndicator')}
                            </span>
                          )}
                        </>
                      )}
                    </div>
                  </div>
                  {msg.reactions && msg.reactions.length > 0 && (
                    <div className={cn('flex flex-wrap gap-1 mt-1', isMe ? 'justify-end' : 'justify-start')}>
                      {msg.reactions.map((reaction) => (
                        <button
                          key={reaction.emoji}
                          onClick={() => handleReaction(msg.id, reaction.emoji)}
                          className={cn(
                            'flex items-center gap-1 px-2 py-0.5 rounded-full text-xs transition-colors border',
                            reaction.hasReacted
                              ? 'bg-indigo-500/20 text-indigo-300 border-indigo-500/30'
                              : 'bg-white/5 text-gray-400 border-white/5 hover:bg-white/10',
                          )}
                        >
                          <span>{reaction.emoji}</span>
                          <span className="font-medium">{reaction.count}</span>
                        </button>
                      ))}
                    </div>
                  )}
                </div>
              </div>
            </div>
          );
        })}
      </div>

      {isMultiSelect && (
        <div className="sticky bottom-4 left-1/2 -translate-x-1/2 w-max bg-[#2b2b2d] border border-white/10 rounded-full shadow-2xl px-6 py-3 flex items-center gap-6 z-50 mx-auto mt-auto">
          <span className="text-sm text-gray-300">{t('chat.messageList.multiSelect.selected', { count: selectedIds.size })}</span>
          <div className="w-px h-4 bg-white/10" />
          <button 
            className="flex items-center gap-2 text-sm text-gray-300 hover:text-white transition-colors disabled:opacity-50" 
            onClick={() => handleForward(messages.filter(m => selectedIds.has(m.id)))}
            disabled={selectedIds.size === 0}
          >
            <Forward size={16} /> {t('chat.messageList.multiSelect.forward')}
          </button>
          <button 
            className="flex items-center gap-2 text-sm text-red-400 hover:text-red-300 transition-colors disabled:opacity-50" 
            onClick={() => handleDelete(selectedIds)}
            disabled={selectedIds.size === 0}
          >
            <Trash2 size={16} /> {t('chat.messageList.multiSelect.delete')}
          </button>
          <div className="w-px h-4 bg-white/10" />
          <button className="p-1 text-gray-400 hover:text-white transition-colors" onClick={() => { setIsMultiSelect(false); setSelectedIds(new Set()); }}>
            <X size={18} />
          </button>
        </div>
      )}

      {contextMenu && (
        <ContextMenu 
          x={contextMenu.x} 
          y={contextMenu.y} 
          items={getContextMenuItems()} 
          onClose={() => setContextMenu(null)} 
        />
      )}

      <ForwardModal 
        isOpen={isForwardModalOpen} 
        onClose={() => setIsForwardModalOpen(false)} 
        messages={forwardMessages} 
      />
      
      <MediaViewer 
        isOpen={viewerState.isOpen}
        items={mediaItems}
        currentIndex={viewerState.currentIndex}
        onIndexChange={(idx) => setViewerState(prev => ({ ...prev, currentIndex: idx }))}
        onClose={() => setViewerState(prev => ({ ...prev, isOpen: false }))}
      />
    </div>
  );
};
