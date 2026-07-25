import { useTranslation } from "react-i18next";
import React from "react";
import { Package } from "lucide-react";
import { motion } from "motion/react";

export const EnterpriseProductsTab = ({
  products,
  onSelectProduct,
}: {
  products: any[];
  onSelectProduct: (p: any) => void;
}) => {
  return (
    <motion.div
      key="products"
      initial={{ opacity: 0, y: 5 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0 }}
      transition={{ duration: 0.2 }}
      className="bg-white dark:bg-[#2c2d2e] flex flex-col"
    >
      {products.map((prod, i) => (
        <div
          key={i}
          className="p-4 border-b border-border-color/50 flex gap-4 cursor-pointer active:bg-chat-active-bg transition-colors group"
          onClick={() => onSelectProduct(prod)}
        >
          <div className="w-24 h-24 bg-[#f5f6f8] dark:bg-[#1a1b1c] rounded-xl flex items-center justify-center shrink-0 border border-border-color/30 group-hover:shadow-md transition-shadow">
            <Package className="w-10 h-10 text-primary-blue/60" />
          </div>
          <div className="flex flex-col flex-1 py-1">
            <h3 className="text-[16px] font-bold text-text-main line-clamp-1 mb-1.5 group-hover:text-primary-blue transition-colors">
              {prod.name}
            </h3>
            <p className="text-[13px] text-text-sub line-clamp-2 leading-relaxed flex-1">
              {prod.desc}
            </p>
            <span className="text-[15px] font-extrabold text-[#FF7D00] leading-none mt-2">
              {prod.price}
            </span>
          </div>
        </div>
      ))}
    </motion.div>
  );
};
