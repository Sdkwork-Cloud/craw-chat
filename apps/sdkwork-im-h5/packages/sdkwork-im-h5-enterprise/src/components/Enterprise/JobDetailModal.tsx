import React from "react";
import { motion } from "motion/react";
import { X, Briefcase } from "lucide-react";
import { useTranslation } from "react-i18next";

export interface JobDetailModalProps {
  job: {
    title: string;
    salary: string;
    req: string;
    desc: string;
  } | null;
  onClose: () => void;
  onChat: () => void;
}

export const JobDetailModal: React.FC<JobDetailModalProps> = ({
  job,
  onClose,
  onChat,
}) => {
  const { t } = useTranslation();

  if (!job) return null;

  const reqParts = job.req.split(" | ");

  return (
    <div className="fixed inset-0 z-50 flex flex-col justify-end">
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        transition={{ duration: 0.2 }}
        className="absolute inset-0 bg-black/40 backdrop-blur-[2px]"
        onClick={onClose}
      />
      <motion.div
        initial={{ y: "100%" }}
        animate={{ y: 0 }}
        exit={{ y: "100%" }}
        transition={{ type: "spring", damping: 25, stiffness: 200 }}
        className="relative bg-bg-color rounded-t-2xl overflow-hidden pb-safe flex flex-col max-h-[85vh]"
      >
        <div className="flex items-center justify-between p-4 bg-chat-other-bg border-b border-border-color">
          <h3 className="text-[17px] font-bold text-text-main">
            {t("enterprise.auto_3b8d0de0", "职位详情")}
          </h3>
          <div
            className="w-8 h-8 rounded-full bg-bg-color flex items-center justify-center cursor-pointer"
            onClick={onClose}
          >
            <X className="w-5 h-5 text-text-sub" />
          </div>
        </div>
        <div className="overflow-y-auto p-5 bg-chat-other-bg">
          <div className="flex justify-between items-start mb-3">
            <h2 className="text-[22px] font-bold text-text-main leading-tight">
              {job.title}
            </h2>
            <span className="text-[18px] font-extrabold text-red-500 whitespace-nowrap ml-4">
              {job.salary}
            </span>
          </div>
          <div className="flex items-center gap-2 mb-6">
            {reqParts.map((part, idx) => (
              <span
                key={idx}
                className="px-2.5 py-1 bg-bg-color text-text-sub text-[13px] font-medium rounded-sm border border-border-color/50"
              >
                {part}
              </span>
            ))}
          </div>

          <div className="flex items-center gap-3 mb-6 p-4 rounded-xl bg-bg-color border border-border-color/30">
            <div className="w-12 h-12 rounded-full bg-gradient-to-br from-blue-100 to-indigo-100 dark:from-blue-900/40 dark:to-indigo-900/40 flex items-center justify-center shrink-0">
              <Briefcase className="w-6 h-6 text-primary-blue" />
            </div>
            <div>
              <h4 className="text-[15px] font-bold text-text-main">
                {t("enterprise.auto_n5277daa7", "招聘负责人")}
              </h4>
              <span className="text-[13px] text-text-sub">
                {t("enterprise.auto_51555a28", "极客科技宇宙 HR")}
              </span>
            </div>
          </div>

          <div className="mb-2">
            <h4 className="text-[16px] font-bold text-text-main mb-3 flex items-center gap-2">
              <span className="w-1 h-3.5 bg-primary-blue rounded-full"></span>
              {t("enterprise.auto_3b886242", "职位描述")}
            </h4>
            <p className="text-[15px] text-text-sub leading-relaxed whitespace-pre-wrap">
              {job.desc}
            </p>
          </div>
        </div>
        <div className="p-4 bg-chat-other-bg border-t border-border-color mt-2 shadow-[0_-4px_10px_rgba(0,0,0,0.02)]">
          <button
            className="w-full bg-gradient-to-r from-blue-500 to-indigo-600 text-white rounded-full py-3.5 text-[16px] font-bold shadow-lg shadow-blue-500/30 active:scale-[0.98] transition-transform"
            onClick={onChat}
          >
            {t("enterprise.auto_39188763", "立即沟通")}
          </button>
        </div>
      </motion.div>
    </div>
  );
};
