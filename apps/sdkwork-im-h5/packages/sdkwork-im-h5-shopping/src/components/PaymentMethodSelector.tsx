import React from "react";
import { useTranslation } from "react-i18next";

interface PaymentMethodSelectorProps {
  selectedPayment: string;
  setSelectedPayment: (payment: string) => void;
}

export const PaymentMethodSelector: React.FC<PaymentMethodSelectorProps> = ({
  selectedPayment,
  setSelectedPayment,
}) => {
  const { t } = useTranslation();

  return (
    <>
      <div className="text-[14px] text-text-sub mb-3 ml-1">
        {t("shopping.auto_768acc76", "请选择支付方式")}
      </div>

      <div className="bg-chat-other-bg rounded-xl overflow-hidden shadow-sm">
        {/* WeChat Pay */}
        <div
          className="flex items-center justify-between p-4 border-b border-border-color/50 active:bg-chat-active-bg transition-colors cursor-pointer"
          onClick={() => setSelectedPayment("wechat")}
        >
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-full bg-[#07C160]/10 flex items-center justify-center">
              <svg
                className="w-5 h-5 text-[#07C160]"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path d="M8.5,14.5 C7.11928813,14.5 6,13.3807119 6,12 C6,10.6192881 7.11928813,9.5 8.5,9.5 C9.88071187,9.5 11,10.6192881 11,12 C11,13.3807119 9.88071187,14.5 8.5,14.5 Z M15.5,14.5 C14.1192881,14.5 13,13.3807119 13,12 C13,10.6192881 14.1192881,9.5 15.5,9.5 C16.8807119,9.5 18,10.6192881 18,12 C18,13.3807119 16.8807119,14.5 15.5,14.5 Z M12,2 C17.5228475,2 22,6.4771525 22,12 C22,17.5228475 17.5228475,22 12,22 C6.4771525,22 2,17.5228475 2,12 C2,6.4771525 6.4771525,2 12,2 Z" />
              </svg>
            </div>
            <span className="text-[15px] text-text-main font-medium">
              {t("shopping.auto_2cb6c4bc", "微信支付")}
            </span>
          </div>
          <div
            className={`w-5 h-5 rounded-full border flex items-center justify-center ${selectedPayment === "wechat" ? "bg-[#07C160] border-[#07C160]" : "border-border-color bg-bg-color"}`}
          >
            {selectedPayment === "wechat" && (
              <svg
                className="w-3 h-3 text-white"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={3}
                  d="M5 13l4 4L19 7"
                />
              </svg>
            )}
          </div>
        </div>

        {/* Alipay */}
        <div
          className="flex items-center justify-between p-4 border-b border-border-color/50 active:bg-chat-active-bg transition-colors cursor-pointer"
          onClick={() => setSelectedPayment("alipay")}
        >
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-full bg-[#1677FF]/10 flex items-center justify-center">
              <svg
                className="w-5 h-5 text-[#1677FF]"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm-1-13h2v6h-2zm0 8h2v2h-2z" />
              </svg>
            </div>
            <span className="text-[15px] text-text-main font-medium">
              {t("shopping.auto_185bd34", "支付宝")}
            </span>
          </div>
          <div
            className={`w-5 h-5 rounded-full border flex items-center justify-center ${selectedPayment === "alipay" ? "bg-[#07C160] border-[#07C160]" : "border-border-color bg-bg-color"}`}
          >
            {selectedPayment === "alipay" && (
              <svg
                className="w-3 h-3 text-white"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={3}
                  d="M5 13l4 4L19 7"
                />
              </svg>
            )}
          </div>
        </div>

        {/* Balance */}
        <div
          className="flex items-center justify-between p-4 active:bg-chat-active-bg transition-colors cursor-pointer"
          onClick={() => setSelectedPayment("balance")}
        >
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-full bg-[#FFAA00]/10 flex items-center justify-center">
              <svg
                className="w-5 h-5 text-[#FFAA00]"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm-1.5 5.5v2h-2v2h2v4h3v-2h-2.5v-2h2.5c1.1 0 2-.9 2-2s-.9-2-2-2h-3v-2h-1.5z" />
              </svg>
            </div>
            <div className="flex flex-col">
              <span className="text-[15px] text-text-main font-medium">
                {t("shopping.auto_12dc7b", "零钱")}
              </span>
              <span className="text-[12px] text-text-sub mt-0.5">
                {t("shopping.auto_23354321", "可用余额 ¥120.00")}
              </span>
            </div>
          </div>
          <div
            className={`w-5 h-5 rounded-full border flex items-center justify-center ${selectedPayment === "balance" ? "bg-[#07C160] border-[#07C160]" : "border-border-color bg-bg-color"}`}
          >
            {selectedPayment === "balance" && (
              <svg
                className="w-3 h-3 text-white"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={3}
                  d="M5 13l4 4L19 7"
                />
              </svg>
            )}
          </div>
        </div>
      </div>
    </>
  );
};
