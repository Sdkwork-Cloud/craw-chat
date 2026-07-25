import React from "react";
import { useTranslation } from "react-i18next";
import { Shop } from "../types";

interface ShopHeaderBannerProps {
  shop: Shop;
}

export const ShopHeaderBanner: React.FC<ShopHeaderBannerProps> = ({ shop }) => {
  const { t } = useTranslation();

  return (
    <div className="w-full relative h-[180px] bg-gradient-to-br from-gray-700 to-gray-900 pt-safe">
      <img
        src={shop.logo}
        alt={shop.name}
        className="absolute inset-0 w-full h-full object-cover opacity-30"
      />
      <div className="absolute inset-0 bg-black/20" />
      <div className="absolute bottom-4 left-4 right-4 flex items-end gap-3 z-10">
        <img
          src={shop.logo}
          alt={shop.name}
          className="w-[60px] h-[60px] rounded-lg border-2 border-white object-cover shadow-sm bg-white"
        />
        <div className="flex-1 text-white pb-1">
          <h1 className="text-[18px] font-medium leading-tight mb-1 shadow-sm">
            {shop.name}
          </h1>
          <div className="flex items-center gap-2 text-[12px] opacity-90">
            <span>{t("shopping.fans_count", `粉丝 ${shop.fansCount}`)}</span>
            <span>{t("shopping.rating", `评价 ${shop.rating}`)}</span>
          </div>
        </div>
        <button className="h-7 px-4 rounded-full bg-[#FA5151] text-white text-[13px] font-medium active:scale-95 transition-transform mb-1 flex items-center justify-center">
          {t("shopping.follow", "关注")}
        </button>
      </div>
    </div>
  );
};
