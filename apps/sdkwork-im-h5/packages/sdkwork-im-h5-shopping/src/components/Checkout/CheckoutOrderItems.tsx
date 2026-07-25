import React from "react";
import { Store, ChevronRight } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Shop } from "../../types";

interface CheckoutOrderItemsProps {
  shop: Shop | null;
  displayItems: any[];
  isVirtualOrder: boolean;
}

export const CheckoutOrderItems: React.FC<CheckoutOrderItemsProps> = ({
  shop,
  displayItems,
  isVirtualOrder,
}) => {
  const { t } = useTranslation();

  return (
    <div className="bg-chat-other-bg rounded-xl p-4 mb-3">
      {shop && (
        <div className="flex items-center gap-2 mb-3">
          <Store className="w-4 h-4 text-text-main" />
          <span className="text-[14px] font-medium text-text-main">
            {shop.name}
          </span>
        </div>
      )}

      <div className="flex flex-col gap-4">
        {displayItems.map((item, idx) => (
          <div key={idx} className="flex gap-3">
            <img
              src={item.sku?.image || item.product.image}
              className="w-[80px] h-[80px] rounded-lg border border-border-color/30 object-cover"
              alt={item.product.title}
            />
            <div className="flex-1 flex flex-col pt-1">
              <span className="text-[14px] text-text-main leading-tight line-clamp-2 mb-1">
                {item.product.title}
              </span>
              <span className="text-[12px] text-text-sub bg-bg-color w-max px-1.5 py-0.5 rounded-sm line-clamp-1">
                {item.sku
                  ? Object.values(item.sku.specValues || {})
                      .map((vId) => {
                        const spec = item.product.specs?.find((s: any) =>
                          s.options.some((o: any) => o.id === vId)
                        );
                        return (
                          spec?.options.find((o: any) => o.id === vId)?.name ||
                          vId
                        );
                      })
                      .join(", ")
                  : "默认规格"}
              </span>
              <div className="flex items-center justify-between mt-auto">
                <span className="text-[16px] font-bold text-text-main">
                  <span className="text-[12px]">¥</span>
                  {parseFloat(item.sku?.price || item.product.price)}
                </span>
                <span className="text-[13px] text-text-sub">
                  x{item.quantity}
                </span>
              </div>
            </div>
          </div>
        ))}
      </div>

      {!isVirtualOrder && (
        <div className="flex justify-between items-center mt-4 pt-4 border-t border-border-color/50">
          <span className="text-[14px] text-text-main">
            {t("shopping.auto_44363088", "配送服务")}
          </span>
          <span className="text-[13px] text-text-sub">
            {t("shopping.auto_30806876", "普通快递 免费送达")}
            <ChevronRight className="w-4 h-4 inline" />
          </span>
        </div>
      )}
      <div className="flex justify-between items-center mt-3">
        <span className="text-[14px] text-text-main">
          {t("shopping.auto_250ee18d", "买家留言")}
        </span>
        <span className="text-[13px] text-text-sub">
          {t("shopping.auto_18d2da7", "无留言")}
          <ChevronRight className="w-4 h-4 inline" />
        </span>
      </div>
    </div>
  );
};
