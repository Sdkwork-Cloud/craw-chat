import React from "react";
import { CircleDollarSign } from "lucide-react";

export interface TokenAmount {
  id: string;
  amount: number;
  price: number;
  bonus?: string;
}

interface RechargeTabContentProps {
  tokenAmounts: TokenAmount[];
  selectedSku: string;
  onSelectSku: (id: string) => void;
}

export const RechargeTabContent: React.FC<RechargeTabContentProps> = ({
  tokenAmounts,
  selectedSku,
  onSelectSku,
}) => {
  return (
    <div className="space-y-4">
      <div className="bg-gradient-to-r from-blue-500 to-cyan-500 rounded-2xl p-6 text-white mb-6">
        <div className="flex items-center gap-2 mb-2 opacity-90">
          <CircleDollarSign className="w-5 h-5" />
          <span className="text-[14px]">当前 Token</span>
        </div>
        <div className="text-[36px] font-bold font-mono">12,500</div>
      </div>
      
      <h3 className="text-[15px] font-bold text-text-main px-1 mb-2">选择充值金额</h3>
      <div className="grid grid-cols-2 gap-3">
        {tokenAmounts.map((item) => (
          <div 
            key={item.id} 
            onClick={() => onSelectSku(item.id)}
            className={`bg-white dark:bg-[#1A1A1A] p-5 rounded-2xl border-2 transition-all cursor-pointer flex flex-col items-center justify-center relative ${selectedSku === item.id ? "border-primary-blue bg-blue-50 dark:bg-blue-900/20" : "border-border-color"}`}
          >
            {item.bonus && (
              <div className="absolute top-0 right-0 bg-[#FF4C4C] text-white text-[10px] px-2 py-0.5 rounded-bl-lg rounded-tr-xl">
                {item.bonus}
              </div>
            )}
            <div className="flex items-end gap-1 text-text-main font-bold mb-2">
               <span className="text-[20px] leading-none">{item.amount}</span>
               <span className="text-[12px] opacity-80 mb-0.5">T</span>
            </div>
            <div className="text-[14px] text-primary-blue font-medium">¥ {item.price}</div>
          </div>
        ))}
      </div>
    </div>
  );
};
