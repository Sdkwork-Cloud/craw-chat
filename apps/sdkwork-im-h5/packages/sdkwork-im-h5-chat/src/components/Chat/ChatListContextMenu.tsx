import React from "react";
import { Pin, BellOff, Bell, Trash2 } from "lucide-react";
import { motion, AnimatePresence } from "motion/react";
import type { Chat } from "@sdkwork/im-h5-types";
import { useTranslation } from "react-i18next";

interface ChatListContextMenuProps {
  contextMenu: { isOpen: boolean; x: number; y: number; chatId: string | null };
  setContextMenu: React.Dispatch<
    React.SetStateAction<{
      isOpen: boolean;
      x: number;
      y: number;
      chatId: string | null;
    }>
  >;
  chats: Chat[];
  handlePinChat: (chatId: string, isPinned: boolean) => void;
  handleMarkAsUnread: (chatId: string) => void;
  handleToggleMute: (chatId: string, isMuted: boolean) => void;
  handleDeleteChat: (chatId: string) => void;
}

export const ChatListContextMenu: React.FC<ChatListContextMenuProps> = ({
  contextMenu,
  setContextMenu,
  chats,
  handlePinChat,
  handleMarkAsUnread,
  handleToggleMute,
  handleDeleteChat,
}) => {
  const { t } = useTranslation();
  const currentChat = chats.find((c) => c.id === contextMenu.chatId);
return (
    <AnimatePresence>
      {contextMenu.isOpen && (
        <>
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.15 }}
            className="fixed inset-0 z-40 bg-black/20 dark:bg-black/40 backdrop-blur-sm"
            onClick={() =>
              setContextMenu((prev) => ({ ...prev, isOpen: false }))
            }
            onTouchStart={() =>
              setContextMenu((prev) => ({ ...prev, isOpen: false }))
            }
          />
          <motion.div
            initial={{ opacity: 0, scale: 0.9, y: 10 }}
            animate={{ opacity: 1, scale: 1, y: 0 }}
            exit={{ opacity: 0, scale: 0.9, y: 10 }}
            transition={{ type: "spring", stiffness: 400, damping: 25 }}
            style={{ left: contextMenu.x, top: contextMenu.y }}
            className="fixed z-50 w-48 bg-white/95 dark:bg-[#2C2C2C]/95 backdrop-blur-xl rounded-2xl shadow-2xl border border-black/5 dark:border-white/10 overflow-hidden"
          >
            <div className="flex flex-col">
              <div
                className="flex items-center justify-between px-4 py-3.5 text-[15px] text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer"
                onClick={(e) => {
                  e.stopPropagation();
                  if (contextMenu.chatId) {
                    const chat = chats.find((c) => c.id === contextMenu.chatId);
                    handlePinChat(contextMenu.chatId, !chat?.isPinned);
                  }
                }}
              >
                <span>
                  {currentChat?.isPinned
                    ? t('chat.list.unpin')
                    : t('chat.list.pin')}
                </span>
                <Pin className="w-4 h-4 opacity-70" />
              </div>
              <div className="h-[1px] bg-black/5 dark:bg-white/5 mx-4" />
              <div
                className="flex items-center justify-between px-4 py-3.5 text-[15px] text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer"
                onClick={(e) => {
                  e.stopPropagation();
                  if (contextMenu.chatId)
                    handleMarkAsUnread(contextMenu.chatId);
                }}
              >
                <span>{t('chat.list.mark_unread')}</span>
                <BellOff className="w-4 h-4 opacity-70" />
              </div>
              <div className="h-[1px] bg-black/5 dark:bg-white/5 mx-4" />
              <div
                className="flex items-center justify-between px-4 py-3.5 text-[15px] text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer"
                onClick={(e) => {
                  e.stopPropagation();
                  if (contextMenu.chatId) {
                    handleToggleMute(contextMenu.chatId, !currentChat?.settings?.isMuted);
                  }
                }}
              >
                <span>
                  {currentChat?.settings?.isMuted
                    ? t('chat.list.unmute')
                    : t('chat.list.mute')}
                </span>
                {currentChat?.settings?.isMuted ? (
                  <BellOff className="w-4 h-4 opacity-70" />
                ) : (
                  <Bell className="w-4 h-4 opacity-70" />
                )}
              </div>
              <div className="h-[1px] bg-black/5 dark:bg-white/5 mx-4" />
              <div
                className="flex items-center justify-between px-4 py-3.5 text-[15px] text-accent-red active:bg-red-50 dark:active:bg-red-950/30 transition-colors cursor-pointer"
                onClick={(e) => {
                  e.stopPropagation();
                  if (contextMenu.chatId) handleDeleteChat(contextMenu.chatId);
                }}
              >
                <span>{t('chat.list.delete_chat')}</span>
                <Trash2 className="w-4 h-4" />
              </div>
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  );
};
