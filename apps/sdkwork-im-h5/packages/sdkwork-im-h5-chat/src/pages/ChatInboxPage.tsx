import { useCallback, useEffect, useRef, useState } from "react";

import type { ConversationInboxEntry } from "@sdkwork/im-sdk";



import { formatRelativeTime } from "@sdkwork/im-h5-commons";



import { fetchChatInbox } from "../services/chatInboxService";

import { subscribeInboxLiveRefresh } from "../services/chatRealtimeService";



export function ChatInboxPage() {

  const [entries, setEntries] = useState<ConversationInboxEntry[]>([]);

  const [loading, setLoading] = useState(true);

  const [error, setError] = useState<string | null>(null);

  const [liveConnected, setLiveConnected] = useState(false);

  const loadInboxRef = useRef<(options?: { silent?: boolean }) => void>(() => undefined);



  const loadInbox = useCallback((options?: { silent?: boolean }) => {

    if (!options?.silent) {

      setLoading(true);

    }

    setError(null);



    fetchChatInbox()

      .then((response) => {

        setEntries(response.items ?? []);

      })

      .catch((cause: unknown) => {

        const message = cause instanceof Error ? cause.message : "Failed to load inbox";

        setError(message);

      })

      .finally(() => {

        if (!options?.silent) {

          setLoading(false);

        }

      });

  }, []);



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

    return <p className="im-h5-chat-status">Loading inbox…</p>;

  }



  if (error) {

    return (

      <div className="im-h5-chat-error" role="alert">

        <p>{error}</p>

      </div>

    );

  }



  if (entries.length === 0) {

    return <p className="im-h5-chat-status">No conversations yet.</p>;

  }



  return (

    <section className="im-h5-chat-inbox" aria-label="Chat inbox">

      <div className="im-h5-chat-conversation-heading">

        <h1 className="im-h5-chat-title">Inbox</h1>

        {liveConnected ? (

          <span className="im-h5-chat-live-badge" aria-label="Live updates connected">

            Live

          </span>

        ) : null}

      </div>

      <ul className="im-h5-chat-list">

        {entries.map((entry) => {

          const conversationId = entry.conversationId;

          const title =

            entry.displayName

            ?? entry.peer?.displayName

            ?? `Conversation ${conversationId}`;

          const preview = entry.lastSummary ?? "";

          const updatedAt = entry.lastMessageAt ?? entry.lastActivityAt;



          return (

            <li key={String(conversationId ?? title)} className="im-h5-chat-item">

              <a

                className="im-h5-chat-item-link"

                href={`#/chat/conversations/${encodeURIComponent(String(conversationId))}`}

              >

                <div className="im-h5-chat-item-main">

                  <strong>{title}</strong>

                  {preview ? <p>{preview}</p> : null}

                </div>

                <time className="im-h5-chat-item-time">{formatRelativeTime(updatedAt)}</time>

              </a>

            </li>

          );

        })}

      </ul>

    </section>

  );

}

