import React from "react";
import { motion } from "motion/react";
import { Star, Flame } from "lucide-react";
import { ShopCategory, ShopProduct } from "../services/ShopService";
import { ProductCard } from "./ProductCard";
import { useTranslation } from "react-i18next";

export const ShopHome = ({
  categories,
  products,
  onOpenProduct,
  onAddToCart,
  activeCategoryId,
  setActiveCategoryId,
}: {
  categories: ShopCategory[];
  products: ShopProduct[];
  onOpenProduct: (p: ShopProduct) => void;
  onAddToCart: (product: ShopProduct, skuId?: string) => void;
  activeCategoryId: string | null;
  setActiveCategoryId: (id: string | null) => void;
}) => {
  const { t } = useTranslation(["shop"]);
  const displayProducts = activeCategoryId
    ? products.filter((p) => p.categoryId === activeCategoryId)
    : products;

  return (
    <motion.div
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -10 }}
      className="p-6 pb-20 max-w-[1400px] mx-auto min-h-full flex flex-col"
    >
      {/* Banner */}
      <motion.div
        initial={{ opacity: 0, scale: 0.95 }}
        animate={{ opacity: 1, scale: 1 }}
        transition={{ duration: 0.6, ease: "easeOut" }}
        className="w-full h-[280px] lg:h-[360px] rounded-3xl overflow-hidden relative mb-8 group cursor-pointer shadow-2xl shadow-pink-500/10 shrink-0"
      >
        <div className="absolute inset-0 bg-gradient-to-br from-pink-600 via-purple-700 to-indigo-900 transition-transform duration-1000 group-hover:scale-105" />
        <div className="absolute inset-0 z-20 flex flex-col justify-center p-10 lg:p-16">
          <motion.span
            initial={{ opacity: 0, x: -20 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ delay: 0.3, duration: 0.5 }}
            className="text-pink-300 font-bold tracking-[0.2em] mb-3 text-sm uppercase"
          >
            {t("shop:bannerSub")}
          </motion.span>
          <motion.h2
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.4, duration: 0.5 }}
            className="text-4xl lg:text-5xl font-bold text-white mb-6 leading-tight drop-shadow-lg"
          >
            {t("shop:season")}
            <br />
            {t("shop:slogan")}
          </motion.h2>
          <motion.button
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.5, duration: 0.5 }}
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            className="bg-white text-gray-900 px-8 py-3 rounded-full w-fit font-bold hover:bg-pink-50 hover:text-pink-600 transition-all shadow-xl shadow-black/20"
            onClick={() =>
              document
                .getElementById("featured-products")
                ?.scrollIntoView({ behavior: "smooth" })
            }
          >
            {t("shop:explore")}
          </motion.button>
        </div>
      </motion.div>

      {/* Main Layout: Left Sidebar + Right Grid */}
      <div className="flex gap-8 items-start flex-1" id="featured-products">
        {/* Left Sidebar Categories */}
        <div className="w-56 shrink-0 sticky top-4 hidden md:flex flex-col gap-2">
          <h3 className="text-lg font-bold text-gray-100 flex items-center gap-2 mb-4 px-2">
            <Star size={18} className="text-yellow-500" />
            {t("shop:featuredCategories")}
          </h3>
          <div
            onClick={() => setActiveCategoryId(null)}
            className={`px-4 py-3 rounded-xl flex items-center gap-3 cursor-pointer transition-all \${!activeCategoryId ? 'bg-pink-500 text-white font-medium shadow-lg shadow-pink-500/25' : 'text-gray-400 hover:bg-white/5 hover:text-gray-200'}`}
          >
            <Flame
              size={18}
              className={
                !activeCategoryId ? "text-yellow-300" : "text-gray-500"
              }
            />
            {t("shop:recommendedForYou", { defaultValue: "为您推荐" })}
          </div>
          {categories.map((cat) => {
            const isActive = activeCategoryId === cat.id;
            return (
              <div
                key={cat.id}
                onClick={() => setActiveCategoryId(isActive ? null : cat.id)}
                className={`px-4 py-3 rounded-xl flex items-center gap-3 cursor-pointer transition-all \${
                  isActive 
                    ? 'bg-pink-500 text-white font-medium shadow-lg shadow-pink-500/25' 
                    : 'text-gray-400 hover:bg-white/5 hover:text-gray-200'
                }`}
              >
                <span className="text-lg opacity-80">{cat.icon}</span>
                <span>{cat.name}</span>
              </div>
            );
          })}
        </div>

        {/* Right Content */}
        <div className="flex-1 min-w-0">
          <div className="flex items-center justify-between mb-6">
            <h3 className="text-xl font-medium text-gray-100 hidden md:block">
              {activeCategoryId
                ? categories.find((c) => c.id === activeCategoryId)?.name ||
                  t("shop:categoryProducts")
                : t("shop:recommendedForYou")}
            </h3>
            <h3 className="text-xl font-medium text-gray-100 md:hidden">
              {activeCategoryId
                ? t("shop:categoryProducts")
                : t("shop:recommendedForYou")}
            </h3>
            {activeCategoryId && (
              <button
                onClick={() => setActiveCategoryId(null)}
                className="text-sm text-pink-400 hover:text-pink-300 transition-colors md:hidden"
              >
                {t("shop:viewAll")}
              </button>
            )}
          </div>
          {/* Mobile Categories Horizontal */}
          <div className="flex md:hidden gap-3 overflow-x-auto pb-4 mb-4 custom-scrollbar snap-x">
            {categories.map((cat) => {
              const isActive = activeCategoryId === cat.id;
              return (
                <div
                  key={cat.id}
                  onClick={() => setActiveCategoryId(isActive ? null : cat.id)}
                  className={`snap-start shrink-0 px-4 py-2 border rounded-full flex items-center gap-2 cursor-pointer transition-all \${
                    isActive 
                      ? 'bg-pink-500 border-pink-500 text-white' 
                      : 'bg-[#2b2b2d] border-white/5 text-gray-400 hover:bg-[#343436]'
                  }`}
                >
                  <span>{cat.icon}</span>
                  <span className="text-sm whitespace-nowrap">{cat.name}</span>
                </div>
              );
            })}
          </div>

          <div className="grid grid-cols-2 md:grid-cols-3 xl:grid-cols-4 gap-4 lg:gap-6">
            {displayProducts.map((product) => (
              <div
                key={product.id}
                className="h-full animate-in fade-in slide-in-from-bottom-4 duration-500"
              >
                <ProductCard
                  product={product}
                  onClick={() => onOpenProduct(product)}
                  onAddToCart={(skuId) => onAddToCart(product, skuId)}
                />
              </div>
            ))}
          </div>

          {displayProducts.length === 0 && (
            <div className="py-20 flex flex-col items-center justify-center text-gray-500">
              <span className="text-4xl mb-4">🛒</span>
              <p>没有找到该分类下的商品</p>
            </div>
          )}
        </div>
      </div>
    </motion.div>
  );
};
