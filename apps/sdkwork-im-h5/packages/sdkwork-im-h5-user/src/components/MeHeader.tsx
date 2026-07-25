import React from "react";
import { Contact } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";

interface MeHeaderProps {
  onContactClick: () => void;
}

export const MeHeader: React.FC<MeHeaderProps> = ({ onContactClick }) => {
  return (
    <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 shrink-0 pt-safe bg-[#f4f6f9]/90 dark:bg-[#0a0a0a]/90 backdrop-blur-xl">
      <div className="w-[32px]" />
      <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
        <h1 className="text-[17px] font-bold text-text-main tracking-tight">我</h1>
      </div>
      <div className="flex justify-end">
        <IconButton
          icon={<Contact className="w-[22px] h-[22px] text-text-main" />}
          onClick={onContactClick}
        />
      </div>
    </header>
  );
};
