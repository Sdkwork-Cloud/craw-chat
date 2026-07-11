import { useCallback, useEffect, useRef, useState } from "react";

import type { ConversationInboxEntry } from "@sdkwork/im-sdk";

import { formatRelativeTime, useI18n } from "@sdkwork/im-h5-commons";

import { fetchChatInboxPage, markConversationRead, readInboxPageState } from "../services/chatInboxService";
import { mergeInboxEntries } from "../services/chatInboxUtils";
import {
  rememberConversationTitle,
  resolveConversationInboxEntryDisplayTitle,
} from "../services/chatConversationTitleStore";
import { subscribeInboxLiveRefresh } from "../services/chatRealtimeService";

export function ChatInboxPage() {
  const { t } = useI18n();
  const [entries, setEntries] = useState<ConversationInboxEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [loadingMore, setLoadingMore] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [liveConnected, setLiveConnected] = useState(false);
  const [nextCursor, setNextCursor] = useState<string | undefined>(undefined);
  const [hasMore, setHasMore] = useState(false);
  const loadInboxRef = useRef<(options?: { silent?: boolean }) => void>(() => undefined);

  const loadInbox = useCallback((options?: { silent?: boolean }) => {
    if (!options?.silent) {
      setLoading(true);
    }
    setError(null);

    fetchChatInboxPage()
      .then((response) => {
        setEntries(response.items ?? []);
        const pageState = readInboxPageState(response);
        setNextCursor(pageState.nextCursor);
        setHasMore(pageState.hasMore);
      })
      .catch((cause: unknown) => {
        const message = cause instanceof Error ? cause.message : t("chat.inbox.loadError");
        setError(message);
      })
      .finally(() => {
        if (!options?.silent) {
          setLoading(false);
        }
      });
  }, [t]);

  const loadMoreInbox = useCallback(() => {
    if (!hasMore || !nextCursor || loadingMore) {
      return;
    }

    setLoadingMore(true);
    setError(null);

    fetchChatInboxPage({ cursor: nextCursor })
      .then((response) => {
        setEntries((previous) => mergeInboxEntries(previous, response.items ?? []));
        const pageState = readInboxPageState(response);
        setNextCursor(pageState.nextCursor);
        setHasMore(pageState.hasMore);
      })
      .catch((cause: unknown) => {
        const message = cause instanceof Error ? cause.message : t("chat.inbox.loadMoreError");
        setError(message);
      })
      .finally(() => {
        setLoadingMore(false);
      });
  }, [hasMore, loadingMore, nextCursor, t]);

  loadInboxRef.current = loadInbox;

  useEffect(() => {
    loadInbox();
  }, [loadInbox]);

  useEffect(() => {
    let cancelled = false;
    let unsubscribe: (() => void) | undefined;

    void subscribeInboxLiveRefresh(() => {
      loadInboxRef.current({ silent: true });
    })
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
      unsubscribe?.();
      setLiveConnected(false);
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
    return <p className="im-h5-chat-status">{t("chat.inbox.empty")}</p>;
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
