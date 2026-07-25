import React from "react";
import { useTranslation } from "react-i18next";

interface ThirdPartyLoginProps {
  mode: string;
  onLogin: (platform: string) => void;
}

export const ThirdPartyLogin: React.FC<ThirdPartyLoginProps> = ({ mode, onLogin }) => {
  const { t } = useTranslation();
if (!mode.startsWith("login")) return null;

  return (
    <div className="mt-10 mb-4 flex flex-col items-center">
      <div className="flex items-center w-full mb-6 relative">
        <div className="flex-1 h-px bg-border-color"></div>
        <span className="px-4 text-[12px] text-text-sub bg-bg-color relative z-10 w-fit">
          {t("auth.other_login_methods", "其他开放平台登录")}
        </span>
        <div className="flex-1 h-px bg-border-color"></div>
      </div>
      <div className="flex items-center justify-center gap-8 w-full max-w-[240px]">
        {/* WeChat Icon */}
        <div 
          onClick={() => onLogin(t("auth.platform_wechat", "微信"))}
          className="w-12 h-12 rounded-full border border-border-color flex items-center justify-center bg-white dark:bg-[#2C2C2C] active:scale-95 transition-transform cursor-pointer"
        >
           <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M8.25 15.6C3.9615 15.6 0.5 12.35 0.5 8.35C0.5 4.35 4.095 1.1 8.35 1.1C12.605 1.1 16.2 4.35 16.2 8.35C16.2 12.28 12.82 15.6 8.25 15.6ZM10.5 19.35C10.5 19.35 13.9 19.35 16.2 17.5C19.98 17.5 23.1 15 23.1 11.25C23.1 7.5 19.467 4.9 15.5 4.9C15.352 4.9 15.205 4.908 15.06 4.922C16.892 5.922 18.1 7.788 18.1 10.05C18.1 13.58 14.735 16.6 10.5 16.6C10.155 16.6 9.82 16.57 9.49 16.512C9.774 18.528 10.5 19.35 10.5 19.35Z" fill="#07C160" />
              <ellipse cx="6" cy="6" rx="1" ry="1" fill="white" />
              <ellipse cx="10" cy="6" rx="1" ry="1" fill="white" />
           </svg>
        </div>
        {/* Apple Icon */}
        <div 
          onClick={() => onLogin("Apple")}
          className="w-12 h-12 rounded-full border border-border-color flex items-center justify-center bg-white dark:bg-[#2C2C2C] active:scale-95 transition-transform cursor-pointer"
        >
           <svg width="22" height="26" viewBox="0 0 22 26" fill="currentColor" xmlns="http://www.w3.org/2000/svg" className="text-black dark:text-white">
              <path d="M12.915 4.887C13.593 4.091 13.957 3.013 13.957 1.831C13.957 1.706 13.945 1.579 13.921 1.455C12.784 1.503 11.453 2.193 10.638 3.129C10.038 3.821 9.577 4.931 9.577 6.064C9.577 6.202 9.59 6.342 9.613 6.477C9.742 6.489 9.883 6.495 10.021 6.495C11.085 6.495 12.213 5.727 12.915 4.887ZM17.411 15.688C17.426 11.968 20.528 10.457 20.672 10.383C18.91 7.828 16.141 7.428 15.19 7.391C12.87 7.159 10.61 8.756 9.421 8.756C8.211 8.756 6.355 7.41 4.417 7.447C1.942 7.485 0 9.215 0 11.583C0 12.756 -0.003 14.542 1.341 17.513C2.261 19.539 3.513 22.043 5.485 22.107C7.387 22.169 8.125 20.973 10.315 20.973C12.484 20.973 13.167 22.107 15.148 22.107C17.065 22.107 18.17 19.789 19.117 17.729C20.219 15.342 20.675 14.161 20.702 14.027C20.658 14.015 17.394 15.197 17.411 15.688Z" />
           </svg>
        </div>
        {/* Google Icon alternative placeholder (using mail for now or simulated google) */}
        <div 
          onClick={() => onLogin("Google")}
          className="w-12 h-12 rounded-full border border-border-color flex items-center justify-center bg-white dark:bg-[#2C2C2C] active:scale-95 transition-transform cursor-pointer font-medium font-serif text-[24px] text-[#4285F4]"
        >
           G
        </div>
      </div>
    </div>
  );
};
