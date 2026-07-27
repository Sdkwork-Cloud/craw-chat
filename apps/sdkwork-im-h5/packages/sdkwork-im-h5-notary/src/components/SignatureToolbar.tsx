import { useTranslation } from "react-i18next";
import React from "react";
import { Crop, RotateCcw } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";

interface SignatureToolbarProps {
  penColor: string;
  colors: { label: string; value: string }[];
  ratio: number | null;
  ratioOptions: { label: string; value: number | null }[];
  onShowColorSelector: () => void;
  onShowRatioSelector: () => void;
  onClear: () => void;
}

export const SignatureToolbar: React.FC<SignatureToolbarProps> = ({
  penColor,
  colors,
  ratio,
  ratioOptions,
  onShowColorSelector,
  onShowRatioSelector,
  onClear,
}) => {
  const { t } = useTranslation();
return (
    <div className="flex items-center justify-between px-2 pt-4">
      <div className="flex items-center gap-6">
        <button onClick={onShowColorSelector} className="flex items-center gap-2 active:opacity-70 transition-opacity">
          <div className="w-5 h-5 rounded-full border border-black/10 shadow-sm" style={{ backgroundColor: penColor }} />
          <span className="text-[14px] font-medium max-w-[80px] truncate">{colors.find(c => c.value === penColor)?.label || "颜色"}</span>
        </button>
        
        <button onClick={onShowRatioSelector} className="flex items-center gap-1.5 active:opacity-70 transition-opacity">
          <Crop className="w-5 h-5 text-text-main" strokeWidth={2} />
          <span className="text-[14px] font-medium max-w-[80px] truncate">{ratioOptions.find(r => r.value === ratio)?.label || "比例"}</span>
        </button>
      </div>

      <button onClick={onClear} className="flex items-center gap-1.5 active:opacity-70 cursor-pointer text-text-sub transition-opacity">
        <RotateCcw className="w-5 h-5" strokeWidth={2} />
        <span className="text-[14px] font-medium">{t('notary.auto_4388c5d1', '重写笔迹')}</span>
      </button>
    </div>
  );
};
