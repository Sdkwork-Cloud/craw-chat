import React from "react";
import { Trash2 } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";
import type { CartItem } from "../../types";

interface ShoppingCartItemProps {
  item: CartItem;
  onToggleCheck: (id: string, checked: boolean) => void;
  onUpdateQuantity: (id: string, quantity: number) => void;
  onRemove: (id: string) => void;
  onNavigateToProduct: (productId: string) => void;
}

export const ShoppingCartItem: React.FC<ShoppingCartItemProps> = ({
  item,
  onToggleCheck,
  onUpdateQuantity,
  onRemove,
  onNavigateToProduct,
}) => {
  return (
    <div className="bg-chat-other-bg rounded-xl p-3 flex gap-3 items-center">
      <div
        className={cn(
          "w-5 h-5 rounded-full border shrink-0 flex items-center justify-center cursor-pointer transition-colors",
          item.checked ? "bg-[#FA5151] border-[#FA5151]" : "border-text-sub/40"
        )}
        onClick={() => onToggleCheck(item.id, !item.checked)}
      >
        {item.checked && <div className="w-1.5 h-1.5 bg-white rounded-full" />}
      </div>

      <img
        src={item.product.image}
        className="w-20 h-20 rounded-lg object-cover bg-chat-other-bg shrink-0 cursor-pointer"
        onClick={() => onNavigateToProduct(item.productId)}
        alt={item.product.title}
      />

      <div className="flex-1 flex flex-col min-w-0 py-1">
        <span
          className="text-[14px] text-text-main leading-tight line-clamp-2 mb-1 cursor-pointer"
          onClick={() => onNavigateToProduct(item.productId)}
        >
          {item.product.title}
        </span>
        {item.selectedSpecs && item.product.specs && (
          <div className="bg-text-sub/5 text-text-sub text-[11px] px-1.5 py-0.5 rounded w-fit mb-2 mt-0.5 line-clamp-1">
            {Object.values(item.selectedSpecs)
              .map((vId) => {
                for (const s of item.product.specs || []) {
                  const opt = s.options.find((o) => o.id === vId);
                  if (opt) return opt.name;
                }
                return vId;
              })
              .join("，")}
          </div>
        )}
        <div className="flex items-center justify-between mt-auto">
          <span className="text-[#FA5151] font-bold text-[16px]">
            <span className="text-[12px]">¥</span>
            {item.sku?.price || item.product.price}
          </span>
          <div className="flex items-center border border-border-color rounded-md overflow-hidden">
            <button
              className="w-7 h-6 flex items-center justify-center bg-chat-other-bg text-[14px] active:bg-[#E5E5E5] transition-colors"
              onClick={() => onUpdateQuantity(item.id, item.quantity - 1)}
            >
              -
            </button>
            <span className="w-8 h-6 flex items-center justify-center text-[13px] bg-chat-other-bg border-x border-border-color">
              {item.quantity}
            </span>
            <button
              className="w-7 h-6 flex items-center justify-center bg-chat-other-bg text-[14px] active:bg-[#E5E5E5] transition-colors"
              onClick={() => onUpdateQuantity(item.id, item.quantity + 1)}
            >
              +
            </button>
          </div>
        </div>
      </div>

      <div className="shrink-0 h-full flex items-center pl-2">
        <div
          className="p-2 text-text-sub/50 active:text-text-main transition-colors cursor-pointer"
          onClick={() => onRemove(item.id)}
        >
          <Trash2 className="w-4 h-4" />
        </div>
      </div>
    </div>
  );
};
