import React, { useState, useRef, useEffect, useCallback } from "react";
import { useNavigate } from "react-router";
import {
  Search,
  PlusCircle,
  MessageSquarePlus,
  UserPlus,
  Bot,
  Scan,
  Pin,
  BellOff,
  Trash2,
} from "lucide-react";
import { format } from "date-fns";
import {
  Avatar,
  Badge,
  IconButton,
  cn,
} from "@sdkwork/im-h5-commons";
import type { Chat } from "@sdkwork/im-h5-types";
import { ChatService } from "../services/ChatService";
import { motion, AnimatePresence } from "motion/react";
import { ChatListContextMenu } from "../components/Chat/ChatListContextMenu";
import { ChatListItem } from "../components/Chat/ChatListItem";
import { ChatListHeader } from "../components/Chat/ChatListHeader";
import { useTranslation } from "react-i18next";

export const ChatList: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const [chats, setChats] = useState<Chat[]>([]);
  const menuRef = useRef<HTMLDivElement>(null);

  // Context Menu State
  const [contextMenu, setContextMenu] = useState<{
    isOpen: boolean;
    x: number;
    y: number;
    chatId: string | null;
  }>({ isOpen: false, x: 0, y: 0, chatId: null });

  const longPressTimer = useRef<NodeJS.Timeout | null>(null);

  const loadChats = useCallback(() => {
    ChatService.getChats().then((data) => {
      // Sort pinned chats to top
      const sorted = [...data].sort((a, b) => {
        if (a.isPinned && !b.isPinned) return -1;
        if (!a.isPinned && b.isPinned) return 1;
        return 0; // Keep original order for others
      });
      setChats(sorted);
    });
  }, []);

  useEffect(() => {
    loadChats();
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
