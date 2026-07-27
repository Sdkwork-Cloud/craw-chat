import React from "react";
import { motion } from "motion/react";
import { X } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import type { Message, User, Chat } from "@sdkwork/im-h5-types";
import { useTranslation } from "react-i18next";

interface ReplyingToBannerProps {
  replyingTo: Message;
  currentUser: User | null;
  chat: Chat | null;
  onClearReply: () => void;
}

export const ReplyingToBanner: React.FC<ReplyingToBannerProps> = ({
  replyingTo,
  currentUser,
  chat,
  onClearReply,
}) => {
  const { t } = useTranslation();

  return (
    <motion.div
      initial={{ opacity: 0, height: 0 }}
      animate={{ opacity: 1, height: "auto" }}
      exit={{ opacity: 0, height: 0 }}
      className="px-3 py-2 bg-black/5 dark:bg-white/5 border-b border-border-color flex items-center justify-between overflow-hidden"
    >
      <div className="flex flex-col text-sm truncate pr-4">
        <span className="text-text-sub font-medium text-[12px] mb-0.5">
          {t("chat.detail.reply_to")}{" "}
          {replyingTo.senderId === currentUser?.id
            ? t("chat.detail.me")
            : chat?.participants.find((p) => p.id === replyingTo.senderId)?.name ||
              t("chat.detail.unknown")}
          :
        </span>
        <span className="text-text-main truncate text-[14px]">
          {replyingTo.type === "text"
            ? replyingTo.content
            : `[${
                replyingTo.type === "image"
                  ? t("chat.detail.type_image")
                  : replyingTo.type === "voice"
                  ? t("chat.detail.type_voice")
                  : replyingTo.type === "video"
                  ? t("chat.detail.type_video")
                  : replyingTo.type === "file"
                  ? t("chat.detail.type_file")
                  : t("chat.detail.type_media")
              }]`}
        </span>
      </div>
      <IconButton
        icon={<X className="w-5 h-5 text-text-sub" />}
        onClick={onClearReply}
        className="shrink-0 p-1 bg-black/5 dark:bg-white/10 rounded-full hover:bg-black/10 dark:hover:bg-white/20"
      />
    </motion.div>
  );
};
