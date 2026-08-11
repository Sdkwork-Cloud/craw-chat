import { useCallback, useEffect, useRef, useState } from "react";
import { useEditor } from "@tiptap/react";
import Placeholder from "@tiptap/extension-placeholder";
import StarterKit from "@tiptap/starter-kit";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { uuid } from "@sdkwork/utils";

import { showPrompt, showToast } from "@sdkwork/im-h5-commons";
import { Pin } from "lucide-react";
import { useAppStore } from "@sdkwork/im-h5-core";
import type { ImDecodedMessage } from "@sdkwork/im-h5-core/sdk";
import type { Chat, Message } from "@sdkwork/im-h5-types";

import { ChatHeader } from "../components/Chat/ChatHeader";
import { ChatInputArea } from "../components/Chat/ChatInputArea";
import { MessageContextMenu } from "../components/Chat/MessageContextMenu";
import { MessageList } from "../components/Chat/MessageList";
import { VoiceRecordingOverlay } from "../components/Chat/VoiceRecordingOverlay";
import { FullscreenMediaOverlay } from "../components/Chat/FullscreenMediaOverlay";
import { ChatService } from "../services/ChatService";
import {
  subscribeConversationLiveMessages,
  subscribeInboxLiveRefresh,
} from "../services/chatRealtimeService";

// Resident message window: the newest N messages stay rendered while older
// history stays reachable through the server cursor; beyond this cap the
// oldest entries are trimmed so deep browsing cannot accumulate unbounded
// memory (the newest messages are never dropped).
const MAX_RENDERED_MESSAGES = 500;

