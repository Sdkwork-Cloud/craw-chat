import { useTranslation } from "react-i18next";
import React from "react";
import { PageLayout, showToast } from "@sdkwork/im-h5-commons";
import { Building2, Rocket, ShieldCheck, Gift } from "lucide-react";
import { useNavigate } from "react-router";

export const EnterpriseInvite = () => {
  const { t } = useTranslation();
const navigate = useNavigate();

  const handleJoin = () => {
  navigate("/enterprise/join");
  };

  return (
    <PageLayout title="邀请入驻" bgClass="bg-bg-color">
      <div className="flex flex-col items-center p-4 pb-[100px] h-full overflow-y-auto relative">
        <div className="w-full bg-gradient-to-br from-blue-600 to-indigo-600 rounded-2xl p-6 text-white shadow-lg mb-4 relative overflow-hidden flex flex-col items-center mt-2 shrink-0">
          <div className="absolute -bottom-10 -right-10 w-40 h-40 bg-white/10 rounded-full blur-2xl"></div>
          <div className="absolute -top-10 -left-10 w-40 h-40 bg-white/10 rounded-full blur-2xl"></div>

          <div className="w-16 h-16 bg-white/20 backdrop-blur-md rounded-2xl flex items-center justify-center mb-3 z-10 shadow-inner border border-white/30">
            <Building2 className="w-8 h-8 text-white" />
          </div>
          <h2 className="text-[18px] font-extrabold mb-1 z-10 text-center leading-tight">张三 邀请您入驻极客企业中心</h2>
          <p className="text-[13px] opacity-90 z-10 text-center font-medium mt-0">拓展生意脉络，发现更多商机</p>
        </div>

        <div className="w-full bg-chat-other-bg rounded-2xl p-5 shadow-sm mb-4 flex flex-col gap-5 shrink-0">
          <h3 className="text-[15px] font-bold text-text-main">入驻专享权益</h3>
          
          <div className="flex items-start gap-4">
            <div className="w-10 h-10 rounded-full bg-orange-50 dark:bg-orange-900/20 flex items-center justify-center shrink-0">
              <Rocket className="w-5 h-5 text-orange-500" />
            </div>
            <div className="flex flex-col justify-center">
              <span className="text-[15px] font-bold text-text-main mb-1">海量商机推荐</span>
              <span className="text-[13px] text-text-sub">精准匹配供需信息，促成高效合作</span>
            </div>
          </div>
          
          <div className="flex items-start gap-4">
            <div className="w-10 h-10 rounded-full bg-blue-50 dark:bg-blue-900/20 flex items-center justify-center shrink-0">
              <ShieldCheck className="w-5 h-5 text-blue-500" />
            </div>
            <div className="flex flex-col justify-center">
              <span className="text-[15px] font-bold text-text-main mb-1">官方企业认证</span>
              <span className="text-[13px] text-text-sub">提升企业信任度，获取专属蓝V标识</span>
            </div>
          </div>
          
          <div className="flex items-start gap-4">
            <div className="w-10 h-10 rounded-full bg-green-50 dark:bg-green-900/20 flex items-center justify-center shrink-0">
              <Gift className="w-5 h-5 text-green-500" />
            </div>
            <div className="flex flex-col justify-center">
              <span className="text-[15px] font-bold text-text-main mb-1">价值 ￥1999 新人礼包</span>
              <span className="text-[13px] text-text-sub">包含招聘名额、置顶展示等专属福利</span>
            </div>
          </div>
        </div>
      </div>

      <div className="fixed bottom-0 left-0 right-0 px-4 pt-4 pb-[calc(env(safe-area-inset-bottom,0px)+1rem)] bg-chat-other-bg border-t border-border-color shadow-[0_-4px_10px_rgba(0,0,0,0.02)] z-40">
        <button 
          className="w-full bg-primary-blue text-white rounded-full py-3 text-[15px] font-bold shadow-lg shadow-blue-500/30 active:scale-[0.98] transition-transform"
          onClick={handleJoin}
        >接受邀请并入驻</button>
        <div className="text-center mt-3 text-[12px] text-text-sub">点击接受邀请，即表示您同意<span className="text-primary-blue cursor-pointer" onClick={() =>showToast("企业服务协议")}>《企业服务协议》</span>
        </div>
      </div>
    </PageLayout>
  );
};

