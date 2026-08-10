import React, { useState, useEffect } from "react";
import {
  PageLayout,
  IconButton,
} from "@sdkwork/im-h5-commons";
import { Search, Filter, Plus, Briefcase } from "lucide-react";
import { RecruitmentService, CandidateRecord } from "../services/RecruitmentService";
import { motion } from "motion/react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";
import { RecruitmentHeader } from "../components/RecruitmentHeader";
import { CandidateItemCard } from "../components/CandidateItemCard";

export const RecruitmentApp = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  
  const [candidates, setCandidates] = useState<CandidateRecord[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    setIsLoading(true);
    RecruitmentService.getCandidates().then((data) => {
      setCandidates(data);
      setIsLoading(false);
    });
  }, []);

  return (
    <PageLayout title={t('recruitment.title')}>
      <div className="flex flex-col h-full bg-bg-color">
        {/* Header Stats */}
        <RecruitmentHeader ongoingCount={12} interviewCount={2} reviewCount={5} />

        <div className="flex-1 overflow-y-auto px-4 -mt-6">
          <div className="flex justify-between items-center mb-3 mt-4 px-1">
            <h2 className="text-[14px] font-medium text-text-sub">
              {t('recruitment.listTitle')} ({candidates.length})
            </h2>
            <div className="flex gap-2">
              <IconButton
                icon={<Filter className="w-4 h-4 text-text-sub" />}
                className="bg-chat-other-bg p-1.5 w-auto h-auto rounded-md shadow-sm"
              />
              <IconButton
                icon={<Search className="w-4 h-4 text-text-sub" />}
                className="bg-chat-other-bg p-1.5 w-auto h-auto rounded-md shadow-sm"
              />
            </div>
          </div>

          <div className="flex flex-col gap-3 pb-20">
            {isLoading ? (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-white animate-spin mb-3"></div>
                <span className="text-[14px]">{t('recruitment.loading')}</span>
              </div>
            ) : candidates.length > 0 ? (
              candidates.map((candidate) => (
                <CandidateItemCard key={candidate.id} candidate={candidate} />
              ))
            ) : (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <Briefcase className="w-12 h-12 mb-3 stroke-current opacity-40" />
                <span className="text-[14px]">{t('recruitment.empty')}</span>
              </div>
            )}
          </div>
        </div>

        <motion.button
          whileTap={{ scale: 0.9 }}
          whileHover={{ scale: 1.05 }}
          onClick={() => navigate("/workspace/recruitment/create")}
          className="absolute bottom-6 right-6 w-14 h-14 bg-gradient-to-tr from-blue-600 to-primary-blue text-white rounded-full flex items-center justify-center shadow-lg shadow-blue-500/30 z-10"
        >
          <Plus className="w-7 h-7" />
        </motion.button>
      </div>
    </PageLayout>
  );
};
