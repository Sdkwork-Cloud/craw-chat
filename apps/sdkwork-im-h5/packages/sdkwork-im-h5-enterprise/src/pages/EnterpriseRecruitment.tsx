import { useTranslation } from "react-i18next";
import React from "react";
import { PageLayout } from "@sdkwork/im-h5-commons";
import { MapPin, Briefcase } from "lucide-react";

export const EnterpriseRecruitment = () => {
  const { t } = useTranslation();
return (
    <PageLayout title={t('enterprise.auto_prop_251b6dd6', 'Jobs')}>
      <div className="flex flex-col min-h-full bg-bg-color p-4 gap-4">
        {[
          {
            title: "高级前端工程师",
            salary: "20k-40k",
            req: "3-5年 | 本科 | React",
            location: "北京·海淀",
          },
          {
            title: "产品经理",
            salary: "15k-30k",
            req: "1-3年 | 本科 | AI方向",
            location: "北京·朝阳",
          },
          {
            title: "全栈开发架构师",
            salary: "35k-60k",
            req: "5-10年 | 硕士 | Node.js/Go",
            location: "深圳·南山",
          }
        ].map((job, i) => (
          <div key={i} className="bg-chat-other-bg rounded-xl p-4 shadow-sm flex flex-col gap-3 cursor-pointer active:scale-95 transition-transform">
             <div className="flex justify-between items-center">
                <h3 className="text-[16px] font-bold text-text-main">{job.title}</h3>
                <span className="text-red-500 font-bold text-[15px]">{job.salary}</span>
             </div>
             <div className="flex text-[13px] text-text-sub items-center gap-4">
                <span className="flex items-center"><Briefcase className="w-3.5 h-3.5 mr-1" /> {job.req}</span>
                <span className="flex items-center"><MapPin className="w-3.5 h-3.5 mr-1" /> {job.location}</span>
             </div>
             <div className="mt-2 pt-3 border-t border-black/5 dark:border-white/5 flex items-center justify-between">
                <div className="flex items-center gap-2">
                   <div className="w-6 h-6 bg-blue-100 dark:bg-blue-900/30 rounded-full flex items-center justify-center text-blue-500 text-[10px] font-bold">HR</div>
                   <span className="text-[12px] text-text-sub">{t('enterprise.auto_5252526b', 'Manager Zhang · Active just now')}</span>
                </div>
                <button className="px-3 py-1 bg-blue-500 text-white rounded-full text-[12px]">{t('enterprise.auto_2efc34e3', 'Apply')}</button>
             </div>
          </div>
        ))}
      </div>
    </PageLayout>
  );
};
