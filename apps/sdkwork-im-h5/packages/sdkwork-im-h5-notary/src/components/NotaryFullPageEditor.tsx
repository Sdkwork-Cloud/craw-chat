import React from "react";
import { ChevronLeft } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";

interface FullPageEditorProps {
  field: string;
  title: string;
  placeholder: string;
  value: string;
  isTextArea?: boolean;
  inputType?: string;
  onChange: (value: string) => void;
  onSave: () => void;
  onClose: () => void;
}

export const NotaryFullPageEditor: React.FC<FullPageEditorProps> = ({
  title,
  placeholder,
  value,
  isTextArea,
  inputType,
  onChange,
  onSave,
  onClose,
}) => {
  const { t } = useTranslation();
return (
    <div className="fixed inset-0 z-[200] bg-bg-color flex flex-col animate-in slide-in-from-right">
      <header className="h-[44px] flex items-center justify-between sticky top-0 shrink-0 pt-safe px-1 z-20 bg-bg-color border-b border-border-color">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={
              <ChevronLeft
                className="w-6 h-6 text-text-main"
                strokeWidth={2.5}
              />
            }
            onClick={onClose}
          />
        </div>
        <div className="flex items-center justify-center font-bold text-[17px] pointer-events-none">
          {title}
        </div>
        <div className="flex justify-end z-10 flex-1 pr-4">
          <button
            onClick={onSave}
            className="text-primary-blue font-medium text-[15px] active:opacity-70 transition-opacity"
          >
            {t("notary.save")}
          </button>
        </div>
      </header>
      <div className="flex-1 overflow-y-auto p-4 bg-[#f4f6f9] dark:bg-black">
        <div className="bg-bg-color rounded-xl p-3 border border-border-color shadow-sm">
          {isTextArea ? (
            <textarea
              autoFocus
              value={value}
              onChange={(e) => onChange(e.target.value)}
              placeholder={placeholder}
              className="w-full bg-transparent outline-none text-text-main placeholder-text-sub text-[15px] min-h-[120px] resize-none"
            />
          ) : (
            <input
              autoFocus
              type={inputType || "text"}
              value={value}
              onChange={(e) => onChange(e.target.value)}
              placeholder={placeholder}
              className="w-full bg-transparent outline-none text-text-main placeholder-text-sub text-[15px]"
            />
          )}
        </div>
      </div>
    </div>
  );
};
