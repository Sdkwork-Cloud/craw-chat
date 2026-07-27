import React from "react";
import { ChevronRight } from "lucide-react";

interface ProfileListItemProps {
  label: React.ReactNode;
  rightText?: React.ReactNode;
  rightElement?: React.ReactNode;
  onClick?: () => void;
}

export const ProfileListItem: React.FC<ProfileListItemProps> = ({
  label,
  rightText,
  rightElement,
  onClick,
}) => (
  <div
    onClick={onClick}
    className="flex items-center justify-between px-4 py-3.5 bg-chat-other-bg active:bg-active-bg transition-colors cursor-pointer border-b border-border-color last:border-none"
  >
    <span className="text-[16px] text-text-main">{label}</span>
    <div className="flex items-center gap-2 text-text-sub">
      {rightText && <span className="text-[15px]">{rightText}</span>}
      {rightElement}
      <ChevronRight className="w-5 h-5 opacity-50" />
    </div>
  </div>
);
