import { useCallback, useEffect, useRef, useState } from "react";
import { useVirtualizer } from "@tanstack/react-virtual";

import type { TimelineViewEntry } from "@sdkwork/im-sdk";

import { useI18n } from "@sdkwork/im-h5-commons";

import {
  fetchConversationTimeline,
  fetchConversationTimelineDelta,
  sendConversationImage,
  sendConversationText,
} from "../services/chatConversationService";
import { subscribeConversationLiveMessages } from "../services/chatRealtimeService";
import {
  mergeTimelineEntries,
  pickTimelinePagination,
  resolveLatestMessageSeq,
  type TimelinePaginationState,
} from "../services/chatTimelineUtils";

interface ChatConversationPageProps {
  conversationId: string;
  title?: string;
}

export function ChatConversationPage({ conversationId, title }: ChatConversationPageProps) {
  const { t } = useI18n();
  const [entries, setEntries] = useState<TimelineViewEntry[]>([]);
  const [pagination, setPagination] = useState<TimelinePaginationState>({
    hasMore: false,
    nextAfterSeq: 0,
  });
  const [loading, setLoading] = useState(true);
  const [loadingOlder, setLoadingOlder] = useState(false);
  const [uploading, setUploading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [draft, setDraft] = useState("");
  const [sending, setSending] = useState(false);
  const [liveConnected, setLiveConnected] = useState(false);
  const latestSeqRef = useRef(0);
  const timelineRef = useRef<HTMLDivElement>(null);
  const loadingOlderRef = useRef(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const rowVirtualizer = useVirtualizer({
    count: entries.length,
    getScrollElement: () => timelineRef.current,
    estimateSize: () => 76,
    overscan: 10,
  });

  const applyTimelineResponse = useCallback((items: TimelineViewEntry[], responsePagination: TimelinePaginationState, mode: "replace" | "append" | "merge") => {
    setEntries((previous) => {
      const next = mode === "replace"
        ? items
        : mode === "append"
          ? mergeTimelineEntries(previous, items)
          : mergeTimelineEntries(previous, items);
      latestSeqRef.current = resolveLatestMessageSeq(next);
      return next;
    });
    setPagination(responsePagination);
  }, []);

  const loadTimeline = useCallback((options?: { silent?: boolean }) => {
    if (!options?.silent) {
      setLoading(true);
    }
    setError(null);

    fetchConversationTimeline(conversationId)
      .then((response) => {
        applyTimelineResponse(
          response.items ?? [],
          pickTimelinePagination(response),
          "replace",
        );
      })
      .catch((cause: unknown) => {
        const message = cause instanceof Error ? cause.message : t("chat.conversation.loadError");
        setError(message);
      })
      .finally(() => {
        if (!options?.silent) {
          setLoading(false);
        }
      });
  }, [applyTimelineResponse, conversationId, t]);

  const appendNewTimelineEntries = useCallback(async () => {
    const afterSeq = latestSeqRef.current;
    if (afterSeq <= 0) {
      return;
    }
    try {
      const response = await fetchConversationTimelineDelta(conversationId, afterSeq);
      if ((response.items ?? []).length === 0) {
        return;
      }
      applyTimelineResponse(response.items ?? [], pickTimelinePagination(response), "merge");
    } catch {
      // Keep existing timeline visible when incremental sync fails.
    }
  }, [applyTimelineResponse, conversationId]);

  const loadOlderMessages = useCallback(async () => {
    if (loadingOlderRef.current || !pagination.hasMore) {
      return;
    }
    loadingOlderRef.current = true;
    setLoadingOlder(true);
    const listElement = timelineRef.current;
    const previousHeight = listElement?.scrollHeight ?? 0;

    try {
      const response = await fetchConversationTimeline(conversationId, {
        afterSeq: pagination.nextAfterSeq,
        pageSize: 50,
      });
      applyTimelineResponse(response.items ?? [], pickTimelinePagination(response), "append");
      requestAnimationFrame(() => {
        if (listElement) {
          listElement.scrollTop = listElement.scrollHeight - previousHeight;
        }
      });
    } catch (cause: unknown) {
      const message = cause instanceof Error ? cause.message : t("chat.conversation.loadEarlierError");
      setError(message);
    } finally {
      loadingOlderRef.current = false;
      setLoadingOlder(false);
    }
  }, [applyTimelineResponse, conversationId, pagination.hasMore, pagination.nextAfterSeq, t]);

  useEffect(() => {
    loadTimeline();
  }, [loadTimeline]);

  useEffect(() => {
    let cancelled = false;
    let unsubscribe: (() => void) | undefined;

    void subscribeConversationLiveMessages(conversationId, () => {
      void appendNewTimelineEntries();
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
  }, [appendNewTimelineEntries, conversationId]);

  const handleSend = async () => {
    const text = draft.trim();
    if (!text || sending) {
      return;
    }
    setSending(true);
    try {
      await sendConversationText(conversationId, text);
      setDraft("");
      await appendNewTimelineEntries();
    } catch (cause: unknown) {
      const message = cause instanceof Error ? cause.message : t("chat.conversation.sendError");
      setError(message);
    } finally {
      setSending(false);
    }
  };

  const handleImageSelected = async (file: File | undefined) => {
    if (!file || uploading) {
      return;
    }
    setUploading(true);
    setError(null);
    try {
      await sendConversationImage(conversationId, file);
      await appendNewTimelineEntries();
    } catch (cause: unknown) {
      const message = cause instanceof Error ? cause.message : t("chat.conversation.uploadError");
      setError(message);
    } finally {
      setUploading(false);
      if (fileInputRef.current) {
        fileInputRef.current.value = "";
      }
    }
  };

  const handleTimelineScroll = () => {
    const element = timelineRef.current;
    if (!element || element.scrollTop > 80) {
      return;
    }
    void loadOlderMessages();
  };

  const heading = title ?? t("chat.conversation.fallbackTitle", { id: conversationId });

  return (
    <section className="im-h5-chat-conversation" aria-label={t("chat.conversation.aria")}>
      <header className="im-h5-chat-conversation-header">
        <a className="im-h5-chat-back-link" href="#/chat/inbox">
          {t("chat.conversation.back")}
        </a>
        <div className="im-h5-chat-conversation-heading">
          <h1 className="im-h5-chat-title">{heading}</h1>
          {liveConnected ? (
            <span className="im-h5-chat-live-badge" aria-label={t("chat.conversation.liveAria")}>
              {t("chat.conversation.live")}
            </span>
          ) : null}
        </div>
      </header>

      {loading ? <p className="im-h5-chat-status">{t("chat.conversation.loading")}</p> : null}
      {loadingOlder ? (
        <p className="im-h5-chat-status" role="status">
          {t("chat.conversation.loadingOlder")}
        </p>
      ) : null}
      {error ? (
        <div className="im-h5-chat-error" role="alert">
          <p>{error}</p>
        </div>
      ) : null}

      {!loading && !error ? (
        <div
          ref={timelineRef}
          className="im-h5-chat-timeline"
          onScroll={handleTimelineScroll}
        >
          {entries.length === 0 ? (
            <p className="im-h5-chat-status">{t("chat.conversation.empty")}</p>
          ) : (
            <ul
              className="im-h5-chat-timeline-list"
              style={{
                height: `${rowVirtualizer.getTotalSize()}px`,
                position: "relative",
              }}
            >
              {rowVirtualizer.getVirtualItems().map((virtualRow) => {
                const entry = entries[virtualRow.index];
                return (
                  <li
                    key={entry.messageId}
                    className="im-h5-chat-timeline-item"
                    style={{
                      position: "absolute",
                      top: 0,
                      left: 0,
                      width: "100%",
                      transform: `translateY(${virtualRow.start}px)`,
                    }}
                  >
                    <div className="im-h5-chat-timeline-meta">
                      <strong>{entry.sender?.displayName ?? entry.sender?.id ?? t("chat.conversation.unknownSender")}</strong>
                      <time>{entry.occurredAt}</time>
                    </div>
                    <p>{entry.body?.text ?? entry.summary ?? ""}</p>
                  </li>
                );
              })}
            </ul>
          )}
        </div>
      ) : null}

      <footer className="im-h5-chat-composer">
        <input
          ref={fileInputRef}
          type="file"
          accept="image/*"
          hidden
          onChange={(event) => {
            void handleImageSelected(event.target.files?.[0]);
          }}
        />
        <button
          type="button"
          className="im-h5-chat-composer-send"
          disabled={uploading}
          aria-label={t("chat.conversation.uploadAria")}
          onClick={() => fileInputRef.current?.click()}
        >
          {uploading ? t("chat.conversation.uploading") : t("chat.conversation.image")}
        </button>
        <textarea
          className="im-h5-chat-composer-input"
          rows={2}
          value={draft}
          placeholder={t("chat.conversation.placeholder")}
          aria-label={t("chat.conversation.messageAria")}
          onChange={(event) => setDraft(event.target.value)}
          onKeyDown={(event) => {
            if (event.key === "Enter" && !event.shiftKey) {
              event.preventDefault();
              void handleSend();
            }
          }}
        />
        <button
          type="button"
          className="im-h5-chat-composer-send"
          disabled={sending || draft.trim().length === 0}
          onClick={() => void handleSend()}
        >
          {sending ? t("chat.conversation.sending") : t("chat.conversation.send")}
        </button>
      </footer>
    </section>
  );
}
