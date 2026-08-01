import React, { useRef, useState, useEffect } from "react";
import { motion, AnimatePresence } from "motion/react";
import { EditorContent, Editor } from "@tiptap/react";
import {
  Keyboard,
  Mic,
  Smile,
  PlusCircle,
  Send,
  X,
  ImageIcon,
  Video,
  Music,
  Folder,
  Phone,
  Link as LinkIcon,
  ShoppingBag,
} from "lucide-react";
import { IconButton, cn } from "@sdkwork/im-h5-commons";
import type { Message, User, Chat } from "@sdkwork/im-h5-types";
import { ReplyingToBanner } from "./ReplyingToBanner";
import { ChatActionPanel } from "./ChatActionPanel";
import { ChatEmojiPanel } from "./ChatEmojiPanel";
import { useTranslation } from "react-i18next";

interface ChatInputAreaProps {
  id?: string;
  currentUser: User | null;
  chat: Chat | null;
  replyingTo: Message | null;
  setReplyingTo: (msg: Message | null) => void;
  editor: Editor | null;
  inputValue: string;
  isVoiceMode: boolean;
  setIsVoiceMode: (mode: boolean) => void;
  activePanel: "none" | "emoji" | "action";
  setActivePanel: (panel: "none" | "emoji" | "action") => void;
  isRecording: boolean;
  startRecording: () => void;
  handleSendVoice: () => void;
  cancelRecording: () => void;
  handleSend: () => void;
  handleSendCustom: (
    type: Message["type"],
    content: string,
    metadata?: Record<string, any>,
  ) => void;
  emojis: string[];
  onFileSelected?: (file: File, kind: "image" | "video" | "file") => void;
}

export const ChatInputArea: React.FC<ChatInputAreaProps> = ({
  id,
  currentUser,
  chat,
  replyingTo,
  setReplyingTo,
  editor,
  inputValue,
  isVoiceMode,
  setIsVoiceMode,
  activePanel,
  setActivePanel,
  isRecording,
  startRecording,
  handleSendVoice,
  cancelRecording,
  handleSend,
  handleSendCustom,
  emojis,
  onFileSelected,
}) => {
  const { t } = useTranslation();
const togglePanel = (panel: "emoji" | "action") => {
  if (activePanel === panel) {
      setActivePanel("none");
      editor?.commands.focus();
    } else {
      setActivePanel(panel);
      setIsVoiceMode(false);
    }
  };

  const handleInputFocus = () => {
  setActivePanel("none");
  };

  return (
    <div className="bg-input-bg border-t border-border-color shrink-0 flex flex-col pb-safe transition-all duration-300">
      <AnimatePresence>
        {replyingTo && (
          <ReplyingToBanner
            replyingTo={replyingTo}
            currentUser={currentUser}
            chat={chat}
            onClearReply={() => setReplyingTo(null)}
          />
        )}
      </AnimatePresence>

      <div className="px-2 py-2 flex items-end gap-1.5">
        <IconButton
          icon={
            isVoiceMode ? (
              <Keyboard className="w-7 h-7 text-text-main" />
            ) : (
              <Mic className="w-7 h-7 text-text-main" />
            )
          }
          onClick={() => {
            setIsVoiceMode(!isVoiceMode);
            setActivePanel("none");
          }}
          className="shrink-0 mb-0.5"
        />

        <div className="flex-1 flex items-end min-h-[40px] py-1">
          {isVoiceMode ? (
            <motion.button
              className={cn(
                "w-full h-10 rounded-lg font-bold text-[16px] transition-colors select-none flex items-center justify-center gap-2",
                isRecording
                  ? "bg-primary-blue text-white shadow-lg shadow-primary-blue/30"
                  : "bg-chat-other-bg text-text-main border border-border-color",
              )}
              animate={{ scale: isRecording ? 0.96 : 1 }}
              transition={{ type: "spring", stiffness: 400, damping: 25 }}
              onPointerDown={(e) => {
                e.preventDefault();
                startRecording();
              }}
              onPointerUp={() => {
                if (isRecording) {
                  handleSendVoice();
                }
              }}
              onPointerLeave={() => {
                if (isRecording) {
                  cancelRecording();
                }
              }}
              onPointerCancel={() => {
                if (isRecording) {
                  cancelRecording();
                }
              }}
            >
              {isRecording ? (
                <>
                  <Mic className="w-5 h-5 animate-pulse" />
                  {t('chat.detail.release_to_send')}
                </>
              ) : (
                t('chat.detail.hold_to_talk')
              )}
            </motion.button>
          ) : (
            <div
              className="w-full"
              onKeyDownCapture={(e) => {
                if (e.key === "Enter" && !e.shiftKey) {
                  e.preventDefault();
                  e.stopPropagation();
                  handleSend();
                }
              }}
              onClick={handleInputFocus}
            >
              <EditorContent editor={editor} />
            </div>
          )}
        </div>

        <IconButton
          icon={<Smile className="w-7 h-7 text-text-main" />}
          onClick={() => togglePanel("emoji")}
          className="shrink-0 mb-0.5"
        />

        {!isVoiceMode && inputValue.trim() ? (
          <button
            onClick={handleSend}
            className="shrink-0 w-12 h-9 bg-primary-blue rounded-lg flex items-center justify-center text-white mb-1.5 mr-1 active:opacity-80 transition-opacity"
          >
            <Send className="w-4 h-4" />
          </button>
        ) : (
          <IconButton
            icon={<PlusCircle className="w-7 h-7 text-text-main" />}
            onClick={() => togglePanel("action")}
            className="shrink-0 mb-0.5"
          />
        )}
      </div>

      <AnimatePresence initial={false}>
        {activePanel === "action" && (
          <ChatActionPanel handleSendCustom={handleSendCustom} onFileSelected={onFileSelected} />
        )}
        {activePanel === "emoji" && (
          <ChatEmojiPanel emojis={emojis} editor={editor} />
        )}
      </AnimatePresence>
    </div>
  );
};
