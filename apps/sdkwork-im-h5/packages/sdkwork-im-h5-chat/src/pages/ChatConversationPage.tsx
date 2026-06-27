import { useCallback, useEffect, useRef, useState } from "react";

import type { TimelineViewEntry } from "@sdkwork/im-sdk";



import {

  fetchConversationTimeline,

  sendConversationText,

} from "../services/chatConversationService";

import { subscribeConversationLiveMessages } from "../services/chatRealtimeService";



interface ChatConversationPageProps {

  conversationId: string;

  title?: string;

}



export function ChatConversationPage({ conversationId, title }: ChatConversationPageProps) {

  const [entries, setEntries] = useState<TimelineViewEntry[]>([]);

  const [loading, setLoading] = useState(true);

  const [error, setError] = useState<string | null>(null);

  const [draft, setDraft] = useState("");

  const [sending, setSending] = useState(false);

  const [liveConnected, setLiveConnected] = useState(false);

  const loadTimelineRef = useRef<(options?: { silent?: boolean }) => void>(() => undefined);



  const loadTimeline = useCallback((options?: { silent?: boolean }) => {

    if (!options?.silent) {

      setLoading(true);

    }

    setError(null);

    fetchConversationTimeline(conversationId)

      .then((response) => {

        setEntries(response.items ?? []);

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

  }, [conversationId]);



  loadTimelineRef.current = loadTimeline;



  useEffect(() => {

    loadTimeline();

  }, [loadTimeline]);



  useEffect(() => {

    let cancelled = false;

    let unsubscribe: (() => void) | undefined;



    void subscribeConversationLiveMessages(conversationId, () => {

      loadTimelineRef.current({ silent: true });

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

  }, [conversationId]);



  const handleSend = async () => {

    const text = draft.trim();

    if (!text || sending) {

      return;

    }

    setSending(true);

    try {

      await sendConversationText(conversationId, text);

      setDraft("");

      loadTimeline({ silent: true });

    } catch (cause: unknown) {

      const message = cause instanceof Error ? cause.message : "Failed to send message";

      setError(message);

    } finally {

      setSending(false);

    }

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

      {error ? (

        <div className="im-h5-chat-error" role="alert">

          <p>{error}</p>

        </div>

      ) : null}



      {!loading && !error ? (

        <ul className="im-h5-chat-timeline">

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

        <textarea

          className="im-h5-chat-composer-input"

          rows={2}

          value={draft}

          placeholder="Type a message"

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

