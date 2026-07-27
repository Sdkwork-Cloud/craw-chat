import React from "react";
import { ChevronLeft, Plus, Search } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";

interface CalendarHeaderProps {
  year: number;
  month: number;
  onBack: () => void;
  onSearch?: () => void;
  onAdd: () => void;
}

export const CalendarHeader: React.FC<CalendarHeaderProps> = ({
  year,
  month,
  onBack,
  onSearch,
  onAdd,
}) => {
  return (
    <header className="h-[44px] flex items-center justify-between sticky top-0 shrink-0 pt-safe px-2 z-20 bg-bg-color border-b border-border-color">
      <div className="flex items-center z-10 w-[80px]">
        <IconButton
          icon={
            <ChevronLeft className="w-7 h-7 text-text-main" strokeWidth={2} />
          }
          onClick={onBack}
        />
      </div>
      <div className="flex items-center justify-center font-medium text-[17px] pointer-events-none flex-1 gap-2">
        <span className="cursor-pointer pointer-events-auto">{`${year}年${month + 1}月`}</span>
      </div>
      <div className="flex justify-end z-10 w-[80px] pr-2 gap-2">
        <IconButton
          icon={<Search className="w-5 h-5 text-text-main" />}
          onClick={onSearch}
        />
        <IconButton
          icon={<Plus className="w-6 h-6 text-text-main" />}
          onClick={onAdd}
        />
      </div>
    </header>
  );
};
