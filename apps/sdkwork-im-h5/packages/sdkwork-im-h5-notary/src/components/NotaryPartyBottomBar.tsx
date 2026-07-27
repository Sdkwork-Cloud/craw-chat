import React from "react";
import { useTranslation } from "react-i18next";

export interface NotaryPartyBottomBarProps {
  isReadonly?: boolean;
  onBack: () => void;
  onSave: () => void;
}

export const NotaryPartyBottomBar: React.FC<NotaryPartyBottomBarProps> = ({
  isReadonly,
  onBack,
  onSave,
}) => {
  const { t } = useTranslation();

  return (
    <div className="fixed bottom-0 left-0 right-0 p-3 bg-bg-color border-t border-border-color pb-safe z-20 flex gap-3 shadow-[0_-4px_20px_rgba(0,0,0,0.03)] dark:shadow-none">
      {isReadonly ? (
        <button
          onClick={onBack}
          className="w-full h-12 rounded-xl font-bold text-[15px] flex items-center justify-center transition-opacity shadow-sm bg-primary-blue text-white active:scale-[0.98]"
        >
          {t("notary.add_party.back")}
        </button>
      ) : (
        <>
          <button
            onClick={onBack}
            className="flex-[1] h-12 rounded-xl font-bold text-[15px] flex items-center justify-center bg-active-bg text-text-main active:opacity-70 transition-opacity"
          >
            {t("notary.add_party.cancel")}
          </button>
          <button
            onClick={onSave}
            className="flex-[2] h-12 rounded-xl font-bold text-[15px] flex items-center justify-center transition-opacity shadow-sm bg-primary-blue text-white active:scale-[0.98]"
          >
            {t("notary.add_party.save")}
          </button>
        </>
      )}
    </div>
  );
};
