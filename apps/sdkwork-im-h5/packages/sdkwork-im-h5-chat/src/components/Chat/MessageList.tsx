import React, { useRef, useEffect, useMemo, useCallback } from "react";
import type { Message, Chat, User } from "@sdkwork/im-h5-types";
import { MessageItem } from "../MessageItem";
import { useTranslation } from "react-i18next";

export const formatMessageTime = (
  timestamp: number,
  t: (key: string, options?: any) => string
) => {
  const date = new Date(timestamp);
  const now = new Date();

  const isToday = date.toDateString() === now.toDateString();
  const isYesterday =
    new Date(now.getTime() - 86400000).toDateString() === date.toDateString();

  const hour = date.getHours();
  const min = date.getMinutes().toString().padStart(2, "0");
  const period = hour < 12 ? t('chat.date.am') : t('chat.date.pm');
  const hour12 = hour % 12 || 12;
  const timeStr = `${period}${hour12}:${min}`;

  if (isToday) return timeStr;
  if (isYesterday) return `${t('chat.date.yesterday')} ${timeStr}`;
  if (now.getFullYear() === date.getFullYear()) {
    if (t('chat.date.month') === "-") {
      return `${date.getMonth() + 1}-${date.getDate()} ${timeStr}`;
    }
    return `${date.getMonth() + 1}${t('chat.date.month')}${date.getDate()}${t('chat.date.day')} ${timeStr}`;
  }
  
  if (t('chat.date.month') === "-") {
    return `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()} ${timeStr}`;
  }
  return `${date.getFullYear()}${t('chat.date.year')}${date.getMonth() + 1}${t('chat.date.month')}${date.getDate()}${t('chat.date.day')} ${timeStr}`;
};

interface MessageListProps {
  messages: Message[];
  chat: Chat | null;
  currentUser: User | null;
  cleanMode: boolean;
  showAvatar: boolean;
  contextMenu: any;
  handleTouchStart: (e: any, id: string) => void;
  handleTouchEnd: () => void;
  handleTouchMove: () => void;
  setFullscreenMedia: (media: any) => void;
  highlightedMsgId: string | null;
  setHighlightedMsgId: (id: string | null) => void;
  setActivePanel: (panel: "none" | "emoji" | "action") => void;
  onScrollToTop?: () => void;
  onRetry?: (msg: Message) => void;
  /** More history is available on the server (show the top indicator). */
  hasMoreTop?: boolean;
  /** A cursor page for older history is being fetched. */
  loadingMore?: boolean;
}

