import { useTranslation } from "react-i18next";
import React, { useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, Trash2, ShoppingCart } from "lucide-react";
import { IconButton, cn } from "@sdkwork/im-h5-commons";
import { useCartStore } from "../store/useCartStore";

export const ShoppingCartPage = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const {
    items,
    loadCart,
    updateQuantity,
    toggleItemCheck,
    toggleAllCheck,
    removeFromCart,
    getCheckedItems,
    getTotalPrice,
  } = useCartStore();

  useEffect(() => {
    loadCart();
  }, [loadCart]);

  const allChecked = items.length > 0 && items.every((i) => i.checked);
  const checkedItems = getCheckedItems();
  const totalPrice = getTotalPrice();

  const handleCheckout = () => {
  if (checkedItems.length === 0) return;
    navigate("/checkout?from=cart");
  };

  const handleRemove = (id: string) => {
  removeFromCart([id]);
  };

  return (
    <div className="flex flex-col h-full bg-bg-color pt-safe">
      <header className="flex items-center px-2 h-[56px] bg-chat-other-bg border-b border-border-color sticky top-0 z-10 shrink-0">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          onClick={() => navigate(-1)}
        />
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h2 className="text-[17px] font-medium text-text-main">{t('shopping.auto_n7e08e6a2', '购物车 {items.length > 0 ? `(${items.length})` : ""}')}</h2>
        </div>
      </header>

      <div className="flex-1 overflow-y-auto p-4 pb-[80px]">
        {items.length === 0 ? (
          <div className="h-full flex flex-col items-center justify-center pb-20">
            <div className="w-24 h-24 bg-chat-other-bg rounded-full flex items-center justify-center mb-4 text-text-sub/40">
              <ShoppingCart className="w-10 h-10" />
            </div>
            <p className="text-[15px] text-text-sub mb-4">{t('shopping.auto_7500ef67', '购物车空空如也')}</p>
            <button
              className="border border-[#FA5151] text-[#FA5151] px-6 py-1.5 rounded-full text-[14px]"
              onClick={() => navigate("/discover/shopping")}
            >{t('shopping.auto_14c545b', '去逛逛')}</button>
          </div>
        ) : (
          <div className="flex flex-col gap-3">
            {items.map((item) => (
              <div
                key={item.id}
                className="bg-chat-other-bg rounded-xl p-3 flex gap-3 items-center"
              >
                <div
                  className={cn(
                    "w-5 h-5 rounded-full border shrink-0 flex items-center justify-center cursor-pointer transition-colors",
                    item.checked
                      ? "bg-[#FA5151] border-[#FA5151]"
                      : "border-text-sub/40",
                  )}
                  onClick={() => toggleItemCheck(item.id, !item.checked)}
                >
                  {item.checked && (
                    <div className="w-1.5 h-1.5 bg-white rounded-full" />
                  )}
                </div>

                <img
                  src={item.product.image}
                  className="w-20 h-20 rounded-lg object-cover bg-chat-other-bg shrink-0 cursor-pointer"
                  onClick={() => navigate(`/product/${item.productId}`)}
                />

                <div className="flex-1 flex flex-col min-w-0 py-1">
                  <span
                    className="text-[14px] text-text-main leading-tight line-clamp-2 mb-1 cursor-pointer"
                    onClick={() => navigate(`/product/${item.productId}`)}
                  >
                    {item.product.title}
                  </span>
                  {item.selectedSpecs && item.product.specs && (
                    <div className="bg-text-sub/5 text-text-sub text-[11px] px-1.5 py-0.5 rounded w-fit mb-2 mt-0.5 line-clamp-1">
                      {Object.values(item.selectedSpecs).map(vId => {
                        for(const s of item.product.specs || []) {
                          const opt = s.options.find(o => o.id === vId);
                          if (opt) return opt.name;
                        }
                        return vId;
                      }).join("，")}
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
                        onClick={() =>
                          updateQuantity(item.id, item.quantity - 1)
                        }
                      >
                        -
                      </button>
                      <span className="w-8 h-6 flex items-center justify-center text-[13px] bg-chat-other-bg border-x border-border-color">
                        {item.quantity}
                      </span>
                      <button
                        className="w-7 h-6 flex items-center justify-center bg-chat-other-bg text-[14px] active:bg-[#E5E5E5] transition-colors"
                        onClick={() =>
                          updateQuantity(item.id, item.quantity + 1)
                        }
                      >
                        +
                      </button>
                    </div>
                  </div>
                </div>

                <div className="shrink-0 h-full flex items-center pl-2">
                  <div
                    className="p-2 text-text-sub/50 active:text-text-main transition-colors cursor-pointer"
                    onClick={() => handleRemove(item.id)}
                  >
                    <Trash2 className="w-4 h-4" />
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Bottom bar */}
      {items.length > 0 && (
        <div className="absolute bottom-0 left-0 right-0 bg-chat-other-bg border-t border-border-color pb-safe px-4 py-2 flex items-center justify-between h-[60px]">
          <div
            className="flex items-center gap-2 cursor-pointer"
            onClick={() => toggleAllCheck(!allChecked)}
          >
            <div
              className={cn(
                "w-5 h-5 rounded-full border shrink-0 flex items-center justify-center transition-colors",
                allChecked
                  ? "bg-[#FA5151] border-[#FA5151]"
                  : "border-text-sub/40",
              )}
            >
              {allChecked && (
                <div className="w-1.5 h-1.5 bg-white rounded-full" />
              )}
            </div>
            <span className="text-[14px] text-text-sub">{t('shopping.auto_a6ba1', '全选')}</span>
          </div>

          <div className="flex items-center gap-3">
            <div className="flex items-baseline">
              <span className="text-[13px] text-text-main pr-1">{t('shopping.auto_14c5ac1', '合计:')}</span>
              <span className="text-[#FA5151] font-bold text-[18px]">
                <span className="text-[14px]">¥</span>
                {totalPrice.toFixed(2)}
              </span>
            </div>
            <button
              className={cn(
                "px-6 py-2 rounded-full text-[14px] font-medium transition-colors text-white",
                checkedItems.length > 0
                  ? "bg-[#FA5151] active:scale-95 transition-transform"
                  : "bg-[#FA5151]/50 pointer-events-none",
              )}
              onClick={handleCheckout}
            >{t('shopping.auto_7da9390', '结算({checkedItems.length})')}</button>
          </div>
        </div>
      )}
    </div>
  );
};
