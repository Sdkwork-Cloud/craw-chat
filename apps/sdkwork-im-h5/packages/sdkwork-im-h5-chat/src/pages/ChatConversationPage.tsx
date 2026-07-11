import { useCallback, useEffect, useRef, useState } from "react";
import { useVirtualizer } from "@tanstack/react-virtual";

import type { ConversationMessageEntry } from "@sdkwork/im-sdk";

import { useI18n } from "@sdkwork/im-h5-commons";

import { markConversationRead } from "../services/chatInboxService";
import {
  fetchConversationProfile,
  fetchConversationMessageDelta,
  fetchConversationMessages,
  sendConversationImage,
  sendConversationText,
} from "../services/chatConversationService";
import {
  enqueuePendingTextSend,
  isRetryableH5SendError,
  releasePendingTextSendClaim,
  removePendingTextSend,
  runPendingTextSendFlush,
} from "../services/offlineSendQueue";
import { subscribeConversationLiveMessages } from "../services/chatRealtimeService";
import {
  mergeConversationMessageEntries,
  pickMessageHistoryPagination,
  resolveLatestMessageSeq,
  type MessageHistoryPaginationState,
} from "../services/chatMessageHistoryUtils";
import {
  readRememberedConversationTitle,
  rememberConversationTitle,
  resolveConversationProfileDisplayTitle,
} from "../services/chatConversationTitleStore";

interface ChatConversationPageProps {
  conversationId: string;
  title?: string;
}

