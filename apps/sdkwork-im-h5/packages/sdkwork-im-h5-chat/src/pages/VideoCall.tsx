import React from "react";
import { ChevronLeft, PhoneOff } from "lucide-react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";

export const VideoCall: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <div className="flex h-full flex-col bg-bg-color text-text-main">
      <header className="flex h-[56px] shrink-0 items-center border-b border-border-color px-2 pt-safe">
        <button
          type="button"
          className="flex h-10 w-10 items-center justify-center"
          aria-label={t("common.back", "Back")}
          onClick={() => navigate(-1)}
        >
          <ChevronLeft className="h-6 w-6" />
        </button>
      </header>
      <main className="flex flex-1 flex-col items-center justify-center gap-4 px-8 text-center">
        <PhoneOff className="h-12 w-12 text-text-sub" aria-hidden="true" />
        <h1 className="text-[20px] font-semibold">
          {t("chat.call.unavailable_title", "Calls unavailable")}
        </h1>
        <p className="max-w-sm text-[14px] text-text-sub">
          {t(
            "chat.call.unavailable_description",
            "Calling is not available in this client.",
          )}
        </p>
        <button
          type="button"
          className="mt-2 text-[15px] font-medium text-primary-blue"
          onClick={() => navigate(-1)}
        >
          {t("common.back", "Back")}
        </button>
      </main>
    </div>
  );
};
