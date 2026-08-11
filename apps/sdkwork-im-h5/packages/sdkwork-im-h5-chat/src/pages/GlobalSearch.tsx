import React, { useState, useRef, useEffect } from "react";
import { useNavigate } from "react-router";
import { Search, X } from "lucide-react";
import { ChatService } from "../services/ChatService";
import { ContactService } from "@sdkwork/im-h5-contacts";
import type { Chat, User as UserType } from "@sdkwork/im-h5-types";
import { useTranslation } from "react-i18next";
import { GlobalSearchQuickTags } from "../components/Chat/GlobalSearchQuickTags";
import { GlobalSearchResults } from "../components/Chat/GlobalSearchResults";

export const GlobalSearch: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [query, setQuery] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  const [chats, setChats] = useState<Chat[]>([]);
  const [contacts, setContacts] = useState<UserType[]>([]);
  const [isSearching, setIsSearching] = useState(false);
  // Request sequence guard: only the latest query's response may render, so a
  // slow earlier search cannot overwrite a newer one (mirrors ChatList).
  const searchRequestSeq = useRef(0);

  useEffect(() => {
    inputRef.current?.focus();
  }, []);

  useEffect(() => {
    const doSearch = async () => {
      if (!query.trim()) {
        searchRequestSeq.current += 1;
        setChats([]);
        setContacts([]);
        setIsSearching(false);
        return;
      }
      const requestSeq = ++searchRequestSeq.current;
      setIsSearching(true);
      const searchChatsPromise = ChatService.searchChats(query);
      const searchContactsPromise = ContactService.searchContacts(query);

      const [searchedChats, searchedContacts] = await Promise.all([
        searchChatsPromise,
        searchContactsPromise,
      ]);

      if (requestSeq !== searchRequestSeq.current) return;
      setChats(searchedChats);
      setContacts(searchedContacts);

      setIsSearching(false);
    };

    const timer = setTimeout(doSearch, 300);
    return () => clearTimeout(timer);
  }, [query]);

  return (
    <div className="flex flex-col h-full bg-bg-color">
      {/* Header */}
      <header className="h-[56px] flex items-center px-3 glass-header sticky top-0 z-10 shrink-0 pt-safe gap-3">
        <div className="flex-1 flex items-center bg-chat-other-bg rounded-lg h-9 px-2.5 border border-border-color transition-colors focus-within:border-primary-blue focus-within:bg-bg-color">
          <Search className="w-4 h-4 text-text-sub shrink-0" />
          <input
            ref={inputRef}
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder={t('chat.search.placeholder')}
            className="flex-1 bg-transparent border-none outline-none px-2 text-[16px] text-text-main placeholder:text-text-sub min-w-0"
          />
          {query && (
            <div
              onClick={() => setQuery("")}
              className="p-1 cursor-pointer shrink-0"
            >
              <X className="w-3.5 h-3.5 text-white bg-black/20 dark:bg-white/20 rounded-full p-0.5" />
            </div>
          )}
        </div>
        <button
          onClick={() => navigate(-1)}
          className="text-[16px] text-text-main font-medium whitespace-nowrap shrink-0 active:opacity-70"
        >
          {t('chat.search.cancel')}
        </button>
      </header>

      {/* Content */}
      <div className="flex-1 overflow-y-auto">
        {!query ? (
          <GlobalSearchQuickTags t={t} />
        ) : (
          <GlobalSearchResults
            t={t}
            query={query}
            isSearching={isSearching}
            contacts={contacts}
            chats={chats}
            onSelectContact={(contactId) =>
              navigate("/workspace/contacts", {
                state: { searchUser: contactId },
              })
            }
            onSelectChat={(chatId) => navigate(`/chat/${chatId}`)}
          />
        )}
      </div>
    </div>
  );
};
