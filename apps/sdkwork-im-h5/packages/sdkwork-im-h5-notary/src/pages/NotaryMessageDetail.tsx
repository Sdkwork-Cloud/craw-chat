import React from "react";
import { ChevronLeft, Info, CheckCircle, Bell, ArrowRight } from "lucide-react";
import { IconButton, cn } from "@sdkwork/im-h5-commons";
import { useNavigate, useParams, useLocation } from "react-router";
import { useTranslation } from "react-i18next";

export const NotaryMessageDetail: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const location = useLocation();
  const { id } = useParams<{ id: string }>();

  // Use state passed from the list, or fallback
  const message = location.state?.message || {
    id: "unknown",
    title: "消息详情",
    content: "这是一个消息的详细内容。",
    time: "刚刚",
  };

  const getStyle = (title: string, id: string) => {
  if (id === "1" || title.includes("补充材料")) return { icon: Info, color: "text-blue-500", bg: "bg-blue-500/10" };
    if (id === "2" || title.includes("成功")) return { icon: CheckCircle, color: "text-green-500", bg: "bg-green-500/10" };
    return { icon: Bell, color: "text-orange-500", bg: "bg-orange-500/10" };
  };

  const style = getStyle(message.title, message.id);
  const Icon = style.icon;

  const getAction = () => {
  if (message.id === "1" || message.title.includes("补充")) {
      return { label: t("notary.messages.go_to_add"), path: "/notary/files" };
    }
    if (message.id === "3" || message.title.includes("进度")) {
      return { label: t("notary.messages.view_my"), path: "/notary" };
    }
    return null;
  };

  const action = getAction();

  let titleKey = `notary.messages.mock_${message.id}_title`;
  let contentKey = `notary.messages.mock_${message.id}_content`;
  let timeKey = message.time === "昨天" ? "notary.messages.time_yesterday" : message.time === "星期一" ? "notary.messages.time_monday" : message.time === "10:45" ? "notary.messages.time_1045" : "notary.messages.just_now";
  
  const displayTitle = message.id === "unknown" ? t("notary.messages.detail_title") as string : t(titleKey, message.title) as string;
  const displayContent = message.id === "unknown" ? message.content : t(contentKey, message.content) as string;

  return (
    <div className="flex flex-col h-full bg-bg-color fixed inset-0 z-50">
      <header className="h-[56px] flex items-center justify-between px-1 glass-header shrink-0 pt-safe relative">
        <div className="flex items-center z-10 w-16">
          <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} onClick={() => navigate(-1)} />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center pointer-events-none">
          <h1 className="text-[17px] font-medium text-text-main">{t("notary.messages.detail_title")}</h1>
        </div>
        <div className="w-16" />
      </header>

      <div className="flex-1 overflow-y-auto p-4 flex flex-col items-center">
        <span className="text-[12px] text-text-sub bg-black/5 dark:bg-white/10 px-3 py-1 rounded-full mb-6">
          {t(timeKey, message.time) as string}
        </span>
        
        <div className="w-full bg-white dark:bg-[#1c1c1e] rounded-2xl p-5 shadow-sm border border-border-color/50">
          <div className="flex items-center gap-3 mb-4">
             <div className={cn("w-10 h-10 rounded-full flex items-center justify-center", style.bg)}>
                <Icon className={cn("w-5 h-5", style.color)} />
             </div>
             <h2 className="text-[18px] font-bold text-text-main">{displayTitle}</h2>
          </div>
          
          <div className="text-[15px] text-text-main leading-relaxed mb-6">
            {displayContent}
          </div>

          <div className="h-[1px] w-full bg-border-color/50 mb-4" />
          
          <div className="text-[13px] text-text-sub space-y-2">
            <p>{t("notary.messages.notary_office")}{t("notary.messages.office_name")}</p>
            <p>{t("notary.messages.recipient")}{t("notary.messages.recipient_name")}</p>
          </div>
        </div>

        {action && (
          <button 
            className="mt-6 w-full max-w-[300px] h-12 bg-primary-blue text-white rounded-xl font-medium flex items-center justify-center gap-2 active:scale-95 transition-transform"
            onClick={() => navigate(action.path)}
          >
            {action.label}
            <ArrowRight className="w-4 h-4" />
          </button>
        )}
      </div>
    </div>
  );
};
