import { useTranslation } from "react-i18next";
import React from "react";
import { Product } from "../../types";

export const ProductInfo = ({
  product,
  displayPrice,
  displayOriginalPrice,
}: {
  product: Product;
  displayPrice?: number;
  displayOriginalPrice?: number;
}) => {
  const { t } = useTranslation();
return (
    <div className="bg-chat-other-bg p-4 mb-2">
      <div className="flex items-baseline mb-2">
        <span className="text-[#FA5151] font-bold text-[24px]">
          <span className="text-[16px]">¥</span>
          {displayPrice}
        </span>
        {displayOriginalPrice && (
          <span className="text-text-sub text-[14px] line-through ml-2">
            ¥{displayOriginalPrice}
          </span>
        )}
        {product.isVirtual && (
          <span className="ml-3 border border-[#FA5151] text-[#FA5151] rounded-sm px-1 py-[1px] text-[10px]">电子虚拟商品</span>
        )}
      </div>
      <h1 className="text-[17px] font-medium text-text-main leading-tight mb-2">
        {product.title}
      </h1>
      <div className="flex justify-between items-center text-[13px] text-text-sub">
        <span>{product.sales}</span>
        <span>{product.isVirtual ? '自动发货' : '发货地：浙江杭州'}</span>
      </div>
    </div>
  );
};
