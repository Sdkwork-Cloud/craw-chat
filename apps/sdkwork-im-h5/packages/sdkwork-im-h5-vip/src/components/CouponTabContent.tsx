import React from "react";

export interface CouponItem {
  id: string;
  name: string;
  discount: number;
  minSpend: number;
  validTo: string;
}

interface CouponTabContentProps {
  coupons: CouponItem[];
  onPay: (item: CouponItem, type: string) => void;
}

export const CouponTabContent: React.FC<CouponTabContentProps> = ({ coupons, onPay }) => {
  return (
    <div className="space-y-4">
      {coupons.map((coupon) => (
        <div 
          key={coupon.id} 
          className="bg-gradient-to-r from-orange-50 to-red-50 dark:from-[#2A1F1C] dark:to-[#331C1A] rounded-2xl p-5 border border-orange-100 dark:border-orange-900 flex items-center relative overflow-hidden"
        >
          <div className="absolute -left-3 top-1/2 -translate-y-1/2 w-6 h-6 bg-white dark:bg-black rounded-full" />
          <div className="absolute -right-3 top-1/2 -translate-y-1/2 w-6 h-6 bg-white dark:bg-black rounded-full" />
          
          <div className="flex-1 pl-4 border-r border-orange-200 dark:border-orange-800 border-dashed pr-4">
             <div className="text-[18px] font-bold text-orange-600 dark:text-orange-400 mb-1">{coupon.name}</div>
             <div className="text-[12px] text-orange-600/70 dark:text-orange-400/70">
               {coupon.minSpend > 0 ? `满 ${coupon.minSpend} 元可用` : '无门槛使用'}
             </div>
             <div className="text-[10px] text-orange-600/50 dark:text-orange-400/50 mt-2">
               有效期至 {coupon.validTo}
             </div>
          </div>
          <div className="w-[100px] flex flex-col items-center justify-center gap-2">
             <div className="text-orange-600 dark:text-orange-400 font-bold flex items-baseline">
                <span className="text-[14px] mr-0.5">¥</span>
                <span className="text-[28px] leading-none">{coupon.discount}</span>
             </div>
             <button 
                className="bg-gradient-to-r from-orange-500 to-red-500 text-white text-[12px] px-3 py-1 rounded-full active:scale-95"
                onClick={() => onPay(coupon, 'coupon')}
             >
               立即领取
             </button>
          </div>
        </div>
      ))}
    </div>
  );
};
