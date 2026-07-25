import React from "react";
import { Package } from "lucide-react";

export interface TokenPlan {
  id: string;
  name: string;
  tokens: number;
  price: number;
  desc: string;
}

interface PlanTabContentProps {
  plans: TokenPlan[];
  onPay: (item: TokenPlan, type: string) => void;
}

export const PlanTabContent: React.FC<PlanTabContentProps> = ({ plans, onPay }) => {
  return (
    <div className="space-y-4">
      {plans.map((plan) => (
        <div 
          key={plan.id}
          className="bg-white dark:bg-[#1A1A1A] p-5 rounded-2xl border border-border-color shadow-sm flex flex-col gap-3"
        >
           <div className="flex justify-between items-start">
              <div className="flex items-center gap-2 text-text-main font-bold text-[16px]">
                 <Package className="w-5 h-5 text-purple-500" strokeWidth={2} />
                 {plan.name}
              </div>
              <div className="text-[#FF4C4C] font-bold text-[18px]">¥{plan.price}</div>
           </div>
           <div className="text-[13px] text-text-sub">{plan.desc}</div>
           <div className="flex items-center justify-between mt-2 pt-3 border-t border-border-color">
              <div className="text-[14px] font-medium text-text-main">
                内含<span className="text-purple-500 font-bold">{plan.tokens.toLocaleString()}</span> Tokens
              </div>
              <button 
                 className="bg-purple-500 text-white px-4 py-1.5 rounded-full text-[13px] font-medium active:opacity-80"
                 onClick={() => onPay(plan, 'plan')}
              >
                立即购买
              </button>
           </div>
        </div>
      ))}
    </div>
  );
};
