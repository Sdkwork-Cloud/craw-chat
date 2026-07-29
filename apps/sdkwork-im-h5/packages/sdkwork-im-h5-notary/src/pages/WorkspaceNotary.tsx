import React from "react";
import { ChevronLeft, FilePlus2, FileStack } from "lucide-react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";

export const WorkspaceNotary: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  return (
    <div className="flex h-full flex-col bg-bg-color">
      <header className="flex h-[56px] shrink-0 items-center border-b border-border-color px-2 pt-safe">
        <button
          type="button"
          className="flex h-10 w-10 items-center justify-center"
          onClick={() => navigate("/")}
          aria-label={t("common.back", "Back")}
        >
          <ChevronLeft className="h-6 w-6" />
        </button>
        <h1 className="flex-1 pr-10 text-center text-[17px] font-semibold">
          {t("notary.title", "Notary")}
        </h1>
      </header>
      <main className="flex flex-1 flex-col gap-3 p-4">
        <button
          type="button"
          className="flex items-center gap-4 border-b border-border-color px-2 py-4 text-left"
          onClick={() => navigate("/notary")}
        >
          <FileStack className="h-7 w-7 text-primary-blue" />
          <span>
            <span className="block text-[16px] font-semibold text-text-main">
              {t("notary.records.title", "Notary cases")}
            </span>
            <span className="block text-[13px] text-text-sub">
              {t("notary.workspace.records_desc", "Review current case status and materials")}
            </span>
          </span>
        </button>
        <button
          type="button"
          className="flex items-center gap-4 border-b border-border-color px-2 py-4 text-left"
          onClick={() => navigate("/notary/create")}
        >
          <FilePlus2 className="h-7 w-7 text-primary-blue" />
          <span>
            <span className="block text-[16px] font-semibold text-text-main">
              {t("notary.create_steps.title", "Create notary case")}
            </span>
            <span className="block text-[13px] text-text-sub">
              {t("notary.workspace.create_desc", "Submit a case through the authorized Notary service")}
            </span>
          </span>
        </button>
      </main>
    </div>
  );
};
