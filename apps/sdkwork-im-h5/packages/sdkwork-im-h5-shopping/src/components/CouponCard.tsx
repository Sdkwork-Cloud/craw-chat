import { useTranslation } from "react-i18next";
import React from "react";
import { Product } from "../types";
import { cn } from "@sdkwork/im-h5-commons";

interface CouponCardProps {
  product: Product;
  onClick?: (product: Product) => void;
}

export const CouponCard: React.FC<CouponCardProps> = ({ product, onClick }) => {
  const { t } = useTranslation();
return (
    <div
      className="bg-white dark:bg-[#1C1C1E] rounded-xl overflow-hidden shadow-sm flex cursor-pointer active:scale-[0.98] transition-transform duration-200 relative mb-1"
      onClick={() => onClick && onClick(product)}
    >
      {/* Left side: Price & Image */}
      <div className="w-[110px] shrink-0 bg-[#FA5151]/5 dark:bg-[#FA5151]/10 flex flex-col items-center justify-center relative p-3 border-r border-dashed border-[#FA5151]/30">
        <div className="w-12 h-12 rounded-full overflow-hidden mb-2 bg-white flex items-center justify-center shrink-0 border border-black/5 dark:border-white/10 shadow-sm relative z-10">
           <img 
             src={product.image} 
             className="w-full h-full object-cover" 
             onLoad={(e) => {
               (e.target as HTMLImageElement).style.opacity = '1';
             }}
             style={{ opacity: 0, transition: 'opacity 0.3s' }}
           />
        </div>
        <span className="text-[#FA5151] font-bold text-[20px] leading-none flex items-baseline relative z-10">
            <span className="text-[12px] mr-[1px]">¥</span>
            {product.price?.split('.')[0] || "0"}
            {product.price?.includes('.') && <span className="text-[14px]">.{product.price.split('.')[1]}</span>}
        </span>
        
        {/* Ticket Cutouts */}
        <div className="absolute -left-2.5 top-1/2 -translate-y-1/2 w-5 h-5 bg-bg-color rounded-full shadow-inner" style={{ boxShadow: 'inset -2px 0 3px rgba(0,0,0,0.03)' }} />
        <div className="absolute -right-2.5 top-1/2 -translate-y-1/2 w-5 h-5 bg-bg-color rounded-full shadow-inner z-20" style={{ boxShadow: 'inset 2px 0 3px rgba(0,0,0,0.03)' }} />
      </div>
      
      {/* Right side: Details */}
      <div className="p-3.5 flex flex-col flex-1 justify-between gap-2">
        <div>
          <div className="flex items-start justify-between gap-2 mb-1">
             <span className="text-[15px] text-text-main font-semibold leading-snug line-clamp-2">
               {product.title}
             </span>
             <div className="px-1.5 py-0.5 rounded bg-[#FA5151]/10 text-[#FA5151] text-[10px] shrink-0 font-medium whitespace-nowrap mt-0.5">{product.virtualType === 'coupon' ? t('shopping.coupon_type_coupon', '卡券') : t('shopping.coupon_type_life', '生活')}</div>
          </div>
          <p className="text-[12px] text-text-sub line-clamp-2 mt-1">
            {product.description}
          </p>
        </div>
        
        <div className="flex items-center justify-between mt-auto">
          <span className="text-[11px] text-text-sub">
            {product.sales}
          </span>
          <button className="bg-[#FA5151] text-white text-[12px] font-medium px-4 py-1.5 rounded-full shadow-sm">{t('shopping.auto_39174ed3', '立即抢购')}</button>
        </div>
      </div>
    </div>
  );
};

