import React from "react";
import { Eye, EyeOff } from "lucide-react";
import { motion } from "motion/react";
import { useTranslation } from "react-i18next";
import { AuthMode } from "./types";

interface AuthFormInputsProps {
  mode: AuthMode;
  account: string;
  setAccount: (val: string) => void;
  password: string;
  setPassword: (val: string) => void;
  code: string;
  setCode: (val: string) => void;
  showPwd: boolean;
  setShowPwd: (val: boolean) => void;
  countdown: number;
  handleSendCode: () => void;
}

export const AuthFormInputs: React.FC<AuthFormInputsProps> = ({
  mode,
  account,
  setAccount,
  password,
  setPassword,
  code,
  setCode,
  showPwd,
  setShowPwd,
  countdown,
  handleSendCode,
}) => {
  const { t } = useTranslation();

  return (
    <div className="flex flex-col gap-4 w-full">
      {/* Account Input */}
      <div className="flex items-center border-b border-border-color py-3 focus-within:border-[#07C160] transition-colors group">
        <span className="text-[16px] text-text-main mr-4 font-medium opacity-50">
          {t("auth.account", "Account")}
        </span>
        <input
          type="text"
          placeholder={t("auth.account_placeholder", "Phone number or email")}
          value={account}
          onChange={(e) => setAccount(e.target.value.trim())}
          className="flex-1 bg-transparent text-[16px] text-text-main outline-none placeholder:text-text-sub/50"
        />
      </div>

      {/* Password Input */}
      {(mode === "login-pwd" || mode === "register" || mode === "forgot") && (
        <motion.div
          initial={{ height: 0, opacity: 0 }}
          animate={{ height: "auto", opacity: 1 }}
          exit={{ height: 0, opacity: 0 }}
          className="flex items-center border-b border-border-color py-3 focus-within:border-[#07C160] transition-colors"
        >
          <input
            type={showPwd ? "text" : "password"}
            placeholder={
              mode === "forgot"
                ? t("auth.set_new_password", "Set a new password")
                : t("auth.enter_password", "Enter your password")
            }
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            className="flex-1 bg-transparent text-[16px] text-text-main outline-none placeholder:text-text-sub/50"
          />
          <div
            onClick={() => setShowPwd(!showPwd)}
            className="pl-4 pr-1 text-text-sub/50 active:scale-90 transition-transform cursor-pointer"
          >
            {showPwd ? <Eye className="w-5 h-5" /> : <EyeOff className="w-5 h-5" />}
          </div>
        </motion.div>
      )}

      {/* Verification Code Input */}
      {(mode === "login-code" || mode === "register" || mode === "forgot") && (
        <motion.div
          initial={{ height: 0, opacity: 0 }}
          animate={{ height: "auto", opacity: 1 }}
          exit={{ height: 0, opacity: 0 }}
          className="flex items-center border-b border-border-color py-3 focus-within:border-[#07C160] transition-colors"
        >
          <input
            type="text"
            placeholder={t("auth.enter_code", "Enter the verification code")}
            maxLength={6}
            value={code}
            onChange={(e) => setCode(e.target.value.replace(/\D/g, ""))}
            className="flex-1 bg-transparent text-[16px] text-text-main outline-none placeholder:text-text-sub/50"
          />
          <button
            onClick={handleSendCode}
            disabled={countdown > 0}
            className="text-[#576B95] text-[15px] pl-4 font-medium disabled:opacity-50 active:opacity-70 transition-opacity"
          >
            {countdown > 0 ? `${countdown}s` : t("auth.get_code", "Get code")}
          </button>
        </motion.div>
      )}
    </div>
  );
};
