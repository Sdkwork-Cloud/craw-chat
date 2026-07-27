import React, { useRef, useEffect } from "react";
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
}) => {
  const { t } = useTranslation();
const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (highlightedMsgId) {
      setTimeout(() => {
        const el = document.getElementById(`msg-${highlightedMsgId}`);
        if (el) el.scrollIntoView({ behavior: "smooth", block: "center" });
      }, 100);
      return;
    }

    if (messagesEndRef.current) {
      setTimeout(() => {
        messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
      }, 50);
    }
  }, [messages, highlightedMsgId]);

  return (
    <div
      className="flex-1 overflow-y-auto p-4 flex flex-col"
      onClick={() => setActivePanel("none")}
    >
      {messages.map((msg, index) => {
        const isMe = msg.senderId === currentUser?.id;
        const sender = isMe
          ? currentUser
          : chat?.participants.find((p) => p.id === msg.senderId);
        const isAgent = msg.senderId.startsWith("agent_");

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
        const hideTail = hideAvatar || isAgent || isConsecutive;

        let replyToMsg: Message | undefined;
        let replyToSenderName: string | undefined;

        if (msg.metadata?.replyTo) {
          replyToMsg = messages.find((m) => m.id === msg.metadata.replyTo);
          if (replyToMsg) {
            const replyIsMe = replyToMsg.senderId === currentUser?.id;
            replyToSenderName = replyIsMe
              ? t('chat.detail.me')
              : chat?.participants.find((p) => p.id === replyToMsg!.senderId)
                  ?.name || t('chat.detail.unknown');
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
                onReplyClick={(id) => {
                  setHighlightedMsgId(id);
                  setTimeout(() => setHighlightedMsgId(null), 3000);
                  document
                    .getElementById(`msg-${id}`)
                    ?.scrollIntoView({ behavior: "smooth", block: "center" });
                }}
              />
            </div>
          </React.Fragment>
        );
      })}
      <div ref={messagesEndRef} />
    </div>
  );
};
