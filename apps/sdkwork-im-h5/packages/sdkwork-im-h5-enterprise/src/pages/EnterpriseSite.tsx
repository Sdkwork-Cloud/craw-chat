import { useTranslation } from "react-i18next";
import React, { useState } from "react";
import { showToast, PageLayout, ListItem } from "@sdkwork/im-h5-commons";
import { Building2, Globe, Flag, Users, MapPin, Share2, CheckCircle2, ChevronRight, Phone, MessageSquare, Package, Briefcase, X } from "lucide-react";
import { motion, AnimatePresence } from "motion/react";
import { useNavigate } from "react-router";
import { EnterpriseAboutTab } from "../components/Enterprise/EnterpriseAboutTab";
import { EnterpriseProductsTab } from "../components/Enterprise/EnterpriseProductsTab";
import { EnterpriseJobsTab } from "../components/Enterprise/EnterpriseJobsTab";

export const EnterpriseSite = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState("about"); // about, products, jobs
  const [selectedProduct, setSelectedProduct] = useState<any>(null);
  const [selectedJob, setSelectedJob] = useState<any>(null);

  const openChat = () => {
  navigate("/chat/geek_corp_123");
  };

  const PRODUCTS = [
    { name: "极客科技宇宙企业版大模型", desc: "私有化部署的安全可靠大模型解决方案，支持百亿级参数训练与微调，帮助企业快速构建垂直领域智能助理。提供全套API对接及长达一年的技术支持。", price: "面议" },
    { name: "智能客服系统引擎", desc: "全渠道接入的下一代客户沟通平台。融合语义理解、多轮对话技术，实现7x24小时无人值守自动回复，并能平滑转交人工客服。支持微信、App、网站等多端接入。", price: "¥9,999/年" }
  ];

  const JOBS = [
    { title: "高级前端开发工程师", salary: "20k-40k", req: "3-5年 | 本科 | 北京", desc: "岗位职责：\n1. 负责公司核心中后台及移动端H5产品的研发；\n2. 参与前端工程化体系建设及性能优化；\n\n任职要求：\n1. 熟练掌握React、Vue等主流框架，熟悉TypeScript；\n2. 具备良好的复杂系统架构设计能力。" },
    { title: "资深产品经理 (AI方向)", salary: "25k-45k", req: "5-10年 | 本科 | 北京", desc: "岗位职责：\n1. 负责AI相关商业产品的规划、落地及商业化变现；\n2. 协同技术团队，探索大语言模型在B端场景的应用；\n\n任职要求：\n1. 具备5年以上SaaS或AI产品经验；\n2. 强烈的自驱力与业务Sense。" }
  ];

  return (
    <PageLayout
      title={t('enterprise.auto_prop_n1c94083e', '极客科技宇宙')}
      bgClass="bg-[#f5f6f8] dark:bg-[#1a1b1c]"
      rightElement={
        <div className="flex items-center pr-2">
          <div 
            className="w-8 h-8 flex items-center justify-center rounded-full cursor-pointer transition-colors active:bg-black/5 dark:active:bg-white/5 text-text-main" 
            onClick={() => showToast(t('enterprise.auto_fn_52609618', '已准备分享'))}
          >
            <Share2 className="w-5 h-5" />
          </div>
        </div>
      }
    >
      <div className="flex flex-col min-h-full pb-[80px]">
        {/* Header Hero */}
        <div className="w-full h-[160px] bg-gradient-to-r from-slate-800 to-slate-900 dark:from-slate-900 dark:to-black relative overflow-hidden shrink-0">
          <div className="absolute inset-0 bg-[url('https://api.dicom.cn/1')] opacity-30 object-cover mix-blend-overlay" />
          <div className="absolute inset-x-0 bottom-0 h-32 bg-gradient-to-t from-black/80 to-transparent" />
        </div>

        {/* Company Profile Area */}
        <div className="bg-white dark:bg-[#2c2d2e] px-4 pb-5 rounded-b-3xl shadow-sm relative z-10">
          <div className="flex justify-between items-end -mt-10 relative z-20 mb-3">
            <div className="w-20 h-20 bg-white dark:bg-[#2c2d2e] rounded-2xl flex items-center justify-center p-1 shadow-md border border-border-color/50">
              <div className="w-full h-full bg-blue-50 dark:bg-blue-900/30 rounded-xl flex items-center justify-center">
                <Building2 className="w-10 h-10 text-blue-600 dark:text-blue-400" />
              </div>
            </div>
            <div className="flex gap-2">
              <button 
                className="px-5 py-1.5 border-2 border-primary-blue text-primary-blue rounded-full text-[13px] font-bold active:bg-primary-blue/5 transition-colors"
                onClick={() => showToast(t('enterprise.auto_fn_52851cc0', '已关注企业'))}
              >{t('enterprise.auto_1e4dea', '+ 关注')}</button>
            </div>
          </div>

          <div className="flex items-center gap-2 mb-1">
            <h1 className="text-[24px] font-extrabold text-text-main leading-none">{t('enterprise.auto_n1c94083e', '极客科技宇宙')}</h1>
            <CheckCircle2 className="w-5 h-5 text-green-500 shrink-0" />
          </div>
          
          <div className="flex flex-wrap items-center gap-2 mt-3">
            <span className="px-2.5 py-1 rounded-sm bg-[#f5f6f8] dark:bg-[#1a1b1c] text-text-sub text-[11px] font-bold border border-border-color/50">{t('enterprise.auto_2536e16e', '人工智能')}</span>
            <span className="px-2.5 py-1 rounded-sm bg-[#f5f6f8] dark:bg-[#1a1b1c] text-text-sub text-[11px] font-bold border border-border-color/50">{t('enterprise.auto_2303e00', '高新技术企业')}</span>
            <span className="px-2.5 py-1 rounded-sm bg-[#f5f6f8] dark:bg-[#1a1b1c] text-text-sub text-[11px] font-bold border border-border-color/50">{t('enterprise.auto_4ddd94ae', '专精特新重点')}</span>
          </div>

          <div className="flex items-center gap-5 mt-5 text-[13px] font-medium text-text-main">
            <div className="flex items-center gap-1.5 bg-[#f5f6f8] dark:bg-[#1a1b1c] px-3 py-1.5 rounded-full">
              <Users className="w-4 h-4 text-text-sub" />
              <span>{t('enterprise.auto_6721ef63', '500-1000人')}</span>
            </div>
            <div 
              className="flex items-center gap-1.5 bg-[#f5f6f8] dark:bg-[#1a1b1c] px-3 py-1.5 rounded-full cursor-pointer active:opacity-70"
              onClick={() => showToast(t('enterprise.auto_fn_39cd0c2b', '正在打开官网'))}
            >
              <Globe className="w-4 h-4 text-text-sub" />
              <span className="text-primary-blue">geekcosmo.com</span>
            </div>
          </div>
        </div>

        {/* Tabs */}
        <div className="bg-white dark:bg-[#2c2d2e] mt-2 border-b border-border-color/50 sticky top-0 z-30 shadow-sm">
          <div className="flex w-full items-center px-4">
            {[
              { id: "about", label: "企业简介" },
              { id: "products", label: "产品服务" },
              { id: "jobs", label: "在招职位" }
            ].map(tab => (
              <div 
                key={tab.id}
                className="mr-8 flex flex-col items-center justify-center py-3.5 relative cursor-pointer group"
                onClick={() => setActiveTab(tab.id)}
              >
                <span className={`text-[15px] transition-colors duration-300 ${activeTab === tab.id ? "text-primary-blue font-bold tracking-wide" : "text-text-sub font-medium group-hover:text-text-main"}`}>
                  {tab.label}
                </span>
                {activeTab === tab.id && (
                  <motion.div 
                    layoutId="enterpriseSiteTab"
                    className="absolute bottom-0 w-8 h-[3px] bg-primary-blue rounded-t-full shadow-[0_-1px_4px_rgba(59,130,246,0.3)]"
                  />
                )}
              </div>
            ))}
          </div>
        </div>

        {/* Content Area */}
        <div className="flex-1 mt-2">
          <AnimatePresence mode="wait">
            {activeTab === "about" && <EnterpriseAboutTab />}

            {activeTab === "products" && (
              <EnterpriseProductsTab
                products={PRODUCTS}
                onSelectProduct={setSelectedProduct}
              />
            )}

            {activeTab === "jobs" && (
              <EnterpriseJobsTab jobs={JOBS} onSelectJob={setSelectedJob} openChat={openChat} />
            )}
          </AnimatePresence>
        </div>
      </div>

      {/* Bottom Action Bar */}
      <div className="fixed bottom-0 left-0 right-0 h-[68px] bg-white dark:bg-[#2c2d2e] border-t border-border-color flex items-center px-4 gap-4 z-40 pb-safe shadow-[0_-4px_10px_rgba(0,0,0,0.02)]">
        <div className="flex items-center justify-center flex-col shrink-0 px-2 cursor-pointer active:opacity-70 text-text-main" onClick={() => navigate("/enterprise/invite")}>
          <Share2 className="w-5 h-5 mb-1" />
          <span className="text-[11px] font-medium">{t('enterprise.auto_a3d65', '分享')}</span>
        </div>
        <div className="flex-1 flex gap-3 h-11">
          <button 
            className="flex-1 rounded-full bg-[#f0f9ff] dark:bg-blue-900/20 text-primary-blue font-extrabold text-[15px] flex items-center justify-center gap-1.5 active:scale-95 transition-transform border border-blue-100 dark:border-blue-800/50"
            onClick={() => window.location.href = "tel:01088889999"}
          >
            <Phone className="w-4.5 h-4.5" />{t('enterprise.auto_375f5782', '电话咨询')}</button>
          <button 
            className="flex-[1.2] rounded-full bg-gradient-to-r from-blue-500 to-indigo-600 text-white font-extrabold text-[15px] flex items-center justify-center gap-1.5 shadow-lg shadow-blue-500/30 active:scale-95 transition-transform"
            onClick={openChat}
          >
            <MessageSquare className="w-4.5 h-4.5" />{t('enterprise.auto_29850f11', '在线咨询')}</button>
        </div>
      </div>

      <AnimatePresence>
        {selectedProduct && (
          <div className="fixed inset-0 z-50 flex flex-col justify-end">
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              transition={{ duration: 0.2 }}
              className="absolute inset-0 bg-black/40 backdrop-blur-[2px]"
              onClick={() => setSelectedProduct(null)}
            />
            <motion.div
              initial={{ y: "100%" }}
              animate={{ y: 0 }}
              exit={{ y: "100%" }}
              transition={{ type: "spring", damping: 25, stiffness: 200 }}
              className="relative bg-[#f5f6f8] dark:bg-[#1a1b1c] rounded-t-2xl overflow-hidden pb-safe flex flex-col max-h-[85vh]"
            >
              <div className="flex items-center justify-between p-4 bg-white dark:bg-[#2c2d2e]">
                <h3 className="text-[17px] font-bold text-text-main">{t('enterprise.auto_25105619', '产品详情')}</h3>
                <div className="w-8 h-8 rounded-full bg-[#f5f6f8] dark:bg-[#1a1b1c] flex items-center justify-center cursor-pointer" onClick={() => setSelectedProduct(null)}>
                  <X className="w-5 h-5 text-text-sub" />
                </div>
              </div>
              <div className="overflow-y-auto p-5">
                <div className="w-24 h-24 bg-white dark:bg-[#2c2d2e] rounded-2xl flex items-center justify-center mb-5 border border-border-color/30 shadow-sm mx-auto">
                  <Package className="w-12 h-12 text-primary-blue/60" />
                </div>
                <h2 className="text-[20px] font-bold text-text-main text-center mb-2">{selectedProduct.name}</h2>
                <div className="text-center mb-6">
                  <span className="text-[20px] font-extrabold text-[#FF7D00]">{selectedProduct.price}</span>
                </div>
                <div className="bg-white dark:bg-[#2c2d2e] rounded-xl p-4 shadow-sm">
                  <h4 className="text-[14px] font-bold text-text-main mb-2">{t('enterprise.auto_250baa7b', '产品描述')}</h4>
                  <p className="text-[14px] text-text-sub leading-relaxed whitespace-pre-wrap">{selectedProduct.desc}</p>
                </div>
              </div>
              <div className="p-4 bg-white dark:bg-[#2c2d2e] border-t border-border-color">
                <button 
                  className="w-full bg-gradient-to-r from-blue-500 to-indigo-600 text-white rounded-full py-3.5 text-[16px] font-bold shadow-lg shadow-blue-500/30 active:scale-[0.98] transition-transform"
                  onClick={() => { setSelectedProduct(null); openChat(); }}
                >{t('enterprise.auto_289280b4', '咨询产品')}</button>
              </div>
            </motion.div>
          </div>
        )}
        
        {selectedJob && (
          <div className="fixed inset-0 z-50 flex flex-col justify-end">
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              transition={{ duration: 0.2 }}
              className="absolute inset-0 bg-black/40 backdrop-blur-[2px]"
              onClick={() => setSelectedJob(null)}
            />
            <motion.div
              initial={{ y: "100%" }}
              animate={{ y: 0 }}
              exit={{ y: "100%" }}
              transition={{ type: "spring", damping: 25, stiffness: 200 }}
              className="relative bg-[#f5f6f8] dark:bg-[#1a1b1c] rounded-t-2xl overflow-hidden pb-safe flex flex-col max-h-[85vh]"
            >
              <div className="flex items-center justify-between p-4 bg-white dark:bg-[#2c2d2e] border-b border-border-color">
                <h3 className="text-[17px] font-bold text-text-main">{t('enterprise.auto_3b8d0de0', '职位详情')}</h3>
                <div className="w-8 h-8 rounded-full bg-[#f5f6f8] dark:bg-[#1a1b1c] flex items-center justify-center cursor-pointer" onClick={() => setSelectedJob(null)}>
                  <X className="w-5 h-5 text-text-sub" />
                </div>
              </div>
              <div className="overflow-y-auto p-5 bg-white dark:bg-[#2c2d2e]">
                <div className="flex justify-between items-start mb-3">
                  <h2 className="text-[22px] font-bold text-text-main leading-tight">{selectedJob.title}</h2>
                  <span className="text-[18px] font-extrabold text-red-500 whitespace-nowrap ml-4">{selectedJob.salary}</span>
                </div>
                <div className="flex items-center gap-2 mb-6">
                  <span className="px-2.5 py-1 bg-[#f5f6f8] dark:bg-[#1a1b1c] text-text-sub text-[13px] font-medium rounded-sm border border-border-color/50">{selectedJob.req.split(" | ")[0]}</span>
                  <span className="px-2.5 py-1 bg-[#f5f6f8] dark:bg-[#1a1b1c] text-text-sub text-[13px] font-medium rounded-sm border border-border-color/50">{selectedJob.req.split(" | ")[1]}</span>
                  <span className="px-2.5 py-1 bg-[#f5f6f8] dark:bg-[#1a1b1c] text-text-sub text-[13px] font-medium rounded-sm border border-border-color/50">{selectedJob.req.split(" | ")[2]}</span>
                </div>
                
                <div className="flex items-center gap-3 mb-6 p-4 rounded-xl bg-[#f5f6f8] dark:bg-[#1a1b1c] border border-border-color/30">
                  <div className="w-12 h-12 rounded-full bg-gradient-to-br from-blue-100 to-indigo-100 dark:from-blue-900/40 dark:to-indigo-900/40 flex items-center justify-center shrink-0">
                    <Briefcase className="w-6 h-6 text-primary-blue" />
                  </div>
                  <div>
                    <h4 className="text-[15px] font-bold text-text-main">{t('enterprise.auto_n5277daa7', '招聘负责人')}</h4>
                    <span className="text-[13px] text-text-sub">{t('enterprise.auto_51555a28', '极客科技宇宙 HR')}</span>
                  </div>
                </div>

                <div className="mb-2">
                  <h4 className="text-[16px] font-bold text-text-main mb-3 flex items-center gap-2">
                    <span className="w-1 h-3.5 bg-primary-blue rounded-full"></span>{t('enterprise.auto_3b886242', '职位描述')}</h4>
                  <p className="text-[15px] text-text-sub leading-relaxed whitespace-pre-wrap">{selectedJob.desc}</p>
                </div>
              </div>
              <div className="p-4 bg-white dark:bg-[#2c2d2e] border-t border-border-color mt-2 shadow-[0_-4px_10px_rgba(0,0,0,0.02)]">
                <button 
                  className="w-full bg-gradient-to-r from-blue-500 to-indigo-600 text-white rounded-full py-3.5 text-[16px] font-bold shadow-lg shadow-blue-500/30 active:scale-[0.98] transition-transform"
                  onClick={() => { setSelectedJob(null); openChat(); }}
                >{t('enterprise.auto_39188763', '立即沟通')}</button>
              </div>
            </motion.div>
          </div>
        )}
      </AnimatePresence>
    </PageLayout>
  );
};


