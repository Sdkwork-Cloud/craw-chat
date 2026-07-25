import React from "react";
import { useTranslation } from "react-i18next";

export interface VipPlan {
  id: string;
  name: string;
  price: string;
  originalPrice: string;
  desc: string;
  badge?: string;
}

interface VipPlanSelectorProps {
  plans: VipPlan[];
  selectedPlan: string;
  onSelectPlan: (id: string) => void;
}

export const VipPlanSelector: React.FC<VipPlanSelectorProps> = ({
  plans,
  selectedPlan,
  onSelectPlan,
}) => {
  const { t } = useTranslation();
  const currentPlan = plans.find((p) => p.id === selectedPlan);

  return (
    <div className="bg-white dark:bg-[#1A1A1A] rounded-2xl p-5 shadow-sm border border-[#EBEBEB] dark:border-[#333]">
      <h3 className="text-[16px] font-bold text-text-main mb-4">
        {t('vip.auto_7cbfa07c', '选择订阅套餐')}
      </h3>
      <div className="grid grid-cols-3 gap-3">
        {plans.map((plan) => (
          <div
            key={plan.id}
            onClick={() => onSelectPlan(plan.id)}
            className={`relative p-4 rounded-xl border-2 transition-all flex flex-col items-center justify-center cursor-pointer ${
              selectedPlan === plan.id
                ? "border-[#D4AF37] bg-[#FDFBF2] dark:bg-[#2A2410]"
                : "border-[#EBEBEB] dark:border-[#333] bg-transparent"
            }`}
          >
            {plan.badge && (
              <div className="absolute -top-2 left-1/2 -translate-x-1/2 bg-[#FF4C4C] text-white text-[10px] px-2 py-0.5 rounded-full whitespace-nowrap z-10">
                {plan.badge}
              </div>
            )}
            <div className="text-[13px] text-text-sub font-medium mb-1 whitespace-nowrap">
              {plan.name}
            </div>
            <div className="text-[#D4AF37] font-bold text-xl leading-none flex items-baseline">
              <span className="text-[12px] font-normal mr-0.5">¥</span>
              {plan.price}
            </div>
            <div className="text-[10px] text-text-sub/70 line-through mt-1">
              ¥{plan.originalPrice}
            </div>
          </div>
        ))}
      </div>

      {currentPlan && (
        <div className="mt-4 p-3 bg-gray-50 dark:bg-white/5 rounded-lg">
          <p className="text-[12px] text-text-sub text-center">{currentPlan.desc}</p>
        </div>
      )}
    </div>
  );
};
