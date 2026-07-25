import { useTranslation } from "react-i18next";
import React from "react";
import { X } from "lucide-react";
import { Product } from "../../types";

interface ProductSkuPanelProps {
  product: Product;
  currentSku: any;
  displayPrice: string;
  quantity: number;
  setQuantity: (val: number) => void;
  selectedSpecs: Record<string, string>;
  setSelectedSpecs: (specs: Record<string, string>) => void;
  setShowSkuPanel: (show: boolean) => void;
  skuAction: "cart" | "buy";
  handleConfirmSku: () => void;
}

export const ProductSkuPanel: React.FC<ProductSkuPanelProps> = ({
  product,
  currentSku,
  displayPrice,
  quantity,
  setQuantity,
  selectedSpecs,
  setSelectedSpecs,
  setShowSkuPanel,
  skuAction,
  handleConfirmSku,
}) => {
  const { t } = useTranslation();
return (
    <div className="absolute inset-0 z-50 flex flex-col justify-end">
      {/* Overlay Background */}
      <div
        className="absolute inset-0 bg-black/40 backdrop-blur-sm"
        onClick={() => setShowSkuPanel(false)}
      />

      {/* Panel Content */}
      <div className="relative bg-bg-color rounded-t-2xl w-full min-h-[400px] flex flex-col pt-4 pb-safe animate-in slide-in-from-bottom duration-300">
        {/* Close Button */}
        <div
          className="absolute top-3 right-3 w-8 h-8 flex items-center justify-center rounded-full bg-chat-other-bg cursor-pointer"
          onClick={() => setShowSkuPanel(false)}
        >
          <X className="w-5 h-5 text-text-sub" />
        </div>

        {/* Product Header */}
        <div className="flex px-4 gap-3 mb-6">
          <img
            src={currentSku?.image || product.image}
            className="w-[100px] h-[100px] rounded-lg border-2 border-border-color/30 object-cover -mt-8 bg-chat-other-bg shrink-0"
            alt={product.name}
          />
          <div className="flex flex-col justify-end">
            <span className="text-[#FA5151] font-bold text-[22px] leading-none mb-1">
              <span className="text-[14px]">¥</span>
              {displayPrice}
            </span>
            <span className="text-[13px] text-text-sub mb-1">
              {currentSku ? t('shopping.stock_count', `库存 {{count}} 件`, { count: currentSku.stock }) : t('shopping.please_select_spec', "请选择规格")}
            </span>
            <span className="text-[13px] text-text-main line-clamp-2 pr-6">
              {t('shopping.selected_spec', "已选:")}{" "}
              {currentSku
                ? Object.values(currentSku.specValues)
                    .map((vId) => {
                      for (const s of product.specs || []) {
                        const opt = s.options.find((o: any) => o.id === vId);
                        if (opt) return opt.name;
                      }
                      return vId;
                    })
                    .join(", ")
                : t('shopping.default_spec', "默认规格")}
              ，{t('shopping.quantity_count', "{{count}}件", { count: quantity })}
            </span>
          </div>
        </div>

        {currentSku?.description && (
          <div className="px-4 mb-4">
            <div className="bg-[#FA5151]/5 border border-[#FA5151]/20 rounded-lg p-2.5 text-[12.5px] text-text-sub leading-[1.4]">
              <span className="text-[#FA5151] font-medium mr-1.5">{t('shopping.auto_219bae0', '说明:')}</span>
              {currentSku.description}
            </div>
          </div>
        )}

        <div className="flex-1 overflow-y-auto px-4">
          {product.specs && product.specs.length > 0 ? (
            product.specs.map((spec) => (
              <div key={spec.id} className="mb-6">
                <h3 className="text-[15px] font-medium text-text-main mb-3">
                  {spec.name}
                </h3>
                <div className="flex flex-wrap gap-3">
                  {spec.options.map((opt) => {
                    const isSelected = selectedSpecs[spec.id] === opt.id;
                    return (
                      <span
                        key={opt.id}
                        onClick={() =>
                          setSelectedSpecs({ ...selectedSpecs, [spec.id]: opt.id })
                        }
                        className={`px-4 py-1.5 rounded-full text-[13px] border cursor-pointer transition-colors ${
                          isSelected
                            ? "bg-[#FA5151]/10 text-[#FA5151] border-[#FA5151]/50"
                            : "bg-bg-color text-text-main border-border-color"
                        }`}
                      >
                        {opt.name}
                      </span>
                    );
                  })}
                </div>
              </div>
            ))
          ) : (
            <div className="mb-6">
              <h3 className="text-[15px] font-medium text-text-main mb-3">{t('shopping.auto_1116f8', '规格')}</h3>
              <div className="flex flex-wrap gap-3">
                <span className="bg-[#FA5151]/10 text-[#FA5151] border border-[#FA5151]/50 px-4 py-1.5 rounded-full text-[13px]">{t('shopping.auto_4a521bc4', '默认规格')}</span>
              </div>
            </div>
          )}

          {/* Quantity */}
          <div className="flex items-center justify-between py-4 border-t border-border-color">
            <span className="text-[15px] font-medium text-text-main">{t('shopping.auto_41601482', '购买数量')}</span>
            <div className="flex items-center">
              <button
                className="w-8 h-8 flex items-center justify-center bg-chat-other-bg text-[18px] text-text-main active:bg-chat-active-bg transition-colors disabled:opacity-50"
                onClick={() => setQuantity(Math.max(1, quantity - 1))}
                disabled={quantity <= 1}
              >
                -
              </button>
              <span className="w-10 h-8 flex items-center justify-center text-[14px] bg-chat-other-bg border-x border-border-color text-text-main">
                {quantity}
              </span>
              <button
                className="w-8 h-8 flex items-center justify-center bg-chat-other-bg text-[18px] text-text-main active:bg-chat-active-bg transition-colors"
                onClick={() => setQuantity(quantity + 1)}
              >
                +
              </button>
            </div>
          </div>
        </div>

        {/* Confirm Button */}
        <div className="px-4 py-2 border-t border-border-color">
          <button
            className={`w-full py-2.5 rounded-full text-[15px] font-medium text-white shadow-sm active:scale-[0.98] transition-transform ${
              skuAction === "cart" ? "bg-[#FFAA00]" : "bg-[#FA5151]"
            }`}
            onClick={handleConfirmSku}
          >{t('shopping.auto_ef0ec', '确定')}</button>
        </div>
      </div>
    </div>
  );
};
