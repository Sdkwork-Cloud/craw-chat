import React from "react";
import { motion, AnimatePresence } from "motion/react";
import { Check } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";

interface RatioSelectionSheetProps {
  show: boolean;
  onClose: () => void;
  ratio: number;
  setRatio: (ratio: number) => void;
  ratioOptions: { label: string; value: number; desc: string }[];
}

export const RatioSelectionSheet: React.FC<RatioSelectionSheetProps> = ({
  show,
  onClose,
  ratio,
  setRatio,
  ratioOptions,
}) => {
  const { t } = useTranslation();
return (
    <AnimatePresence>
      {show && (
        <div className="absolute inset-0 z-[110] flex flex-col justify-end">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.2 }}
            className="absolute inset-0 bg-black/40 backdrop-blur-sm"
            onClick={onClose}
          />
          <motion.div
            initial={{ y: "100%" }}
            animate={{ y: 0 }}
            exit={{ y: "100%" }}
            transition={{ type: "spring", damping: 25, stiffness: 300 }}
            className="relative bg-white dark:bg-[#161718] rounded-t-3xl overflow-hidden pb-safe shadow-2xl"
          >
            <div className="p-4 border-b border-border-color/50 relative">
              <div className="text-center font-bold text-[17px] text-text-main">
                {t("notary.signature.select_ratio")}
              </div>
              <div className="text-center text-[12px] text-text-sub mt-0.5">
                {t("notary.signature.ratio_hint")}
              </div>
            </div>
            <div className="flex flex-col p-2 space-y-1 bg-[#f8f9fa] dark:bg-black/20">
              {ratioOptions.map((opt) => (
                <button
                  key={opt.label}
                  onClick={() => {
                    setRatio(opt.value);
                    onClose();
                  }}
                  className={cn(
                    "flex items-center justify-between p-4 rounded-2xl transition-all active:scale-[0.98]",
                    ratio === opt.value
                      ? "bg-primary-blue/10 border border-primary-blue/20"
                      : "bg-white dark:bg-[#1a1b1c] border border-transparent shadow-sm"
                  )}
                >
                  <div className="flex items-center gap-4">
                    {/* Visual representation of ratio */}
                    <div className="w-10 h-10 bg-gray-100 dark:bg-gray-800 rounded-lg flex items-center justify-center shrink-0">
                      <div 
                        className={cn(
                          "border-2 rounded-sm",
                          ratio === opt.value ? "border-primary-blue bg-primary-blue/20" : "border-text-sub bg-border-color/30"
                        )}
                        style={{
                          width: opt.value === null ? '24px' : opt.value >= 1 ? '24px' : `${24 * opt.value}px`,
                          height: opt.value === null ? '24px' : opt.value >= 1 ? `${24 / opt.value}px` : '24px',
                          borderStyle: opt.value === null ? 'dashed' : 'solid'
                        }}
                      />
                    </div>
                    <div className="flex flex-col items-start gap-0.5">
                      <span className={cn("text-[16px] font-medium", ratio === opt.value ? "text-primary-blue" : "text-text-main")}>{opt.label}</span>
                      <span className="text-[12px] text-text-sub">{opt.desc}</span>
                    </div>
                  </div>
                  {ratio === opt.value && (
                    <div className="w-6 h-6 rounded-full bg-primary-blue flex items-center justify-center shrink-0">
                      <Check className="w-4 h-4 text-white" strokeWidth={3} />
                    </div>
                  )}
                </button>
              ))}
            </div>
            <div className="p-4 bg-white dark:bg-[#161718]">
              <button
                className="w-full py-4 text-center text-[16px] font-bold text-text-main rounded-2xl bg-[#f1f2f4] dark:bg-[#202122] active:scale-95 transition-transform"
                onClick={onClose}
              >
                {t("notary.signature.cancel")}
              </button>
            </div>
          </motion.div>
        </div>
      )}
    </AnimatePresence>
  );
};
