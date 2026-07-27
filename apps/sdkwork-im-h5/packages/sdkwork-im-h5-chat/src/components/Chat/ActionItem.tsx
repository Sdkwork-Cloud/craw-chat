import React from "react";

interface ActionItemProps {
  icon: React.FC<any>;
  label: string;
  onClick: () => void;
}

export const ActionItem: React.FC<ActionItemProps> = ({
  icon: Icon,
  label,
  onClick,
}) => {
  return (
    <div className="flex flex-col items-center gap-2" onClick={onClick}>
      <div className="w-14 h-14 bg-bg-color rounded-2xl flex items-center justify-center shadow-sm active:bg-chat-active-bg transition-colors cursor-pointer border border-border-color">
        <Icon className="w-6 h-6 text-text-sub" />
      </div>
      <span className="text-[12px] text-text-sub">{label}</span>
    </div>
  );
};
