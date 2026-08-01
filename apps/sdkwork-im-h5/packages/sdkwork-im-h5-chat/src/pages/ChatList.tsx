import React, { useState, useRef, useEffect, useCallback } from "react";
import type { Chat } from "@sdkwork/im-h5-types";
import { ChatService } from "../services/ChatService";
import { motion, AnimatePresence } from "motion/react";
import { ChatListContextMenu } from "../components/Chat/ChatListContextMenu";
import { ChatListItem } from "../components/Chat/ChatListItem";
import { ChatListHeader } from "../components/Chat/ChatListHeader";
import { useTranslation } from "react-i18next";
import { subscribeInboxLiveRefresh } from "../services/chatRealtimeService";
import { showToast } from "@sdkwork/im-h5-commons";

export const ChatList: React.FC = () => {
  const { t } = useTranslation();
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const [chats, setChats] = useState<Chat[]>([]);
  const [nextCursor, setNextCursor] = useState<string>();
  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [loadError, setLoadError] = useState(false);
  const chatsRef = useRef<Chat[]>([]);
  const menuRef = useRef<HTMLDivElement>(null);

  // Context Menu State
  const [contextMenu, setContextMenu] = useState<{
    isOpen: boolean;
    x: number;
    y: number;
    chatId: string | null;
  }>({ isOpen: false, x: 0, y: 0, chatId: null });

  const longPressTimer = useRef<NodeJS.Timeout | null>(null);

  const loadChats = useCallback(async (cursor?: string) => {
    if (cursor) setIsLoadingMore(true);
    else setIsLoading(true);
    setLoadError(false);
    try {
      const page = await ChatService.listChatPage(cursor);
      const merged = mergeChats(cursor ? chatsRef.current : [], page.items);
      const sorted = sortChats(merged);
      chatsRef.current = sorted;
      setChats(sorted);
      setNextCursor(page.hasMore ? page.nextCursor : undefined);
    } catch (error) {
      console.error(error);
      setLoadError(true);
      showToast(t("chat.list.load_failed", "Unable to load conversations"));
    } finally {
      setIsLoading(false);
      setIsLoadingMore(false);
    }
  }, [t]);

  useEffect(() => {
    void loadChats();
    const unsubscribe = subscribeInboxLiveRefresh(() => { void loadChats(); });
    return unsubscribe;
  }, [loadChats]);

  // Handle click outside to close menu
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent | TouchEvent) => {
  if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setIsMenuOpen(false);
      }
      if (contextMenu.isOpen) {
        setContextMenu((prev) => ({ ...prev, isOpen: false }));
      }
    };

    if (isMenuOpen || contextMenu.isOpen) {
      document.addEventListener("mousedown", handleClickOutside);
      document.addEventListener("touchstart", handleClickOutside);
    }

    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
      document.removeEventListener("touchstart", handleClickOutside);
    };
  }, [isMenuOpen, contextMenu.isOpen]);

  const handleTouchStart = useCallback(
    (e: React.TouchEvent | React.MouseEvent, chatId: string) => {
      if (longPressTimer.current) clearTimeout(longPressTimer.current);

      // Get coordinates
      let clientX, clientY;
      if ("touches" in e) {
        clientX = e.touches[0].clientX;
        clientY = e.touches[0].clientY;
      } else {
        clientX = (e as React.MouseEvent).clientX;
        clientY = (e as React.MouseEvent).clientY;
      }

      longPressTimer.current = setTimeout(() => {
        if (navigator.vibrate) navigator.vibrate(50);

        // Calculate position to prevent overflowing screen edges
        const menuWidth = 180;
        const menuHeight = 160;
        const x = Math.min(clientX, window.innerWidth - menuWidth - 20);
        const y = Math.min(clientY, window.innerHeight - menuHeight - 20);

        setContextMenu({
          isOpen: true,
          x: Math.max(20, x),
          y: Math.max(20, y),
          chatId,
        });
      }, 500); // 500ms long press
    },
    [],
  );

  const handleTouchEnd = useCallback(() => {
    if (longPressTimer.current) {
      clearTimeout(longPressTimer.current);
      longPressTimer.current = null;
    }
  }, []);

  const handleTouchMove = useCallback(() => {
    if (longPressTimer.current) {
      clearTimeout(longPressTimer.current);
      longPressTimer.current = null;
    }
  }, []);

  const handlePinChat = async (chatId: string, isPinned: boolean) => {
    await ChatService.pinChat(chatId, isPinned);
    loadChats();
    setContextMenu((prev) => ({ ...prev, isOpen: false }));
  };

  const handleMarkAsUnread = async (chatId: string) => {
    await ChatService.markAsUnread(chatId);
    loadChats();
    setContextMenu((prev) => ({ ...prev, isOpen: false }));
  };

  const handleDeleteChat = async (chatId: string) => {
    await ChatService.deleteChat(chatId);
    loadChats();
    setContextMenu((prev) => ({ ...prev, isOpen: false }));
  };

  return (
    <div className="flex flex-col h-full bg-bg-color">
      {/* Header */}
      <ChatListHeader
        menuRef={menuRef}
        isMenuOpen={isMenuOpen}
        setIsMenuOpen={setIsMenuOpen}
      />

      {/* Chat List */}
      <div className="flex-1 overflow-y-auto pt-1 pb-[84px]">
        {isLoading && chats.length === 0 && (
          <div className="flex h-24 items-center justify-center text-[14px] text-text-sub">
            {t("common.loading", "Loading...")}
          </div>
        )}
        {!isLoading && loadError && chats.length === 0 && (
          <button type="button" className="flex h-24 w-full items-center justify-center text-[14px] text-primary-blue" onClick={() => void loadChats()}>
            {t("common.retry", "Tap to retry")}
          </button>
        )}
        {!isLoading && !loadError && chats.length === 0 && (
          <div className="flex h-24 items-center justify-center text-[14px] text-text-sub">
            {t("chat.list.empty", "No conversations yet")}
          </div>
        )}
        {chats.map((chat, index) => (
          <ChatListItem
            key={chat.id}
            chat={chat}
            index={index}
            chatsLength={chats.length}
            contextMenu={contextMenu}
            handleTouchStart={handleTouchStart}
            handleTouchEnd={handleTouchEnd}
            handleTouchMove={handleTouchMove}
          />
        ))}
        {nextCursor && (
          <button
            type="button"
            disabled={isLoadingMore}
            className="h-12 w-full text-[14px] font-medium text-primary-blue disabled:opacity-50"
            onClick={() => loadChats(nextCursor)}
          >
            {isLoadingMore ? t("common.loading", "Loading...") : t("common.load_more", "Load more")}
          </button>
        )}
      </div>

      <ChatListContextMenu
        contextMenu={contextMenu}
        setContextMenu={setContextMenu}
        chats={chats}
        handlePinChat={handlePinChat}
        handleMarkAsUnread={handleMarkAsUnread}
        handleDeleteChat={handleDeleteChat}
      />
    </div>
  );
};

function mergeChats(previous: readonly Chat[], incoming: readonly Chat[]): Chat[] {
  const chats = new Map(previous.map((chat) => [chat.id, chat]));
  for (const chat of incoming) chats.set(chat.id, chat);
  return Array.from(chats.values());
}

function sortChats(chats: readonly Chat[]): Chat[] {
  return [...chats].sort((left, right) => Number(Boolean(right.isPinned)) - Number(Boolean(left.isPinned)));
}
