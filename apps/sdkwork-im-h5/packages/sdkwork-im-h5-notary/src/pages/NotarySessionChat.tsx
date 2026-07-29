import React from "react";
import { ChevronLeft, MessageCircleOff } from "lucide-react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";

export const NotarySessionChat: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  return (
    <div className="flex h-full flex-col bg-bg-color">
      <header className="flex h-[56px] items-center border-b border-border-color px-2 pt-safe">
        <button
          type="button"
          className="flex h-10 w-10 items-center justify-center"
          onClick={() => navigate(-1)}
          aria-label={t("common.back", "Back")}
        >
          <ChevronLeft className="h-6 w-6" />
        </button>
        <h1 className="flex-1 pr-10 text-center text-[17px] font-semibold">
          {t("notary.chat.title", "Notary conversation")}
        </h1>
      </header>
      <main className="flex flex-1 flex-col items-center justify-center gap-3 p-8 text-center text-text-sub">
        <MessageCircleOff className="h-10 w-10" />
        <p className="text-[14px]">
          {t(
            "notary.chat.unavailable",
            "A notary conversation is unavailable until the owner API returns an authorized IM conversation ID.",
          )}
        </p>
      </main>
    </div>
  );
};
