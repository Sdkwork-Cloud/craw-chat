import React from "react";
import { useTranslation } from "react-i18next";

export const CheckoutOrderSummary = ({
  totalPrice,
  isAllVirtual,
  onSubmitOrder,
  isSubmitting = false,
}: any) => {
  const { t, i18n } = useTranslation(["checkout", "common"]);

  return (
    <div className="w-full lg:w-[380px] shrink-0">
      <div className="bg-[#2b2b2d] rounded-3xl border border-white/5 p-8 sticky top-6 shadow-xl shadow-black/20">
        <h3 className="text-xl font-bold text-gray-100 mb-8 flex items-center gap-2">
          <div className="w-1.5 h-6 bg-pink-500 rounded-full"></div>
          {t("checkout:orderInfo")}
        </h3>

        <div className="space-y-5 text-base mb-8">
          <div className="flex justify-between text-gray-400 items-center">
            <span>{t("checkout:merchandiseTotal")}</span>
            <span className="font-medium text-gray-300">
              {t("common:currencySymbol")}{" "}
              {totalPrice.toLocaleString(
                i18n.language === "en-US" ? "en-US" : "zh-CN",
              )}
            </span>
          </div>
          <div className="flex justify-between text-gray-400 items-center">
            <span>{t("checkout:shipping")}</span>
            <span className="font-medium text-gray-300">
              {isAllVirtual
                ? t("checkout:noShipping")
                : t("checkout:freeShipping")}
            </span>
          </div>
          <div className="flex justify-between text-gray-400 items-center">
            <span>{t("checkout:discount")}</span>
            <span className="text-pink-500 font-medium">
              - {t("common:currencySymbol")} 0.00
            </span>
          </div>

          <div className="h-px bg-gradient-to-r from-transparent via-white/10 to-transparent my-6" />

          <div className="flex justify-between items-end">
            <span className="text-gray-200 font-medium pb-1.5">
              {t("checkout:actualPayment")}
            </span>
            <div className="flex flex-col items-end">
              <span className="text-4xl font-bold text-pink-500 tracking-tight">
                <span className="text-xl mr-1 font-semibold">
                  {t("common:currencySymbol")}
                </span>
                {totalPrice.toLocaleString(
                  i18n.language === "en-US" ? "en-US" : "zh-CN",
                )}
              </span>
            </div>
          </div>
        </div>

        <button
          type="button"
          disabled={isSubmitting}
          className="w-full py-4 bg-gradient-to-r from-pink-600 to-pink-500 hover:from-pink-500 hover:to-pink-400 disabled:opacity-60 text-white rounded-2xl font-bold text-lg transition-all shadow-[0_8px_16px_rgba(236,72,153,0.2)] hover:shadow-[0_8px_20px_rgba(236,72,153,0.3)] flex items-center justify-center gap-2 active:scale-[0.98] mt-4"
          onClick={() => void onSubmitOrder()}
        >
          {t("checkout:submitOrder", { defaultValue: "提交订单" })}
        </button>
      </div>
    </div>
  );
};
