import type { LucideIcon } from "lucide-react";
import { ChevronLeft } from "lucide-react";
import { useNavigate } from "react-router";

import { IconButton } from "@sdkwork/im-h5-commons";

export interface CapabilityUnavailablePageProps {
  icon: LucideIcon;
  message: string;
  title: string;
}

export function CapabilityUnavailablePage({
  icon: Icon,
  message,
  title,
}: CapabilityUnavailablePageProps) {
  const navigate = useNavigate();

  return (
    <div className="flex h-full flex-col bg-bg-color">
      <header className="flex h-[56px] shrink-0 items-center border-b border-border-color px-1 pt-safe">
        <IconButton
          icon={<ChevronLeft className="h-6 w-6 text-text-main" />}
          onClick={() => navigate(-1)}
        />
        <h2 className="flex-1 pr-10 text-center text-[17px] font-medium text-text-main">
          {title}
        </h2>
      </header>
      <div className="flex flex-1 flex-col items-center justify-center gap-3 px-8 text-center">
        <Icon className="h-10 w-10 text-text-sub" />
        <p className="max-w-sm text-[15px] text-text-main">{message}</p>
      </div>
    </div>
  );
}
