import { useTranslation } from "react-i18next";
import React, { useEffect, useState } from "react";
import { createPortal } from "react-dom";
import { motion, AnimatePresence } from "motion/react";
import { cn } from "../utils/cn";

export interface ActionSheetOption {
  label: string;
  danger?: boolean;
  onClick: () => void;
}

export interface ActionSheetProps {
  isOpen: boolean;
  onClose: () => void;
  title?: string;
  options: ActionSheetOption[];
  cancelText?: string;
}

export const ActionSheet: React.FC<ActionSheetProps> = ({
  isOpen,
  onClose,
  title,
  options,
  cancelText = "取消",
}) => {
  const { t } = useTranslation();
const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  if (!mounted) return null;

  return createPortal(
    <AnimatePresence>
      {isOpen && (
        <div className="fixed inset-0 z-50 flex flex-col justify-end">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.2 }}
            className="absolute inset-0 bg-black/40"
            onClick={onClose}
          />
          <motion.div
            initial={{ y: "100%" }}
            animate={{ y: 0 }}
            exit={{ y: "100%" }}
            transition={{ type: "spring", damping: 25, stiffness: 200 }}
            className="relative bg-[#f1f2f4] dark:bg-black rounded-t-xl overflow-hidden pb-safe"
          >
            {title && (
              <div className="py-3 px-4 text-center text-[13px] text-text-sub font-medium border-b border-border-color">
                {title}
              </div>
            )}
            <div className="flex flex-col bg-bg-color">
              {options.map((option, index) => (
                <div
                  key={index}
                  className={cn(
                    "py-4 text-center text-[17px] cursor-pointer active:bg-active-bg transition-colors border-b border-border-color",
                    option.danger
                      ? "text-red-500 font-medium"
                      : "text-text-main",
                    index === options.length - 1 ? "border-none" : "",
                  )}
                  onClick={() => {
                    option.onClick();
                    onClose();
                  }}
                >
                  {option.label}
                </div>
              ))}
            </div>
            <div className="mt-2 bg-bg-color">
              <div
                className="py-4 text-center text-[17px] text-text-main font-medium cursor-pointer active:bg-active-bg transition-colors"
                onClick={onClose}
              >
                {cancelText}
              </div>
            </div>
          </motion.div>
        </div>
      )}
    </AnimatePresence>,
    document.body,
  );
};
