import { useTranslation } from "react-i18next";
import React, { useEffect, useState } from "react";
import { motion, AnimatePresence } from "motion/react";
import { createRoot } from "react-dom/client";

export interface ToastProps {
  message: string;
  duration?: number;
  onClose?: () => void;
}

const ToastInner: React.FC<ToastProps> = ({
  message,
  duration = 2000,
  onClose,
}) => {
  const { t } = useTranslation();
  useEffect(() => {
    const timer = setTimeout(() => {
      onClose?.();
    }, duration);
    return () => clearTimeout(timer);
  }, [duration, onClose]);

  return (
    <div className="pointer-events-none fixed inset-x-0 bottom-[15vh] z-[9999] flex justify-center px-6">
      <motion.div
        initial={{ opacity: 0, y: 16, scale: 0.92 }}
        animate={{ opacity: 1, y: 0, scale: 1 }}
        exit={{ opacity: 0, y: 8, scale: 0.95 }}
        transition={{ duration: 0.2, ease: "easeOut" }}
        className="max-w-full rounded-xl bg-black/80 px-4 py-2.5 text-center text-[15px] font-medium leading-snug text-white shadow-lg dark:bg-white/90 dark:text-black"
      >
        {message}
      </motion.div>
    </div>
  );
};

let toastRoot: ReturnType<typeof createRoot> | null = null;
let toastSequence = 0;

export const showToast = (message: string, duration?: number) => {
  if (typeof window === "undefined") return;

  let container = document.getElementById("toast-container");
  if (!container) {
    container = document.createElement("div");
    container.id = "toast-container";
    document.body.appendChild(container);
  }

  if (!toastRoot) {
    toastRoot = createRoot(container);
  }

  const handleClose = () => {
    toastRoot?.render(<AnimatePresence></AnimatePresence>);
  };
  const toastKey = ++toastSequence;

  toastRoot.render(
    <AnimatePresence>
      <ToastInner
        key={toastKey}
        message={message}
        duration={duration}
        onClose={handleClose}
      />
    </AnimatePresence>,
  );
};
