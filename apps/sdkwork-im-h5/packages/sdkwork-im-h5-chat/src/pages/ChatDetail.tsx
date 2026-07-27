import React, { useState, useRef, useEffect, useCallback } from "react";
import { useNavigate, useParams, useSearchParams } from "react-router";
import { ChevronLeft, MoreHorizontal, Phone, Video } from "lucide-react";
import { useTranslation } from "react-i18next";
import { IconButton, cn, showToast } from "@sdkwork/im-h5-commons";
import { useAppStore } from "@sdkwork/im-h5-core";
import { ChatService } from "../services/ChatService";
import type { Message, Chat, User } from "@sdkwork/im-h5-types";
import { useEditor, EditorContent } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import Placeholder from "@tiptap/extension-placeholder";
import { motion, AnimatePresence } from "motion/react";

import { MessageItem } from "../components/MessageItem";
import { VoiceRecordingOverlay } from "../components/Chat/VoiceRecordingOverlay";
import { FullscreenMediaOverlay } from "../components/Chat/FullscreenMediaOverlay";
import { MessageContextMenu } from "../components/Chat/MessageContextMenu";
import { ChatInputArea } from "../components/Chat/ChatInputArea";
import { MessageList } from "../components/Chat/MessageList";
import { ChatHeader } from "../components/Chat/ChatHeader";

