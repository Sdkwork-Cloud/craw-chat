import React, { useState } from "react";
import { motion, AnimatePresence } from "motion/react";
import { ArrowLeft, Ticket, CheckCircle2, QrCode } from "lucide-react";
import { ShopProduct, CartItem } from "../services/ShopService";
import { useTranslation } from "react-i18next";
import { CouponDetailView } from "./CouponDetailView";

export const OrderDetailView = ({
  orderId,
  items,
  products,
  generatedCoupons = [],
  onReturn,
}: {
  orderId: string;
  items: CartItem[];
  products: ShopProduct[];
  generatedCoupons?: { item: CartItem; product: ShopProduct; code: string }[];
  onReturn: () => void;
}) => {
  const { t, i18n } = useTranslation(["common", "checkout", "cart"]);
  const [selectedCoupon, setSelectedCoupon] = useState<{
    item: CartItem;
    product: ShopProduct;
    code: string;
  } | null>(null);

  const renderCouponItem = (
    coupon: { item: CartItem; product: ShopProduct; code: string },
    index: number,
  ) => {
    const { item, product, code } = coupon;
    return (
      <div
        key={`${item.id}-${index}`}
        className="bg-gradient-to-r from-pink-500/10 to-pink-600/5 border border-pink-500/20 rounded-2xl p-4 flex items-center gap-4 cursor-pointer hover:border-pink-500/40 transition-colors"
        onClick={() => setSelectedCoupon({ item, product, code })}
      >
        <div className="w-12 h-12 rounded-xl bg-pink-500/20 text-pink-500 flex items-center justify-center shrink-0">
          <Ticket size={24} />
        </div>
        <div className="flex-1 min-w-0">
          <div className="font-medium text-gray-100">{product.title}</div>
          {item.skuId && (
            <div className="text-sm text-pink-400 mt-1">
              {product.skus?.find((s) => s.id === item.skuId)?.name}
            </div>
          )}
          <div className="text-xs text-gray-500 mt-1 font-mono">{code}</div>
        </div>
        <div className="text-pink-500">
          <QrCode size={20} />
        </div>
      </div>
    );
  };

  if (selectedCoupon) {
    return (
      <CouponDetailView
        product={selectedCoupon.product}
        item={selectedCoupon.item}
        code={selectedCoupon.code}
        onBack={() => setSelectedCoupon(null)}
      />
    );
  }

  // Use pre-generated coupons provided by the parent via the newly added prop.
  const coupons: React.ReactNode[] = generatedCoupons.map((c, i) =>
    renderCouponItem(c, i),
  );

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      className="absolute inset-0 z-20 bg-[#1e1e20] overflow-y-auto custom-scrollbar flex flex-col"
    >
      <div className="h-16 border-b border-white/5 flex items-center px-8 shrink-0 bg-[#1e1e20] shadow-sm z-10 sticky top-0">
        <button
          onClick={onReturn}
          className="flex items-center gap-2 hover:bg-white/5 px-3 py-1.5 rounded-full transition-colors text-gray-400 hover:text-white"
        >
          <ArrowLeft size={18} />
          <span className="text-sm font-medium">
            {t("common:back", { defaultValue: "返回购物中心" })}
          </span>
        </button>
      </div>

      <div className="flex-1 p-6 w-full max-w-5xl mx-auto flex flex-col pt-8">
        <div className="bg-[#2b2b2d] rounded-3xl p-8 border border-white/5 flex flex-col items-center shrink-0 mb-8 shadow-xl shadow-black/20">
          <div className="w-20 h-20 bg-gradient-to-br from-green-400 to-green-600 text-white rounded-full flex items-center justify-center mb-6 shadow-lg shadow-green-500/30">
            <CheckCircle2 size={40} />
          </div>
          <h2 className="text-3xl font-bold text-white mb-2 tracking-tight">
            {t("checkout:paySuccess", { defaultValue: "订单支付成功" })}
          </h2>
          <p className="text-gray-400 text-sm font-mono mb-8">
            {t("checkout:orderNo", {
              orderId,
              defaultValue: `订单号: ${orderId}`,
            })}
          </p>

          {coupons.length > 0 && (
            <div className="w-full mb-8">
              <h3 className="text-lg font-medium text-gray-200 mb-4 flex items-center gap-2">
                <Ticket size={20} className="text-pink-500" />
                {t("checkout:yourCoupons", {
                  count: coupons.length,
                  defaultValue: `您的卡券 (${coupons.length})`,
                })}
              </h3>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {coupons}
              </div>
              <p className="text-xs text-gray-500 mt-4 text-center">
                {t("checkout:couponSentTips", {
                  defaultValue: "优惠券详情已发送至您的聊天会话",
                })}
              </p>
            </div>
          )}

          <div className="w-full bg-[#1e1e20] rounded-2xl p-6 border border-white/5 mt-4 group hover:border-pink-500/20 transition-colors">
            <h3 className="text-base font-medium text-gray-200 mb-4 flex items-center justify-between">
              {t("checkout:productListTitle", { defaultValue: "商品清单" })}
              <span className="text-sm font-normal text-gray-500">
                {items.length} 件
              </span>
            </h3>
            <div className="space-y-4">
              {items.map((item) => {
                const product = products.find((p) => p.id === item.productId);
                if (!product) return null;
                return (
                  <div key={item.id} className="flex items-center gap-4">
                    <img
                      src={product.imageUrl}
                      alt={product.title}
                      referrerPolicy="no-referrer"
                      className="w-16 h-16 md:w-20 md:h-20 rounded-xl object-cover bg-black/20 border border-white/5"
                    />
                    <div className="flex-1 min-w-0">
                      <div className="text-sm font-medium text-gray-200 truncate">
                        {product.title}
                      </div>
                      <div className="text-xs text-gray-500 mt-1.5 flex items-center gap-2">
                        <span className="bg-white/5 px-2 py-0.5 rounded text-gray-400">
                          {t("checkout:quantity", {
                            count: item.quantity,
                            defaultValue: `数量: ${item.quantity}`,
                          })}
                        </span>
                        {item.skuId && (
                          <span className="bg-white/5 px-2 py-0.5 rounded text-gray-400 truncate">
                            {
                              product.skus?.find((s) => s.id === item.skuId)
                                ?.name
                            }
                          </span>
                        )}
                      </div>
                    </div>
                  </div>
                );
              })}
            </div>
          </div>
        </div>
      </div>

      {/* Recommended Products */}
      <div className="shrink-0 pb-12">
        <div className="flex items-center gap-4 mb-6">
          <div className="flex-1 h-px bg-gradient-to-r from-transparent to-white/10"></div>
          <h3 className="text-lg font-medium text-gray-300 flex items-center gap-2">
            <span className="w-1.5 h-1.5 rounded-full bg-pink-500"></span>
            {t("checkout:recommended", { defaultValue: "猜你喜欢" })}
            <span className="w-1.5 h-1.5 rounded-full bg-pink-500"></span>
          </h3>
          <div className="flex-1 h-px bg-gradient-to-l from-transparent to-white/10"></div>
        </div>
        <div className="grid grid-cols-2 lg:grid-cols-4 gap-4 md:gap-6">
          {products.slice(0, 4).map((product) => (
            <div
              key={product.id}
              className="bg-[#2b2b2d] rounded-2xl overflow-hidden border border-white/5 hover:border-pink-500/30 transition-all cursor-pointer group hover:-translate-y-1 shadow-lg hover:shadow-pink-500/10"
            >
              <div className="aspect-square bg-[#1e1e20] relative overflow-hidden">
                <img
                  src={product.imageUrl}
                  alt={product.title}
                  referrerPolicy="no-referrer"
                  className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
                />
              </div>
              <div className="p-4">
                <h4 className="text-sm font-medium text-gray-200 line-clamp-2 mb-2 group-hover:text-pink-400 transition-colors">
                  {product.title}
                </h4>
                <div className="flex items-baseline gap-1 opacity-90">
                  <span className="text-xs text-pink-500 font-bold">
                    {t("common:currencySymbol")}
                  </span>
                  <span className="text-lg font-bold text-pink-500">
                    {product.price.toLocaleString(
                      i18n.language === "en-US" ? "en-US" : "zh-CN",
                    )}
                  </span>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </motion.div>
  );
};
