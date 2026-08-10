import React, { useState, useEffect, useCallback } from "react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";
import { ChevronLeft, Users } from "lucide-react";
import { IconButton, showToast } from "@sdkwork/im-h5-commons";
import type { Chat } from "@sdkwork/im-h5-types";
import { ChatService } from "../services/ChatService";
import { ChatListItem } from "../components/Chat/ChatListItem";
import { subscribeInboxLiveRefresh } from "../services/chatRealtimeService";

const NO_CONTEXT_MENU = { isOpen: false, chatId: null as string | null };

export const GroupChatListPage: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [groups, setGroups] = useState<Chat[]>([]);
  const [nextCursor, setNextCursor] = useState<string>();
  const [hasMore, setHasMore] = useState(false);
  const [loading, setLoading] = useState(true);
  const [loadingMore, setLoadingMore] = useState(false);
  const [loadError, setLoadError] = useState(false);

  const load = useCallback(async (cursor?: string) => {
    cursor ? setLoadingMore(true) : setLoading(true);
    setLoadError(false);
    try {
      const page = await ChatService.listChatPage(cursor, undefined, "group");
      setGroups((previous) => mergeGroups(cursor ? previous : [], page.items));
      setNextCursor(page.hasMore ? page.nextCursor : undefined);
      setHasMore(page.hasMore);
    } catch (error) {
      console.error(error);
      setLoadError(true);
      showToast(t("chat.list.load_failed", "Unable to load conversations"));
    } finally {
      setLoading(false);
      setLoadingMore(false);
    }
  }, [t]);

  useEffect(() => {
    void load();
    const unsubscribe = subscribeInboxLiveRefresh(() => { void load(); });
    return unsubscribe;
  }, [load]);

  return (
    <div className="flex flex-col h-full bg-bg-color">
      <header className="h-[52px] flex items-center justify-between px-2 bg-bg-color/90 backdrop-blur-md sticky top-0 z-20 shrink-0 pt-safe">
        <div className="flex items-center z-10 w-[80px]">
          <IconButton
            icon={<ChevronLeft className="w-7 h-7 text-text-main" strokeWidth={2} />}
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute inset-x-0 text-center font-semibold text-[17px] text-text-main pointer-events-none">
          {t('contacts.group_chats')}
        </div>
        <div className="w-[80px]" />
      </header>

      <div
        className="flex-1 overflow-y-auto no-scrollbar"
        onScroll={(event) => {
          if (loadingMore || !hasMore || !nextCursor) return;
          const element = event.currentTarget;
          if (element.scrollTop + element.clientHeight >= element.scrollHeight - 120) {
            void load(nextCursor);
          }
        }}
      >
        {groups.map((chat, index) => (
          <ChatListItem
            key={chat.id}
            chat={chat}
            index={index}
            chatsLength={groups.length}
            contextMenu={NO_CONTEXT_MENU}
            handleTouchStart={() => undefined}
            handleTouchEnd={() => undefined}
            handleTouchMove={() => undefined}
          />
        ))}
        {!loading && !loadError && groups.length === 0 && (
          <div className="flex flex-col items-center justify-center py-20 text-text-sub gap-2">
            <Users className="w-12 h-12 text-text-sub/30" />
            <span className="text-[14px]">{t('contacts.no_group_chats', '暂无群聊')}</span>
          </div>
        )}
        {loading && (
          <div className="flex justify-center p-6 mt-10">
            <div className="w-6 h-6 border-2 border-primary-blue border-t-transparent rounded-full animate-spin"></div>
          </div>
        )}
        {!loading && loadError && (
          <button
            type="button"
            className="w-full p-10 text-center text-[14px] text-primary-blue"
            onClick={() => void load()}
          >
            {t('contacts.load_failed', 'Unable to load conversations')}
          </button>
        )}
        {loadingMore && (
          <div className="flex justify-center p-4">
            <div className="w-5 h-5 border-2 border-primary-blue border-t-transparent rounded-full animate-spin"></div>
          </div>
        )}
      </div>
    </div>
  );
};

function mergeGroups(previous: readonly Chat[], incoming: readonly Chat[]): Chat[] {
  const groups = new Map(previous.map((chat) => [chat.id, chat]));
  for (const chat of incoming) groups.set(chat.id, chat);
  return Array.from(groups.values());
}
