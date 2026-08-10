import React from "react";
import {
  PageLayout,
  showToast,
} from "@sdkwork/im-h5-commons";
import { Download, ChevronRight, Check, X } from "lucide-react";
import { useParams, useNavigate } from "react-router";
import { RecruitmentService } from "../services/RecruitmentService";
import { useTranslation } from "react-i18next";
import { CandidateHeader } from "../components/CandidateHeader";
import { CandidateProgress } from "../components/CandidateProgress";
import { CandidateBaseInfo } from "../components/CandidateBaseInfo";

export const CandidateDetail = () => {
  const { t } = useTranslation();
  
const { id } = useParams();
  const navigate = useNavigate();
  
  const [candidate, setCandidate] = React.useState<any>(null);

  React.useEffect(() => {
    RecruitmentService.getCandidates().then((data) => {
      setCandidate(data.find((c) => c.id === id) || data[0]);
    });
  }, [id]);

  if (!candidate)
    return (
      <PageLayout title={t('recruitment.detail.title')}>
        <div className="flex flex-col h-full bg-bg-color items-center justify-center text-text-sub opacity-70">
          <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
          <span className="text-[14px]">{t('recruitment.loading')}</span>
        </div>
      </PageLayout>
    );

  return (
    <PageLayout title={t('recruitment.detail.title')}>
      <div className="p-4">
        {/* Header */}
        <CandidateHeader candidate={candidate} />

        {/* Process */}
        <CandidateProgress candidate={candidate} />

        {/* Base Info */}
        <CandidateBaseInfo />

        {/* Resume */}
        <div
          className="bg-chat-other-bg rounded-xl p-4 mb-6 shadow-sm border border-border-color/30 flex justify-between items-center active:scale-95 transition-transform"
          onClick={() => showToast(t('recruitment.detail.downloadStart'))}
        >
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 bg-primary-blue/10 rounded-lg flex items-center justify-center">
              <Download className="w-5 h-5 text-primary-blue" />
            </div>
            <div>
              <div className="text-[15px] font-medium text-text-main">
                {t('recruitment.detail.viewResume')}
              </div>
              <div className="text-[12px] text-text-sub mt-0.5">
                PDF · 1.2MB
              </div>
            </div>
          </div>
          <ChevronRight className="w-4 h-4 text-text-sub" />
        </div>

        {/* Actions */}
        <div className="flex gap-3">
          <button
            className="flex-1 bg-chat-other-bg border border-border-color text-text-main py-3 rounded-lg font-medium active:bg-bg-color flex justify-center items-center gap-2"
            onClick={async () => {
              await RecruitmentService.updateCandidateStage(
                candidate.id,
                t('recruitment.detail.eliminateSuccess'),
              );
              showToast(t('recruitment.detail.eliminateSuccess'));
              navigate(-1);
            }}
          >
            <X className="w-4 h-4" /> {t('recruitment.detail.eliminate')}
          </button>
          <button
            className="flex-1 bg-primary-blue text-white py-3 rounded-lg font-medium active:opacity-90 flex justify-center items-center gap-2"
            onClick={async () => {
              await RecruitmentService.updateCandidateStage(
                candidate.id,
                t('recruitment.detail.advanceSuccess'),
              );
              showToast(t('recruitment.detail.advanceSuccess'));
              navigate(-1);
            }}
          >
            <Check className="w-4 h-4" /> {t('recruitment.detail.advance')}
          </button>
        </div>
      </div>
    </PageLayout>
  );
};
