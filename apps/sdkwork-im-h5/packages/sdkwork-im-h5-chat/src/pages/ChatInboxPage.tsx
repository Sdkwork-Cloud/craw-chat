import { useCallback, useEffect, useState } from "react";
import { Loader2, MessageCircle, RefreshCw } from "lucide-react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";
import { MAX_LIST_PAGE_SIZE } from "@sdkwork/utils";
import type {
  ConversationInboxEntry,
  ConversationInboxPage,
} from "@sdkwork/im-sdk";

import { listInbox } from "../services/chatConversationService";
import { subscribeInboxLiveRefresh } from "../services/chatRealtimeService";

interface ChatInboxPageProps {
  onOpenConversation?: (conversationId: string) => void;
}

const INBOX_PAGE_SIZE = 50;

export function ChatInboxPage({ onOpenConversation }: ChatInboxPageProps) {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [items, setItems] = useState<ConversationInboxEntry[]>([]);
  const [nextCursor, setNextCursor] = useState<string | undefined>();
  const [isLoading, setIsLoading] = useState(true);
  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const [error, setError] = useState(false);

  const load = useCallback(async (cursor?: string) => {
    if (cursor) {
      setIsLoadingMore(true);
    } else {
      setIsLoading(true);
      setError(false);
    }
    try {
      const page = await listInbox({
        pageSize: Math.min(INBOX_PAGE_SIZE, MAX_LIST_PAGE_SIZE),
        ...(cursor ? { cursor } : {}),
      });
      assertCursorPage(page);
      setItems((previous) => mergeInboxItems(cursor ? previous : [], page.items));
      setNextCursor(
        page.pageInfo.hasMore && page.pageInfo.nextCursor
          ? page.pageInfo.nextCursor
          : undefined,
      );
    } catch {
      setError(true);
    } finally {
      setIsLoading(false);
      setIsLoadingMore(false);
    }
  }, []);

  useEffect(() => {
    void load();
    const unsubscribe = subscribeInboxLiveRefresh(() => {
      void load();
    });
    return unsubscribe;
  }, [load]);

  const openConversation = (conversationId: string) => {
    if (onOpenConversation) {
      onOpenConversation(conversationId);
      return;
    }
    navigate("/chat/" + conversationId);
  };

  return (
    <div className="flex h-full flex-col bg-bg-color pb-safe">
      <header className="flex h-[56px] shrink-0 items-center justify-between border-b border-border-color px-4 pt-safe">
        <h1 className="text-[20px] font-semibold text-text-main">
          {t("chat.inbox.title", "Conversations")}
        </h1>
        <button
          type="button"
          className="flex h-9 w-9 items-center justify-center text-text-sub"
          onClick={() => void load()}
          aria-label={t("common.refresh", "Refresh")}
        >
          <RefreshCw className="h-5 w-5" />
        </button>
      </header>
      <div className="min-h-0 flex-1 overflow-y-auto pb-20">
        {isLoading && (
          <div className="flex justify-center p-8">
            <Loader2 className="h-6 w-6 animate-spin text-primary-blue" />
          </div>
        )}
        {!isLoading && error && (
          <div className="flex flex-col items-center gap-3 p-8 text-center text-text-sub">
            <MessageCircle className="h-10 w-10" />
            <p className="text-[14px]">
              {t("chat.inbox.load_failed", "Unable to load conversations")}
            </p>
            <button
              type="button"
              className="text-[14px] font-medium text-primary-blue"
              onClick={() => void load()}
            >
              {t("common.retry", "Retry")}
            </button>
          </div>
        )}
        {!isLoading && !error && items.length === 0 && (
          <div className="flex flex-col items-center gap-3 p-12 text-text-sub">
            <MessageCircle className="h-10 w-10" />
            <p className="text-[14px]">
              {t("chat.inbox.empty", "No conversations")}
            </p>
          </div>
        )}
        {!isLoading && !error && items.map((item) => (
          <button
            key={item.conversationId}
            type="button"
            className="flex w-full items-center gap-3 border-b border-border-color px-4 py-3 text-left active:bg-active-bg"
            onClick={() => openConversation(item.conversationId)}
          >
            <div className="flex h-11 w-11 shrink-0 items-center justify-center rounded-full bg-primary-blue/10 text-primary-blue">
              <MessageCircle className="h-5 w-5" />
            </div>
            <div className="min-w-0 flex-1">
              <p className="truncate text-[15px] font-medium text-text-main">
                {item.displayName || item.conversationId}
              </p>
              <p className="truncate text-[13px] text-text-sub">
                {item.lastSummary || t("chat.inbox.no_messages", "No messages")}
              </p>
            </div>
            {item.unreadCount > 0 && (
              <span className="flex h-5 min-w-5 items-center justify-center rounded-full bg-primary-blue px-1 text-[11px] text-white">
                {item.unreadCount > 99 ? "99+" : item.unreadCount}
              </span>
            )}
          </button>
        ))}
        {!isLoading && !error && nextCursor && items.length < MAX_LIST_PAGE_SIZE && (
          <button
            type="button"
            disabled={isLoadingMore}
            className="flex h-12 w-full items-center justify-center text-[13px] font-medium text-primary-blue disabled:opacity-50"
            onClick={() => void load(nextCursor)}
          >
            {isLoadingMore
              ? t("common.loading", "Loading...")
              : t("common.load_more", "Load more")}
          </button>
        )}
      </div>
    </div>
  );
}

function assertCursorPage(page: ConversationInboxPage): void {
  if (page.pageInfo.mode !== "cursor") {
    throw new Error("IM inbox must use cursor pagination");
  }
  if (page.pageInfo.hasMore && !page.pageInfo.nextCursor) {
    throw new Error("IM inbox returned hasMore without nextCursor");
  }
}

function mergeInboxItems(
  previous: readonly ConversationInboxEntry[],
  incoming: readonly ConversationInboxEntry[],
): ConversationInboxEntry[] {
  const merged: ConversationInboxEntry[] = [];
  const ids = new Set<string>();
  for (const item of [...previous, ...incoming]) {
    if (merged.length >= MAX_LIST_PAGE_SIZE || ids.has(item.conversationId)) {
      continue;
    }
    ids.add(item.conversationId);
    merged.push(item);
  }
  return merged;
}

export default ChatInboxPage;
