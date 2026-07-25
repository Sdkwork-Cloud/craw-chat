import React from "react";
import { useNavigate } from "react-router";
import {
  ChevronLeft,
  Info,
  FileText,
  UploadCloud,
  CheckCircle2,
} from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";
import { NotaryActionCard } from "../components/NotaryActionCard";

export const WorkspaceNotary: React.FC = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  

  return (
    <div className="flex flex-col h-full bg-bg-color">
      {/* Header */}
      <header className="h-[44px] flex items-center justify-between glass-header sticky top-0 z-10 shrink-0 pt-safe px-1 relative">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={
              <ChevronLeft
                className="w-6 h-6 text-text-main"
                strokeWidth={2.5}
              />
            }
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h1 className="text-[17px] font-semibold text-text-main">{t('notary.title')}</h1>
        </div>
        <div className="flex items-center justify-end z-10 flex-1 pr-1">
          <IconButton icon={<Info className="w-5 h-5 text-text-main" />} />
        </div>
      </header>

      {/* Content */}
      <div className="flex-1 overflow-y-auto px-4 pt-4 pb-12 flex flex-col gap-4">
        <div className="rounded-2xl p-5 bg-gradient-to-br from-blue-500 to-indigo-600 text-white shadow-md relative overflow-hidden">
          <div className="relative z-10">
            <h2 className="text-[20px] font-bold mb-1">{t('notary.online_service')}</h2>
            <p className="text-[14px] text-white/80">
              {t('notary.online_service_desc')}
            </p>
          </div>
          <div className="absolute right-[-20px] top-[-20px] w-32 h-32 bg-white/10 rounded-full blur-2xl" />
        </div>

        <h2 className="text-[15px] font-bold text-text-main mt-4 px-1">
          {t('notary.handle_business')}
        </h2>
        <div className="flex flex-col gap-3">
          <NotaryActionCard
            icon={FileText}
            title={t('notary.contract_review')}
            desc={t('notary.contract_review_desc')}
            color="text-blue-500"
          />
          <NotaryActionCard
            icon={UploadCloud}
            title={t('notary.evidence_deposit')}
            desc={t('notary.evidence_deposit_desc')}
            color="text-indigo-500"
          />
          <NotaryActionCard
            icon={CheckCircle2}
            title={t('notary.qualification_audit')}
            desc={t('notary.qualification_audit_desc')}
            color="text-green-500"
          />
        </div>
      </div>
    </div>
  );
};
