import React, { useState, useEffect, useMemo } from "react";
import { motion, AnimatePresence } from "motion/react";
import {
  ArrowLeft,
  Check,
  X,
  Flame,
  Star,
  Heart,
  Share2,
  ShoppingCart,
} from "lucide-react";
import { toast } from "@sdkwork/im-pc-chat";
import { ShopProduct, shopService } from "../services/ShopService";
import { useTranslation } from "react-i18next";

export const ProductDetail = ({
  product,
  onClose,
  onAddToCart,
  onBuyNow,
}: {
  product: ShopProduct;
  onClose: () => void;
  onAddToCart: (skuId?: string) => void;
  onBuyNow: (skuId?: string) => void;
}) => {
  const { t, i18n } = useTranslation(["product", "common"]);
  const [isFav, setIsFav] = useState(false);

  const initialOptions = useMemo(() => {
    if (product.skus && product.skus.length > 0) {
      const firstAvailableSku = product.skus.find((sku) => sku.stock > 0);
      if (firstAvailableSku && firstAvailableSku.specs) {
        return firstAvailableSku.specs;
      }
      if (product.skus[0].specs) return product.skus[0].specs;
    }
    return {};
  }, [product]);

  const [selectedOptions, setSelectedOptions] =
    useState<Record<string, string>>(initialOptions);

  const [selectedSkuId, setSelectedSkuId] = useState<string | null>(() => {
    if (product.skus && product.skus.length > 0 && !product.options) {
      const firstAvailableSku = product.skus.find((sku) => sku.stock > 0);
      return firstAvailableSku ? firstAvailableSku.id : product.skus[0].id;
    }
    return null;
  });

  useEffect(() => {
    shopService.isFavorite(product.id).then(setIsFav);
  }, [product.id]);

  const selectedSku = useMemo(() => {
    if (product.options && product.skus) {
      return (
        product.skus.find((sku) => {
          if (!sku.specs) return false;
          return Object.entries(selectedOptions).every(
            ([k, v]) => sku.specs![k] === v,
          );
        }) || null
      );
    }
    return product.skus?.find((s) => s.id === selectedSkuId) || null;
  }, [product, selectedOptions, selectedSkuId]);

  const currentPrice = selectedSku ? selectedSku.price : product.price;

  return (
    <motion.div
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: -20 }}
      className="flex flex-col md:flex-row w-full h-full bg-[#1e1e20] text-gray-200 absolute inset-0 z-10"
    >
      {/* Floating Back Button */}
      <button
        onClick={onClose}
        className="absolute top-6 left-6 z-20 w-12 h-12 rounded-full bg-black/40 hover:bg-black/60 backdrop-blur-md flex items-center justify-center text-white transition-all transform hover:scale-110 shadow-lg border border-white/10"
      >
        <ArrowLeft size={24} />
      </button>

      {/* Image Area */}
      <div className="w-full md:w-1/2 h-[40vh] md:h-full bg-[#2b2b2d] relative overflow-hidden group shrink-0">
        <motion.img
          src={selectedSku?.imageUrl || product.imageUrl}
          alt={product.title}
          referrerPolicy="no-referrer"
          className="w-full h-full object-cover transition-transform duration-700 ease-out group-hover:scale-105"
        />
        <div className="absolute inset-0 bg-gradient-to-t from-[#1e1e20]/80 via-transparent to-transparent md:bg-none" />
      </div>

      {/* Info Area */}
      <div className="w-full md:w-1/2 flex flex-col flex-1 min-h-0 overflow-y-auto bg-[#1e1e20] custom-scrollbar">
        <div className="p-8 md:p-12 lg:p-16 pb-24 flex-1 w-full max-w-2xl mx-auto flex flex-col">
          <div className="flex items-center gap-2 mb-6">
            {product.tags &&
              product.tags.map((tag: string) => (
                <span
                  key={tag}
                  className="bg-pink-500/10 text-pink-400 text-sm px-3 py-1.5 rounded-full font-medium border border-pink-500/20"
                >
                  {tag}
                </span>
              ))}
          </div>

          <h2 className="text-4xl lg:text-5xl font-bold text-gray-100 mb-6 leading-tight tracking-tight">
            {product.title}
          </h2>

          <p className="text-gray-400 text-lg leading-relaxed mb-8">
            {product.description}
          </p>

          <div className="flex flex-wrap items-center gap-x-8 gap-y-4 text-gray-400 mb-8 pb-8 border-b border-white/5">
            <span className="flex items-center gap-2 text-base">
              <Star size={20} className="text-yellow-500 fill-yellow-500" />{" "}
              <span className="text-gray-100 font-medium">
                {product.rating}
              </span>{" "}
              {t("product:rating")}
            </span>
            <span className="flex items-center gap-2 text-base">
              <Flame size={20} className="text-orange-500" />{" "}
              {t("product:sold")}{" "}
              <span className="text-gray-100 font-medium">
                {product.salesVolume.toLocaleString()}
              </span>
            </span>
            <span
              className={`flex items-center gap-1.5 text-base font-medium ${!selectedSku || selectedSku.stock === 0 ? "text-red-400" : "text-green-400"}`}
            >
              {!selectedSku || selectedSku.stock === 0 ? (
                <X size={18} />
              ) : (
                <Check size={18} />
              )}
              {!selectedSku || selectedSku.stock === 0
                ? t("product:outOfStock", { defaultValue: "暂无库存" })
                : t("product:inStock")}
            </span>
          </div>

          {product.options ? (
            <div className="mb-8">
              {product.options.map((option) => (
                <div key={option.name} className="mb-6">
                  <h3 className="text-gray-400 text-sm mb-3 font-medium">
                    {option.name}
                  </h3>
                  <div className="flex flex-wrap gap-3">
                    {option.values.map((val) => {
                      const isSelected = selectedOptions[option.name] === val;

                      // Check universally out of stock
                      const isExistingWithStock = product.skus?.some(
                        (sku) =>
                          sku.specs?.[option.name] === val && sku.stock > 0,
                      );

                      // Check combination availability
                      const isCombinationAvailable =
                        product.skus?.some((sku) => {
                          if (sku.specs?.[option.name] !== val) return false;
                          const matchesOthers = Object.entries(
                            selectedOptions,
                          ).every(([k, v]) => {
                            if (k === option.name) return true;
                            return sku.specs?.[k] === v;
                          });
                          return matchesOthers && sku.stock > 0;
                        }) ?? true;

                      let btnClass =
                        "relative px-5 py-2.5 rounded-xl border-2 transition-all duration-200 text-sm font-medium outline-none overflow-hidden ";
                      if (!isExistingWithStock) {
                        btnClass +=
                          "border-transparent text-gray-500 bg-white/5 cursor-not-allowed opacity-50 line-through decoration-gray-500 decoration-2";
                      } else if (isSelected) {
                        btnClass +=
                          "border-pink-500 bg-pink-500/10 text-pink-500 font-bold z-10 shadow-[0_4px_12px_rgba(236,72,153,0.15)] ring-1 ring-pink-500 ring-offset-0";
                      } else if (!isCombinationAvailable) {
                        btnClass +=
                          "border-white/10 text-gray-400 bg-black/20 hover:border-pink-500/40 hover:text-pink-400 border-dashed";
                      } else {
                        btnClass +=
                          "border-white/5 text-gray-200 bg-[#2b2b2d] hover:border-pink-500/50 hover:text-pink-400 hover:bg-[#333336] shadow-sm";
                      }

                      return (
                        <button
                          key={val}
                          onClick={() => {
                            if (!isExistingWithStock) return;

                            setSelectedOptions((prev) => {
                              const newSelected = {
                                ...prev,
                                [option.name]: val,
                              };

                              const isValidCombo = product.skus?.some((sku) => {
                                const matchesAll = Object.entries(
                                  newSelected,
                                ).every(([k, v]) => sku.specs?.[k] === v);
                                return matchesAll && sku.stock > 0;
                              });

                              if (!isValidCombo) {
                                const fallbackSku = product.skus?.find(
                                  (sku) =>
                                    sku.specs?.[option.name] === val &&
                                    sku.stock > 0,
                                );
                                if (fallbackSku && fallbackSku.specs) {
                                  return { ...fallbackSku.specs };
                                }
                              }
                              return newSelected;
                            });
                          }}
                          className={btnClass}
                        >
                          {val}
                          {isSelected && (
                            <svg
                              className="absolute bottom-0 right-0 w-5 h-5 text-pink-500"
                              viewBox="0 0 20 20"
                              fill="currentColor"
                            >
                              <path d="M20 0L0 20H20V0Z" />
                              <path
                                d="M10 16L13.5 19L19 12"
                                stroke="white"
                                strokeWidth="2"
                                fill="none"
                                strokeLinecap="round"
                                strokeLinejoin="round"
                              />
                            </svg>
                          )}
                        </button>
                      );
                    })}
                  </div>
                </div>
              ))}
            </div>
          ) : (
            product.skus &&
            product.skus.length > 0 && (
              <div className="mb-8">
                <h3 className="text-gray-400 text-sm mb-3 font-medium">
                  {t("product:specs")}
                </h3>
                <div className="flex flex-wrap gap-3">
                  {product.skus.map((sku) => {
                    const isSelected = selectedSkuId === sku.id;
                    const isAvailable = sku.stock > 0;

                    let btnClass =
                      "relative px-5 py-2.5 rounded-xl border-2 transition-all duration-200 text-sm font-medium outline-none overflow-hidden ";
                    if (!isAvailable) {
                      btnClass +=
                        "border-transparent text-gray-500 bg-white/5 cursor-not-allowed opacity-50 line-through decoration-gray-500 decoration-2";
                    } else if (isSelected) {
                      btnClass +=
                        "border-pink-500 bg-pink-500/10 text-pink-500 font-bold z-10 shadow-[0_4px_12px_rgba(236,72,153,0.15)] ring-1 ring-pink-500 ring-offset-0";
                    } else {
                      btnClass +=
                        "border-white/5 text-gray-200 bg-[#2b2b2d] hover:border-pink-500/50 hover:text-pink-400 hover:bg-[#333336] shadow-sm";
                    }

                    return (
                      <button
                        key={sku.id}
                        onClick={() => isAvailable && setSelectedSkuId(sku.id)}
                        className={btnClass}
                      >
                        {sku.name}
                        {isSelected && (
                          <svg
                            className="absolute bottom-0 right-0 w-5 h-5 text-pink-500"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                          >
                            <path d="M20 0L0 20H20V0Z" />
                            <path
                              d="M10 16L13.5 19L19 12"
                              stroke="white"
                              strokeWidth="2"
                              fill="none"
                              strokeLinecap="round"
                              strokeLinejoin="round"
                            />
                          </svg>
                        )}
                      </button>
                    );
                  })}
                </div>
              </div>
            )
          )}

          <div className="flex items-end gap-4 mb-10 bg-gradient-to-r from-[#252527] to-[#2b2b2d] p-8 rounded-3xl border border-white/5 shadow-inner">
            <span className="text-pink-500 font-medium mb-2 text-xl">
              {t("common:currencySymbol")}
            </span>
            <span className="text-6xl font-bold text-transparent bg-clip-text bg-gradient-to-br from-pink-400 to-pink-600 tracking-tight">
              {currentPrice.toLocaleString(
                i18n.language === "en-US" ? "en-US" : "zh-CN",
              )}
            </span>
            {product.originalPrice && (
              <div className="flex flex-col ml-4">
                <span className="text-gray-500 line-through text-xl">
                  {t("common:currencySymbol")}
                  {product.originalPrice.toLocaleString(
                    i18n.language === "en-US" ? "en-US" : "zh-CN",
                  )}
                </span>
                <span className="mt-1 text-sm font-semibold text-pink-500/90 bg-pink-500/10 px-2 py-0.5 rounded text-center">
                  {t("product:saved", {
                    amount: (
                      product.originalPrice - currentPrice
                    ).toLocaleString(
                      i18n.language === "en-US" ? "en-US" : "zh-CN",
                    ),
                    currencySymbol: t("common:currencySymbol"),
                  })}
                </span>
              </div>
            )}
          </div>

          <div className="grid grid-cols-2 gap-4 mb-12 text-sm text-gray-400">
            <div className="flex items-center gap-2.5">
              <div className="w-6 h-6 rounded-full bg-pink-500/10 flex items-center justify-center">
                <Check size={14} className="text-pink-500" />
              </div>{" "}
              {t("product:feature1")}
            </div>
            <div className="flex items-center gap-2.5">
              <div className="w-6 h-6 rounded-full bg-pink-500/10 flex items-center justify-center">
                <Check size={14} className="text-pink-500" />
              </div>{" "}
              {product.isVirtual
                ? t("product:feature2_virtual")
                : t("product:feature2_physical", {
                    currencySymbol: t("common:currencySymbol"),
                  })}
            </div>
            <div className="flex items-center gap-2.5">
              <div className="w-6 h-6 rounded-full bg-pink-500/10 flex items-center justify-center">
                <Check size={14} className="text-pink-500" />
              </div>{" "}
              {product.isVirtual
                ? t("product:feature3_virtual")
                : t("product:feature3_physical")}
            </div>
            {!product.isVirtual && (
              <div className="flex items-center gap-2.5">
                <div className="w-6 h-6 rounded-full bg-pink-500/10 flex items-center justify-center">
                  <Check size={14} className="text-pink-500" />
                </div>{" "}
                {t("product:feature4")}
              </div>
            )}
          </div>

          {/* Action Buttons */}
          <div className="flex items-center gap-4 mt-auto">
            <motion.button
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              className="w-16 h-16 rounded-2xl border border-white/10 flex items-center justify-center text-gray-400 hover:text-pink-500 hover:border-pink-500/50 hover:bg-pink-500/10 transition-colors shrink-0 bg-[#252527]"
              style={{ color: isFav ? "#ec4899" : "" }}
              onClick={async (e) => {
                const newState = await shopService.toggleFavorite(product.id);
                setIsFav(newState);
                if (!newState) {
                  toast(t("product:unfavorited"), "success");
                } else {
                  toast(t("product:favorited"), "success");
                }
              }}
            >
              <Heart
                size={24}
                className={
                  isFav ? "fill-pink-500" : "group-hover:fill-pink-500"
                }
              />
            </motion.button>
            <motion.button
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              className="w-16 h-16 rounded-2xl border border-white/10 flex items-center justify-center text-gray-400 hover:text-white hover:border-white/30 hover:bg-white/5 transition-colors shrink-0 bg-[#252527]"
              onClick={() => {
                navigator.clipboard.writeText(
                  `https://shop.sdkwork.com/product/${product.id}`,
                );
                toast(t("product:copied"), "success");
              }}
            >
              <Share2 size={24} />
            </motion.button>

            <motion.button
              whileHover={{
                scale: !selectedSku || selectedSku.stock === 0 ? 1 : 1.02,
              }}
              whileTap={{
                scale: !selectedSku || selectedSku.stock === 0 ? 1 : 0.98,
              }}
              className={`flex-1 h-16 rounded-2xl font-bold text-lg transition-colors flex items-center justify-center gap-3 ${
                !selectedSku || selectedSku.stock === 0
                  ? "bg-black/20 border border-white/5 text-gray-600 cursor-not-allowed hidden"
                  : "bg-gradient-to-r from-orange-500 to-amber-500 text-white shadow-lg shadow-orange-500/20"
              }`}
              onClick={() => {
                if (!selectedSku || selectedSku.stock === 0) return;
                onAddToCart(selectedSku.id);
                onClose();
              }}
              style={{
                display:
                  !selectedSku || selectedSku.stock === 0 ? "none" : "flex",
              }}
            >
              <ShoppingCart size={20} />
              {t("product:addToCart")}
            </motion.button>

            <motion.button
              whileHover={{
                scale: !selectedSku || selectedSku.stock === 0 ? 1 : 1.02,
              }}
              whileTap={{
                scale: !selectedSku || selectedSku.stock === 0 ? 1 : 0.98,
              }}
              className={`flex-1 h-16 outline-none rounded-2xl font-bold text-lg transition-colors flex items-center justify-center gap-3 ${
                !selectedSku || selectedSku.stock === 0
                  ? "bg-black/30 text-gray-500 border border-white/5 cursor-not-allowed w-full"
                  : "bg-gradient-to-r from-red-600 to-rose-500 text-white shadow-lg shadow-red-500/20"
              }`}
              onClick={() => {
                if (!selectedSku || selectedSku.stock === 0) return;
                onBuyNow(selectedSku.id);
              }}
            >
              <Check
                size={20}
                className={
                  !selectedSku || selectedSku.stock === 0 ? "hidden" : "block"
                }
              />
              {!selectedSku || selectedSku.stock === 0
                ? t("product:outOfStock", { defaultValue: "暂无库存" })
                : t("product:buyNow")}
            </motion.button>
          </div>
        </div>
      </div>
    </motion.div>
  );
};
