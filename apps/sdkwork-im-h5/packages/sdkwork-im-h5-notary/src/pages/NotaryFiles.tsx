import React from "react";
import { ChevronLeft, CloudOff } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { IconButton } from "@sdkwork/im-h5-commons";

export const NotaryFiles: React.FC = () => {
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
          {t("notary.files.title")}
        </h1>
      </header>
      <main className="flex flex-1 flex-col items-center justify-center gap-3 px-8 text-center">
        <CloudOff className="h-10 w-10 text-text-sub" />
        <p className="text-[14px] text-text-sub">
          {t(
            "notary.files.unavailable",
            "The Notary App SDK does not expose a general cloud file browser.",
          )}
        </p>
      </main>
    </div>
  );
};
