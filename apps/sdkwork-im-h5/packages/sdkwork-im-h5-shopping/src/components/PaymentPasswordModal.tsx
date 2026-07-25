import React from "react";
import { useTranslation } from "react-i18next";

interface PaymentPasswordModalProps {
  amount: string;
  password: string;
  setPassword: (val: string) => void;
  onClose: () => void;
  onSubmit: () => void;
}

export const PaymentPasswordModal: React.FC<PaymentPasswordModalProps> = ({
  amount,
  password,
  setPassword,
  onClose,
  onSubmit,
}) => {
  const { t } = useTranslation();

  return (
    <div className="fixed inset-0 bg-black/50 z-50 flex items-center justify-center">
      <div className="bg-chat-other-bg border border-border-color/60 text-text-main rounded-xl w-[300px] p-6 flex flex-col items-center">
        <h3 className="text-[18px] font-medium mb-4 text-text-main">
          {t("shopping.auto_n49a5d7d3", "请输入支付密码")}
        </h3>
        <p className="text-[28px] font-bold mb-6 text-text-main">¥{amount}</p>
        <input
          type="password"
          maxLength={6}
          className="w-full h-12 border border-border-color bg-bg-color text-text-main rounded-lg text-center text-[24px] tracking-[1em] outline-none focus:border-[#07C160]"
          value={password}
          onChange={(e) => setPassword(e.target.value.replace(/\D/g, ""))}
          autoFocus
        />
        <div className="flex gap-4 w-full mt-6">
          <button
            className="flex-1 h-10 border border-border-color text-text-main rounded-lg active:bg-hover-bg"
            onClick={onClose}
          >
            {t("shopping.auto_a9472", "取消")}
          </button>
          <button
            className="flex-1 h-10 bg-[#07C160] text-white rounded-lg active:opacity-90"
            onClick={onSubmit}
          >
            {t("shopping.auto_f20f6", "确认")}
          </button>
        </div>
      </div>
    </div>
  );
};
