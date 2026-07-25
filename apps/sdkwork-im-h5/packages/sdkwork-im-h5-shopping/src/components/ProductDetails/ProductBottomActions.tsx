import { useTranslation } from "react-i18next";
import React from "react";
import { ShoppingCart, Store, Headphones } from "lucide-react";
import { useNavigate } from "react-router";
import { Shop } from "../../types";

export const ProductBottomActions = ({
  shop,
  cartItemCount,
  onAddToCartClick,
  onBuyNowClick,
}: {
  shop: Shop | null;
  cartItemCount: number;
  onAddToCartClick: () => void;
  onBuyNowClick: () => void;
}) => {
  const navigate = useNavigate();

  return (
    <div className="absolute bottom-0 left-0 right-0 bg-chat-other-bg border-t border-border-color pb-safe px-2 py-2 flex items-center z-40">
      <div className="flex gap-4 pr-3 border-r border-border-color/50 px-2">
        {shop && (
          <>
            <div
              className="flex flex-col items-center justify-center text-text-sub cursor-pointer"
              onClick={() => navigate(`/shop/${shop.id}`)}
            >
              <Store className="w-5 h-5 mb-1" />
              <span className="text-[10px]">店铺</span>
            </div>
            <div
              className="flex flex-col items-center justify-center text-text-sub cursor-pointer"
              onClick={() => navigate(`/shop-chat/${shop.id}`)}
            >
              <Headphones className="w-5 h-5 mb-1" />
              <span className="text-[10px]">客服</span>
            </div>
          </>
        )}
        <div
          className="flex flex-col items-center justify-center text-text-sub cursor-pointer relative"
          onClick={() => navigate("/cart")}
        >
          <ShoppingCart className="w-5 h-5 mb-1 ml-1 text-text-main" />
          <span className="text-[10px] ml-1">购物车</span>
          {cartItemCount > 0 && (
            <span className="absolute -top-1 right-0 bg-[#FA5151] text-white text-[10px] scale-[0.8] px-1.5 py-0.5 rounded-full border border-white">
              {cartItemCount}
            </span>
          )}
        </div>
      </div>
      <div className="flex-1 flex gap-2 pl-3">
        <button
          className="flex-1 py-2 rounded-full text-[14px] font-medium bg-[#FFAA00] text-white active:scale-95 transition-transform"
          onClick={onAddToCartClick}
        >
          加入购物车
        </button>
        <button
          className="flex-1 py-2 rounded-full text-[14px] font-medium bg-[#FA5151] text-white active:scale-95 transition-transform"
          onClick={onBuyNowClick}
        >
          立即购买
        </button>
      </div>
    </div>
  );
};
