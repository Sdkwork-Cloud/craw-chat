import React from "react";
import { BellOff, ChevronLeft } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { IconButton } from "@sdkwork/im-h5-commons";

export const NotaryMessages: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  return (
    <div className="flex h-full flex-col bg-bg-color">
      <header className="glass-header relative flex h-[56px] shrink-0 items-center px-1 pt-safe">
        <IconButton
          icon={<ChevronLeft className="h-6 w-6 text-text-main" />}
          onClick={() => navigate(-1)}
        />
        <h1 className="pointer-events-none absolute left-1/2 -translate-x-1/2 text-[17px] font-medium text-text-main">
          {t("notary.messages.title")}
        </h1>
      </header>
      <main className="flex flex-1 flex-col items-center justify-center gap-3 px-8 text-center">
        <BellOff className="h-10 w-10 text-text-sub" />
        <p className="text-[14px] text-text-sub">
          {t(
            "notary.messages.unavailable",
            "Notary notifications are unavailable until the owner SDK exposes this capability.",
          )}
        </p>
      </main>
    </div>
  );
};