export function ChatDetail() {
  const { conversationId, id } = useParams();
  const chatId = conversationId ?? id ?? "";
  const { t } = useTranslation();
  const sessionUser = useAppStore((state) => state.currentUser);
  const [chat, setChat] = useState<Chat | null>(null);
  const [messages, setMessages] = useState<Message[]>([]);
  const [nextCursor, setNextCursor] = useState<string>();
  const [loadingMore, setLoadingMore] = useState(false);
  const [sending, setSending] = useState(false);
  const [replyingTo, setReplyingTo] = useState<Message | null>(null);
  const [isVoiceMode, setIsVoiceMode] = useState(false);
  const [activePanel, setActivePanel] = useState<"none" | "emoji" | "action">("none");
  const [isRecording, setIsRecording] = useState(false);
  const [mediaSending, setMediaSending] = useState(false);
  const mediaRecorder = useRef<MediaRecorder | null>(null);
  const recordedChunks = useRef<Blob[]>([]);
  const cancelVoiceRef = useRef(false);
  const [recordingTime, setRecordingTime] = useState(0);
  const [emojis, setEmojis] = useState<string[]>([]);
  const [highlightedMsgId, setHighlightedMsgId] = useState<string | null>(null);
  const [pinnedMessageIds, setPinnedMessageIds] = useState<string[]>([]);
  const [fullscreenMedia, setFullscreenMedia] = useState<{ type: "image" | "video"; url: string } | null>(null);
  const [loadError, setLoadError] = useState(false);
  const [contextMenu, setContextMenu] = useState({ isOpen: false, x: 0, y: 0, messageId: null as string | null });
  const longPressTimer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  const recordingTimer = useRef<ReturnType<typeof setInterval> | undefined>(undefined);
  const mediaInputRef = useRef<HTMLInputElement | null>(null);
  const lastReadWatermarkRef = useRef(0);
  const chatIdRef = useRef(chatId);
  // Monotonic load generation: a fresh page load, a cursor (older history)
  // load, or a chat switch supersedes any in-flight request, so stale
  // responses cannot regress the message window or the pagination cursor.
  const loadSeqRef = useRef(0);
  const [mediaInputKind, setMediaInputKind] = useState<"image" | "video" | "file">("image");

  useEffect(() => {
    chatIdRef.current = chatId;
  }, [chatId]);

  const editor = useEditor({
    extensions: [StarterKit, Placeholder.configure({ placeholder: t("chat.detail.placeholder") })],
    content: "",
    editorProps: { attributes: { class: "min-h-8 max-h-28 overflow-y-auto rounded-lg bg-chat-other-bg px-3 py-2 text-[16px] text-text-main focus:outline-none" } },
  });
  const inputValue = editor?.getText() ?? "";

  const load = useCallback(async (cursor?: string) => {
    if (!chatId) return;
    const requestChatId = chatId;
    const requestSeq = ++loadSeqRef.current;
    if (cursor) setLoadingMore(true);
    try {
      const page = await ChatService.getMessagePage(requestChatId, cursor);
      // Ignore responses that belong to a previous conversation (fast chat
      // switching) or a superseded request (a newer load started later).
      if (chatIdRef.current !== requestChatId || requestSeq !== loadSeqRef.current) return;
      setLoadError(false);
      setMessages((previous) => mergeMessages(cursor ? previous : [], page.items));
      setNextCursor(page.hasMore ? page.nextCursor : undefined);
      if (!cursor && page.highWatermark > lastReadWatermarkRef.current) {
        lastReadWatermarkRef.current = page.highWatermark;
        await ChatService.markAsRead(requestChatId);
      }
    } catch (error) {
      console.error(error);
      if (chatIdRef.current !== requestChatId || requestSeq !== loadSeqRef.current) return;
      setLoadError(true);
      showToast(t("chat.detail.load_failed", "Unable to load messages"));
    } finally {
      // Only the latest request owns the loading spinner.
      if (requestSeq === loadSeqRef.current) setLoadingMore(false);
    }
  }, [chatId, t]);

  useEffect(() => {
    if (!chatId) return undefined;
    // Per-conversation read watermark: reset when switching chats so the new
    // conversation always commits its read cursor.
    lastReadWatermarkRef.current = 0;
    void Promise.all([ChatService.getChatById(chatId), ChatService.getEmojis()]).then(([value, emojiList]) => {
      if (chatIdRef.current !== chatId) return;
      if (value) {
        const participants = sessionUser
          ? value.participants.filter((participant) => participant.id !== sessionUser.id)
          : value.participants;
        setChat({ ...value, participants });
      }
      setEmojis(emojiList);
    }).catch((error) => {
      console.error(error);
      if (chatIdRef.current !== chatId) return;
      setChat(null);
      setEmojis([]);
    });
    void ChatService.listPinnedMessages(chatId).then(setPinnedMessageIds).catch((error) => {
      console.error(error);
      setPinnedMessageIds([]);
    });
    void load();
    // Live messages merge incrementally (dedupe by id) instead of re-pulling
    // the whole page per event. The inbox refresh subscription only fires on
    // recovery-driven reconnects (initial subscription opens never invoke the
    // handlers), so `load()` runs exactly when a catch-up reload is needed.
    const unsubscribeLiveMessages = subscribeConversationLiveMessages(chatId, (decoded) => {
      void handleLiveMessage(decoded);
    });
    const unsubscribeInboxRefresh = subscribeInboxLiveRefresh(() => {
      void load();
    });
    return () => {
      unsubscribeLiveMessages();
      unsubscribeInboxRefresh();
    };
  }, [chatId, load, sessionUser]);

  const handleLiveMessage = useCallback(async (decoded: ImDecodedMessage) => {
    const mapped = await ChatService.mapRealtimeMessage(decoded);
    if (!mapped) return;
    // The user may have switched conversations while the mapping resolved.
    if (chatIdRef.current !== mapped.chatId) return;
    setMessages((previous) => mergeMessages(previous, [mapped]));
  }, []);

  const handlePinMessage = (messageId: string) => {
    const isPinned = pinnedMessageIds.includes(messageId);
    void (isPinned
      ? ChatService.unpinMessage(chatId, messageId)
      : ChatService.pinMessage(chatId, messageId))
      .then(() => {
        setPinnedMessageIds((previous) => isPinned
          ? previous.filter((id) => id !== messageId)
          : [...previous, messageId]);
      })
      .catch((error) => {
        console.error(error);
        showToast(t("chat.detail.pin_failed", "Unable to pin message"));
      });
    setContextMenu((value) => ({ ...value, isOpen: false }));
  };

  useEffect(() => () => {
    if (recordingTimer.current) clearInterval(recordingTimer.current);
    if (longPressTimer.current) clearTimeout(longPressTimer.current);
    cancelVoiceRef.current = true;
    mediaRecorder.current?.stop();
  }, []);

  const send = async () => {
    const content = editor?.getText().trim() ?? "";
    if (!content || sending || !chatId) return;
    setSending(true);
    const localMessage: Message = {
      id: `local-${uuid()}`,
      chatId,
      senderId: sessionUser?.id ?? "",
      content,
      timestamp: Date.now(),
      type: "text",
      sendState: "sending",
    };
    setMessages((previous) => mergeMessages(previous, [localMessage]));
    try {
      const replyTo = replyingTo ? {
        messageId: replyingTo.id,
        senderDisplayName: replyingTo.senderId === sessionUser?.id
          ? t("chat.detail.me", "Me")
          : chat?.participants.find((participant) => participant.id === replyingTo.senderId)?.name ?? replyingTo.senderId,
        contentPreview: replyingTo.content.slice(0, 200),
      } : undefined;
      // The idempotency key is derived from the local message id so a retry of
      // the same local message reuses the same clientMsgId; the server then
      // deduplicates instead of posting a second message.
      const message = await ChatService.sendMessage(
        chatId,
        sessionUser?.id ?? "",
        content,
        "text",
        undefined,
        replyTo,
        idempotencyKeyForLocalMessage(localMessage.id),
      );
      // The realtime echo may already have merged the server message while the
      // placeholder was pending: merge by id instead of replacing blindly.
      setMessages((previous) => mergeMessages(replaceLocalMessage(previous, localMessage.id, message), []));
      editor?.commands.clearContent();
      setReplyingTo(null);
    } catch (error) {
      console.error(error);
      setMessages((previous) => markLocalMessageFailed(previous, localMessage.id));
      showToast(t("chat.detail.send_failed", "Unable to send message"));
    } finally {
      setSending(false);
    }
  };

  const startRecording = () => {
    setRecordingTime(0);
    setIsRecording(true);
    recordingTimer.current = setInterval(() => setRecordingTime((value) => value + 1), 1000);
  };
  const stopRecording = () => {
    if (recordingTimer.current) clearInterval(recordingTimer.current);
    recordingTimer.current = undefined;
    setIsRecording(false);
  };

  const sendMedia = async (file: File | Blob, kind: "image" | "video" | "file" | "voice") => {
    if (!chatId || mediaSending) return;
    setMediaSending(true);
    const localMessage: Message = {
      id: `local-${uuid()}`,
      chatId,
      senderId: sessionUser?.id ?? "",
      content: "",
      timestamp: Date.now(),
      type: kind,
      sendState: "sending",
      ...(file instanceof File ? { metadata: { fileName: file.name, mimeType: file.type } } : {}),
    };
    setMessages((previous) => mergeMessages(previous, [localMessage]));
    try {
      const message = await ChatService.sendMediaMessage(
        chatId,
        file,
        kind,
        {
          ...(file instanceof File ? { fileName: file.name, mimeType: file.type } : {}),
        },
        idempotencyKeyForLocalMessage(localMessage.id),
      );
      setMessages((previous) => mergeMessages(replaceLocalMessage(previous, localMessage.id, message), []));
      setActivePanel("none");
    } catch (error) {
      console.error(error);
      setMessages((previous) => markLocalMessageFailed(previous, localMessage.id));
      showToast(t("chat.detail.media_send_failed", "Unable to send attachment"));
    } finally {
      setMediaSending(false);
    }
  };

  const retrySend = useCallback(async (message: Message) => {
    if (message.sendState !== "failed" || !chatId) return;
    // Retries reuse the failed placeholder's original idempotency key, so a
    // send that actually reached the server cannot duplicate the message.
    // Media retries are not offered: the original File/Blob is not retained
    // after a failed send (delete the placeholder via the context menu).
    if (message.type !== "text") return;
    setMessages((previous) => previous.map((item) => item.id === message.id ? { ...item, sendState: "sending" as const } : item));
    try {
      const sent = await ChatService.sendMessage(
        chatId,
        message.senderId,
        message.content,
        "text",
        undefined,
        undefined,
        idempotencyKeyForLocalMessage(message.id),
      );
      setMessages((previous) => mergeMessages(replaceLocalMessage(previous, message.id, sent), []));
    } catch (error) {
      console.error(error);
      setMessages((previous) => previous.map((item) => item.id === message.id ? { ...item, sendState: "failed" as const } : item));
      showToast(t("chat.detail.send_failed", "Unable to send message"));
    }
  }, [chatId, t]);

  const handleSendCustom = (type: Message["type"]) => {
    if (type === "image" || type === "video" || type === "file") {
      setMediaInputKind(type);
      // Set accept synchronously: React batches the state update, so the DOM
      // attribute must be patched before opening the picker.
      if (mediaInputRef.current) {
        mediaInputRef.current.accept = type === "image" ? "image/*" : type === "video" ? "video/*" : "";
      }
      mediaInputRef.current?.click();
      return;
    }
    showToast(t("chat.detail.attachments_unavailable"));
  };

  const handleMediaInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    event.target.value = "";
    if (!file || !chatId) return;
    void sendMedia(file, mediaInputKind);
  };

  const startVoiceRecording = async () => {
    if (isRecording) return;
    if (!navigator.mediaDevices?.getUserMedia || typeof MediaRecorder === "undefined") {
      showToast(t("chat.detail.voice_unavailable", "Voice recording is unavailable"));
      return;
    }
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      recordedChunks.current = [];
      cancelVoiceRef.current = false;
      const recorder = new MediaRecorder(stream);
      mediaRecorder.current = recorder;
      recorder.ondataavailable = (event) => { if (event.data.size > 0) recordedChunks.current.push(event.data); };
      recorder.onstop = () => {
        stream.getTracks().forEach((track) => track.stop());
        const blob = new Blob(recordedChunks.current, { type: recorder.mimeType || "audio/webm" });
        if (!cancelVoiceRef.current && blob.size > 0) void sendMedia(blob, "voice");
      };
      recorder.start();
      startRecording();
    } catch (error) {
      console.error(error);
      showToast(t("chat.detail.voice_unavailable", "Voice recording is unavailable"));
    }
  };

  const stopVoiceRecording = () => {
    if (mediaRecorder.current?.state === "recording") mediaRecorder.current.stop();
    stopRecording();
  };
  const cancelVoiceRecording = () => {
    cancelVoiceRef.current = true;
    stopVoiceRecording();
  };

  const handleTouchStart = useCallback((event: React.TouchEvent | React.MouseEvent, messageId: string) => {
    const point = "touches" in event ? event.touches[0] : event;
    longPressTimer.current = setTimeout(() => {
      setContextMenu({ isOpen: true, x: Math.min(point.clientX, window.innerWidth - 180), y: Math.min(point.clientY, window.innerHeight - 420), messageId });
    }, 500);
  }, []);

  const handleTouchEnd = useCallback(() => {
    if (longPressTimer.current) {
      clearTimeout(longPressTimer.current);
      longPressTimer.current = undefined;
    }
  }, []);

  const handleTouchMove = useCallback(() => {
    if (longPressTimer.current) {
      clearTimeout(longPressTimer.current);
      longPressTimer.current = undefined;
    }
  }, []);

  return (
    <div className="relative flex h-full flex-col overflow-hidden bg-bg-color">
      <ChatHeader chat={chat} id={chatId} />
      {pinnedMessageIds.length > 0 && (
        <button
          type="button"
          className="flex items-center gap-2 px-4 py-2 bg-chat-other-bg border-b border-border-color shrink-0 active:bg-active-bg transition-colors"
          onClick={() => {
            const firstPinnedId = pinnedMessageIds[0];
            if (firstPinnedId) {
              document.getElementById(`msg-${firstPinnedId}`)?.scrollIntoView({ behavior: "smooth", block: "center" });
            }
          }}
        >
          <Pin className="w-4 h-4 text-primary-blue shrink-0" />
          <span className="text-[13px] text-text-main truncate flex-1">
            {(() => {
              const firstPinned = messages.find((item) => item.id === pinnedMessageIds[0]);
              return firstPinned
                ? `${firstPinned.content || `[${firstPinned.type}]`}`
                : t("chat.detail.pinned_messages", "Pinned messages");
            })()}
          </span>
          <span className="text-[12px] text-text-sub shrink-0">
            {pinnedMessageIds.length > 1 ? `${pinnedMessageIds.length} 条` : ""}
          </span>
        </button>
      )}
      <MessageList
        onScrollToTop={() => {
          if (loadingMore || !nextCursor) {
            return;
          }
          void load(nextCursor);
        }}
        messages={messages}
        chat={chat}
        currentUser={sessionUser}
        cleanMode={chat?.settings?.cleanMode ?? false}
        showAvatar={chat?.settings?.showAvatar ?? true}
        contextMenu={contextMenu}
        handleTouchStart={handleTouchStart}
        handleTouchEnd={handleTouchEnd}
        handleTouchMove={handleTouchMove}
        setFullscreenMedia={setFullscreenMedia}
        highlightedMsgId={highlightedMsgId}
        setHighlightedMsgId={setHighlightedMsgId}
        setActivePanel={setActivePanel}
        hasMoreTop={Boolean(nextCursor)}
        loadingMore={loadingMore}
        onRetry={(message) => void retrySend(message)}
      />
      {loadError && messages.length === 0 && (
        <button type="button" className="absolute inset-0 m-auto z-10 w-fit h-fit text-[14px] text-primary-blue" onClick={() => void load()}>
          {t("common.retry", "Tap to retry")}
        </button>
      )}
      <ChatInputArea
        id={chatId}
        currentUser={sessionUser}
        chat={chat}
        replyingTo={replyingTo}
        setReplyingTo={setReplyingTo}
        editor={editor}
        inputValue={inputValue}
        isVoiceMode={isVoiceMode}
        setIsVoiceMode={setIsVoiceMode}
        activePanel={activePanel}
        setActivePanel={setActivePanel}
        isRecording={isRecording}
        startRecording={startVoiceRecording}
        handleSendVoice={stopVoiceRecording}
        cancelRecording={cancelVoiceRecording}
        handleSend={() => void send()}
        handleSendCustom={handleSendCustom}
        emojis={emojis}
        onFileSelected={(file, kind) => void sendMedia(file, kind)}
      />
      <VoiceRecordingOverlay isRecording={isRecording} recordingTime={recordingTime} />
      <FullscreenMediaOverlay media={fullscreenMedia} onClose={() => setFullscreenMedia(null)} />
      <input
        ref={mediaInputRef}
        type="file"
        accept={mediaInputKind === "image" ? "image/*" : mediaInputKind === "video" ? "video/*" : undefined}
        className="hidden"
        onChange={handleMediaInputChange}
      />
      <MessageContextMenu
        contextMenu={contextMenu}
        messages={messages}
        onClose={() => setContextMenu((value) => ({ ...value, isOpen: false }))}
        onCopy={(messageId) => {
          const message = messages.find((item) => item.id === messageId);
          if (message) {
            if (navigator.clipboard?.writeText) {
              void navigator.clipboard.writeText(message.content).catch(() => showToast(t("chat.detail.copy_failed", "Unable to copy message")));
            } else {
              showToast(t("chat.detail.copy_unavailable", "Copy is unavailable in this browser"));
            }
          }
          setContextMenu((value) => ({ ...value, isOpen: false }));
        }}
        onReply={setReplyingTo}
        onStar={(messageId) => {
          const message = messages.find((item) => item.id === messageId);
          if (message) {
            void ChatService.starMessage(chatId, messageId, !message.isStarred)
              .then(() => {
                setMessages((previous) => previous.map((item) => item.id === messageId ? { ...item, isStarred: !item.isStarred } : item));
              })
              .catch((error) => {
                console.error(error);
                showToast(t("chat.detail.star_failed", "Unable to update favorite"));
              });
          }
          setContextMenu((value) => ({ ...value, isOpen: false }));
        }}
        onDelete={(messageId) => {
          const message = messages.find((item) => item.id === messageId);
          if (message?.sendState === "failed" || message?.sendState === "sending") {
            setMessages((previous) => previous.filter((item) => item.id !== messageId));
            setContextMenu((value) => ({ ...value, isOpen: false }));
            return;
          }
          void ChatService.deleteMessage(chatId, messageId).then(() => {
            setMessages((previous) => previous.filter((item) => item.id !== messageId));
            setContextMenu((value) => ({ ...value, isOpen: false }));
          });
        }}
        onEdit={(messageId) => {
          const message = messages.find((item) => item.id === messageId);
          setContextMenu((value) => ({ ...value, isOpen: false }));
          if (!message) return;
          void showPrompt(t("chat.context.edit", "Edit"), message.content).then((text) => {
            if (text === null) return;
            const normalized = text.trim();
            if (!normalized || normalized === message.content) return;
            void ChatService.editMessage(chatId, messageId, normalized)
              .then((updated) => setMessages((previous) => mergeMessages(previous, [updated])))
              .catch((error) => {
                console.error(error);
                showToast(t("chat.context.edit_failed", "Unable to edit message"));
              });
          });
        }}
        onRecall={(messageId) => {
          setContextMenu((value) => ({ ...value, isOpen: false }));
          void ChatService.recallMessage(chatId, messageId)
            .then(() => void load())
            .catch((error) => {
              console.error(error);
              showToast(t("chat.context.recall_failed", "Unable to recall message"));
            });
        }}
        onPin={handlePinMessage}
        pinnedMessageIds={pinnedMessageIds}
        currentUserId={sessionUser?.id}
      />
    </div>
  );
}

