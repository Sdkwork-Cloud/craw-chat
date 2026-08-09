import { useCallback, useEffect, useRef, useState } from "react";
import { useEditor } from "@tiptap/react";
import Placeholder from "@tiptap/extension-placeholder";
import StarterKit from "@tiptap/starter-kit";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";

import { showToast } from "@sdkwork/im-h5-commons";
import { useAppStore } from "@sdkwork/im-h5-core";
import type { Chat, Message } from "@sdkwork/im-h5-types";

import { ChatHeader } from "../components/Chat/ChatHeader";
import { ChatInputArea } from "../components/Chat/ChatInputArea";
import { MessageContextMenu } from "../components/Chat/MessageContextMenu";
import { MessageList } from "../components/Chat/MessageList";
import { VoiceRecordingOverlay } from "../components/Chat/VoiceRecordingOverlay";
import { FullscreenMediaOverlay } from "../components/Chat/FullscreenMediaOverlay";
import { ChatService } from "../services/ChatService";
import { subscribeConversationLiveMessages } from "../services/chatRealtimeService";

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
  const [fullscreenMedia, setFullscreenMedia] = useState<{ type: "image" | "video"; url: string } | null>(null);
  const [loadError, setLoadError] = useState(false);
  const [contextMenu, setContextMenu] = useState({ isOpen: false, x: 0, y: 0, messageId: null as string | null });
  const longPressTimer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  const recordingTimer = useRef<ReturnType<typeof setInterval> | undefined>(undefined);

  const editor = useEditor({
    extensions: [StarterKit, Placeholder.configure({ placeholder: t("chat.detail.placeholder") })],
    content: "",
    editorProps: { attributes: { class: "min-h-8 max-h-28 overflow-y-auto rounded-lg bg-chat-other-bg px-3 py-2 text-[16px] text-text-main focus:outline-none" } },
  });
  const inputValue = editor?.getText() ?? "";

  const load = useCallback(async (cursor?: string) => {
    if (!chatId) return;
    if (cursor) setLoadingMore(true);
    try {
      const page = await ChatService.getMessagePage(chatId, cursor);
      setLoadError(false);
      setMessages((previous) => mergeMessages(cursor ? previous : [], page.items));
      setNextCursor(page.hasMore ? page.nextCursor : undefined);
      if (!cursor) await ChatService.markAsRead(chatId);
    } catch (error) {
      console.error(error);
      setLoadError(true);
      showToast(t("chat.detail.load_failed", "Unable to load messages"));
    } finally {
      setLoadingMore(false);
    }
  }, [chatId, t]);

  useEffect(() => {
    if (!chatId) return undefined;
    void Promise.all([ChatService.getChatById(chatId), ChatService.getEmojis()]).then(([value, emojiList]) => {
      if (value) {
        const participants = sessionUser
          ? value.participants.filter((participant) => participant.id !== sessionUser.id)
          : value.participants;
        setChat({ ...value, participants });
      }
      setEmojis(emojiList);
    });
    void load();
    return subscribeConversationLiveMessages(chatId, () => void load());
  }, [chatId, load, sessionUser]);

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
    try {
      const replyTo = replyingTo ? {
        messageId: replyingTo.id,
        senderDisplayName: replyingTo.senderId === sessionUser?.id
          ? t("chat.detail.me", "Me")
          : chat?.participants.find((participant) => participant.id === replyingTo.senderId)?.name ?? replyingTo.senderId,
        contentPreview: replyingTo.content.slice(0, 200),
      } : undefined;
      const message = await ChatService.sendMessage(chatId, sessionUser?.id ?? "", content, "text", undefined, replyTo);
      setMessages((previous) => mergeMessages(previous, [message]));
      editor?.commands.clearContent();
      setReplyingTo(null);
    } catch (error) {
      console.error(error);
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
    try {
      const message = await ChatService.sendMediaMessage(chatId, file, kind, {
        ...(file instanceof File ? { fileName: file.name, mimeType: file.type } : {}),
      });
      setMessages((previous) => mergeMessages(previous, [message]));
      setActivePanel("none");
    } catch (error) {
      console.error(error);
      showToast(t("chat.detail.media_send_failed", "Unable to send attachment"));
    } finally {
      setMediaSending(false);
    }
  };

  const startVoiceRecording = async () => {
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

  const handleTouchStart = (event: React.TouchEvent | React.MouseEvent, messageId: string) => {
    const point = "touches" in event ? event.touches[0] : event;
    longPressTimer.current = setTimeout(() => {
      setContextMenu({ isOpen: true, x: Math.min(point.clientX, window.innerWidth - 180), y: Math.min(point.clientY, window.innerHeight - 280), messageId });
    }, 500);
  };

  return (
    <div className="relative flex h-full flex-col overflow-hidden bg-bg-color">
      <ChatHeader chat={chat} id={chatId} />
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
        handleTouchEnd={() => longPressTimer.current && clearTimeout(longPressTimer.current)}
        handleTouchMove={() => longPressTimer.current && clearTimeout(longPressTimer.current)}
        setFullscreenMedia={setFullscreenMedia}
        highlightedMsgId={highlightedMsgId}
        setHighlightedMsgId={setHighlightedMsgId}
        setActivePanel={setActivePanel}
      />
      {loadError && messages.length === 0 && (
        <button type="button" className="absolute inset-x-0 top-1/2 z-10 mx-auto w-fit -translate-y-1/2 text-[14px] text-primary-blue" onClick={() => void load()}>
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
        handleSendCustom={() => showToast(t("chat.detail.attachments_unavailable"))}
        emojis={emojis}
        onFileSelected={(file, kind) => void sendMedia(file, kind)}
      />
      <VoiceRecordingOverlay isRecording={isRecording} recordingTime={recordingTime} />
      <FullscreenMediaOverlay media={fullscreenMedia} onClose={() => setFullscreenMedia(null)} />
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
          if (message) void ChatService.starMessage(chatId, messageId, !message.isStarred).then(() => {
            setMessages((previous) => previous.map((item) => item.id === messageId ? { ...item, isStarred: !item.isStarred } : item));
          });
          setContextMenu((value) => ({ ...value, isOpen: false }));
        }}
        onDelete={(messageId) => void ChatService.deleteMessage(chatId, messageId).then(() => {
          setMessages((previous) => previous.filter((item) => item.id !== messageId));
          setContextMenu((value) => ({ ...value, isOpen: false }));
        })}
      />
    </div>
  );
}

function mergeMessages(previous: readonly Message[], incoming: readonly Message[]): Message[] {
  const messages = new Map(previous.map((message) => [message.id, message]));
  for (const message of incoming) messages.set(message.id, message);
  return Array.from(messages.values()).sort((left, right) => left.timestamp - right.timestamp);
}
