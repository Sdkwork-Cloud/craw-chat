import React from "react";
import { useTranslation } from "react-i18next";

interface TermsModalProps {
  showTerms: string | null;
  onClose: () => void;
}

export const TermsModal: React.FC<TermsModalProps> = ({ showTerms, onClose }) => {
  const { t } = useTranslation();
if (!showTerms) return null;

  return (
    <div
      className="fixed inset-0 z-50 bg-black/50 flex flex-col items-center justify-center p-6 pb-20"
      onClick={onClose}
    >
      <div
        className="bg-white dark:bg-[#1C1C1E] w-full max-w-[320px] rounded-2xl flex flex-col overflow-hidden max-h-[70vh]"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="py-4 border-b border-border-color text-center font-medium text-[16px]">
          {showTerms}
        </div>
        <div className="flex-1 overflow-y-auto p-6 text-[14px] text-text-sub leading-relaxed">
          <p className="mb-4">{t("auth.mock_terms_content", "这是一段模拟的协议内容。真实环境中应展示完整的法律条文。")}</p>
          <p className="mb-4">{t("auth.mock_terms_1", "1. 您必须遵守本应用的使用规范，不得利用本应用从事违法活动。")}</p>
          <p className="mb-4">
            2. {t("auth.mock_terms_2", "我们会收集您的部分使用数据以优化服务，但承诺保护您的隐私安全。")}
          </p>
          <p>{t("auth.mock_terms_3", "3. 若您继续使用，即表示完全理解并接受所有条款。")}</p>
        </div>
        <div
          className="py-4 text-center text-[#576B95] font-medium active:bg-active-bg cursor-pointer border-t border-border-color"
          onClick={onClose}
        >
          {t("auth.got_it", "知道了")}
        </div>
      </div>
    </div>
  );
};
