import { useTranslation } from "react-i18next";
import React from "react";
import { ChevronRight } from "lucide-react";
import { useNavigate } from "react-router";
import { Shop } from "../../types";

export const ProductShopCard = ({ shop }: { shop: Shop }) => {
  const { t } = useTranslation();
const navigate = useNavigate();

  return (
    <div
      className="bg-chat-other-bg p-4 mb-2 cursor-pointer active:bg-chat-active-bg transition-colors"
      onClick={() => navigate(`/shop/${shop.id}`)}
    >
      <div className="flex items-center gap-3 mb-3">
        <img
          src={shop.logo}
          className="w-12 h-12 rounded-lg border border-border-color/30 object-cover"
          alt={shop.name}
        />
        <div className="flex-1 flex flex-col justify-center">
          <div className="flex items-center gap-1.5">
            <span className="text-[16px] font-medium text-text-main leading-tight">
              {shop.name}
            </span>
            {shop.isOfficial && (
              <span className="bg-[#FA5151] text-white text-[10px] px-1 py-0.5 rounded-sm leading-none">官方</span>
            )}
          </div>
          <div className="flex items-center gap-3 text-[12px] text-text-sub mt-1">
            <span>粉丝数 {shop.fansCount}</span>
            <span>综合评分 {shop.rating}</span>
          </div>
        </div>
        <ChevronRight className="w-5 h-5 text-text-sub/60" />
      </div>
      <div className="flex items-center gap-4">
        <div
          className="flex-1 h-8 rounded-full border border-border-color flex items-center justify-center text-[13px] text-text-main hover:bg-chat-other-bg transition-colors"
          onClick={(e) => {
            e.stopPropagation();
            navigate(`/shop/${shop.id}`);
          }}
        >
          进店逛逛
        </div>
        <div
          className="flex-1 h-8 rounded-full border border-border-color flex items-center justify-center text-[13px] text-text-main hover:bg-chat-other-bg transition-colors"
          onClick={(e) => {
            e.stopPropagation();
            navigate(`/shop-chat/${shop.id}`);
          }}
        >
          联系客服
        </div>
      </div>
    </div>
  );
};
