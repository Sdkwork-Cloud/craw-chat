import { useTranslation } from "react-i18next";
import React, { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router";
import { ChevronLeft, Share2, Search, Package } from "lucide-react";
import { ProductService } from "../services/ProductService";
import { Product, Shop } from "../types";

import { ShopHeaderBanner } from "../components/ShopHeaderBanner";

export const ShopDetails = () => {
  const { t } = useTranslation();
const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [shop, setShop] = useState<Shop | null>(null);
  const [products, setProducts] = useState<Product[]>([]);

  useEffect(() => {
    if (id) {
      ProductService.getShopById(id).then(setShop);
      ProductService.getProductsByShop(id).then(setProducts);
    }
  }, [id]);

  if (!shop) {
    return (
      <div className="flex flex-col h-full bg-bg-color items-center justify-center text-text-sub opacity-70">
        <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
        <span className="text-[14px]">{t('shopping.auto_7f6f37e', '加载中...')}</span>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      <header className="absolute top-0 left-0 right-0 z-10 flex items-center justify-between px-2 pt-safe h-[56px] text-white">
        <div
          className="w-8 h-8 rounded-full bg-black/30 flex items-center justify-center backdrop-blur-sm cursor-pointer ml-2"
          onClick={() => navigate(-1)}
        >
          <ChevronLeft className="w-5 h-5" />
        </div>
        <div className="w-8 h-8 rounded-full bg-black/30 flex items-center justify-center backdrop-blur-sm cursor-pointer mr-2">
          <Share2 className="w-4 h-4" />
        </div>
      </header>

      <div className="flex-1 overflow-y-auto">
        {/* Shop Header Banner */}
        <ShopHeaderBanner shop={shop} />

        {/* Shop Details & Tags */}
        <div className="bg-chat-other-bg px-4 py-3 border-b border-border-color">
          <div className="flex gap-2 mb-2">
            {shop.isOfficial && (
              <span className="text-[10px] bg-[#FA5151]/10 text-[#FA5151] px-1.5 py-0.5 rounded-sm">{t('shopping.auto_b7d21', '官方')}</span>
            )}
            {shop.tags?.map((t) => (
              <span
                key={t}
                className="text-[10px] bg-chat-other-bg text-text-sub px-1.5 py-0.5 rounded-sm"
              >
                {t}
              </span>
            ))}
          </div>
          {shop.description && (
            <p className="text-[13px] text-text-sub leading-normal line-clamp-2">
              {shop.description}
            </p>
          )}
        </div>

        {/* Search Bar */}
        <div className="bg-chat-other-bg p-3 border-b border-border-color sticky top-0 z-10">
          <div className="bg-chat-other-bg rounded-full h-9 flex items-center px-4 gap-2">
            <Search className="w-4 h-4 text-text-sub" />
            <input
              type="text"
              placeholder={t('shopping.auto_prop_n45fb4069', '搜索店铺内商品')}
              className="bg-transparent flex-1 text-[14px] text-text-main outline-none"
            />
          </div>
        </div>

        {/* Products Grid */}
        <div className="p-3 columns-2 gap-3 space-y-3 pb-safe">
          {products.map((p) => (
            <div
              key={p.id}
              className="bg-chat-other-bg rounded-xl overflow-hidden shadow-sm border border-border-color/30 flex flex-col active:scale-[0.98] transition-transform cursor-pointer break-inside-avoid"
              onClick={() => navigate(`/product/${p.id}`)}
            >
              <div className="w-full relative bg-chat-other-bg">
                <img src={p.image} className="w-full h-auto object-cover" />
              </div>
              <div className="p-3 flex flex-col py-3">
                <span className="text-[14px] text-text-main font-medium leading-tight mb-2 line-clamp-2">
                  {p.title}
                </span>
                <div className="flex items-center justify-between mt-auto">
                  <span className="text-[#FA5151] font-bold text-[16px]">
                    <span className="text-[12px]">¥</span>
                    {p.price}
                  </span>
                  <span className="text-[11px] text-text-sub">{p.sales}</span>
                </div>
              </div>
            </div>
          ))}
          {products.length === 0 && (
            <div className="col-span-2 py-20 flex flex-col items-center justify-center text-text-sub opacity-70 break-inside-avoid">
              <Package className="w-12 h-12 mb-3 stroke-current opacity-40" />
              <span className="text-[14px]">{t('shopping.auto_30220859', '暂无商品')}</span>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
