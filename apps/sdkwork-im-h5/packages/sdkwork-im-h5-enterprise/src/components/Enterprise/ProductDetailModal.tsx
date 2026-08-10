import React from "react";
import { motion } from "motion/react";
import { X, Package } from "lucide-react";
import { useTranslation } from "react-i18next";

export interface ProductDetailModalProps {
  product: {
    name: string;
    desc: string;
    price: string;
  } | null;
  onClose: () => void;
  onConsult: () => void;
}

export const ProductDetailModal: React.FC<ProductDetailModalProps> = ({
  product,
  onClose,
  onConsult,
}) => {
  const { t } = useTranslation();

  if (!product) return null;

  return (
    <div className="fixed inset-0 z-50 flex flex-col justify-end">
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        transition={{ duration: 0.2 }}
        className="absolute inset-0 bg-black/40 backdrop-blur-[2px]"
        onClick={onClose}
      />
      <motion.div
        initial={{ y: "100%" }}
        animate={{ y: 0 }}
        exit={{ y: "100%" }}
        transition={{ type: "spring", damping: 25, stiffness: 200 }}
        className="relative bg-bg-color rounded-t-2xl overflow-hidden pb-safe flex flex-col max-h-[85vh]"
      >
        <div className="flex items-center justify-between p-4 bg-chat-other-bg">
          <h3 className="text-[17px] font-bold text-text-main">
            {t("enterprise.auto_25105619", "产品详情")}
          </h3>
          <div
            className="w-8 h-8 rounded-full bg-bg-color flex items-center justify-center cursor-pointer"
            onClick={onClose}
          >
            <X className="w-5 h-5 text-text-sub" />
          </div>
        </div>
        <div className="overflow-y-auto p-5">
          <div className="w-24 h-24 bg-chat-other-bg rounded-2xl flex items-center justify-center mb-5 border border-border-color/30 shadow-sm mx-auto">
            <Package className="w-12 h-12 text-primary-blue/60" />
          </div>
          <h2 className="text-[20px] font-bold text-text-main text-center mb-2">
            {product.name}
          </h2>
          <div className="text-center mb-6">
            <span className="text-[20px] font-extrabold text-[#FF7D00]">
              {product.price}
            </span>
          </div>
          <div className="bg-chat-other-bg rounded-xl p-4 shadow-sm">
            <h4 className="text-[14px] font-bold text-text-main mb-2">
              {t("enterprise.auto_250baa7b", "产品描述")}
            </h4>
            <p className="text-[14px] text-text-sub leading-relaxed whitespace-pre-wrap">
              {product.desc}
            </p>
          </div>
        </div>
        <div className="p-4 bg-chat-other-bg border-t border-border-color">
          <button
            className="w-full bg-gradient-to-r from-blue-500 to-indigo-600 text-white rounded-full py-3.5 text-[16px] font-bold shadow-lg shadow-blue-500/30 active:scale-[0.98] transition-transform"
            onClick={onConsult}
          >
            {t("enterprise.auto_289280b4", "咨询产品")}
          </button>
        </div>
      </motion.div>
    </div>
  );
};
