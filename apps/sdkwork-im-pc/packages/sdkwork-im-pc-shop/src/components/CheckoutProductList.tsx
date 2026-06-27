import React from "react";
import { useTranslation } from "react-i18next";

export const CheckoutProductList = ({ selectedItems, products }: any) => {
  const { t, i18n } = useTranslation(["checkout", "common"]);

  return (
    <div className="bg-[#2b2b2d] rounded-3xl border border-white/5 overflow-hidden shadow-xl shadow-black/10">
      <div className="px-8 py-5 border-b border-white/5 bg-white/5 flex items-center justify-between">
        <h3 className="font-bold text-gray-200 text-lg">
          {t("checkout:productListTitle", { defaultValue: "商品详情" })}
        </h3>
        <span className="text-sm text-gray-400">
          共 {selectedItems.length} 件
        </span>
      </div>
      <div className="p-8 space-y-6">
        {selectedItems.map((item: any) => {
          const product = products.find((p: any) => p.id === item.productId);
          if (!product) return null;
          const sku = product.skus?.find((s: any) => s.id === item.skuId);
          const price = sku ? sku.price : product.price;

          return (
            <div
              key={item.id}
              className="flex gap-6 p-4 rounded-2xl hover:bg-white/5 transition-colors border border-transparent hover:border-white/5"
            >
              <div className="w-24 h-24 rounded-xl overflow-hidden shrink-0 bg-[#1e1e20] shadow-sm">
                <img
                  src={sku?.imageUrl || product.imageUrl}
                  alt={product.title}
                  referrerPolicy="no-referrer"
                  className="w-full h-full object-cover"
                />
              </div>
              <div className="flex-1 min-w-0 flex flex-col justify-center">
                <h4 className="text-base font-medium text-gray-200 line-clamp-2 mb-2 leading-relaxed">
                  {product.title}
                </h4>
                {sku && (
                  <div className="text-sm text-gray-400 mb-3 bg-white/5 inline-flex w-fit px-2 py-1 rounded-md">
                    {sku.name}
                  </div>
                )}
              </div>
              <div className="flex flex-col items-end justify-center shrink-0 ml-4">
                <div className="text-lg font-bold text-gray-200 mb-2">
                  <span className="text-sm font-medium mr-0.5">
                    {t("common:currencySymbol")}
                  </span>
                  {price.toLocaleString(
                    i18n.language === "en-US" ? "en-US" : "zh-CN",
                  )}
                </div>
                <div className="text-sm text-gray-500 font-medium bg-black/20 px-3 py-1 rounded-full">
                  x {item.quantity}
                </div>
              </div>
            </div>
          );
        })}
      </div>
      <div className="px-8 py-5 border-t border-white/5 bg-black/10 flex justify-between items-center text-gray-300">
        <span>配送方式</span>
        <span className="font-medium text-gray-200">
          普通快递 运费 ¥0.00
        </span>
      </div>
    </div>
  );
};
