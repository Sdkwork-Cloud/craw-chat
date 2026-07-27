import React from "react";
import { cn } from "@sdkwork/im-h5-commons";

interface RechargeTabsHeaderProps {
  activeTab: "recharge" | "plan" | "coupon";
  setActiveTab: (tab: "recharge" | "plan" | "coupon") => void;
}

export const RechargeTabsHeader: React.FC<RechargeTabsHeaderProps> = ({
  activeTab,
  setActiveTab,
}) => {
  return (
    <div className="flex bg-white dark:bg-[#1A1A1A] border-b border-border-color shadow-sm relative z-20">
      <div 
        className={cn("flex-1 text-center py-4 text-[15px] font-medium transition-colors border-b-2 cursor-pointer", activeTab === "recharge" ? "text-primary-blue border-primary-blue" : "text-text-sub border-transparent")}
        onClick={() => setActiveTab("recharge")}
      >
        Token 充值
      </div>
      <div 
        className={cn("flex-1 text-center py-4 text-[15px] font-medium transition-colors border-b-2 cursor-pointer", activeTab === "plan" ? "text-primary-blue border-primary-blue" : "text-text-sub border-transparent")}
        onClick={() => setActiveTab("plan")}
      >
        Token Plan
      </div>
      <div 
        className={cn("flex-1 text-center py-4 text-[15px] font-medium transition-colors border-b-2 cursor-pointer", activeTab === "coupon" ? "text-primary-blue border-primary-blue" : "text-text-sub border-transparent")}
        onClick={() => setActiveTab("coupon")}
      >
        优惠券
      </div>
    </div>
  );
};
