import React from "react";
import { useNavigate } from "react-router";
import { format } from "date-fns";
import { BellOff } from "lucide-react";
import { Avatar, cn } from "@sdkwork/im-h5-commons";
import type { Chat } from "@sdkwork/im-h5-types";
import { useTranslation } from "react-i18next";

interface ChatListItemProps {
  chat: Chat;
  index: number;
  chatsLength: number;
  contextMenu: {
    isOpen: boolean;
    chatId?: string | null;
  };
  handleTouchStart: (e: React.TouchEvent | React.MouseEvent, chatId: string) => void;
  handleTouchEnd: () => void;
  handleTouchMove: () => void;
}

export const ChatListItem: React.FC<ChatListItemProps> = ({
  chat,
  index,
  chatsLength,
  contextMenu,
  handleTouchStart,
  handleTouchEnd,
  handleTouchMove,
}) => {
  const { t } = useTranslation();
const navigate = useNavigate();

  const isGroup = chat.type === "group";
  const name = isGroup ? chat.name : chat.participants[0]?.name;
  const avatar = isGroup ? chat.avatar : chat.participants[0]?.avatar;
  const isOnline = !isGroup && chat.participants[0]?.status === "online";
  const isPinned = chat.isPinned;

  let timeStr = "";
  if (chat.lastMessage?.timestamp) {
    const date = new Date(chat.lastMessage.timestamp);
    const now = new Date();
    const isToday = date.toDateString() === now.toDateString();
    const isYesterday =
      new Date(now.getTime() - 86400000).toDateString() === date.toDateString();

    if (isToday) {
      timeStr = format(date, "HH:mm");
    } else if (isYesterday) {
      timeStr = t("chat.list.yesterday");
    } else if (now.getFullYear() === date.getFullYear()) {
      timeStr = format(date, "MM-dd");
    } else {
      timeStr = format(date, "yyyy-MM-dd");
    }
  }

  return (
    <div
      onClick={() => {
        if (!contextMenu.isOpen) navigate(`/chat/${chat.id}`);
      }}
      onContextMenu={(e) => {
        e.preventDefault();
        handleTouchStart(e, chat.id);
      }}
      onTouchStart={(e) => handleTouchStart(e, chat.id)}
      onTouchEnd={handleTouchEnd}
      onTouchMove={handleTouchMove}
      onMouseDown={(e) => handleTouchStart(e, chat.id)}
      onMouseUp={handleTouchEnd}
      onMouseLeave={handleTouchEnd}
      className={cn(
        "flex px-4 items-stretch gap-3 transition-colors cursor-pointer select-none",
        isPinned ? "bg-black/[0.03] dark:bg-white/[0.03]" : "bg-chat-other-bg",
        contextMenu.isOpen && contextMenu.chatId === chat.id
          ? "bg-active-bg"
          : "active:bg-black/5 dark:active:bg-white/5",
      )}
    >
      <div className="relative shrink-0 flex items-center py-3">
        <Avatar src={avatar} alt={name} size="lg" className="rounded-xl object-cover" />
        {chat.unreadCount > 0 && (
          <div className="absolute top-2 -right-1.5 bg-[#FF3B30] text-white text-[11px] font-medium h-[18px] min-w-[18px] px-1.5 rounded-full flex items-center justify-center z-10 border-2 border-white dark:border-[#1a1b1c] leading-none">
            {chat.unreadCount > 99 ? "99+" : chat.unreadCount}
          </div>
        )}
        {(isOnline || index === 0) && chat.unreadCount === 0 && (
          <div className="absolute bottom-2 -right-1 w-3.5 h-3.5 bg-[#34C759] border-2 border-white dark:border-[#1a1b1c] rounded-full z-10" />
        )}
      </div>

      <div
        className={cn(
          "flex-1 flex flex-col justify-center min-w-0 py-3",
          index !== chatsLength - 1 &&
            "border-b border-black/[0.06] dark:border-white/[0.06]",
        )}
      >
        <div className="flex justify-between items-baseline mb-0.5">
          <span className="font-medium text-[16px] text-text-main truncate">
            {name}
          </span>
          <div className="flex items-center gap-1 shrink-0 ml-2">
            {chat.settings?.isMuted && (
              <BellOff className="w-3.5 h-3.5 text-text-sub/60" />
            )}
            <span className="text-[12px] text-text-sub/70 font-medium tracking-tight">
              {timeStr}
            </span>
          </div>
        </div>
        <div className="flex justify-between items-center">
          <span className="text-[14px] text-text-sub truncate leading-tight">
            {chat.lastMessage?.content.includes(t("chat:list.at_me", "@我")) ||
            chat.lastMessage?.content.includes("@Me") ? (
              <>
                <span className="text-primary-blue">{t("chat.list.me")}</span>
                {chat.lastMessage.content
                  .replace(t("chat:list.at_me", "@我"), "")
                  .replace("@Me", "")}
              </>
            ) : (
              chat.lastMessage?.content
            )}
          </span>
        </div>
      </div>
    </div>
  );
};
