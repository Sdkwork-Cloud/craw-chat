import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "motion/react";
import { ArrowLeft, ShoppingCart, Check, Minus, Plus } from "lucide-react";
import { toast } from "@sdkwork/im-pc-chat";
import { ShopProduct, CartItem, shopService } from "../services/ShopService";
import { CheckoutView } from "./CheckoutView";
import { useTranslation } from "react-i18next";

const Trash2Icon = ({ size }: { size: number }) => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
  >
    <path d="M3 6h18"></path>
    <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path>
    <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path>
    <line x1="10" y1="11" x2="10" y2="17"></line>
    <line x1="14" y1="11" x2="14" y2="17"></line>
  </svg>
);

export const ShopCart = ({
  products,
  onUpdateCount,
  onReturn,
  onCheckoutSuccess,
}: {
  products: ShopProduct[];
  onUpdateCount: () => void;
  onReturn: () => void;
  onCheckoutSuccess: (
    orderId: string,
    items: CartItem[],
    generatedCoupons?: { item: CartItem; product: ShopProduct; code: string }[],
  ) => void;
}) => {
  const { t, i18n } = useTranslation(["cart", "common", "checkout"]);
  const [cartItems, setCartItems] = useState<CartItem[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadCart();
  }, []);

  const loadCart = () => {
    setLoading(true);
    shopService.getCart().then((items) => {
      setCartItems(items);
      setLoading(false);
      onUpdateCount();
    });
  };

  const handleUpdateQuantity = async (id: string, qty: number) => {
    await shopService.updateCartItem(id, qty);
    loadCart();
  };

  const handleRemove = async (id: string) => {
    await shopService.removeCartItem(id);
    loadCart();
  };

  const handleToggleSelect = async (id: string) => {
    await shopService.toggleCartItemSelection(id);
    loadCart();
  };

  const handleToggleAll = async (select: boolean) => {
    await shopService.toggleAllCartItems(select);
    loadCart();
  };

  const [showCheckout, setShowCheckout] = useState(false);
  const selectedItems = cartItems.filter((i) => i.selected);
  const allSelected =
    cartItems.length > 0 && selectedItems.length === cartItems.length;

  const totalPrice = selectedItems.reduce((sum, item) => {
    const product = products.find((p) => p.id === item.productId);
    const sku = product?.skus?.find((s) => s.id === item.skuId);
    const price = sku ? sku.price : product?.price;
    return sum + (price || 0) * item.quantity;
  }, 0);

  if (showCheckout) {
    return (
      <CheckoutView
        products={products}
        selectedItems={selectedItems}
        totalPrice={totalPrice}
        onBack={() => setShowCheckout(false)}
        onComplete={async () => {
          setShowCheckout(false);
          try {
            const result = await shopService.checkout();
            toast(
              t("checkout:checkoutSuccess", {
                orderId: result.orderId,
                total: result.total.toLocaleString(
                  i18n.language === "en-US" ? "en-US" : "zh-CN",
                ),
                currencySymbol: t("common:currencySymbol"),
              }),
              "success",
            );
            loadCart();
            onCheckoutSuccess(
              result.orderId,
              result.items,
              result.generatedCoupons,
            );
          } catch (e) {
            toast(
              t("checkout:checkoutFailed", { defaultValue: "结算失败" }),
              "error",
            );
          }
        }}
      />
    );
  }

  return (
    <motion.div
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: -20 }}
      className="p-6 w-full max-w-7xl mx-auto h-full flex flex-col"
    >
      <div className="flex items-center gap-3 mb-8">
        <button
          onClick={onReturn}
          className="p-2 hover:bg-white/10 rounded-full transition-colors text-gray-400 hover:text-white"
        >
          <ArrowLeft size={20} />
        </button>
        <h2 className="text-2xl font-bold text-gray-100 flex items-center gap-2">
          {t("cart:cart")}{" "}
          <span className="text-gray-500 font-normal text-sm">
            ({cartItems.length})
          </span>
        </h2>
      </div>

      {loading ? (
        <div className="flex-1 flex items-center justify-center">
          <div className="w-8 h-8 rounded-full border-2 border-pink-500 border-t-transparent animate-spin" />
        </div>
      ) : cartItems.length === 0 ? (
        <div className="flex-1 flex flex-col items-center justify-center text-center p-8 bg-[#2b2b2d] rounded-2xl border border-white/5">
          <div className="w-24 h-24 bg-white/5 rounded-full flex items-center justify-center mb-4">
            <ShoppingCart size={40} className="text-gray-500" />
          </div>
          <h3 className="text-xl font-medium text-gray-200 mb-2">
            {t("cart:emptyCart")}
          </h3>
          <p className="text-sm text-gray-500 mb-6 max-w-sm">
            {t("cart:emptyCartDesc")}
          </p>
          <button
            onClick={onReturn}
            className="px-8 py-3 bg-pink-600 hover:bg-pink-500 text-white rounded-full font-medium transition-colors shadow-lg shadow-pink-500/20"
          >
            {t("cart:goShopping")}
          </button>
        </div>
      ) : (
        <div className="flex flex-col lg:flex-row gap-8 flex-1 overflow-hidden pb-10">
          <div className="flex-1 overflow-y-auto pr-2 space-y-4 custom-scrollbar">
            <div className="flex items-center justify-between p-4 bg-[#2b2b2d] rounded-xl border border-white/5">
              <label className="flex items-center gap-3 cursor-pointer group">
                <div
                  className={`w-5 h-5 rounded-full border flex items-center justify-center transition-colors \${allSelected ? 'bg-pink-500 border-pink-500' : 'border-gray-500 group-hover:border-pink-400'}`}
                >
                  {allSelected && <Check size={12} className="text-white" />}
                </div>
                <input
                  type="checkbox"
                  className="hidden"
                  checked={allSelected}
                  onChange={(e) => handleToggleAll(e.target.checked)}
                />
                <span className="text-sm text-gray-300 select-none">
                  {t("cart:selectAll")}
                </span>
              </label>

              {selectedItems.length > 0 && (
                <button
                  className="text-sm text-gray-400 hover:text-red-400 transition-colors"
                  onClick={async () => {
                    await Promise.all(
                      selectedItems.map((i) =>
                        shopService.removeCartItem(i.id),
                      ),
                    );
                    loadCart();
                  }}
                >
                  {t("cart:deleteSelected")}
                </button>
              )}
            </div>

            <AnimatePresence initial={false}>
              {cartItems.map((item) => {
                const product = products.find((p) => p.id === item.productId);
                if (!product) return null;

                const sku = product.skus?.find((s) => s.id === item.skuId);
                const price = sku ? sku.price : product.price;

                return (
                  <motion.div
                    key={item.id}
                    layout
                    initial={{ opacity: 0, scale: 0.95, y: 10 }}
                    animate={{ opacity: 1, scale: 1, y: 0 }}
                    exit={{ opacity: 0, scale: 0.95, y: -10 }}
                    transition={{ duration: 0.2 }}
                    className="flex gap-4 p-4 bg-[#2b2b2d] rounded-xl border border-white/5 hover:border-white/10 transition-colors group"
                  >
                    <div className="flex items-center justify-center h-24">
                      <label className="cursor-pointer p-2 rounded-full hover:bg-white/5 transition-colors">
                        <div
                          className={`w-5 h-5 rounded-full border flex items-center justify-center transition-colors \${item.selected ? 'bg-pink-500 border-pink-500' : 'border-gray-500'}`}
                        >
                          {item.selected && (
                            <Check size={12} className="text-white" />
                          )}
                        </div>
                        <input
                          type="checkbox"
                          className="hidden"
                          checked={item.selected}
                          onChange={() => handleToggleSelect(item.id)}
                        />
                      </label>
                    </div>

                    <div className="w-24 h-24 rounded-lg overflow-hidden shrink-0 bg-white/5">
                      <img
                        src={sku?.imageUrl || product.imageUrl}
                        alt={product.title}
                        referrerPolicy="no-referrer"
                        className="w-full h-full object-cover"
                      />
                    </div>

                    <div className="flex-1 flex flex-col justify-between py-1 min-w-0">
                      <div className="flex justify-between items-start gap-4">
                        <div>
                          <h4 className="font-medium text-gray-200 line-clamp-2 text-sm leading-snug">
                            {product.title}
                          </h4>
                          {sku && (
                            <div className="text-xs text-gray-500 mt-1">
                              {sku.name}
                            </div>
                          )}
                        </div>
                        <button
                          onClick={() => handleRemove(item.id)}
                          className="text-gray-500 hover:text-red-400 transition-colors p-1 opacity-0 group-hover:opacity-100 shrink-0"
                        >
                          <Trash2Icon size={16} />
                        </button>
                      </div>

                      <div className="flex items-end justify-between mt-auto">
                        <div className="text-lg font-bold text-pink-500">
                          <span className="text-xs mr-0.5">
                            {t("common:currencySymbol")}
                          </span>
                          {price.toLocaleString(
                            i18n.language === "en-US" ? "en-US" : "zh-CN",
                          )}
                        </div>

                        <div className="flex items-center bg-[#1e1e20] rounded-full border border-white/10">
                          <button
                            className="w-8 h-8 flex items-center justify-center text-gray-400 hover:text-white disabled:opacity-30 disabled:hover:text-gray-400 rounded-l-full transition-colors"
                            disabled={item.quantity <= 1}
                            onClick={() =>
                              handleUpdateQuantity(item.id, item.quantity - 1)
                            }
                          >
                            <Minus size={14} />
                          </button>
                          <span className="w-8 text-center text-sm font-medium text-gray-200">
                            {item.quantity}
                          </span>
                          <button
                            className="w-8 h-8 flex items-center justify-center text-gray-400 hover:text-white rounded-r-full transition-colors"
                            onClick={() =>
                              handleUpdateQuantity(item.id, item.quantity + 1)
                            }
                          >
                            <Plus size={14} />
                          </button>
                        </div>
                      </div>
                    </div>
                  </motion.div>
                );
              })}
            </AnimatePresence>
          </div>

          <div className="w-full lg:w-80 shrink-0">
            <div className="bg-[#2b2b2d] rounded-2xl border border-white/5 p-6 sticky top-0">
              <h3 className="text-lg font-medium text-gray-100 mb-6">
                {t("cart:orderSummary")}
              </h3>

              <div className="space-y-4 text-sm mb-6">
                <div className="flex justify-between text-gray-400">
                  <span>{t("cart:subtotal")}</span>
                  <span>
                    {t("common:currencySymbol")}{" "}
                    {totalPrice.toLocaleString(
                      i18n.language === "en-US" ? "en-US" : "zh-CN",
                    )}
                  </span>
                </div>
                <div className="flex justify-between text-gray-400">
                  <span>{t("cart:shipping")}</span>
                  <span>{t("cart:shippingTBD")}</span>
                </div>
                <div className="h-px bg-white/10 my-4" />
                <div className="flex justify-between items-end">
                  <span className="text-gray-200 font-medium pb-1">
                    {t("cart:total")}
                  </span>
                  <span className="text-3xl font-bold text-pink-500 tracking-tight">
                    <span className="text-base mr-1">
                      {t("common:currencySymbol")}
                    </span>
                    {totalPrice.toLocaleString(
                      i18n.language === "en-US" ? "en-US" : "zh-CN",
                    )}
                  </span>
                </div>
              </div>

              <button
                className="w-full py-4 bg-pink-600 hover:bg-pink-500 disabled:bg-gray-700 disabled:text-gray-500 text-white rounded-xl font-medium transition-colors shadow-lg shadow-pink-500/20 disabled:shadow-none flex items-center justify-center gap-2"
                disabled={selectedItems.length === 0}
                onClick={() => setShowCheckout(true)}
              >
                {t("cart:checkout", { count: selectedItems.length })}
              </button>
            </div>
          </div>
        </div>
      )}
    </motion.div>
  );
};
