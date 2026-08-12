import React, { useState, useEffect } from "react";
import { CapabilityUnavailablePage, PageLayout, IconButton } from "@sdkwork/im-h5-commons";
import { Filter, Search, Plus, FileText } from "lucide-react";
import { ApprovalService, ApprovalItem } from "../services/ApprovalService";
import { motion } from "motion/react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";
import { ApprovalHeader } from "../components/ApprovalHeader";
import { ApprovalTabs } from "../components/ApprovalTabs";
import { ApprovalItemCard } from "../components/ApprovalItemCard";

export const ApprovalApp = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  
  const [activeTab, setActiveTab] = useState("pending");
  const [approvals, setApprovals] = useState<ApprovalItem[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [unavailable, setUnavailable] = useState(false);

  useEffect(() => {
    setIsLoading(true);
    ApprovalService.getApprovals()
      .then((data) => {
        setApprovals(data);
      })
      .catch((error) => {
        console.error(error);
        setUnavailable(true);
      })
      .finally(() => {
        setIsLoading(false);
      });
  }, []);

  if (unavailable) {
    return (
      <CapabilityUnavailablePage
        icon={FileText}
        title={t("approval.title")}
        message={t("approval.unavailable")}
        onBack={() => navigate(-1)}
      />
    );
  }

  return (
    <PageLayout title={t('approval.title')}>
      <div className="flex flex-col h-full bg-bg-color">
        {/* Header Stats */}
        <ApprovalHeader pendingCount={0} initiatedCount={0} ccCount={0} />

        <div className="flex-1 overflow-y-auto px-4 -mt-6">
          <ApprovalTabs activeTab={activeTab} setActiveTab={setActiveTab} />

          <div className="flex justify-between items-center mb-3 px-1">
            <h2 className="text-[14px] font-medium text-text-sub">
              {t('approval.allRecords')} ({approvals.length})
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
                 <span className="text-[14px]">{t('approval.loading')}</span>
              </div>
            ) : approvals.length > 0 ? (
              approvals.map((approval) => (
                <ApprovalItemCard
                  key={approval.id}
                  approval={approval}
                  onClick={() => navigate(`/workspace/approval/${approval.id}`)}
                />
              ))
            ) : (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <FileText className="w-12 h-12 mb-3 stroke-current opacity-40" />
                <span className="text-[14px]">{t('approval.empty')}</span>
              </div>
            )}
          </div>
        </div>

        <motion.button
          whileTap={{ scale: 0.9 }}
          whileHover={{ scale: 1.05 }}
          onClick={() => navigate("/workspace/approval/create")}
          className="absolute bottom-6 right-6 w-14 h-14 bg-gradient-to-tr from-blue-600 to-primary-blue text-white rounded-full flex items-center justify-center shadow-lg shadow-blue-500/30 z-10"
        >
          <Plus className="w-7 h-7" />
        </motion.button>
      </div>
    </PageLayout>
  );
};