export const MessageList: React.FC<MessageListProps> = ({
  messages,
  chat,
  currentUser,
  cleanMode,
  showAvatar,
  contextMenu,
  handleTouchStart,
  handleTouchEnd,
  handleTouchMove,
  setFullscreenMedia,
  highlightedMsgId,
  setHighlightedMsgId,
  setActivePanel,
  onScrollToTop,
  onRetry,
  hasMoreTop = false,
  loadingMore = false,
}) => {
  const { t } = useTranslation();
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const isNearBottomRef = useRef(true);
  // Pending scroll timeouts are cleared on unmount so a fast navigation away
  // cannot scrollIntoView a detached node.
  const scrollTimerRef = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  const highlightTimerRef = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  const replyJumpTimerRef = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);

  useEffect(() => {
    if (highlightedMsgId) {
      highlightTimerRef.current = setTimeout(() => {
        const el = document.getElementById(`msg-${highlightedMsgId}`);
        if (el) el.scrollIntoView({ behavior: "smooth", block: "center" });
      }, 100);
      return;
    }

    // Only auto-scroll to the latest message while the user is already at the
    // bottom. Loading older pages (or history refresh) must not yank the view
    // back down while the user reads earlier messages.
    if (isNearBottomRef.current && messagesEndRef.current) {
      scrollTimerRef.current = setTimeout(() => {
        messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
      }, 50);
    }
  }, [messages, highlightedMsgId]);

  useEffect(() => () => {
    if (scrollTimerRef.current) clearTimeout(scrollTimerRef.current);
    if (highlightTimerRef.current) clearTimeout(highlightTimerRef.current);
    if (replyJumpTimerRef.current) clearTimeout(replyJumpTimerRef.current);
  }, []);

  const handleScroll = (event: React.UIEvent<HTMLDivElement>) => {
    const element = event.currentTarget;
    isNearBottomRef.current = element.scrollHeight - element.scrollTop - element.clientHeight < 80;
    if (element.scrollTop <= 40) {
      onScrollToTop?.();
    }
  };

  // Reply lookup: O(1) per message instead of scanning the whole window per
  // rendered item (the previous `messages.find` was quadratic in the window).
  const messagesById = useMemo(() => new Map(messages.map((message) => [message.id, message])), [messages]);
  const participantsById = useMemo(() => {
    const map = new Map<string, User>();
    for (const participant of chat?.participants ?? []) map.set(participant.id, participant);
    return map;
  }, [chat]);
  const systemSender = useMemo(
    () => ({ id: "system", name: t('chat.date.system_agent_name', 'System Agent') }),
    [t],
  );

  const handleReplyClick = useCallback((id: string) => {
    setHighlightedMsgId(id);
    replyJumpTimerRef.current = setTimeout(() => setHighlightedMsgId(null), 3000);
    document
      .getElementById(`msg-${id}`)
      ?.scrollIntoView({ behavior: "smooth", block: "center" });
  }, [setHighlightedMsgId]);

  return (
    <div
      className="flex-1 overflow-y-auto p-4 flex flex-col"
      onClick={() => setActivePanel("none")}
      onScroll={handleScroll}
    >
      {hasMoreTop && (
        <div className="flex justify-center py-2 shrink-0">
          {loadingMore ? (
            <span className="text-[12px] text-text-sub">{t('chat.detail.loading_earlier', 'Loading earlier messages...')}</span>
          ) : (
            <span className="text-[12px] text-text-sub">{t('chat.detail.has_earlier_messages', 'Scroll up for earlier messages')}</span>
          )}
        </div>
      )}
      {messages.map((msg, index) => {
        const isMe = msg.senderId === currentUser?.id;
        const sender = isMe
          ? currentUser ?? undefined
          : msg.senderId === "system"
            ? systemSender
            : participantsById.get(msg.senderId);
        const isAgent = msg.senderId.startsWith("agent_") || msg.senderId === "system";

        const prevMsg = index > 0 ? messages[index - 1] : null;
        let showTime = false;
        if (!cleanMode) {
          if (!prevMsg) {
            showTime = true;
          } else {
            showTime = msg.timestamp - prevMsg.timestamp > 5 * 60 * 1000;
          }
        }

        const isPrevSameSender = prevMsg && prevMsg.senderId === msg.senderId;
        const isPrevTooClose =
          prevMsg && msg.timestamp - prevMsg.timestamp < 60 * 1000;

        const isConsecutive = isPrevSameSender && isPrevTooClose && !showTime;

        const hideAvatar = !showAvatar;
        const isOtherFullWidth = !isMe && (!showAvatar || isAgent);
        const hideTail = Boolean(hideAvatar || isAgent || isConsecutive);

        let replyToMsg: Message | undefined;
        let replyToSenderName: string | undefined;

        if (msg.metadata?.replyTo) {
          replyToMsg = messagesById.get(msg.metadata?.replyTo);
          if (replyToMsg) {
            const replyIsMe = replyToMsg.senderId === currentUser?.id;
            replyToSenderName = replyIsMe
              ? t('chat.detail.me')
              : participantsById.get(replyToMsg.senderId)?.name || t('chat.detail.unknown');
          } else if (msg.replyTo) {
            // The quoted message is outside the loaded pages: fall back to the
            // server-provided reply snapshot so the reference still renders.
            replyToMsg = {
              id: msg.replyTo.id,
              chatId: chat?.id ?? "",
              senderId: "",
              content: msg.replyTo.content,
              timestamp: 0,
              type: "text",
            };
            replyToSenderName = msg.replyTo.senderName || t('chat.detail.unknown');
          }
        }

        return (
          <React.Fragment key={msg.id}>
            {showTime && (
              <div className="flex justify-center my-4">
                <span className="text-[11px] text-text-sub bg-black/5 dark:bg-white/5 px-2 py-0.5 rounded-md">
                  {formatMessageTime(msg.timestamp, t)}
                </span>
              </div>
            )}
            <div
              style={{
                marginTop: index === 0 || showTime ? 0 : isConsecutive ? 4 : 16,
              }}
            >
              {chat?.type === "group" &&
                !isMe &&
                !isConsecutive &&
                !isAgent && (
                  <div className="text-[12px] text-text-sub ml-[52px] mb-1">
                    {sender?.name || t('chat.date.unknown_contact')}
                  </div>
                )}
              <MessageItem
                msg={msg}
                isMe={isMe}
                hideAvatar={hideAvatar}
                hideTail={hideTail}
                isOtherFullWidth={isOtherFullWidth}
                sender={sender}
                currentUser={currentUser}
                contextMenu={contextMenu}
                handleTouchStart={handleTouchStart}
                handleTouchEnd={handleTouchEnd}
                handleTouchMove={handleTouchMove}
                onPreview={setFullscreenMedia}
                isHighlighted={highlightedMsgId === msg.id}
                replyToMsg={replyToMsg}
                replyToSenderName={replyToSenderName}
                onReplyClick={handleReplyClick}
                onRetry={onRetry}
              />
            </div>
          </React.Fragment>
        );
      })}
      <div ref={messagesEndRef} />
    </div>
  );
};
