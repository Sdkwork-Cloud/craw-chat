import React from "react";
import { motion } from "motion/react";
import { Star, Plus } from "lucide-react";
import { ShopProduct } from "../services/ShopService";
import { useTranslation } from "react-i18next";

export const ProductCard = ({
  product,
  onClick,
  onAddToCart,
}: {
  product: ShopProduct;
  onClick: () => void;
  onAddToCart: (skuId?: string) => void;
}) => {
  const { t, i18n } = useTranslation(["shop", "common"]);

  return (
    <motion.div
      className="bg-[#2b2b2d] rounded-2xl border border-white/5 overflow-hidden group cursor-pointer hover:border-pink-500/30 flex flex-col h-full hover:shadow-2xl hover:shadow-pink-500/10"
      onClick={onClick}
      whileHover={{ y: -5 }}
      transition={{ duration: 0.3, ease: "easeOut" }}
    >
      <div className="relative aspect-square overflow-hidden bg-white/5">
        <motion.img
          src={product.imageUrl}
          alt={product.title}
          referrerPolicy="no-referrer"
          className="w-full h-full object-cover transition-all duration-700 transform group-hover:scale-110"
        />
        {product.tags && product.tags.length > 0 && (
          <div className="absolute top-3 left-3 flex flex-col gap-1.5">
            {product.tags.map((tag: string) => (
              <span
                key={tag}
                className="bg-pink-500 text-white text-[10px] px-2.5 py-1 rounded-md shadow-lg shadow-pink-500/20 font-bold tracking-widest uppercase"
              >
                {tag}
              </span>
            ))}
          </div>
        )}
      </div>
      <div className="p-5 flex flex-col flex-1 bg-gradient-to-b from-[#2b2b2d] to-[#252527]">
        <h4 className="font-medium text-gray-200 text-sm line-clamp-2 leading-snug group-hover:text-pink-400 transition-colors mb-2">
          {product.title}
        </h4>
        <div className="flex items-center gap-1.5 text-xs text-gray-500 mb-4">
          <Star size={12} className="fill-yellow-500 text-yellow-500" />
          <span className="font-medium text-gray-400">{product.rating}</span>
          <span className="mx-1.5 text-white/10">|</span>
          <span>
            {t("shop:sold")}{" "}
            {product.salesVolume > 10000
              ? `\${(product.salesVolume/10000).toFixed(1)}\${t('shop:tenThousand')}`
              : product.salesVolume}
          </span>
        </div>
        <div className="mt-auto flex items-end justify-between">
          <div className="flex items-baseline gap-1">
            <span className="text-sm text-pink-500 font-medium">
              {t("common:currencySymbol")}
            </span>
            <span className="text-xl font-bold text-pink-500 tracking-tight">
              {product.price.toLocaleString(
                i18n.language === "en-US" ? "en-US" : "zh-CN",
              )}
            </span>
            {product.originalPrice && (
              <span className="text-xs text-gray-500 line-through ml-1.5">
                {t("common:currencySymbol")}
                {product.originalPrice.toLocaleString(
                  i18n.language === "en-US" ? "en-US" : "zh-CN",
                )}
              </span>
            )}
          </div>
          <button
            className="w-9 h-9 rounded-full bg-pink-500/10 flex items-center justify-center text-pink-500 group-hover:bg-pink-500 group-hover:text-white transition-all duration-300 transform hover:scale-110 hover:shadow-lg hover:shadow-pink-500/30"
            onClick={(e) => {
              e.stopPropagation();
              const defaultSkuId =
                product.skus && product.skus.length > 0
                  ? product.skus[0].id
                  : undefined;
              onAddToCart(defaultSkuId);
            }}
          >
            <Plus size={18} />
          </button>
        </div>
      </div>
    </motion.div>
  );
};
