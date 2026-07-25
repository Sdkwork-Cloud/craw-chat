import React, { useEffect, useRef } from "react";
import { cn } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";

export const CommentInputOverlay = ({
  activeCommentPostId,
  commentText,
  setCommentText,
  onClose,
  onSend,
}: {
  activeCommentPostId: string | null;
  commentText: string;
  setCommentText: (text: string) => void;
  onClose: () => void;
  onSend: () => void;
}) => {
  const { t } = useTranslation();
  const inputRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (activeCommentPostId) {
      setTimeout(() => {
        inputRef.current?.focus();
      }, 100);
    }
  }, [activeCommentPostId]);

  if (!activeCommentPostId) return null;

  return (
    <div className="fixed inset-0 z-50 flex flex-col justify-end pointer-events-auto">
      <div className="absolute inset-0 bg-transparent" onClick={onClose} />
      <div className="bg-bg-color min-h-[56px] w-full border-t border-border-color flex items-end px-4 py-3 pb-safe relative z-10 shadow-[0_-4px_16px_rgba(0,0,0,0.05)]">
        <textarea
          ref={inputRef}
          className="flex-1 bg-chat-other-bg rounded-2xl px-4 py-2 text-[15px] max-h-24 outline-none resize-none placeholder-text-sub text-text-main"
          placeholder={t("community.auto_prop_2eb1a43f", "写评论...")}
          value={commentText}
          onChange={(e) => setCommentText(e.target.value)}
          rows={Math.min(4, commentText.split("\n").length || 1)}
        />
        <button
          className={cn(
            "ml-3 px-4 py-2 rounded-full font-medium text-[14px] shrink-0 transition-colors",
            commentText.trim()
              ? "bg-blue-500 text-white"
              : "bg-chat-other-bg text-text-sub opacity-50"
          )}
          onClick={onSend}
          disabled={!commentText.trim()}
        >
          {t("community.auto_ab650", "发送")}
        </button>
      </div>
    </div>
  );
};
