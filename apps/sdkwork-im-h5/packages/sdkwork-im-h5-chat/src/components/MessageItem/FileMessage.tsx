import { useTranslation } from "react-i18next";
import React from "react";
import type { Message } from "@sdkwork/im-h5-types";
import { FileText } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";

export const FileMessage = ({ msg, isMe, onClick }: { msg: Message; isMe: boolean; onClick?: () => void }) => {
  const { t } = useTranslation();
  return (
  <div className="flex items-center gap-3 min-w-[200px] cursor-pointer" onClick={onClick}>
    <div className="flex-1 flex flex-col min-w-0">
      <span className="text-[15px] font-medium truncate">{msg.metadata?.fileName || msg.content}</span>
      <span
        className={cn(
          "text-[12px] mt-1",
          isMe ? "text-white/70" : "text-text-sub",
        )}
      >
        {msg.metadata?.size} · {msg.metadata?.ext?.toUpperCase()}
      </span>
    </div>
    <div
      className={cn(
        "w-12 h-12 rounded-lg shrink-0 flex items-center justify-center",
        isMe ? "bg-white/20" : "bg-black/5 dark:bg-white/5",
      )}
    >
      <FileText className="w-7 h-7" />
    </div>
  </div>
);
};
