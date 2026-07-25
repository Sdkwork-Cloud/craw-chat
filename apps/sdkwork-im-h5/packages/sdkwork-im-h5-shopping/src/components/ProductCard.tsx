import { useTranslation } from "react-i18next";
import React from "react";
import { Product } from "../types";

interface ProductCardProps {
  product: Product;
  onClick?: (product: Product) => void;
}

export const getAspectRatio = (url: string) => {
  if (!url) return 1;
  const match = url.match(/\/(\d+)\/(\d+)(?:\?.*)?$/) || url.match(/\/(\d+)\/(\d+)/);
  if (match) {
    const width = parseInt(match[1], 10);
    const height = parseInt(match[2], 10);
    if (width > 0 && height > 0) return width / height;
  }
  return 1;
};

export const ProductCard: React.FC<ProductCardProps> = ({ product, onClick }) => {
  const { t } = useTranslation();
const aspectRatio = getAspectRatio(product.image);

  return (
    <div
      className="bg-white dark:bg-[#1C1C1E] rounded-xl overflow-hidden shadow-sm border border-black/5 dark:border-white/5 flex flex-col cursor-pointer active:scale-[0.98] transition-transform duration-200"
      onClick={() => onClick && onClick(product)}
    >
      <div 
        className="w-full relative bg-gray-100 dark:bg-white/5 shrink-0"
        style={{ paddingBottom: `${(1 / aspectRatio) * 100}%` }}
      >
        <img 
          src={product.image} 
          className="absolute inset-0 w-full h-full object-cover" 
          onLoad={(e) => {
            (e.target as HTMLImageElement).style.opacity = '1';
          }} 
          style={{ opacity: 0, transition: 'opacity 0.4s ease-out' }} 
          loading="lazy"
        />
        {/* Subtle inner ring to soften bright images at the edges */}
        <div className="absolute inset-0 ring-1 ring-inset ring-black/5 dark:ring-white/10 rounded-t-xl pointer-events-none" />
      </div>
      <div className="p-2.5 flex flex-col gap-1.5 flex-1">
        <span 
          className="text-[13px] text-text-main font-medium leading-[1.35] line-clamp-2"
          style={{ wordBreak: 'break-all' }}
        >{product.title || "暂无商品名称"}</span>
        <div className="flex items-center justify-between mt-auto pt-0.5">
          <span className="text-[#FA5151] font-semibold flex items-baseline">
            <span className="text-[11px] mr-[1px]">¥</span>
            <span className="text-[16px] tracking-tight">{product.price?.split('.')[0] || "0"}</span>
            {product.price?.includes('.') && <span className="text-[11px]">.{product.price.split('.')[1]}</span>}
          </span>
          <span className="text-[10.5px] text-text-sub font-normal scale-90 origin-right">{product.sales || "0 人付款"}</span>
        </div>
      </div>
    </div>
  );
};

