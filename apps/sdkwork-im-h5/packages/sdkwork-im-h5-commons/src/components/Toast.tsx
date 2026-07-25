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
    <motion.div
      initial={{ opacity: 0, y: 50, scale: 0.9 }}
      animate={{ opacity: 1, y: 0, scale: 1 }}
      exit={{ opacity: 0, y: 20, scale: 0.9 }}
      transition={{ duration: 0.2 }}
      className="fixed bottom-[15vh] left-1/2 -translate-x-1/2 z-[9999] px-4 py-2.5 bg-black/80 dark:bg-white/90 text-white dark:text-black text-[15px] font-medium rounded-xl shadow-lg pointer-events-none whitespace-nowrap"
    >
      {message}
    </motion.div>
  );
};

let toastRoot: ReturnType<typeof createRoot> | null = null;

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
  toastRoot.render(<AnimatePresence></AnimatePresence>);
  };

  toastRoot.render(
    <AnimatePresence>
      <ToastInner
        key={Date.now()}
        message={message}
        duration={duration}
        onClose={handleClose}
      />
    </AnimatePresence>,
  );
};
