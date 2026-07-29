import React from "react";
import { ChevronDown, MoreVertical } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";

interface PlayerHeaderProps {
  title: string;
  onBack: () => void;
  onMoreClick?: () => void;
}

export const PlayerHeader: React.FC<PlayerHeaderProps> = ({
  title,
  onBack,
  onMoreClick,
}) => {
  return (
    <header className="h-[56px] flex items-center justify-between px-4 pt-safe shrink-0 relative z-10">
      <IconButton
        icon={<ChevronDown className="w-8 h-8 text-white" />}
        onClick={onBack}
      />
      <div className="flex flex-col items-center">
        <span className="text-[12px] opacity-70">正在播放</span>
        <span className="text-[14px] font-medium max-w-[200px] truncate">{title}</span>
      </div>
      {onMoreClick ? (
        <IconButton
          icon={<MoreVertical className="w-6 h-6 text-white" />}
          onClick={onMoreClick}
        />
      ) : (
        <span className="w-10" aria-hidden="true" />
      )}
    </header>
  );
};
