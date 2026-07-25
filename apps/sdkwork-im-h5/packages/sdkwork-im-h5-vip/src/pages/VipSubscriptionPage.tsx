import { useTranslation } from "react-i18next";
import React, { useState } from "react";
import { PageLayout } from "@sdkwork/im-h5-commons";
import { Star, Shield, Zap } from "lucide-react";
import { VipBanner } from "../components/VipBanner";
import { VipPlanSelector, VipPlan } from "../components/VipPlanSelector";
import { VipBenefitsGrid, VipBenefit } from "../components/VipBenefitsGrid";
import { VipFooter } from "../components/VipFooter";

const VIP_PLANS: VipPlan[] = [
  {
    id: "month",
    name: "连续包月",
    price: "19",
    originalPrice: "25",
    desc: "首月仅需9元",
  },
  {
    id: "quarter",
    name: "连续包季",
    price: "53",
    originalPrice: "75",
    desc: "折合每月17.6元",
    badge: "推荐",
  },
  {
    id: "year",
    name: "连续包年",
    price: "188",
    originalPrice: "300",
    desc: "折合每月15.6元",
    badge: "超值",
  }
];

const VIP_BENEFITS: VipBenefit[] = [
  { icon: Star, title: "专属标识", desc: "尊贵身份的外显标识" },
  { icon: Shield, title: "安全防护", desc: "高级别的账号找回与安全" },
  { icon: Zap, title: "优先体验", desc: "最新功能提前一周体验" },
];

export const VipSubscriptionPage = () => {
  const { t } = useTranslation();
  const [selectedPlan, setSelectedPlan] = useState("year");

  return (
    <PageLayout title={t('vip.auto_prop_n6732b65a', 'Vip 订阅')} bgClass="bg-[#F8F9FA] dark:bg-black">
      <div className="relative pt-6 pb-24 overflow-y-auto h-full">
        <VipBanner />

        <div className="relative z-10 mt-[100px] px-4 space-y-6">
          <VipPlanSelector
            plans={VIP_PLANS}
            selectedPlan={selectedPlan}
            onSelectPlan={setSelectedPlan}
          />

          <VipBenefitsGrid benefits={VIP_BENEFITS} />
        </div>
      </div>

      <VipFooter />
    </PageLayout>
  );
};

