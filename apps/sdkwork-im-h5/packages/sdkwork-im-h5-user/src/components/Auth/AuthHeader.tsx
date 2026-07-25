import React from "react";
import { MessageCircle } from "lucide-react";
import { useTranslation } from "react-i18next";
import { AuthMode } from "./types";

export const AuthHeader = ({ mode }: { mode: AuthMode }) => {
  const { t } = useTranslation();
return (
    <div className="flex flex-col items-center mb-10">
      <div className="w-16 h-16 bg-[#07C160] rounded-2xl flex items-center justify-center mb-4 shadow-sm">
        <MessageCircle className="w-10 h-10 text-white fill-white" />
      </div>
      <h1 className="text-2xl font-semibold text-text-main text-center">
        {mode === "login-pwd" && t("auth.mode_login_pwd", "密码登录")}
        {mode === "login-code" && t("auth.mode_login_code", "验证码登录")}
        {mode === "register" && t("auth.mode_register", "手机号注册")}
        {mode === "forgot" && t("auth.mode_forgot", "找回密码")}
      </h1>
    </div>
  );
};