export const ChatDetail: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams();
  const [searchParams] = useSearchParams();
  const msgId = searchParams.get("msgId");
  const navigate = useNavigate();
  const { currentUser } = useAppStore();
  const [chat, setChat] = useState<Chat | null>(null);
  const [messages, setMessages] = useState<Message[]>([]);
  const [isVoiceMode, setIsVoiceMode] = useState(false);
  const [isRecording, setIsRecording] = useState(false);
  const [activePanel, setActivePanel] = useState<"none" | "emoji" | "action">(
    "none",
  );
  const [inputValue, setInputValue] = useState("");
  const [recordingTime, setRecordingTime] = useState(0);
  const [showAvatar, setShowAvatar] = useState(true);
  const [cleanMode, setCleanMode] = useState(false);
  const [fullscreenMedia, setFullscreenMedia] = useState<{
    type: "image" | "video";
    url: string;
  } | null>(null);
  const [emojis, setEmojis] = useState<string[]>([]);
  const mediaRecorderRef = useRef<MediaRecorder | null>(null);
  const audioChunksRef = useRef<Blob[]>([]);

  const [highlightedMsgId, setHighlightedMsgId] = useState<string | null>(
    msgId,
  );
  const [replyingTo, setReplyingTo] = useState<Message | null>(null);

  useEffect(() => {
    if (msgId) {
      setHighlightedMsgId(msgId);
      setTimeout(() => setHighlightedMsgId(null), 3000);
    }
  }, [msgId]);

  // Context Menu State
  const [contextMenu, setContextMenu] = useState<{
    isOpen: boolean;
    x: number;
    y: number;
    messageId: string | null;
  }>({ isOpen: false, x: 0, y: 0, messageId: null });

  const longPressTimer = useRef<NodeJS.Timeout | null>(null);

  // Timer for voice recording
  useEffect(() => {
    if (id) {
      ChatService.getChatById(id).then((c) => {
        if (c) {
          setChat(c);
          setShowAvatar(c.settings?.showAvatar ?? true);
          setCleanMode(c.settings?.cleanMode ?? false);
        }
      });
      ChatService.getMessages(id).then(setMessages);
      ChatService.markAsRead(id);
    }
    ChatService.getEmojis().then(setEmojis);
  }, [id]);

  useEffect(() => {
    let interval: NodeJS.Timeout;
    if (isRecording) {
      interval = setInterval(() => setRecordingTime((t) => t + 1), 1000);
    } else {
      setRecordingTime(0);
    }
    return () => clearInterval(interval);
  }, [isRecording]);

  const formatTime = (secs: number) => {
  const m = Math.floor(secs / 60);
    const s = secs % 60;
    return `${m}:${s.toString().padStart(2, "0")}`;
  };

  const editor = useEditor({
    extensions: [
      StarterKit,
      Placeholder.configure({ placeholder: t('chat.detail.placeholder') }),
    ],
    content: "",
    editorProps: {
      attributes: {
        class:
          "w-full bg-chat-other-bg rounded-lg focus:outline-none resize-none text-[16px] py-2 px-3 text-text-main leading-normal overflow-y-auto max-h-[120px] min-h-[40px]",
      },
    },
    onUpdate: ({ editor }) => {
      setInputValue(editor.getText());
    },
  });

  const handleSend = async () => {
    if (!editor || !id || !currentUser) return;
    const text = editor.getText();
    if (!text.trim()) return;

    const metadata = replyingTo ? { replyTo: replyingTo.id } : undefined;
    const newMessage = await ChatService.sendMessage(
      id,
      currentUser.id,
      text,
      "text",
      metadata,
    );

    setMessages((prev) => [...prev, newMessage]);
    editor.commands.clearContent();
    setInputValue("");
    setReplyingTo(null);
    setActivePanel("none");
  };

  const startRecording = async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      const mediaRecorder = new MediaRecorder(stream);
      mediaRecorderRef.current = mediaRecorder;
      audioChunksRef.current = [];

      mediaRecorder.ondataavailable = (e) => {
        if (e.data.size > 0) audioChunksRef.current.push(e.data);
      };

      mediaRecorder.start();
      setIsRecording(true);
      setRecordingTime(0);
    } catch (e) {
      showToast(t('chat.detail.no_mic_permission'));
    }
  };

  const handleSendVoice = async () => {
    if (!id || !currentUser) return;

    if (mediaRecorderRef.current && isRecording) {
      mediaRecorderRef.current.onstop = async () => {
        const audioBlob = new Blob(audioChunksRef.current, {
          type: "audio/webm",
        });
        const url = URL.createObjectURL(audioBlob);

        if (recordingTime < 1) {
          showToast(t('chat.detail.too_short'));
        } else {
          const metadata: Record<string, any> = {
            duration: `${recordingTime}s`,
          };
          if (replyingTo) {
            metadata.replyTo = replyingTo.id;
          }
          const newMessage = await ChatService.sendMessage(
            id,
            currentUser.id,
            url,
            "voice",
            metadata,
          );
          setMessages((prev) => [...prev, newMessage]);
        }

        setIsRecording(false);
        setRecordingTime(0);
        setReplyingTo(null);

        // Stop all tracks
        mediaRecorderRef.current?.stream
          .getTracks()
          .forEach((track) => track.stop());
        mediaRecorderRef.current = null;
      };

      mediaRecorderRef.current.stop();
    } else {
      setIsRecording(false);
      setRecordingTime(0);
    }
  };

  const cancelRecording = () => {
  if (mediaRecorderRef.current && isRecording) {
      mediaRecorderRef.current.onstop = () => {
        setIsRecording(false);
        setRecordingTime(0);
        mediaRecorderRef.current?.stream
          .getTracks()
          .forEach((track) => track.stop());
        mediaRecorderRef.current = null;
      };
      mediaRecorderRef.current.stop();
    } else {
      setIsRecording(false);
      setRecordingTime(0);
    }
  };

  const togglePanel = (panel: "emoji" | "action") => {
  if (activePanel === panel) {
      setActivePanel("none");
      editor?.commands.focus();
    } else {
      setActivePanel(panel);
      setIsVoiceMode(false);
    }
  };

  const handleSendCustom = async (
    type: Message["type"],
    content: string,
    metadata?: Record<string, any>,
  ) => {
    if (!id || !currentUser) return;

    const finalMetadata = { ...metadata };
    if (replyingTo) {
      finalMetadata.replyTo = replyingTo.id;
    }

    const newMessage = await ChatService.sendMessage(
      id,
      currentUser.id,
      content,
      type,
      Object.keys(finalMetadata).length > 0 ? finalMetadata : undefined,
    );
    setMessages((prev) => [...prev, newMessage]);
    setActivePanel("none");
    setReplyingTo(null);
  };

  const handleInputFocus = () => {
  setActivePanel("none");
  };

  const handleTouchStart = useCallback(
    (e: React.TouchEvent | React.MouseEvent, messageId: string) => {
      if (longPressTimer.current) clearTimeout(longPressTimer.current);

      // Get coordinates
      let clientX, clientY;
      if ("touches" in e) {
        clientX = e.touches[0].clientX;
        clientY = e.touches[0].clientY;
      } else {
        clientX = (e as React.MouseEvent).clientX;
        clientY = (e as React.MouseEvent).clientY;
      }

      longPressTimer.current = setTimeout(() => {
        if (navigator.vibrate) navigator.vibrate(50);

        // Calculate position to prevent overflowing screen edges
        const menuWidth = 160;
        const menuHeight = 200;
        const x = Math.min(clientX, window.innerWidth - menuWidth - 20);
        const y = Math.min(clientY, window.innerHeight - menuHeight - 20);

        setContextMenu({
          isOpen: true,
          x: Math.max(20, x),
          y: Math.max(20, y),
          messageId,
        });
      }, 500); // 500ms long press
    },
    [],
  );

  const handleTouchEnd = useCallback(() => {
    if (longPressTimer.current) {
      clearTimeout(longPressTimer.current);
      longPressTimer.current = null;
    }
  }, []);

  const handleTouchMove = useCallback(() => {
    if (longPressTimer.current) {
      clearTimeout(longPressTimer.current);
      longPressTimer.current = null;
    }
  }, []);

  const handleCopy = async (messageId: string) => {
    const msg = messages.find((m) => m.id === messageId);
    if (msg) {
      await navigator.clipboard.writeText(msg.content);
      showToast(t('chat.detail.copied'));
    }
    setContextMenu((prev) => ({ ...prev, isOpen: false }));
  };

  const handleStarMessage = async (messageId: string) => {
    const msg = messages.find((m) => m.id === messageId);
    if (id && msg) {
      await ChatService.starMessage(id, messageId, !msg.isStarred);
      const updatedMessages = await ChatService.getMessages(id);
      setMessages(updatedMessages);
    }
    setContextMenu((prev) => ({ ...prev, isOpen: false }));
  };

  const handleDeleteMessage = async (messageId: string) => {
    if (id) {
      await ChatService.deleteMessage(id, messageId);
      const updatedMessages = await ChatService.getMessages(id);
      setMessages(updatedMessages);
    }
    setContextMenu((prev) => ({ ...prev, isOpen: false }));
  };

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      <VoiceRecordingOverlay
        isRecording={isRecording}
        recordingTime={recordingTime}
      />

      {/* Header */}
      <ChatHeader chat={chat} id={id} />

      {/* Messages Area */}
      <MessageList
        messages={messages}
        chat={chat}
        currentUser={currentUser}
        cleanMode={cleanMode}
        showAvatar={showAvatar}
        contextMenu={contextMenu}
        handleTouchStart={handleTouchStart}
        handleTouchEnd={handleTouchEnd}
        handleTouchMove={handleTouchMove}
        setFullscreenMedia={setFullscreenMedia}
        highlightedMsgId={highlightedMsgId}
        setHighlightedMsgId={setHighlightedMsgId}
        setActivePanel={setActivePanel}
      />

      <ChatInputArea
        id={id}
        currentUser={currentUser}
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
        startRecording={startRecording}
        handleSendVoice={handleSendVoice}
        cancelRecording={cancelRecording}
        handleSend={handleSend}
        handleSendCustom={handleSendCustom}
        emojis={emojis}
      />

      <FullscreenMediaOverlay
        media={fullscreenMedia}
        mediaList={messages
            .filter((m) => m.type === "image" || m.type === "video")
            .map((m) => ({
                 type: m.type as "image" | "video",
                 url: m.content
            }))
        }
        onClose={() => setFullscreenMedia(null)}
      />

      <MessageContextMenu
        contextMenu={contextMenu}
        messages={messages}
        onClose={() => setContextMenu((prev) => ({ ...prev, isOpen: false }))}
        onCopy={handleCopy}
        onReply={(msg) => setReplyingTo(msg)}
        onStar={handleStarMessage}
        onDelete={handleDeleteMessage}
      />
    </div>
  );
};
