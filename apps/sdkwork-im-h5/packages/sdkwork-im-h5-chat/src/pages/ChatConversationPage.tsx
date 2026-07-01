import { useCallback, useEffect, useRef, useState } from "react";

import type { TimelineViewEntry } from "@sdkwork/im-sdk";

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
  const timelineRef = useRef<HTMLUListElement>(null);
  const loadingOlderRef = useRef(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

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
        const message = cause instanceof Error ? cause.message : "Failed to load messages";
        setError(message);
      })
      .finally(() => {
        if (!options?.silent) {
          setLoading(false);
        }
      });
  }, [applyTimelineResponse, conversationId]);

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
        limit: 50,
      });
      applyTimelineResponse(response.items ?? [], pickTimelinePagination(response), "append");
      requestAnimationFrame(() => {
        if (listElement) {
          listElement.scrollTop = listElement.scrollHeight - previousHeight;
        }
      });
    } catch (cause: unknown) {
      const message = cause instanceof Error ? cause.message : "Failed to load earlier messages";
      setError(message);
    } finally {
      loadingOlderRef.current = false;
      setLoadingOlder(false);
    }
  }, [applyTimelineResponse, conversationId, pagination.hasMore, pagination.nextAfterSeq]);

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
      const message = cause instanceof Error ? cause.message : "Failed to send message";
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
      const message = cause instanceof Error ? cause.message : "Failed to upload image";
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

  const heading = title ?? `Conversation ${conversationId}`;

  return (
    <section className="im-h5-chat-conversation" aria-label="Chat conversation">
      <header className="im-h5-chat-conversation-header">
        <a className="im-h5-chat-back-link" href="#/chat/inbox">
          ← Inbox
        </a>
        <div className="im-h5-chat-conversation-heading">
          <h1 className="im-h5-chat-title">{heading}</h1>
          {liveConnected ? (
            <span className="im-h5-chat-live-badge" aria-label="Live updates connected">
              Live
            </span>
          ) : null}
        </div>
      </header>

      {loading ? <p className="im-h5-chat-status">Loading messages…</p> : null}
      {loadingOlder ? (
        <p className="im-h5-chat-status" role="status">
          Loading earlier messages…
        </p>
      ) : null}
      {error ? (
        <div className="im-h5-chat-error" role="alert">
          <p>{error}</p>
        </div>
      ) : null}

      {!loading && !error ? (
        <ul
          ref={timelineRef}
          className="im-h5-chat-timeline"
          onScroll={handleTimelineScroll}
        >
          {entries.length === 0 ? (
            <li className="im-h5-chat-status">No messages yet.</li>
          ) : (
            entries.map((entry) => (
              <li key={entry.messageId} className="im-h5-chat-timeline-item">
                <div className="im-h5-chat-timeline-meta">
                  <strong>{entry.sender?.displayName ?? entry.sender?.id ?? "Unknown"}</strong>
                  <time>{entry.occurredAt}</time>
                </div>
                <p>{entry.body?.text ?? entry.summary ?? ""}</p>
              </li>
            ))
          )}
        </ul>
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
          aria-label="Upload image"
          onClick={() => fileInputRef.current?.click()}
        >
          {uploading ? "Uploading…" : "Image"}
        </button>
        <textarea
          className="im-h5-chat-composer-input"
          rows={2}
          value={draft}
          placeholder="Type a message"
          aria-label="Message text"
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
          {sending ? "Sending…" : "Send"}
        </button>
      </footer>
    </section>
  );
}
