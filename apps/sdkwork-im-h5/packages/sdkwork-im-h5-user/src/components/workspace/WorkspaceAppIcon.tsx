import { useTranslation } from "react-i18next";
import React from "react";

export interface AppIconProps {
  icon: React.ElementType;
  label: string;
  colorClass: string;
  bgClass: string;
  onClick: () => void;
}

export const WorkspaceAppIcon: React.FC<AppIconProps> = ({
  icon: Icon,
  label,
  colorClass,
  bgClass,
  onClick,
}) => {
  const { t } = useTranslation();
  return (
  <div
    className="flex flex-col items-center gap-2 cursor-pointer group active:scale-95 transition-transform"
    onClick={onClick}
  >
    <div
      className={`w-[52px] h-[52px] rounded-[18px] flex items-center justify-center ${bgClass} group-hover:shadow-md transition-all relative overflow-hidden`}
    >
      <div className="absolute inset-0 bg-white/40 dark:bg-black/20 opacity-0 group-hover:opacity-100 transition-opacity mix-blend-overlay"></div>
      <Icon className={`w-7 h-7 relative z-10 ${colorClass}`} strokeWidth={2} />
    </div>
    <span className="text-[12px] text-text-main font-medium text-center leading-tight whitespace-pre-wrap px-1">
      {label}
    </span>
  </div>
);
};

