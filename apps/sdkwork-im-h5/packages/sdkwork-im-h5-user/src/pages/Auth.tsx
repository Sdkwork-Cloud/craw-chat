import React, { useState, useEffect } from "react";
import { useNavigate, useLocation } from "react-router";
import { cn, showToast } from "@sdkwork/im-h5-commons";
import { AuthService } from "../services/AuthService";
import { useTranslation } from "react-i18next";

import { TermsModal } from "../components/TermsModal";
import { ThirdPartyLogin } from "../components/ThirdPartyLogin";
import { AuthHeader } from "../components/Auth/AuthHeader";
import { AuthFooter } from "../components/Auth/AuthFooter";
import { AuthFormInputs } from "../components/Auth/AuthFormInputs";
import { AuthMode } from "../components/Auth/types";

export const AuthPage = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const location = useLocation();
  const [mode, setMode] = useState<AuthMode>("login-pwd");

  const [account, setAccount] = useState("");
  const [password, setPassword] = useState("");
  const [code, setCode] = useState("");
  const [agreed, setAgreed] = useState(false);
  const [showPwd, setShowPwd] = useState(false);
  const [showTerms, setShowTerms] = useState<string | null>(null);

  const [countdown, setCountdown] = useState(0);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    let timer: NodeJS.Timeout | null = null;
    if (countdown > 0) {
      timer = setInterval(() => setCountdown((c) => c - 1), 1000);
    }
    return () => clearInterval(timer);
  }, [countdown]);

  const handleSendCode = async () => {
    if (!account) return showToast(t("auth.enter_account", "请输入账号"));
    setCountdown(60);
    await AuthService.sendCode(account);
    showToast(t("auth.code_sent", "验证码已发送，请查收"));
  };

  const getRedirectPath = () => {
    const searchParams = new URLSearchParams(location.search);
    const redirectParam = searchParams.get("redirect");
    return redirectParam ? decodeURIComponent(redirectParam) : "/";
  };

  const handleSubmit = async () => {
    if (!agreed) return showToast(t("auth.agree_terms_first", "请先阅读并同意条款"));
    if (!account) return showToast(t("auth.enter_valid_account", "请输入正确的账号"));

    setLoading(true);
    try {
      if (mode === "login-pwd") {
        if (!password) {
          setLoading(false);
          return showToast(t("auth.enter_password", "请输入密码"));
        }
        await AuthService.login(account, password);
        navigate(getRedirectPath(), { replace: true });
      } else if (mode === "login-code") {
        if (!code) {
          setLoading(false);
          return showToast(t("auth.enter_code", "请输入验证码"));
        }
        await AuthService.login(account, undefined, code);
        navigate(getRedirectPath(), { replace: true });
      } else if (mode === "register") {
        if (!code) {
          setLoading(false);
          return showToast(t("auth.enter_code", "请输入验证码"));
        }
        await AuthService.register(account, code, password);
        navigate(getRedirectPath(), { replace: true });
      } else if (mode === "forgot") {
        if (!code) {
          setLoading(false);
          return showToast(t("auth.enter_code", "请输入验证码"));
        }
        if (!password) {
          setLoading(false);
          return showToast(t("auth.enter_new_password", "请输入新密码"));
        }
        await AuthService.resetPassword(account, code, password);
        showToast(t("auth.password_reset_success", "密码重置成功，请重新登录"));
        setMode("login-pwd");
      }
    } catch (err) {
      const error = err as Error;
      showToast(error.message || t("auth.operation_failed", "操作失败"));
    } finally {
      setLoading(false);
    }
  };

  const handleThirdPartyLogin = (platform: string) => {
    if (!agreed) return showToast(t("auth.agree_terms_first", "请先阅读并同意条款"));
    showToast(`${t("auth.redirecting_to", "正在跳转至")}${platform}${t("auth.logging_in", "登录...")}`);
    setTimeout(() => {
      navigate(getRedirectPath(), { replace: true });
    }, 1500);
  };

  // Switch mode helper
  const changeMode = (m: AuthMode) => {
    setMode(m);
    setAccount("");
    setPassword("");
    setCode("");
  };

  const isFormValid =
    account.length > 0 &&
    (mode === "login-pwd"
      ? password.length > 0
      : mode === "login-code"
        ? code.length > 0
        : mode === "register"
          ? code.length > 0
          : code.length > 0 && password.length > 0) &&
    agreed;

  return (
    <div className="flex flex-col h-full bg-bg-color pt-safe relative overflow-y-auto">
      <div className="flex-1 flex flex-col justify-center py-8 px-8 min-h-[500px]">
        <AuthHeader mode={mode} />

        <div className="flex flex-col w-full">
          <AuthFormInputs
            mode={mode}
            account={account}
            setAccount={setAccount}
            password={password}
            setPassword={setPassword}
            code={code}
            setCode={setCode}
            showPwd={showPwd}
            setShowPwd={setShowPwd}
            countdown={countdown}
            handleSendCode={handleSendCode}
          />

          <div className="mt-8 flex flex-col gap-5">
            <button
              className={cn(
                "w-full h-12 rounded-lg text-[17px] font-medium transition-all text-white active:scale-[0.98]",
                isFormValid
                  ? "bg-[#07C160] shadow-md shadow-[#07C160]/20"
                  : "bg-[#E5E5E5] dark:bg-[#2C2C2C] text-[#B2B2B2] dark:text-[#5B5B5B]",
              )}
              disabled={loading || !isFormValid}
              onClick={handleSubmit}
            >
              {loading
                ? t("auth.please_wait", "请稍候...")
                : mode.startsWith("login")
                  ? t("auth.agree_and_login", "同意并登录")
                  : mode === "register"
                    ? t("auth.agree_and_register", "同意并注册")
                    : t("auth.confirm", "确认")}
            </button>

            <div className="flex justify-between items-center text-[14px] text-[#576B95] px-1 font-medium">
              {mode === "login-pwd" && (
                <span
                  className="cursor-pointer active:opacity-70"
                  onClick={() => changeMode("login-code")}
                >
                  {t("auth.login_with_code", "用验证码登录")}
                </span>
              )}
              {mode === "login-code" && (
                <span
                  className="cursor-pointer active:opacity-70"
                  onClick={() => changeMode("login-pwd")}
                >
                  {t("auth.login_with_pwd", "用密码登录")}
                </span>
              )}
              {(mode === "login-pwd" || mode === "login-code") && (
                <div className="flex gap-4">
                  <span
                    className="cursor-pointer active:opacity-70"
                    onClick={() => changeMode("forgot")}
                  >
                    {t("auth.forgot_password", "找回密码")}
                  </span>
                  <span
                    className="cursor-pointer active:opacity-70"
                    onClick={() => changeMode("register")}
                  >
                    {t("auth.register_account", "注册账号")}
                  </span>
                </div>
              )}
              {(mode === "register" || mode === "forgot") && (
                <span
                  className="cursor-pointer active:opacity-70"
                  onClick={() => changeMode("login-pwd")}
                >
                  {t("auth.back_to_login", "返回登录")}
                </span>
              )}
            </div>

            {/* Third-party Login */}
            <ThirdPartyLogin mode={mode} onLogin={handleThirdPartyLogin} />
          </div>
        </div>
      </div>

      <AuthFooter agreed={agreed} setAgreed={setAgreed} setShowTerms={setShowTerms} />

      <TermsModal showTerms={showTerms} onClose={() => setShowTerms(null)} />
    </div>
  );
};
