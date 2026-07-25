import { useTranslation } from "react-i18next";
import React from "react";
import { Product } from "../../types";

export const ProductDetailsMock = ({ product }: { product: Product }) => {
  const { t } = useTranslation();
return (
    <div className="bg-chat-other-bg p-4">
      <h2 className="text-[15px] font-medium mb-3">商品详情</h2>
      <p className="text-[14px] text-text-sub leading-relaxed whitespace-pre-wrap">
        {product.description || "这里是商品详情..."}
        <br />
        <br />
        品牌：严选品牌
        <br />
        毛重：0.5kg
      </p>
      <img src={product.image} className="w-full mt-4 rounded-lg object-cover" />
    </div>
  );
};
