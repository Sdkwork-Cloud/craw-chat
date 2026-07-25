import React from "react";
import { motion, AnimatePresence } from "motion/react";
import { Palette, Check } from "lucide-react";
import { useTranslation } from "react-i18next";

interface ColorSettingsModalProps {
  show: boolean;
  onClose: () => void;
  penColor: string;
  setPenColor: (color: string) => void;
  penWidth: number;
  setPenWidth: (width: number) => void;
  colorOptions: { label: string; value: string }[];
}

export const ColorSettingsModal: React.FC<ColorSettingsModalProps> = ({
  show,
  onClose,
  penColor,
  setPenColor,
  penWidth,
  setPenWidth,
  colorOptions,
}) => {
  const { t } = useTranslation();
return (
    <AnimatePresence>
      {show && (
        <div className="absolute inset-0 z-[110] flex items-end sm:items-center justify-center">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.2 }}
            className="absolute inset-0 bg-black/40 backdrop-blur-sm"
            onClick={onClose}
          />
          <motion.div
            initial={{ y: "100%", opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            exit={{ y: "100%", opacity: 0 }}
            transition={{ type: "spring", damping: 25, stiffness: 300 }}
            className="relative w-full sm:w-[400px] bg-white dark:bg-[#161718] rounded-t-3xl sm:rounded-3xl p-6 shadow-2xl pb-safe"
          >
            <div className="w-12 h-1.5 bg-gray-200 dark:bg-gray-800 rounded-full mx-auto mb-6 sm:hidden" />
            <h3 className="text-[18px] font-bold text-center mb-6 flex items-center justify-center gap-2">
              <Palette className="w-5 h-5 text-primary-blue" />
              {t("notary.signature.pen_settings")}
            </h3>
            
            <div className="mb-8">
              <div className="text-[14px] font-medium text-text-sub mb-4 flex justify-between">
                <span>{t("notary.signature.ink_color")}</span>
                <span className="text-text-main">{colorOptions.find(c => c.value === penColor)?.label}</span>
              </div>
              <div className="flex justify-between items-center px-2">
                {colorOptions.map((c) => (
                  <button
                    key={c.value}
                    onClick={() => setPenColor(c.value)}
                    className="relative w-12 h-12 rounded-full shadow-sm flex items-center justify-center transition-transform active:scale-90"
                    style={{ backgroundColor: c.value }}
                  >
                    {penColor === c.value && (
                      <motion.div 
                        layoutId="color-check"
                        className="absolute inset-0 rounded-full border-[3px] border-white dark:border-black flex items-center justify-center shadow-lg"
                      >
                        <Check className="w-5 h-5 text-white" strokeWidth={3} />
                      </motion.div>
                    )}
                  </button>
                ))}
              </div>
            </div>

            <div className="mb-6">
              <div className="text-[14px] font-medium text-text-sub mb-4 flex justify-between">
                <span>{t("notary.signature.stroke_width")}</span>
                <span className="text-text-main">{penWidth.toFixed(1)}x</span>
              </div>
              <div className="flex items-center gap-4 px-2">
                <div className="w-2 h-2 rounded-full bg-border-color shrink-0" />
                <input 
                  type="range" 
                  min="1" 
                  max="6" 
                  step="0.5" 
                  value={penWidth}
                  onChange={(e) => setPenWidth(parseFloat(e.target.value))}
                  className="flex-1 h-2 bg-gray-200 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer accent-primary-blue"
                />
                <div className="w-6 h-6 rounded-full bg-border-color shrink-0" />
              </div>
            </div>

            <button 
              onClick={onClose}
              className="w-full py-4 mt-4 bg-[#f1f2f4] dark:bg-[#202122] rounded-2xl font-bold text-[16px] text-text-main active:scale-95 transition-transform"
            >
              {t("notary.signature.done")}
            </button>
          </motion.div>
        </div>
      )}
    </AnimatePresence>
  );
};
