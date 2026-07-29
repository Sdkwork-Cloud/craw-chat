import React from "react";
import { Search, X } from "lucide-react";
import { Avatar } from "@sdkwork/im-h5-commons";
import { useNavigate } from "react-router";
import type { Chat, Message } from "@sdkwork/im-h5-types";
import { useTranslation } from "react-i18next";

interface SearchHistoryOverlayProps {
  id: string;
  chat: Chat | null;
  showSearch: boolean;
  setShowSearch: (show: boolean) => void;
  searchQuery: string;
  setSearchQuery: (query: string) => void;
  isSearching: boolean;
  searchResults: Message[];
}

export const SearchHistoryOverlay: React.FC<SearchHistoryOverlayProps> = ({
  id,
  chat,
  showSearch,
  setShowSearch,
  searchQuery,
  setSearchQuery,
  isSearching,
  searchResults,
}) => {
  const { t } = useTranslation();
const navigate = useNavigate();

  if (!showSearch) return null;

  return (
    <div className="fixed inset-0 z-50 bg-bg-color flex flex-col animate-in slide-in-from-right">
      <header className="h-[56px] flex items-center gap-3 px-3 glass-header shrink-0 pt-safe border-b border-border-color/50">
        <div className="flex bg-chat-other-bg flex-1 rounded-full items-center px-3 h-9 border border-border-color focus-within:border-primary-blue transition-colors relative">
          <Search className="w-4 h-4 text-text-sub shrink-0" />
          <input
            autoFocus
            type="text"
            placeholder={t('chat.search.search_history')}
            className="bg-transparent flex-1 outline-none text-[15px] text-text-main px-2 ml-1"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
          {searchQuery && (
            <button
              onClick={() => setSearchQuery("")}
              className="p-1 active:opacity-50 text-text-sub"
            >
              <X className="w-4 h-4" />
            </button>
          )}
        </div>
        <button
          onClick={async () => {
            setShowSearch(false);
            setSearchQuery("");
          }}
          className="text-[15px] text-primary-blue font-medium active:opacity-70 px-1"
        >
          {t('chat.search.cancel')}
        </button>
      </header>

      <div className="flex-1 overflow-y-auto">
        {!searchQuery.trim() ? (
          <div className="h-full flex flex-col items-center justify-center text-text-sub opacity-50">
            <Search className="w-12 h-12 mb-3" />
            <span className="text-[15px]">{t('chat.search.enter_keyword')}</span>
          </div>
        ) : isSearching ? (
          <div className="h-full flex flex-col items-center pt-24 text-text-sub">
            <div className="w-6 h-6 border-2 border-primary-blue border-t-transparent rounded-full animate-spin mb-3"></div>
            <span className="text-[14px]">{t('chat.search.searching')}</span>
          </div>
        ) : searchResults.length > 0 ? (
          <div className="flex flex-col gap-1 p-2">
            {searchResults.map((msg) => {
              const sender =
                chat?.participants.find((p) => p.id === msg.senderId) || null;
              return (
                <div
                  key={msg.id}
                  className="bg-chat-other-bg rounded-xl p-3 flex gap-3 active:bg-active-bg transition-colors cursor-pointer"
                  onClick={async () => {
                    setShowSearch(false);
                    setSearchQuery("");
                    navigate(`/chat/${id}?msgId=${msg.id}`);
                  }}
                >
                  <Avatar
                    src={sender?.avatar}
                    fallback={sender?.name}
                    size="md"
                    className="shrink-0"
                  />
                  <div className="flex flex-col flex-1 min-w-0">
                    <div className="flex justify-between items-center mb-1">
                      <span className="text-[14px] font-medium text-text-main truncate max-w-[120px]">
                        {sender?.name || t('chat.search.unknown')}
                      </span>
                      <span className="text-[12px] text-text-sub shrink-0">
                        {new Date(msg.timestamp).toLocaleDateString()}
                      </span>
                    </div>
                    <span className="text-[14px] text-text-main opacity-90 line-clamp-2 leading-relaxed">
                      {msg.content}
                    </span>
                  </div>
                </div>
              );
            })}
          </div>
        ) : (
          <div className="h-full flex flex-col items-center pt-32 text-text-sub opacity-70">
            <span className="text-[15px]">{t('chat.search.no_history')}</span>
          </div>
        )}
      </div>
    </div>
  );
};
