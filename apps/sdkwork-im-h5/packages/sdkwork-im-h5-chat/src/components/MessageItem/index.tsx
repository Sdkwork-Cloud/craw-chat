import React from "react";
import type { Message, User } from "@sdkwork/im-h5-types";
import { Avatar, cn } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";

import { TextMessage } from "./TextMessage";
import { ImageMessage } from "./ImageMessage";
import { VideoMessage } from "./VideoMessage";
import { VoiceMessage } from "./VoiceMessage";
import { CallMessage } from "./CallMessage";
import { LinkMessage } from "./LinkMessage";
import { MiniappMessage } from "./MiniappMessage";
import { FileMessage } from "./FileMessage";
import { MusicMessage } from "./MusicMessage";

export interface MessageItemProps {
  msg: Message;
  isMe: boolean;
  hideAvatar: boolean;
  hideTail: boolean;
  isOtherFullWidth: boolean;
  sender: User | undefined;
  currentUser: User | undefined;
  contextMenu: { isOpen: boolean; messageId: string | null };
  handleTouchStart: (
    e: React.TouchEvent | React.MouseEvent,
    messageId: string,
  ) => void;
  handleTouchEnd: () => void;
  handleTouchMove: () => void;
  onPreview: (preview: { type: "image" | "video"; url: string }) => void;
  isHighlighted: boolean | null;
  replyToMsg?: Message;
  replyToSenderName?: string;
  onReplyClick?: (id: string) => void;
}

export const MessageItem = ({
  msg,
  isMe,
  hideAvatar,
  hideTail,
  isOtherFullWidth,
  sender,
  currentUser,
  contextMenu,
  handleTouchStart,
  handleTouchEnd,
  handleTouchMove,
  onPreview,
  isHighlighted,
  replyToMsg,
  replyToSenderName,
  onReplyClick,
}: MessageItemProps) => {
  const { t } = useTranslation();
return (
    <div
      id={`msg-${msg.id}`}
      className={cn(
        "flex",
        isMe ? "justify-end" : "justify-start",
        isOtherFullWidth ? "w-full" : "",
        isHighlighted &&
          "bg-black/5 dark:bg-white/5 mx-[-16px] px-[16px] py-1 rounded-xl transition-colors duration-300",
      )}
    >
      {!isMe && !hideAvatar && !msg.senderId.startsWith("agent_") && (
        <Avatar
          src={sender?.avatar}
          fallback={sender?.name}
          size="md"
          className="mr-3 shrink-0 rounded-md"
        />
      )}
      {!isMe && !hideAvatar && msg.senderId.startsWith("agent_") && (
        <Avatar
          src={sender?.avatar}
          fallback={sender?.name}
          size="md"
          className="mr-3 shrink-0 rounded-full"
        />
      )}
      {!isMe && hideAvatar && !isOtherFullWidth && (
        <div className="w-10 mr-3 shrink-0" />
      )}
      <div
        onContextMenu={(e) => {
          e.preventDefault();
          handleTouchStart(e, msg.id);
        }}
        onTouchStart={(e) => handleTouchStart(e, msg.id)}
        onTouchEnd={handleTouchEnd}
        onTouchMove={handleTouchMove}
        onMouseDown={(e) => handleTouchStart(e, msg.id)}
        onMouseUp={handleTouchEnd}
        onMouseLeave={handleTouchEnd}
        className={cn(
          "px-4 py-2.5 text-[16px] leading-relaxed break-words relative transition-all select-none",
          ["image", "video"].includes(msg.type)
            ? "p-0 bg-transparent"
            : isMe
              ? "bg-chat-me-bg text-white"
              : "bg-chat-other-bg text-text-main border border-border-color",
          isOtherFullWidth
            ? "rounded-2xl flex-1"
            : hideTail
              ? "rounded-2xl max-w-[75%]"
              : "rounded-lg max-w-[75%]",
          contextMenu.isOpen && contextMenu.messageId === msg.id
            ? "brightness-90 scale-[0.98]"
            : "",
        )}
      >
        {!hideTail && !["image", "video"].includes(msg.type) && (
          <div
            className={cn(
              "absolute top-[14px] w-2.5 h-2.5 rotate-45 border border-transparent",
              isMe
                ? "bg-chat-me-bg -right-1 z-0"
                : "bg-chat-other-bg border-border-color border-b-0 border-r-0 -left-1 z-0",
            )}
          />
        )}
        <div className="relative z-10 w-full">
          {replyToMsg && (
            <div
              className={cn(
                "text-[12px] opacity-70 mb-1.5 border-l-2 pl-2 cursor-pointer break-words line-clamp-2",
                isMe ? "border-white/50" : "border-border-color",
              )}
              onClick={(e) => {
                e.stopPropagation();
                onReplyClick?.(replyToMsg.id);
              }}
            >
              {replyToSenderName}:{" "}
              {replyToMsg.type === "text"
                ? replyToMsg.content
                : `[${replyToMsg.type === "image" ? t('chat.detail.type_image') : replyToMsg.type === "voice" ? t('chat.detail.type_voice') : replyToMsg.type === "video" ? t('chat.detail.type_video') : replyToMsg.type === "file" ? t('chat.detail.type_file') : t('chat.detail.type_media')}]`}
            </div>
          )}
          {msg.type === "text" && <TextMessage msg={msg} />}
          {msg.type === "image" && (
            <ImageMessage
              msg={msg}
              onClick={() => onPreview({ type: "image", url: msg.content })}
            />
          )}
          {msg.type === "video" && (
            <VideoMessage
              msg={msg}
              onClick={() =>
                onPreview({
                  type: "video",
                  url: msg.content,
                })
              }
            />
          )}
          {msg.type === "voice" && <VoiceMessage msg={msg} isMe={isMe} />}
          {msg.type === "call" && <CallMessage msg={msg} />}
          {msg.type === "link" && <LinkMessage msg={msg} isMe={isMe} />}
          {msg.type === "miniapp" && <MiniappMessage msg={msg} isMe={isMe} />}
          {msg.type === "file" && <FileMessage msg={msg} isMe={isMe} />}
          {msg.type === "music" && <MusicMessage msg={msg} isMe={isMe} />}
        </div>
      </div>
      {isMe && !hideAvatar && (
        <Avatar
          src={currentUser?.avatar}
          size="md"
          className="ml-3 shrink-0 rounded-md"
        />
      )}
      {isMe && hideAvatar && !isOtherFullWidth && (
        <div className="w-10 ml-3 shrink-0" />
      )}
    </div>
  );
};
