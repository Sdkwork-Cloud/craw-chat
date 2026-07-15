import { useCallback, useEffect, useRef, useState } from "react";

import type { ConversationInboxEntry } from "@sdkwork/im-sdk";

import { formatRelativeTime, useI18n } from "@sdkwork/im-h5-commons";

import { fetchChatInboxPage, markConversationRead, readInboxPageState } from "../services/chatInboxService";
import { mergeInboxEntries, mergeLatestInboxEntries } from "../services/chatInboxUtils";
import {
  rememberConversationTitle,
  resolveConversationInboxEntryDisplayTitle,
} from "../services/chatConversationTitleStore";
import { resolveInboxRealtimeUserId, subscribeInboxLiveRefresh } from "../services/chatRealtimeService";

export function ChatInboxPage() {
  const { t } = useI18n();
  const [entries, setEntries] = useState<ConversationInboxEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [loadingMore, setLoadingMore] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [liveConnected, setLiveConnected] = useState(false);
  const [nextCursor, setNextCursor] = useState<string | undefined>(undefined);
  const [hasMore, setHasMore] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const loadInboxRef = useRef<(options?: { silent?: boolean }) => void>(() => undefined);
  const requestGenerationRef = useRef(0);
  const loadingMoreRef = useRef(false);
  const loadedAdditionalPagesRef = useRef(false);
  const pendingLiveRefreshRef = useRef(false);

  const loadInbox = useCallback((options?: { silent?: boolean }) => {
    const requestGeneration = ++requestGenerationRef.current;
    if (!options?.silent) {
      setLoading(true);
    }
    setError(null);

    fetchChatInboxPage({ q: searchQuery.trim() || undefined })
      .then((response) => {
        if (requestGeneration !== requestGenerationRef.current) {
          return;
        }
        const responseItems = response.items ?? [];
        const pageState = readInboxPageState(response);
        if (options?.silent && loadedAdditionalPagesRef.current && !searchQuery.trim()) {
          setEntries((previous) => mergeLatestInboxEntries(previous, responseItems));
        } else {
          loadedAdditionalPagesRef.current = false;
          setEntries(responseItems);
          setNextCursor(pageState.nextCursor);
          setHasMore(pageState.hasMore);
        }
      })
      .catch((cause: unknown) => {
        if (requestGeneration !== requestGenerationRef.current) {
          return;
        }
        const message = cause instanceof Error ? cause.message : t("chat.inbox.loadError");
        setError(message);
      })
      .finally(() => {
        if (requestGeneration === requestGenerationRef.current) {
          setLoading(false);
        }
      });
  }, [searchQuery, t]);

  const loadMoreInbox = useCallback(() => {
    if (!hasMore || !nextCursor || loadingMoreRef.current) {
      return;
    }

    const requestGeneration = ++requestGenerationRef.current;
    const requestCursor = nextCursor;
    loadingMoreRef.current = true;
    setLoadingMore(true);
    setError(null);

    fetchChatInboxPage({
      cursor: requestCursor,
      q: searchQuery.trim() || undefined,
    })
      .then((response) => {
        if (requestGeneration !== requestGenerationRef.current) {
          return;
        }
        setEntries((previous) => mergeInboxEntries(previous, response.items ?? []));
        loadedAdditionalPagesRef.current = true;
        const pageState = readInboxPageState(response);
        setNextCursor(pageState.nextCursor);
        setHasMore(pageState.hasMore);
      })
      .catch((cause: unknown) => {
        if (requestGeneration !== requestGenerationRef.current) {
          return;
        }
        const message = cause instanceof Error ? cause.message : t("chat.inbox.loadMoreError");
        setError(message);
      })
      .finally(() => {
        if (requestGeneration === requestGenerationRef.current) {
          loadingMoreRef.current = false;
          setLoadingMore(false);
          if (pendingLiveRefreshRef.current) {
            pendingLiveRefreshRef.current = false;
            loadInboxRef.current({ silent: true });
          }
        }
      });
  }, [hasMore, nextCursor, searchQuery, t]);

  loadInboxRef.current = loadInbox;

  useEffect(() => {
    const timer = window.setTimeout(() => {
      loadInbox();
    }, searchQuery.trim() ? 250 : 0);
    return () => window.clearTimeout(timer);
  }, [loadInbox]);

  useEffect(() => {
    let cancelled = false;
    let unsubscribe: (() => void) | undefined;
    const userId = resolveInboxRealtimeUserId();
    if (!userId) {
      setLiveConnected(false);
      return undefined;
    }

    void subscribeInboxLiveRefresh(() => {
      if (loadingMoreRef.current) {
        pendingLiveRefreshRef.current = true;
        return;
      }
      loadInboxRef.current({ silent: true });
    }, userId)
      .then((dispose) => {
        if (cancelled) {
          dispose();
          return;
        }
        unsubscribe = dispose;
        setLiveConnected(true);
      })
      .catch(() => {
        if (!cancelled) {
          setLiveConnected(false);
        }
      });

    return () => {
      cancelled = true;
      requestGenerationRef.current += 1;
      unsubscribe?.();
    };
  }, []);

  if (loading) {
    return <p className="im-h5-chat-status">{t("chat.inbox.loading")}</p>;
  }

  if (error && entries.length === 0) {
    return (
      <div className="im-h5-chat-error" role="alert">
        <p>{error}</p>
        <button type="button" className="im-h5-chat-retry" onClick={() => loadInbox()}>
          {t("chat.inbox.retry")}
        </button>
      </div>
    );
  }

  if (entries.length === 0) {
    return (
      <div className="im-h5-chat-inbox-empty">
        <label className="im-h5-chat-search-field">
          <span className="im-h5-visually-hidden">{t("chat.inbox.searchAria")}</span>
          <input
            type="search"
            value={searchQuery}
            maxLength={256}
            autoComplete="off"
            placeholder={t("chat.inbox.searchPlaceholder")}
            aria-label={t("chat.inbox.searchAria")}
            onChange={(event) => setSearchQuery(event.target.value)}
          />
        </label>
        <p className="im-h5-chat-status">
          {searchQuery.trim() ? t("chat.inbox.searchEmpty") : t("chat.inbox.empty")}
        </p>
      </div>
    );
  }

  return (
    <section className="im-h5-chat-inbox" aria-label={t("chat.inbox.aria")}>
      <div className="im-h5-chat-conversation-heading">
        <h1 className="im-h5-chat-title">{t("chat.inbox.title")}</h1>
        {liveConnected ? (
          <span className="im-h5-chat-live-badge" aria-label={t("chat.inbox.liveAria")}>
            {t("chat.inbox.live")}
          </span>
        ) : null}
      </div>
      <label className="im-h5-chat-search-field">
        <span className="im-h5-visually-hidden">{t("chat.inbox.searchAria")}</span>
        <input
          type="search"
          value={searchQuery}
          maxLength={256}
          autoComplete="off"
          placeholder={t("chat.inbox.searchPlaceholder")}
          aria-label={t("chat.inbox.searchAria")}
          onChange={(event) => setSearchQuery(event.target.value)}
        />
      </label>
      {error ? (
        <div className="im-h5-chat-error" role="alert">
          <p>{error}</p>
        </div>
      ) : null}
      <ul className="im-h5-chat-list">
        {entries.map((entry) => {
          const conversationId = entry.conversationId;
          const displayTitle = resolveConversationInboxEntryDisplayTitle(entry);
          const title = displayTitle ?? t("chat.inbox.conversationFallback", { id: String(conversationId) });
          const preview = entry.lastSummary ?? "";
          const updatedAt = entry.lastMessageAt ?? entry.lastActivityAt;
          const unreadCount = entry.unreadCount ?? 0;
          const isMarkedUnread = entry.preferences?.isMarkedUnread === true;
          const isUnread = unreadCount > 0 || isMarkedUnread;
          const isMuted = entry.preferences?.isMuted === true;

          return (
            <li key={String(conversationId ?? title)} className="im-h5-chat-item">
              <a
                className="im-h5-chat-item-link"
                href={`#/chat/conversations/${encodeURIComponent(String(conversationId))}`}
                onClick={() => {
                  rememberConversationTitle(String(conversationId), displayTitle);
                  void markConversationRead(String(conversationId), {
                    readSeq: entry.lastMessageSeq ?? 0,
                  }).catch(() => undefined);
                }}
              >
                <div className="im-h5-chat-item-main">
                  <strong>
                    {title}
                    {isMuted ? <span className="im-h5-chat-muted-badge" aria-hidden="true">🔕</span> : null}
                  </strong>
                  {preview ? <p>{preview}</p> : null}
                </div>
                <div className="im-h5-chat-item-meta">
                  <time className="im-h5-chat-item-time">{formatRelativeTime(updatedAt)}</time>
                  {isUnread ? (
                    <span className="im-h5-chat-unread-badge" aria-label={t("chat.inbox.unreadCount", { count: unreadCount || 1 })}>
                      {isMuted ? "" : (unreadCount > 99 ? "99+" : unreadCount || "")}
                    </span>
                  ) : null}
                </div>
              </a>
            </li>
          );
        })}
      </ul>
      {hasMore ? (
        <button
          type="button"
          className="im-h5-chat-load-more"
          disabled={loadingMore}
          onClick={loadMoreInbox}
        >
          {loadingMore ? t("chat.inbox.loadingMore") : t("chat.inbox.loadMore")}
        </button>
      ) : null}
    </section>
  );
}
