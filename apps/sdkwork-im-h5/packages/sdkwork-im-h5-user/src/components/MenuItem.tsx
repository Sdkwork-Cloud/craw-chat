import { useTranslation } from "react-i18next";
import React from "react";
import { ChevronRight } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";

interface MenuItemProps {
  icon: React.ElementType;
  label: string;
  onClick?: () => void;
  rightElement?: React.ReactNode;
  colorClass?: string;
  hideArrow?: boolean;
}

export const MenuItem: React.FC<MenuItemProps> = ({
  icon: Icon,
  label,
  onClick,
  rightElement,
  colorClass = "text-text-main",
  hideArrow = false,
}) => {
  const { t } = useTranslation();
return (
    <div
      onClick={onClick}
      className={cn(
        "flex items-center justify-between px-4 py-3.5 bg-chat-other-bg active:bg-active-bg transition-colors cursor-pointer",
        !onClick && "active:bg-chat-other-bg cursor-default"
      )}
    >
      <div className="flex items-center gap-3">
        <Icon className={cn("w-6 h-6", colorClass)} strokeWidth={2} />
        <span className="text-[16px] text-text-main">{label}</span>
      </div>
      <div className="flex items-center gap-2 text-text-sub">
        {rightElement}
        {!hideArrow && <ChevronRight className="w-5 h-5 opacity-40" />}
      </div>
    </div>
  );
};
