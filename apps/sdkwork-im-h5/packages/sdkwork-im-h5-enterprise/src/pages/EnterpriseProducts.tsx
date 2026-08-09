import { useTranslation } from "react-i18next";
import React from "react";
import { PageLayout } from "@sdkwork/im-h5-commons";
import { LayoutTemplate, Bot, Fingerprint } from "lucide-react";

export const EnterpriseProducts = () => {
  const { t } = useTranslation();
return (
    <PageLayout title={t('enterprise.auto_prop_7c135a88', '产品与服务')}>
      <div className="flex flex-col min-h-full bg-[#f5f6f8] dark:bg-[#1a1b1c] p-4 gap-4">
         {[
           {
             name: "业务大模型定制",
             desc: "结合行业私有数据，训练定制化语料模型，全面提升企业决策分析效率与智能化水平。",
             icon: <Bot className="w-8 h-8 text-blue-500" />,
             tags: ["AI赋能", "专属定制", "私有化"],
           },
           {
             name: "数字化管理中台",
             desc: "一站式集成的员工考勤、审批、报表及内部通讯中台系统。",
             icon: <LayoutTemplate className="w-8 h-8 text-indigo-500" />,
             tags: ["协同办公", "管理提效"],
           },
           {
             name: "智能安防方案",
             desc: "基于深度学习的视觉门禁与访客打卡识别体系，保障企业环境安全。",
             icon: <Fingerprint className="w-8 h-8 text-green-500" />,
             tags: ["安防监控", "物联网"],
           }
         ].map((prod, i) => (
           <div key={i} className="bg-white dark:bg-[#2c2d2e] rounded-xl p-5 shadow-sm flex flex-col gap-3">
              <div className="flex items-center gap-3">
                 <div className="w-12 h-12 bg-black/5 dark:bg-white/5 rounded-xl flex items-center justify-center shrink-0">
                    {prod.icon}
                 </div>
                 <div className="flex flex-col">
                    <h3 className="text-[16px] font-bold text-text-main">{prod.name}</h3>
                    <div className="flex gap-2 mt-1">
                      {prod.tags.map(t => (
                         <span key={t} className="text-[11px] px-1.5 py-0.5 rounded border border-blue-500/30 text-blue-500 bg-blue-500/5">
                           {t}
                         </span>
                      ))}
                    </div>
                 </div>
              </div>
              <p className="text-[14px] text-text-sub leading-relaxed border-t border-black/5 dark:border-white/5 pt-3 mt-1">
                {prod.desc}
              </p>
              <button className="w-full py-2 bg-blue-50 hover:bg-blue-100 dark:bg-blue-500/10 dark:hover:bg-blue-500/20 text-blue-600 font-medium rounded-lg text-[14px] transition-colors mt-2">{t('enterprise.auto_7a5cb7d2', '获取方案与报价')}</button>
           </div>
         ))}
      </div>
    </PageLayout>
  );
};
