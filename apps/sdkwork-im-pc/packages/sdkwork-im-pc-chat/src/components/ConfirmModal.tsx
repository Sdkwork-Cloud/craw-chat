import React from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { useTranslation } from 'react-i18next';

export interface ConfirmModalProps {
  isOpen: boolean;
  title: string;
  message?: string;
  confirmLabel?: string;
  cancelLabel?: string;
  danger?: boolean;
  onConfirm: () => void;
  onCancel: () => void;
}

export const ConfirmModal: React.FC<ConfirmModalProps> = ({
  isOpen,
  title,
  message,
  confirmLabel,
  cancelLabel,
  danger = false,
  onConfirm,
  onCancel,
}) => {
  const { t } = useTranslation();

  return (
    <AnimatePresence>
      {isOpen ? (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          className="fixed inset-0 z-50 flex items-center justify-center"
        >
          <div className="absolute inset-0 bg-black/60 backdrop-blur-sm" onClick={onCancel} />
          <motion.div
            initial={{ scale: 0.95, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            exit={{ scale: 0.95, opacity: 0 }}
            transition={{ type: 'spring', damping: 25, stiffness: 300 }}
            className="relative bg-[#282828] border border-white/10 rounded-2xl w-full max-w-sm shadow-xl p-6"
            role="alertdialog"
            aria-modal="true"
            aria-labelledby="confirm-modal-title"
          >
            <h3 id="confirm-modal-title" className="text-lg font-medium text-white mb-2">{title}</h3>
            {message ? <p className="text-sm text-gray-400 mb-6">{message}</p> : <div className="mb-4" />}
            <div className="flex justify-end gap-3">
              <button
                type="button"
                className="px-5 py-2 text-sm text-gray-300 hover:bg-white/5 rounded-xl transition-colors"
                onClick={onCancel}
              >
                {cancelLabel ?? t('chat.modal.actions.cancel')}
              </button>
              <button
                type="button"
                className={`px-5 py-2 text-sm rounded-xl transition-colors font-medium ${
                  danger
                    ? 'bg-rose-600 hover:bg-rose-500 text-white'
                    : 'bg-indigo-600 hover:bg-indigo-500 text-white'
                }`}
                onClick={onConfirm}
              >
                {confirmLabel ?? t('chat.modal.actions.confirm')}
              </button>
            </div>
          </motion.div>
        </motion.div>
      ) : null}
    </AnimatePresence>
  );
};