function mergeMessages(previous: readonly Message[], incoming: readonly Message[]): Message[] {
  const messages = new Map(previous.map((message) => [message.id, message]));
  for (const message of incoming) messages.set(message.id, message);
  const sorted = Array.from(messages.values()).sort((left, right) =>
    left.timestamp - right.timestamp
    || (left.id < right.id ? -1 : left.id > right.id ? 1 : 0),
  );
  // Bounded window: keep the newest messages resident; older history stays
  // reachable through the server cursor (nextCursor) when the user scrolls
  // to the top. The newest entries are never trimmed.
  return sorted.length > MAX_RENDERED_MESSAGES
    ? sorted.slice(sorted.length - MAX_RENDERED_MESSAGES)
    : sorted;
}

/**
 * The server deduplicates posts by clientMsgId; the key must be stable for
 * the whole lifecycle of one local message (send + retries). The local
 * placeholder id is `local-<uuid>` — strip the prefix so the wire key is the
 * plain uuid.
 */
function idempotencyKeyForLocalMessage(localId: string): string | undefined {
  const prefix = "local-";
  return localId.startsWith(prefix) ? localId.slice(prefix.length) : undefined;
}

function replaceLocalMessage(previous: readonly Message[], localId: string, sent: Message): Message[] {
  return previous.map((item) => (item.id === localId ? sent : item));
}

function markLocalMessageFailed(previous: readonly Message[], localId: string): Message[] {
  return previous.map((item) =>
    item.id === localId ? { ...item, sendState: "failed" as const } : item,
  );
}
