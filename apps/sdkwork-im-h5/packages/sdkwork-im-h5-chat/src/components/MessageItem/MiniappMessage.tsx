import React from "react";
import type { Message } from "@sdkwork/im-h5-types";
import { ShoppingBag } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";

export const MiniappMessage = ({
  msg,
  isMe,
}: {
  msg: Message;
  isMe: boolean;
}) => {
  const { t } = useTranslation();
return (
  <div className="flex flex-col gap-2 min-w-[200px] cursor-pointer">
    <div className="flex items-center gap-1.5 opacity-80 mb-1">
      <ShoppingBag className="w-3.5 h-3.5" />
      <span className="text-[12px]">{t('chat.date.miniapp_title')}</span>
    </div>
    <span className="font-bold text-[15px] leading-snug">
      {msg.metadata?.title}
    </span>
    <div className="flex items-center gap-3 mt-1">
      <div
        className={cn(
          "w-10 h-10 rounded shrink-0 flex items-center justify-center",
          isMe ? "bg-white/20" : "bg-black/5 dark:bg-white/5",
        )}
      >
        {msg.metadata?.icon ? (
          <img
            src={msg.metadata.icon}
            className="w-full h-full object-cover rounded"
          />
        ) : (
          <ShoppingBag className="w-5 h-5" />
        )}
      </div>
      <span
        className={cn(
          "text-[13px] line-clamp-2",
          isMe ? "text-white/80" : "text-text-sub",
        )}
      >
        {msg.metadata?.desc}
      </span>
    </div>
    {msg.metadata?.image && (
      <div className="w-full mt-1">
        <img
          src={msg.metadata.image}
          className="w-full h-auto max-h-[120px] object-cover rounded"
        />
      </div>
    )}
  </div>
  );
};
