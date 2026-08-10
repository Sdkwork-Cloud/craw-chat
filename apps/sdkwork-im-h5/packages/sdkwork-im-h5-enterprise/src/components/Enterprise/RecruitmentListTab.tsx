import React from "react";
import { Briefcase, Building2 } from "lucide-react";
import { useTranslation } from "react-i18next";

export interface RecruitmentItem {
  title: string;
  company: string;
  salary: string;
  req: string;
}

export interface RecruitmentListTabProps {
  recruitments: RecruitmentItem[];
}

export const RecruitmentListTab: React.FC<RecruitmentListTabProps> = ({ recruitments }) => {
  const { t } = useTranslation();

  return (
    <>
      {recruitments.map((job, i) => (
        <div
          key={i}
          className="px-4 py-3 border-b border-border-color/50 flex flex-col gap-2 active:bg-chat-active-bg transition-colors cursor-pointer group hover:bg-hover-bg relative overflow-hidden"
        >
          <div className="absolute inset-0 bg-primary-blue/5 opacity-0 group-active:opacity-100 transition-opacity" />
          <div className="flex justify-between items-center">
            <h3 className="text-[16px] font-bold text-text-main group-hover:text-primary-blue transition-colors">
              {job.title}
            </h3>
            <span className="text-[15px] font-extrabold text-red-500">{job.salary}</span>
          </div>
          <div className="flex items-center gap-3 text-[12px] text-text-sub mt-0.5">
            <div className="flex items-center gap-1.5 bg-bg-color px-2 py-0.5 rounded-sm">
              <Briefcase className="w-3.5 h-3.5 opacity-70" />
              <span className="font-medium opacity-90">{job.req}</span>
            </div>
          </div>
          <div className="mt-1 pt-2 border-t border-border-color/30 flex items-center justify-between">
            <div className="flex items-center gap-2 opacity-90">
              <div className="w-5 h-5 rounded-full bg-slate-100 dark:bg-black/20 flex items-center justify-center">
                <Building2 className="w-3 h-3 text-text-sub" />
              </div>
              <span className="text-[12px] text-text-sub font-medium">{job.company}</span>
            </div>
            <button className="px-3 py-1 bg-primary-blue text-white rounded-full text-[12px] font-medium shadow-md shadow-blue-500/20 active:scale-95 transition-transform">
              {t('enterprise.auto_24da2ec', '马上聊')}
            </button>
          </div>
        </div>
      ))}
    </>
  );
};
