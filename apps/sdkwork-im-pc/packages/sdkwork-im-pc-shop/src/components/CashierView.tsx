import React from "react";
import { motion } from "motion/react";
import { Store, ArrowLeft, Wallet } from "lucide-react";
import { useTranslation } from "react-i18next";
import { PC_SHOP_PAYMENT_CONTRACT_UNAVAILABLE } from "../services/ShopService";

export const CashierView = ({
  amount,
  orderId,
  onCancel,
}: {
  amount: number;
  orderId: string;
  onCancel: () => void;
}) => {
  const { t, i18n } = useTranslation(["checkout", "common"]);

  return (
    <motion.div
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: -20 }}
      className="absolute inset-0 bg-[#f5f5f5] dark:bg-[#1e1e20] z-30 flex flex-col"
    >
      <div className="h-16 bg-white dark:bg-[#2b2b2d] border-b border-gray-200 dark:border-white/5 flex items-center px-10 shrink-0">
        <div className="flex items-center gap-4">
          <button
            type="button"
            onClick={onCancel}
            className="p-2 -ml-2 text-gray-500 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-white/10 dark:hover:text-white rounded-full transition-colors"
          >
            <ArrowLeft size={20} />
          </button>
          <div className="flex items-center gap-2">
            <Store size={24} className="text-pink-600" />
            <span className="text-xl font-medium text-gray-900 dark:text-gray-100">
              {t("checkout:cashierTitle")}
            </span>
          </div>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-8 flex justify-center custom-scrollbar">
        <div className="w-full max-w-3xl space-y-6">
          <div className="bg-white dark:bg-[#2b2b2d] rounded-xl p-8 shadow-sm border border-gray-100 dark:border-white/5">
            <div className="text-gray-900 dark:text-gray-100 font-medium text-lg mb-3">
              {t("checkout:orderTitle")}
            </div>
            <div className="text-sm text-gray-500 flex flex-col gap-2">
              <span>{t("checkout:orderNo", { orderId })}</span>
              <span>
                {t("checkout:amountPayable")}: {t("common:currencySymbol")}
                {amount.toLocaleString(
                  i18n.language === "en-US" ? "en-US" : "zh-CN",
                  { minimumFractionDigits: 2 },
                )}
              </span>
            </div>
          </div>

          <div className="bg-white dark:bg-[#2b2b2d] rounded-xl p-10 shadow-sm border border-gray-100 dark:border-white/5 flex flex-col items-center text-center gap-4">
            <div className="w-16 h-16 rounded-full bg-pink-500/10 flex items-center justify-center">
              <Wallet size={28} className="text-pink-500" />
            </div>
            <p className="text-gray-700 dark:text-gray-200 font-medium">
              {PC_SHOP_PAYMENT_CONTRACT_UNAVAILABLE}
            </p>
            <p className="text-sm text-gray-500 dark:text-gray-400">
              {t("checkout:paymentUnavailable", { defaultValue: "支付合约暂未开放，订单已创建后请通过正式支付渠道完成付款。" })}
            </p>
          </div>
        </div>
      </div>
    </motion.div>
  );
};
