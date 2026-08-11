import { useTranslation } from "react-i18next";
import React from "react";
import { Briefcase } from "lucide-react";
import { motion } from "motion/react";

export const EnterpriseJobsTab = ({
  jobs,
  onSelectJob,
  openChat,
}: {
  jobs: any[];
  onSelectJob: (j: any) => void;
  openChat: () => void;
}) => {
  const { t } = useTranslation();
  return (
    <motion.div
      key="jobs"
      initial={{ opacity: 0, y: 5 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0 }}
      transition={{ duration: 0.2 }}
      className="bg-chat-other-bg flex flex-col"
    >
      {jobs.map((job, i) => (
        <div
          key={i}
          className="p-5 border-b border-border-color/50 flex flex-col gap-3 cursor-pointer active:bg-chat-active-bg transition-colors group"
          onClick={() => onSelectJob(job)}
        >
          <div className="flex justify-between items-start">
            <h3 className="text-[17px] font-bold text-text-main group-hover:text-primary-blue transition-colors">
              {job.title}
            </h3>
            <span className="text-[16px] font-extrabold text-red-500 whitespace-nowrap ml-4">
              {job.salary}
            </span>
          </div>
          <div className="flex items-center gap-2">
            <span className="px-2.5 py-1 bg-bg-color text-text-sub text-[12px] font-bold rounded-sm">
              {job.req.split(" | ")[0]}
            </span>
            <span className="px-2.5 py-1 bg-bg-color text-text-sub text-[12px] font-bold rounded-sm">
              {job.req.split(" | ")[1]}
            </span>
            <span className="px-2.5 py-1 bg-bg-color text-text-sub text-[12px] font-bold rounded-sm">
              {job.req.split(" | ")[2]}
            </span>
          </div>
          <div className="pt-3 border-t border-border-color/30 flex items-center justify-between mt-1">
            <div className="flex items-center gap-2">
              <div className="w-6 h-6 rounded-full bg-primary-blue/10 flex items-center justify-center shrink-0">
                <Briefcase className="w-3.5 h-3.5 text-primary-blue" />
              </div>
              <span className="text-[13px] font-medium text-text-main">{t('enterprise.auto_n5277daa7', 'Recruiting manager')}</span>
            </div>
            <button
              className="px-4 py-1.5 bg-primary-blue text-white rounded-full text-[13px] font-bold shadow-md shadow-blue-500/20"
              onClick={(e) => {
                e.stopPropagation();
                openChat();
              }}
            >{t('enterprise.auto_24da2ec', 'Chat now')}</button>
          </div>
        </div>
      ))}
    </motion.div>
  );
};
