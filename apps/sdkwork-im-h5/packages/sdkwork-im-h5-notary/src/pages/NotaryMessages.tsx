import React, { useState, useEffect } from "react";
import {
  Bell,
  ShieldAlert,
  CheckCircle,
  ChevronLeft,
  Info,
} from "lucide-react";
import { IconButton, cn } from "@sdkwork/im-h5-commons";
import { useNavigate } from "react-router";
import { notaryService } from "../services/notaryService";
import { useTranslation } from "react-i18next";

const MESSAGE_TYPE_STYLE_GETTER = (id: string) => {
  const { t } = useTranslation();
  if (id === "1") return { icon: Info, color: "text-blue-500" };
  if (id === "2") return { icon: CheckCircle, color: "text-green-500" };
  if (id === "3") return { icon: Bell, color: "text-orange-500" };
  return { icon: Bell, color: "text-text-sub" };
};

export const NotaryMessages: React.FC = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  const [messages, setMessages] = useState<any[]>([]);

  useEffect(() => {
    notaryService
      .getNotaryMessages()
      .then((data) => setMessages(data as any[]));
  }, []);

  return (
    <div className="flex flex-col h-full bg-bg-color">
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe relative">
        <div className="flex items-center z-10 w-16">
          <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} onClick={() => navigate(-1)} />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center pointer-events-none">
          <h1 className="text-[17px] font-medium text-text-main">{t("notary.messages.title")}</h1>
        </div>
        <div className="w-16" />
      </header>

      <div className="flex-1 overflow-y-auto pb-[90px] flex flex-col">
        {messages.map((msg, idx) => {
          const style = MESSAGE_TYPE_STYLE_GETTER(msg.id);
          const Icon = style.icon;
          
          let titleKey = `notary.messages.mock_${msg.id}_title`;
          let contentKey = `notary.messages.mock_${msg.id}_content`;
          let timeKey = msg.time === "昨天" ? "notary.messages.time_yesterday" : msg.time === "星期一" ? "notary.messages.time_monday" : msg.time === "10:45" ? "notary.messages.time_1045" : msg.time;

          return (
            <div
              key={msg.id}
              onClick={() => navigate(`/notary/messages/${msg.id}`, { state: { message: msg } })}
              className={cn(
                "px-4 py-4 cursor-pointer active:bg-black/5 dark:active:bg-white/5 transition-colors",
                !msg.unread ? "bg-bg-color" : "bg-primary-blue/5",
                idx !== messages.length - 1
                  ? "border-b border-border-color/50"
                  : "",
              )}
            >
              <div className="flex items-center justify-between mb-1.5">
                <div className="flex items-center gap-2">
                  <Icon className={cn("w-5 h-5", style.color)} />
                  <span className="text-[16px] font-bold text-text-main">
                    {t(titleKey, msg.title) as string}
                  </span>
                  {msg.unread && (
                    <div className="w-1.5 h-1.5 bg-red-500 rounded-full" />
                  )}
                </div>
                <span className="text-[12px] text-text-sub">{t(timeKey, msg.time) as string}</span>
              </div>
              <p className="text-[14px] text-text-sub leading-relaxed pl-7 line-clamp-2">
                {t(contentKey, msg.content) as string}
              </p>
            </div>
          );
        })}
      </div>
    </div>
  );
};
