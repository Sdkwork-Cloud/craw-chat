import { useTranslation } from "react-i18next";
import React from "react";
import { PageLayout } from "@sdkwork/im-h5-commons";
import { Search, MapPin, Phone } from "lucide-react";

export const EnterpriseYellowPages = () => {
  const { t } = useTranslation();
return (
    <PageLayout title={t('enterprise.auto_prop_2522c72a', '企业黄页')}>
      <div className="flex flex-col min-h-full bg-[#f5f6f8] dark:bg-[#1a1b1c]">
        <div className="p-4 bg-white dark:bg-[#2c2d2e] sticky top-0 z-10">
          <div className="bg-black/5 dark:bg-white/5 rounded-full flex items-center px-4 py-2">
            <Search className="w-4 h-4 text-text-sub" />
            <input 
              className="bg-transparent border-none outline-none ml-2 text-[14px] flex-1 text-text-main"
              placeholder={t('enterprise.auto_prop_545dbc2e', '搜索企业、产品或服务')}
            />
          </div>
        </div>
        <div className="p-4 flex flex-col gap-3">
          {[
            {
              name: "大宇智能科技有限公司",
              industry: "人工智能AI",
              address: "高新区科创园A座",
              phone: "010-88888888",
            },
            {
              name: "星河网络传媒",
              industry: "数字营销",
              address: "星河中心大厦22楼",
              phone: "010-66666666",
            },
            {
              name: "绿色农产品直供",
              industry: "现代农业",
              address: "市郊生态农业合作社",
              phone: "010-55555555",
            }
          ].map((item, i) => (
            <div key={i} className="bg-white dark:bg-[#2c2d2e] rounded-xl p-4 shadow-sm flex flex-col cursor-pointer active:scale-95 transition-transform">
              <div className="flex justify-between items-start mb-2">
                 <h3 className="text-[16px] font-bold text-text-main">{item.name}</h3>
                 <span className="text-[11px] text-blue-500 bg-blue-500/10 px-2 py-0.5 rounded">{item.industry}</span>
              </div>
              <div className="flex flex-col gap-2 mt-2">
                 <div className="flex items-center text-text-sub text-[13px]">
                   <MapPin className="w-3.5 h-3.5 mr-1.5 opacity-70" />
                   {item.address}
                 </div>
                 <div className="flex items-center text-text-sub text-[13px]">
                   <Phone className="w-3.5 h-3.5 mr-1.5 opacity-70" />
                   {item.phone}
                 </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </PageLayout>
  );
};
