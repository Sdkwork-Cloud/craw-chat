import { useTranslation } from "react-i18next";
import React, { useState } from "react";
import { PageLayout, IconButton, cn } from "@sdkwork/im-h5-commons";
import { Search, Plus } from "lucide-react";
import { useNavigate } from "react-router";
import { motion } from "motion/react";
import { EnterpriseHeroBanner } from "../components/Enterprise/EnterpriseHeroBanner";
import { EnterpriseListTab } from "../components/Enterprise/EnterpriseListTab";
import { SupplyListTab } from "../components/Enterprise/SupplyListTab";
import { DemandListTab } from "../components/Enterprise/DemandListTab";
import { RecruitmentListTab } from "../components/Enterprise/RecruitmentListTab";

export const EnterpriseCenter = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState("enterprises"); // enterprises, supplies, demands, recruitments
  const [showMenu, setShowMenu] = useState(false);

  const ENTERPRISES = [
    { name: "极客科技宇宙有限公司", industry: "人工智能", location: "北京·海淀", tags: ["专精特新", "高新技术"], logo: "🚀", isAuth: true },
    { name: "云端餐饮集团", industry: "餐饮", location: "上海·浦东", tags: ["连锁品牌", "供应链"], logo: "🍔", isAuth: true },
    { name: "星河网络传媒", industry: "数字营销", location: "广州·天河", tags: ["广告传媒"], logo: "🌟", isAuth: false }
  ];

  const SUPPLIES = [
    { title: "大批量提供高端工业无人机", company: "极客科技宇宙有限公司", type: "工业设备", price: "面议" },
    { title: "优质大米供应商", company: "绿色农产品源头直供", type: "农副产品", price: "¥4500/吨" }
  ];

  const DEMANDS = [
    { title: "急寻企业级服务器供应商", company: "星云数据中心", type: "IT设备", budget: "100万-200万" },
    { title: "求购办公耗材一批", company: "城建集团第三分公司", type: "办公用品", budget: "¥5000" }
  ];

  const RECRUITMENTS = [
    { title: "高级前端开发", company: "极客科技宇宙", salary: "20k-40k", req: "3-5年 | 北京" },
    { title: "大客户销售", company: "星河网络传媒", salary: "10k-30k", req: "1-3年 | 广州" }
  ];

  const handleMenuClick = (action: string) => {
    setShowMenu(false);
    if (action === "invite") navigate("/enterprise/invite");
    if (action === "join") navigate("/enterprise/join");
    if (action === "job") navigate("/enterprise/post-job");
    if (action === "supply") navigate("/enterprise/post-supply");
    if (action === "demand") navigate("/enterprise/post-demand");
  };

  return (
    <PageLayout 
      title={t('enterprise.auto_prop_2518cc2f', '企业中心')}
      bgClass="bg-[#f5f6f8] dark:bg-[#1a1b1c]"
      rightElement={
        <div className="flex items-center gap-1 pl-2 relative z-[100]">
          <IconButton icon={<Search className="w-5 h-5 text-text-main" />} onClick={() => navigate("/enterprise/search")} />
          <IconButton icon={<Plus className="w-5 h-5 text-text-main" />} onClick={() => setShowMenu(!showMenu)} />
          
          {showMenu && (
            <>
              <div className="fixed inset-0 z-[90]" onClick={() => setShowMenu(false)} />
              <div className="absolute right-0 top-full mt-2 w-36 bg-chat-other-bg border border-border-color shadow-lg rounded-xl flex flex-col overflow-hidden z-[100]">
                {[
                  { label: "邀请入驻", action: "invite" },
                  { label: "申请入驻", action: "join" },
                  { label: "发布招聘", action: "job" },
                  { label: "发布供应", action: "supply" },
                  { label: "发布求购", action: "demand" }
                ].map((item, i) => (
                  <div 
                    key={i}
                    className="p-3 border-b last:border-b-0 border-border-color/50 text-[14px] text-text-main active:bg-chat-active-bg cursor-pointer"
                    onClick={() => handleMenuClick(item.action)}
                  >
                    {item.label}
                  </div>
                ))}
              </div>
            </>
          )}
        </div>
      }
    >
      <div className="flex flex-col min-h-full">
        {/* Hero Banner */}
        <EnterpriseHeroBanner />

        {/* Tabs */}
        <div className="bg-white dark:bg-[#2c2d2e] border-b border-border-color/50 sticky top-0 z-20 shadow-sm">
          <div className="flex w-full items-center px-2">
            {[
              { id: "enterprises", label: "找企业" },
              { id: "supplies", label: "找供应" },
              { id: "demands", label: "看求购" },
              { id: "recruitments", label: "找工作" }
            ].map(tab => (
              <div 
                key={tab.id}
                className="flex-1 flex flex-col items-center justify-center py-3 relative cursor-pointer group"
                onClick={() => setActiveTab(tab.id)}
              >
                <span className={cn(
                  "text-[14px] transition-colors duration-300",
                  activeTab === tab.id ? "text-primary-blue font-bold tracking-wide" : "text-text-sub font-medium group-hover:text-text-main"
                )}>
                  {tab.label}
                </span>
                {activeTab === tab.id && (
                  <motion.div 
                    layoutId="enterpriseCenterTab"
                    className="absolute bottom-0 w-6 h-[3px] bg-primary-blue rounded-t-full shadow-[0_-1px_4px_rgba(59,130,246,0.3)]"
                  />
                )}
              </div>
            ))}
          </div>
        </div>

        {/* Content */}
        <div className="flex flex-col bg-white dark:bg-[#2c2d2e]">
          {activeTab === "enterprises" && <EnterpriseListTab enterprises={ENTERPRISES} />}
          {activeTab === "supplies" && <SupplyListTab supplies={SUPPLIES} />}
          {activeTab === "demands" && <DemandListTab demands={DEMANDS} />}
          {activeTab === "recruitments" && <RecruitmentListTab recruitments={RECRUITMENTS} />}
        </div>
      </div>
    </PageLayout>
  );
};


