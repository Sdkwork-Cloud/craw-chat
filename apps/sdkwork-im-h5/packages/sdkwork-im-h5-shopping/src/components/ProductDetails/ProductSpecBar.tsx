import { useTranslation } from "react-i18next";
import React from "react";
import { ChevronRight } from "lucide-react";
import { Product } from "../../types";

export const ProductSpecBar = ({
  product,
  currentSku,
  quantity,
  onClick,
}: {
  product: Product;
  currentSku: any;
  quantity: number;
  onClick: () => void;
}) => {
  return (
    <div
      className="bg-chat-other-bg px-4 py-3 mb-2 flex items-center justify-between cursor-pointer active:bg-chat-active-bg transition-colors"
      onClick={onClick}
    >
      <div className="flex items-center gap-3">
        <span className="text-[14px] text-text-sub">选择</span>
        <span className="text-[14px] text-text-main font-medium line-clamp-1">
          已选:{" "}
          {currentSku
            ? Object.values(currentSku.specValues)
                .map((vId) => {
                  for (const s of product.specs || []) {
                    const opt = s.options.find((o) => o.id === vId);
                    if (opt) return opt.name;
                  }
                  return vId;
                })
                .join(", ")
            : product.specs && product.specs.length > 0
            ? "请选择规格"
            : "默认规格"}
          ，{quantity}件
        </span>
      </div>
      <ChevronRight className="w-5 h-5 text-text-sub/60" />
    </div>
  );
};
