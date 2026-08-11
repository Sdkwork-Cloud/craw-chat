import React from "react";
import { cn } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";

export const AuthFooter = ({
  agreed,
  setAgreed,
  setShowTerms,
}: {
  agreed: boolean;
  setAgreed: (agreed: boolean) => void;
  setShowTerms: (term: string) => void;
}) => {
  const { t } = useTranslation();

  return (
    <div className="pb-10 px-8 flex items-start gap-2">
      <div
        className={cn(
          "w-[18px] h-[18px] mt-0.5 rounded-full border flex items-center justify-center shrink-0 cursor-pointer transition-colors",
          agreed ? "bg-[#07C160] border-[#07C160]" : "border-text-sub/40"
        )}
        onClick={() => setAgreed(!agreed)}
      >
        {agreed && <div className="w-1.5 h-1.5 bg-white rounded-full" />}
      </div>
      <p className="text-[12px] text-text-sub leading-relaxed">
        {t("user.auto_nfa1f23a", '{t("auth.read_and_agree", "I have read and agree to")}{" "}')}
        <span
          className="text-[#576B95] active:opacity-70 cursor-pointer"
          onClick={() => setShowTerms(t("auth.terms_of_service", "Software License and Service Agreement"))}
        >
          {t("auth.terms_of_service", "Software License and Service Agreement")}
        </span>{" "}
        {t("auth.and", "and")}{" "}
        <span
          className="text-[#576B95] active:opacity-70 cursor-pointer"
          onClick={() => setShowTerms(t("auth.privacy_policy", "Privacy Protection Guidelines"))}
        >
          {t("auth.privacy_policy", "Privacy Protection Guidelines")}
        </span>
      </p>
    </div>
  );
};
