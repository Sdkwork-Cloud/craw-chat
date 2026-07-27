import React from "react";
import { ArrowRight, Clock, CheckCircle2, XCircle } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";

const STATUS_MAP: Record<string, any> = {
  processing: {
    labelKey: "notary.records.status_processing",
    icon: Clock,
    color: "text-orange-500",
    bg: "bg-orange-500/10",
  },
  completed: {
    labelKey: "notary.records.status_completed",
    icon: CheckCircle2,
    color: "text-green-500",
    bg: "bg-green-500/10",
  },
  cancelled: {
    labelKey: "notary.records.status_cancelled",
    icon: XCircle,
    color: "text-gray-500",
    bg: "bg-gray-500/10",
  },
};

interface NotaryRecordListItemProps {
  record: any;
  isLast: boolean;
  onClick: () => void;
}

export const NotaryRecordListItem: React.FC<NotaryRecordListItemProps> = ({
  record,
  isLast,
  onClick,
}) => {
  const { t } = useTranslation();
  const statusInfo = STATUS_MAP[record.status] || STATUS_MAP["processing"];
  const Icon = statusInfo.icon;

  return (
    <div
      onClick={onClick}
      className={cn(
        "px-4 py-3.5 flex items-center gap-4 active:bg-active-bg transition-colors cursor-pointer",
        !isLast ? "border-b border-border-color/50" : ""
      )}
    >
      <div
        className={cn(
          "w-12 h-12 rounded-xl flex items-center justify-center shrink-0",
          statusInfo.bg
        )}
      >
        <Icon className={cn("w-6 h-6", statusInfo.color)} />
      </div>
      <div className="flex-1 min-w-0">
        <h3 className="text-[16px] font-bold text-text-main truncate">
          {record.title}
        </h3>
        <p className="text-[13px] text-text-sub mt-1">{record.date}</p>
      </div>
      <div className="flex items-center gap-1">
        <span className={cn("text-[13px] font-medium", statusInfo.color)}>
          {t(statusInfo.labelKey)}
        </span>
        <ArrowRight className="w-4 h-4 text-text-sub opacity-50" />
      </div>
    </div>
  );
};
