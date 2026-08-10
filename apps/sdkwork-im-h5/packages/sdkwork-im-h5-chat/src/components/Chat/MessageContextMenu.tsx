import React from "react";
import { motion, AnimatePresence } from "motion/react";
import { Copy, Reply, Forward, Star, Trash2, Pencil, Undo2, Pin } from "lucide-react";
import { showToast, cn } from "@sdkwork/im-h5-commons";
import type { Message } from "@sdkwork/im-h5-types";
import { useTranslation } from "react-i18next";

interface MessageContextMenuProps {
  contextMenu: {
    isOpen: boolean;
    x: number;
    y: number;
    messageId: string | null;
  };
  messages: Message[];
  onClose: () => void;
  onCopy: (msgId: string) => void;
  onReply: (msg: Message) => void;
  onStar: (msgId: string) => void;
  onDelete: (msgId: string) => void;
  onEdit?: (msgId: string) => void;
  onRecall?: (msgId: string) => void;
  onPin?: (msgId: string) => void;
  pinnedMessageIds?: string[];
  currentUserId?: string;
}

export const MessageContextMenu: React.FC<MessageContextMenuProps> = ({
  contextMenu,
  messages,
  onClose,
  onCopy,
  onReply,
  onStar,
  onDelete,
  onEdit,
  onRecall,
  onPin,
  pinnedMessageIds = [],
  currentUserId,
}) => {
  const { t } = useTranslation();
  const currentMessage = messages.find((m) => m.id === contextMenu.messageId);
  const isOwnMessage = currentUserId !== undefined && currentMessage?.senderId === currentUserId;
  // Locally-composed messages have no server identity yet: edit/recall are
  // unavailable, only removal (handled locally by the caller) applies.
  const hasLocalSendState = currentMessage?.sendState === "sending" || currentMessage?.sendState === "failed";
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
            onClick={onClose}
            onTouchStart={onClose}
          />
          <motion.div
            initial={{ opacity: 0, scale: 0.9, y: 10 }}
            animate={{ opacity: 1, scale: 1, y: 0 }}
            exit={{ opacity: 0, scale: 0.9, y: 10 }}
            transition={{ type: "spring", stiffness: 400, damping: 25 }}
            style={{ left: contextMenu.x, top: contextMenu.y }}
            className="fixed z-50 w-40 bg-white/95 dark:bg-[#2C2C2C]/95 backdrop-blur-xl rounded-2xl shadow-2xl border border-black/5 dark:border-white/10 overflow-hidden"
          >
            <div className="flex flex-col">
              {currentMessage?.type ===
                "text" && (
                <>
                  <div
                    className="flex items-center justify-between px-4 py-3.5 text-[15px] text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer"
                    onClick={(e) => {
                      e.stopPropagation();
                      if (contextMenu.messageId) onCopy(contextMenu.messageId);
                    }}
                  >
                    <span>{t('chat.context.copy')}</span>
                    <Copy className="w-4 h-4 opacity-70" />
                  </div>
                  <div className="h-[1px] bg-black/5 dark:bg-white/5 mx-4" />
                </>
              )}
              {isOwnMessage && currentMessage?.type === "text" && onEdit && !hasLocalSendState && (
                <>
                  <div
                    className="flex items-center justify-between px-4 py-3.5 text-[15px] text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer"
                    onClick={(e) => {
                      e.stopPropagation();
                      if (contextMenu.messageId) onEdit(contextMenu.messageId);
                    }}
                  >
                    <span>{t('chat.context.edit')}</span>
                    <Pencil className="w-4 h-4 opacity-70" />
                  </div>
                  <div className="h-[1px] bg-black/5 dark:bg-white/5 mx-4" />
                </>
              )}
              <div
                className="flex items-center justify-between px-4 py-3.5 text-[15px] text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer"
                onClick={(e) => {
                  e.stopPropagation();
                  const msg = messages.find(
                    (m) => m.id === contextMenu.messageId,
                  );
                  if (msg) onReply(msg);
                  onClose();
                }}
              >
                <span>{t('chat.context.reply')}</span>
                <Reply className="w-4 h-4 opacity-70" />
              </div>
              <div className="h-[1px] bg-black/5 dark:bg-white/5 mx-4" />
              <div
                className="flex items-center justify-between px-4 py-3.5 text-[15px] text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer"
                onClick={(e) => {
                  e.stopPropagation();
                  showToast(t('chat.context.forward_disabled'));
                  onClose();
                }}
              >
                <span>{t('chat.context.forward')}</span>
                <Forward className="w-4 h-4 opacity-70" />
              </div>
              <div className="h-[1px] bg-black/5 dark:bg-white/5 mx-4" />
              <div
                className="flex items-center justify-between px-4 py-3.5 text-[15px] text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer"
                onClick={(e) => {
                  e.stopPropagation();
                  if (contextMenu.messageId) onStar(contextMenu.messageId);
                }}
              >
                <span>
                  {currentMessage?.isStarred
                    ? t('chat.context.unstar')
                    : t('chat.context.star')}
                </span>
                <Star
                  className={cn(
                    "w-4 h-4 opacity-70",
                    currentMessage?.isStarred && "fill-current text-yellow-500 opacity-100",
                  )}
                />
              </div>
              {isOwnMessage && onRecall && !hasLocalSendState && (
                <>
                  <div className="h-[1px] bg-black/5 dark:bg-white/5 mx-4" />
                  <div
                    className="flex items-center justify-between px-4 py-3.5 text-[15px] text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer"
                    onClick={(e) => {
                      e.stopPropagation();
                      if (contextMenu.messageId) onRecall(contextMenu.messageId);
                    }}
                  >
                    <span>{t('chat.context.recall')}</span>
                    <Undo2 className="w-4 h-4 opacity-70" />
                  </div>
                </>
              )}
              {onPin && !hasLocalSendState && (
                <>
                  <div className="h-[1px] bg-black/5 dark:bg-white/5 mx-4" />
                  <div
                    className="flex items-center justify-between px-4 py-3.5 text-[15px] text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer"
                    onClick={(e) => {
                      e.stopPropagation();
                      if (contextMenu.messageId) onPin(contextMenu.messageId);
                    }}
                  >
                    <span>
                      {pinnedMessageIds.includes(contextMenu.messageId ?? "")
                        ? t('chat.context.unpin_message')
                        : t('chat.context.pin_message')}
                    </span>
                    <Pin className="w-4 h-4 opacity-70" />
                  </div>
                </>
              )}
              <div className="h-[1px] bg-black/5 dark:bg-white/5 mx-4" />
              <div
                className="flex items-center justify-between px-4 py-3.5 text-[15px] text-accent-red active:bg-red-50 dark:active:bg-red-950/30 transition-colors cursor-pointer"
                onClick={(e) => {
                  e.stopPropagation();
                  if (contextMenu.messageId) onDelete(contextMenu.messageId);
                }}
              >
                <span>{t('chat.context.delete')}</span>
                <Trash2 className="w-4 h-4" />
              </div>
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  );
};
