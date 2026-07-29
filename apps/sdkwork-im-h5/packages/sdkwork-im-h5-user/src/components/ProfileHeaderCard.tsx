import { ChevronRight, QrCode } from "lucide-react";

import { Avatar } from "@sdkwork/im-h5-commons";
import type { User } from "@sdkwork/im-h5-types";

interface ProfileHeaderCardProps {
  currentUser: User | null;
  onClick: () => void;
}

export function ProfileHeaderCard({ currentUser, onClick }: ProfileHeaderCardProps) {
  return (
    <button
      className="mb-2 flex w-full items-center justify-between border-0 border-b border-border-color bg-chat-other-bg px-4 py-8 text-left transition-colors active:bg-active-bg"
      onClick={onClick}
      type="button"
    >
      <span className="flex min-w-0 flex-1 items-center gap-4">
        <Avatar
          className="h-[68px] w-[68px] shrink-0 rounded-[18px]"
          size="lg"
          src={currentUser?.avatar ?? ""}
        />
        <span className="flex min-w-0 flex-1 flex-col justify-center">
          <span className="mb-1.5 truncate text-[20px] font-bold text-text-main">
            {currentUser?.name ?? ""}
          </span>
          <span className="truncate text-[14px] text-text-sub">{currentUser?.id ?? ""}</span>
        </span>
      </span>
      <span className="flex items-center gap-3 text-text-sub">
        <QrCode aria-hidden="true" className="h-5 w-5" />
        <ChevronRight aria-hidden="true" className="h-5 w-5 opacity-40" />
      </span>
    </button>
  );
}