export function ChatConversationPage({ conversationId, title }: ChatConversationPageProps) {
  const { t } = useI18n();
  const [entries, setEntries] = useState<ConversationMessageEntry[]>([]);
  const [pagination, setPagination] = useState<MessageHistoryPaginationState>({
    hasMore: false,
  });
  const [loading, setLoading] = useState(true);
  const [loadingOlder, setLoadingOlder] = useState(false);
  const [uploading, setUploading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [draft, setDraft] = useState("");
  const [sending, setSending] = useState(false);
  const [liveConnected, setLiveConnected] = useState(false);
  const [resolvedTitle, setResolvedTitle] = useState<string | undefined>(() => (
    title ?? readRememberedConversationTitle(conversationId)
  ));
  const latestSeqRef = useRef(0);
  const messageHistoryRef = useRef<HTMLDivElement>(null);
  const loadingOlderRef = useRef(false);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const conversationIdRef = useRef(conversationId);
  conversationIdRef.current = conversationId;
  const historyRequestGenerationRef = useRef(0);
  const paginationRef = useRef(pagination);
  paginationRef.current = pagination;
  const latestMessageRefreshRef = useRef<Promise<void> | null>(null);
  const latestMessageRefreshPendingRef = useRef(false);

  const rowVirtualizer = useVirtualizer({
    count: entries.length,
    getScrollElement: () => messageHistoryRef.current,
    estimateSize: () => 76,
    overscan: 10,
  });

  const applyConversationMessagePage = useCallback((items: ConversationMessageEntry[], responsePagination: MessageHistoryPaginationState, mode: "replace" | "append" | "merge") => {
    setEntries((previous) => {
      const next = mode === "replace"
        ? items
        : mode === "append"
          ? mergeConversationMessageEntries(previous, items)
          : mergeConversationMessageEntries(previous, items);
      latestSeqRef.current = resolveLatestMessageSeq(next);
      return next;
    });
    if (mode !== "merge") {
      setPagination(responsePagination);
    }
  }, []);

  const loadMessageHistory = useCallback((options?: { silent?: boolean }) => {
    const requestGeneration = ++historyRequestGenerationRef.current;
    const requestConversationId = conversationId;
    if (!options?.silent) {
      setLoading(true);
    }
    setError(null);

    fetchConversationMessages(conversationId)
      .then((response) => {
        if (
          requestGeneration !== historyRequestGenerationRef.current
          || conversationIdRef.current !== requestConversationId
        ) {
          return;
        }
        applyConversationMessagePage(
          response.items ?? [],
          pickMessageHistoryPagination(response),
          "replace",
        );
      })
      .catch((cause: unknown) => {
        if (
          requestGeneration !== historyRequestGenerationRef.current
          || conversationIdRef.current !== requestConversationId
        ) {
          return;
        }
        const message = cause instanceof Error ? cause.message : t("chat.conversation.loadError");
        setError(message);
      })
      .finally(() => {
        if (!options?.silent && requestGeneration === historyRequestGenerationRef.current) {
          setLoading(false);
        }
      });
  }, [applyConversationMessagePage, conversationId, t]);

  const appendNewMessageEntries = useCallback(async () => {
    if (latestMessageRefreshRef.current) {
      latestMessageRefreshPendingRef.current = true;
      return latestMessageRefreshRef.current;
    }
    const requestGeneration = historyRequestGenerationRef.current;
    const requestConversationId = conversationId;
    const refreshPromise = (async () => {
      do {
        latestMessageRefreshPendingRef.current = false;
        try {
          const response = await fetchConversationMessageDelta(conversationId);
          if (
            requestGeneration !== historyRequestGenerationRef.current
            || conversationIdRef.current !== requestConversationId
          ) {
            return;
          }
          if ((response.items ?? []).length > 0) {
            applyConversationMessagePage(response.items, pickMessageHistoryPagination(response), "merge");
          }
        } catch {
          // Keep existing messages visible when incremental sync fails.
        }
      } while (latestMessageRefreshPendingRef.current);
    })().finally(() => {
      if (latestMessageRefreshRef.current === refreshPromise) {
        latestMessageRefreshRef.current = null;
      }
    });
    latestMessageRefreshRef.current = refreshPromise;
    return refreshPromise;
  }, [applyConversationMessagePage, conversationId]);

  const loadOlderMessages = useCallback(async () => {
    const currentPagination = paginationRef.current;
    if (loadingOlderRef.current || !currentPagination.hasMore || !currentPagination.nextCursor) {
      return;
    }
    const requestGeneration = historyRequestGenerationRef.current;
    const requestConversationId = conversationId;
    loadingOlderRef.current = true;
    setLoadingOlder(true);
    const listElement = messageHistoryRef.current;
    const previousHeight = listElement?.scrollHeight ?? 0;

    try {
      const response = await fetchConversationMessages(conversationId, {
        cursor: currentPagination.nextCursor,
        pageSize: 50,
      });
      if (
        requestGeneration !== historyRequestGenerationRef.current
        || conversationIdRef.current !== requestConversationId
      ) {
        return;
      }
      applyConversationMessagePage(response.items ?? [], pickMessageHistoryPagination(response), "append");
      requestAnimationFrame(() => {
        if (listElement) {
          listElement.scrollTop = listElement.scrollHeight - previousHeight;
        }
      });
    } catch (cause: unknown) {
      if (
        requestGeneration !== historyRequestGenerationRef.current
        || conversationIdRef.current !== requestConversationId
      ) {
        return;
      }
      const message = cause instanceof Error ? cause.message : t("chat.conversation.loadEarlierError");
      setError(message);
    } finally {
      if (
        requestGeneration === historyRequestGenerationRef.current
        && conversationIdRef.current === requestConversationId
      ) {
        loadingOlderRef.current = false;
        setLoadingOlder(false);
      }
    }
  }, [applyConversationMessagePage, conversationId, t]);

  useEffect(() => {
    historyRequestGenerationRef.current += 1;
    loadingOlderRef.current = false;
    latestMessageRefreshRef.current = null;
    latestMessageRefreshPendingRef.current = false;
    setEntries([]);
    setPagination({ hasMore: false });
    latestSeqRef.current = 0;
    setError(null);
  }, [conversationId]);

  useEffect(() => {
    loadMessageHistory();
  }, [loadMessageHistory]);

  useEffect(() => {
    let cancelled = false;
    if (title) {
      rememberConversationTitle(conversationId, title);
      setResolvedTitle(title);
      return () => {
        cancelled = true;
      };
    }

    setResolvedTitle(readRememberedConversationTitle(conversationId));
    fetchConversationProfile(conversationId)
      .then((profile) => {
        if (cancelled) {
          return;
        }
        const profileTitle = resolveConversationProfileDisplayTitle(profile);
        rememberConversationTitle(conversationId, profileTitle);
        setResolvedTitle(profileTitle ?? readRememberedConversationTitle(conversationId));
      })
      .catch(() => undefined);

    return () => {
      cancelled = true;
    };
  }, [conversationId, title]);

  useEffect(() => {
    void markConversationRead(conversationId, { readSeq: latestSeqRef.current }).catch(() => undefined);
  }, [conversationId, entries.length]);

  const flushPendingSends = useCallback(async () => {
    await runPendingTextSendFlush(async (pending) => {
      const scoped = pending.filter((payload) => payload.conversationId === conversationId);
      const scopedIds = new Set(scoped.map((payload) => payload.clientMsgId));
      for (const payload of pending) {
        if (!scopedIds.has(payload.clientMsgId)) {
          await releasePendingTextSendClaim(payload.clientMsgId, payload.claimId);
        }
      }
      for (const [index, payload] of scoped.entries()) {
        try {
          await sendConversationText(conversationId, payload.text, {
            clientMsgId: payload.clientMsgId,
          });
          await removePendingTextSend(payload.clientMsgId);
        } catch {
          for (const remaining of scoped.slice(index)) {
            await releasePendingTextSendClaim(remaining.clientMsgId, remaining.claimId);
          }
          break;
        }
      }
      if (scoped.length > 0) {
        await appendNewMessageEntries();
      }
    });
  }, [appendNewMessageEntries, conversationId]);

  useEffect(() => {
    void flushPendingSends();
  }, [flushPendingSends]);

  useEffect(() => {
    const handleOnline = () => {
      void flushPendingSends();
    };
    window.addEventListener("online", handleOnline);
    return () => {
      window.removeEventListener("online", handleOnline);
    };
  }, [flushPendingSends]);

  useEffect(() => {
    let cancelled = false;
    let unsubscribe: (() => void) | undefined;

    void subscribeConversationLiveMessages(conversationId, () => {
      void appendNewMessageEntries();
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
  }, [appendNewMessageEntries, conversationId]);

  const handleSend = async () => {
    const text = draft.trim();
    if (!text || sending) {
      return;
    }
    const clientMsgId = `h5-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
    setSending(true);
    try {
      await sendConversationText(conversationId, text, { clientMsgId });
      await removePendingTextSend(clientMsgId);
      setDraft("");
      await appendNewMessageEntries();
    } catch (cause: unknown) {
      if (isRetryableH5SendError(cause)) {
        await enqueuePendingTextSend({ conversationId, text, clientMsgId });
      }
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
      await appendNewMessageEntries();
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

  const handleMessageHistoryScroll = () => {
    const element = messageHistoryRef.current;
    if (!element || element.scrollTop > 80) {
      return;
    }
    void loadOlderMessages();
  };

  const heading = resolvedTitle ?? t("chat.conversation.fallbackTitle", { id: conversationId });

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
          {entries.length === 0 ? (
            <button type="button" className="im-h5-chat-retry" onClick={() => loadMessageHistory()}>
              {t("chat.conversation.retry")}
            </button>
          ) : null}
        </div>
      ) : null}

      {!loading ? (
        <div
          ref={messageHistoryRef}
          className="im-h5-chat-message-history"
          onScroll={handleMessageHistoryScroll}
        >
          {entries.length === 0 ? (
            <p className="im-h5-chat-status">{t("chat.conversation.empty")}</p>
          ) : (
            <ul
              className="im-h5-chat-message-history-list"
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
                    data-index={virtualRow.index}
                    ref={rowVirtualizer.measureElement}
                    className="im-h5-chat-message-history-item"
                    style={{
                      position: "absolute",
                      top: 0,
                      left: 0,
                      width: "100%",
                      transform: `translateY(${virtualRow.start}px)`,
                    }}
                  >
                    <div className="im-h5-chat-message-history-meta">
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
