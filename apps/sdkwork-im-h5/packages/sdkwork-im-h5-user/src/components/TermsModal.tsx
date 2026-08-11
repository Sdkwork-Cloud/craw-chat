import React from "react";
import { useTranslation } from "react-i18next";

interface TermsModalProps {
  showTerms: string | null;
  onClose: () => void;
}

/**
 * Terms dialog — fail-closed (PRD).
 *
 * No legally confirmed terms text is configured yet, so the dialog must not
 * fabricate mock legal content. It renders a typed "terms not configured"
 * notice until a real, compliance-approved agreement is supplied.
 */
export const TermsModal: React.FC<TermsModalProps> = ({ showTerms, onClose }) => {
  const { t } = useTranslation();
if (!showTerms) return null;

  return (
    <div
      className="fixed inset-0 z-50 bg-black/50 flex flex-col items-center justify-center p-6 pb-20"
      onClick={onClose}
    >
      <div
        className="bg-chat-other-bg w-full max-w-[320px] rounded-2xl flex flex-col overflow-hidden max-h-[70vh]"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="py-4 border-b border-border-color text-center font-medium text-[16px]">
          {showTerms}
        </div>
        <div className="flex-1 overflow-y-auto p-6 text-[14px] text-text-sub leading-relaxed flex flex-col items-center justify-center gap-3 text-center">
          <span className="text-[36px]">📄</span>
          <p>{t("auth.terms_not_configured", "Terms content is not configured yet and will be published after legal review.")}</p>
        </div>
        <div
          className="py-4 text-center text-[#576B95] font-medium active:bg-active-bg cursor-pointer border-t border-border-color"
          onClick={onClose}
        >
          {t("auth.got_it", "Got it")}
        </div>
      </div>
    </div>
  );
};
