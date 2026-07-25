import { useTranslation } from "react-i18next";
import React, { useState } from "react";
import { motion, AnimatePresence } from "motion/react";
import { createRoot } from "react-dom/client";

interface DialogProps {
  title: string;
  type: "prompt" | "confirm";
  defaultValue?: string;
  onConfirm: (val: string | boolean) => void;
  onCancel: () => void;
}

const DialogInner: React.FC<DialogProps> = ({
  title,
  type,
  defaultValue = "",
  onConfirm,
  onCancel,
}) => {
  const { t } = useTranslation();
  const [val, setVal] = useState(defaultValue);

  return (
    <div className="fixed inset-0 z-[9999] flex items-center justify-center bg-black/50 p-4">
      <motion.div
        initial={{ opacity: 0, scale: 0.95 }}
        animate={{ opacity: 1, scale: 1 }}
        exit={{ opacity: 0, scale: 0.95 }}
        className="w-full max-w-[320px] bg-white dark:bg-[#2c2d2e] rounded-xl overflow-hidden shadow-2xl flex flex-col"
      >
        <div className="p-6 text-center">
          <h3 className="text-[17px] font-medium text-text-main mb-4">
            {title}
          </h3>
          {type === "prompt" && (
            <input
              autoFocus
              type="text"
              value={val}
              onChange={(e) => setVal(e.target.value)}
              className="w-full px-3 py-2 bg-bg-color border border-border-color rounded-lg text-[15px] text-text-main outline-none focus:border-[#07C160] transition-colors"
            />
          )}
        </div>
        <div className="flex border-t border-border-color h-12">
          <button
            className="flex-1 flex items-center justify-center text-[17px] text-text-main border-r border-border-color active:bg-active-bg transition-colors font-medium"
            onClick={onCancel}
          >{t('commons.auto_a9472', '取消')}</button>
          <button
            className="flex-1 flex items-center justify-center text-[17px] text-[#576B95] active:bg-active-bg transition-colors font-medium"
            onClick={() => onConfirm(type === "prompt" ? val : true)}
          >{t('commons.auto_ef0ec', '确定')}</button>
        </div>
      </motion.div>
    </div>
  );
};

let dialogRoot: ReturnType<typeof createRoot> | null = null;

const createDialogParams = (props: DialogProps) => {
  let container = document.getElementById("dialog-container");
  if (!container) {
    container = document.createElement("div");
    container.id = "dialog-container";
    document.body.appendChild(container);
  }
  if (!dialogRoot) {
    dialogRoot = createRoot(container);
  }
  dialogRoot.render(
    <AnimatePresence>
      <DialogInner key={Date.now()} {...props} />
    </AnimatePresence>,
  );
};

const closeDialog = () => {
  if (dialogRoot) {
    dialogRoot.render(<AnimatePresence></AnimatePresence>);
  }
};

export const showPrompt = (
  title: string,
  defaultValue?: string,
): Promise<string | null> => {
  

  return new Promise((resolve) => {
    createDialogParams({
      title,
      type: "prompt",
      defaultValue,
      onConfirm: (val) => {
        closeDialog();
        resolve(val as string);
      },
      onCancel: () => {
        closeDialog();
        resolve(null);
      },
    });
  });
};

export const showConfirm = (title: string): Promise<boolean> => {
  return new Promise((resolve) => {
    createDialogParams({
      title,
      type: "confirm",
      onConfirm: () => {
        closeDialog();
        resolve(true);
      },
      onCancel: () => {
        closeDialog();
        resolve(false);
      },
    });
  });
};
