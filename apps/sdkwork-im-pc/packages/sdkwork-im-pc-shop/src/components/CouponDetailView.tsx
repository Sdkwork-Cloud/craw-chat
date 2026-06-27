import React from "react";
import { motion } from "motion/react";
import { ArrowLeft, Ticket, Store, Info, Phone } from "lucide-react";
import { ShopProduct, CartItem } from "../services/ShopService";
import QRCode from "react-qr-code";
import { useTranslation } from "react-i18next";

export const CouponDetailView = ({
  product,
  item,
  code,
  onBack,
}: {
  product: ShopProduct;
  item: CartItem;
  code: string;
  onBack: () => void;
}) => {
  const { t } = useTranslation(["checkout", "common"]);
  const skuName =
    item.skuId && product.skus
      ? product.skus.find((s) => s.id === item.skuId)?.name
      : "";

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      className="p-6 w-full max-w-md mx-auto h-full flex flex-col pt-12"
    >
      <div className="flex items-center gap-3 mb-6">
        <button
          onClick={onBack}
          className="p-2 hover:bg-white/10 rounded-full transition-colors text-gray-400 hover:text-white"
        >
          <ArrowLeft size={20} />
        </button>
        <h2 className="text-xl font-bold text-gray-100 flex items-center gap-2">
          {t("checkout:couponDetail", { defaultValue: "卡券详情" })}
        </h2>
      </div>

      <div className="bg-[#2b2b2d] rounded-3xl border border-white/5 overflow-hidden flex flex-col">
        <div className="h-32 bg-gradient-to-br from-pink-500 to-pink-700 p-6 relative overflow-hidden flex flex-col justify-end">
          <Store
            size={120}
            className="absolute -right-4 -bottom-4 text-white/10"
          />
          <h3 className="text-2xl font-bold text-white relative z-10">
            {product.title}
          </h3>
          {skuName && (
            <div className="text-white/80 font-medium relative z-10 mt-1">
              {skuName}
            </div>
          )}
        </div>

        <div className="p-8 flex flex-col items-center relative border-b border-dashed border-white/20">
          <div className="absolute -top-4 -left-4 w-8 h-8 rounded-full bg-[#1e1e20]" />
          <div className="absolute -top-4 -right-4 w-8 h-8 rounded-full bg-[#1e1e20]" />

          <div className="text-center mb-6">
            <div className="text-sm text-gray-400 mb-2">
              {t("checkout:showQrCode", {
                defaultValue: "向商家出示此二维码核销",
              })}
            </div>
            <div className="bg-white p-4 rounded-xl inline-block w-48 h-48">
              <QRCode
                value={code}
                size={160}
                style={{ height: "auto", maxWidth: "100%", width: "100%" }}
                viewBox={`0 0 160 160`}
              />
            </div>
          </div>

          <div className="w-full bg-[#1e1e20] rounded-xl p-4 flex items-center justify-between">
            <span className="text-gray-400 text-sm">
              {t("checkout:couponCode", { defaultValue: "券码：" })}
            </span>
            <span className="font-mono text-gray-200 tracking-wider font-bold text-lg">
              {code}
            </span>
          </div>
        </div>

        <div className="p-6 space-y-4 bg-[#252527]">
          <div>
            <div className="text-gray-400 text-xs mb-1 flex items-center gap-1.5">
              <Info size={14} />{" "}
              {t("checkout:usageRules", { defaultValue: "使用须知" })}
            </div>
            <div className="text-gray-300 text-sm leading-relaxed">
              {product.description}
              <br />
              {t("checkout:usageRulesDesc", {
                defaultValue: "请在结账前出示此二维码或券码。",
              })}
            </div>
          </div>
          <div>
            <div className="text-gray-400 text-xs mb-1 flex items-center gap-1.5">
              <Store size={14} />{" "}
              {t("checkout:applicableStores", { defaultValue: "适用门店" })}
            </div>
            <div className="text-gray-300 text-sm">
              {t("checkout:allStores", { defaultValue: "全国所有直营门店" })}
            </div>
          </div>
          <div>
            <div className="text-gray-400 text-xs mb-1 flex items-center gap-1.5">
              <Phone size={14} />{" "}
              {t("checkout:customerService", { defaultValue: "客服热线" })}
            </div>
            <div className="text-gray-300 text-sm">400-800-8888</div>
          </div>
        </div>
      </div>
    </motion.div>
  );
};
