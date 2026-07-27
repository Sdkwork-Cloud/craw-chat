import { useNavigate } from "react-router";
import { useParams } from "react-router";
import React, { useState, useEffect } from "react";
import {} from "react-router";
import {
  ChevronLeft,
  Search,
  Bell,
  Pin,
  Image as ImageIcon,
  Trash2,
  Plus,
  ChevronRight,
  Settings2,
  EyeOff,
  X,
} from "lucide-react";
import {
  Avatar,
  IconButton,
  cn,
  showConfirm,
  ListItem,
  Switch,
} from "@sdkwork/im-h5-commons";
import { ChatService } from "../services/ChatService";
import type { Chat, Message } from "@sdkwork/im-h5-types";
import { SearchHistoryOverlay } from "../components/Chat/SearchHistoryOverlay";
import { useTranslation } from "react-i18next";

import { ChatProfileMembers } from "../components/Chat/ChatProfileMembers";

export const ChatProfile: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams();
  const navigate = useNavigate();
  const [chat, setChat] = useState<Chat | null>(null);
  const [isMuted, setIsMuted] = useState(false);
  const [isPinned, setIsPinned] = useState(false);
  const [showAvatar, setShowAvatar] = useState(true);
  const [cleanMode, setCleanMode] = useState(false);

  // Search state
  const [showSearch, setShowSearch] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const [searchResults, setSearchResults] = useState<Message[]>([]);
  const [isSearching, setIsSearching] = useState(false);

  useEffect(() => {
    if (id) {
      ChatService.getChatById(id).then((c) => {
        if (c) {
          setChat(c);
          setShowAvatar(c.settings?.showAvatar ?? true);
          setCleanMode(c.settings?.cleanMode ?? false);
        }
      });
    }
  }, [id]);

  useEffect(() => {
    if (showSearch && searchQuery.trim() && id) {
      setIsSearching(true);
      const timer = setTimeout(async () => {
        const results = await ChatService.searchChatHistory(id, searchQuery);
        setSearchResults(results);
        setIsSearching(false);
      }, 300);
      return () => clearTimeout(timer);
    } else {
      setSearchResults([]);
    }
  }, [searchQuery, showSearch, id]);

  const handleUpdateSettings = async (updates: Partial<Chat["settings"]>) => {
    if (id) {
      await ChatService.updateChatSettings(id, updates);
    }
  };

  return (
    <div className="flex flex-col h-full bg-bg-color overflow-y-auto">
      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h2 className="text-[17px] font-medium text-text-main">{t('chat.profile.title')}</h2>
        </div>
        <div className="flex-1" />
      </header>

      <div className="flex flex-col px-0 sm:px-4 pb-8">
        {/* Members */}
        <ChatProfileMembers
          chat={chat}
          onAddMember={() => navigate(`/create-group?chatId=${id}`)}
        />

        {/* Settings Group 1 */}
        <div className="mb-2 sm:mb-4 sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
          <ListItem
            icon={Search}
            label={t('chat.profile.search_history')}
            onClick={() => setShowSearch(true)}
          />
        </div>

        {/* Settings Group 2 */}
        <div className="mb-2 sm:mb-4 sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
          <ListItem
            icon={Bell}
            label={t('chat.profile.mute')}
            rightElement={<Switch checked={isMuted} onChange={setIsMuted} />}
          />
          <ListItem
            icon={Pin}
            label={t('chat.profile.pin')}
            rightElement={<Switch checked={isPinned} onChange={setIsPinned} />}
          />
        </div>

        {/* Settings Group 3 (Display Settings) */}
        <div className="mb-2 sm:mb-4 sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
          <ListItem
            icon={Settings2}
            label={t('chat.profile.show_avatar')}
            rightElement={
              <Switch
                checked={showAvatar}
                onChange={(val: boolean) => {
                  setShowAvatar(val);
                  handleUpdateSettings({ showAvatar: val });
                }}
              />
            }
          />
          <ListItem
            icon={EyeOff}
            label={t('chat.profile.clean_mode')}
            rightElement={
              <Switch
                checked={cleanMode}
                onChange={(val: boolean) => {
                  setCleanMode(val);
                  handleUpdateSettings({ cleanMode: val });
                }}
              />
            }
          />
        </div>

        {/* Settings Group 4 */}
        <div className="mb-6 sm:mb-8 sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
          <ListItem icon={ImageIcon} label={t('chat.profile.set_background')} />
        </div>

        {/* Danger Zone */}
        <div className="sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
          <ListItem
            icon={Trash2}
            label={t('chat.profile.clear_history')}
            danger={true}
            rightElement={<div />}
            onClick={async () => {
              if (await showConfirm(t('chat.profile.clear_history_confirm'))) {
                ChatService.clearChatHistory(id as string).then(() => {
                  navigate(`/chat/${id}`, { replace: true });
                });
              }
            }}
          />
        </div>
      </div>

      <SearchHistoryOverlay
        id={id as string}
        chat={chat}
        showSearch={showSearch}
        setShowSearch={setShowSearch}
        searchQuery={searchQuery}
        setSearchQuery={setSearchQuery}
        isSearching={isSearching}
        searchResults={searchResults}
      />
    </div>
  );
};
