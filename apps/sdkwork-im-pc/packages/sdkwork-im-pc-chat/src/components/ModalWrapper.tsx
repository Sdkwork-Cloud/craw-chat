import React, { useEffect, useId, useRef } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { X } from 'lucide-react';

export interface ModalWrapperProps {
  isOpen: boolean;
  onClose: () => void;
  title: string;
  children: React.ReactNode;
  height?: string;
  width?: string;
  footer?: React.ReactNode;
  closeLabel?: string;
}

function getFocusableElements(root: HTMLElement): HTMLElement[] {
  return Array.from(root.querySelectorAll<HTMLElement>(
    'a[href],button:not([disabled]),textarea:not([disabled]),input:not([disabled]),select:not([disabled]),[tabindex]:not([tabindex="-1"])',
  )).filter((element) => !element.hasAttribute('aria-hidden'));
}

export const ModalWrapper: React.FC<ModalWrapperProps> = ({
  isOpen,
  onClose,
  title,
  children,
  height,
  width = 'w-[400px]',
  footer,
  closeLabel = 'Close dialog',
}) => {
  const titleId = useId();
  const dialogRef = useRef<HTMLDivElement>(null);
  const restoreFocusRef = useRef<HTMLElement | null>(null);
  const onCloseRef = useRef(onClose);

  useEffect(() => {
    onCloseRef.current = onClose;
  }, [onClose]);

  useEffect(() => {
    if (!isOpen) {
      return undefined;
    }

    restoreFocusRef.current = document.activeElement instanceof HTMLElement
      ? document.activeElement
      : null;

    const focusDialog = (): void => {
      const dialog = dialogRef.current;
      if (!dialog) {
        return;
      }
      const firstFocusable = getFocusableElements(dialog)[0];
      (firstFocusable ?? dialog).focus();
    };
    const animationFrame = typeof window.requestAnimationFrame === 'function'
      ? window.requestAnimationFrame(focusDialog)
      : undefined;

    const handleKeyDown = (event: KeyboardEvent): void => {
      const dialog = dialogRef.current;
      const activeElement = document.activeElement;
      // Multiple dialogs can coexist during an exit animation. Only the
      // dialog containing focus is allowed to consume Escape/Tab.
      if (!dialog || !(activeElement instanceof Node) || !dialog.contains(activeElement)) {
        return;
      }
      if (event.key === 'Escape') {
        event.preventDefault();
        event.stopPropagation();
        onCloseRef.current();
        return;
      }
      if (event.key !== 'Tab') {
        return;
      }

      const focusable = getFocusableElements(dialog);
      if (focusable.length === 0) {
        event.preventDefault();
        dialog.focus();
        return;
      }
      const first = focusable[0];
      const last = focusable[focusable.length - 1];
      if (event.shiftKey && (activeElement === first || !dialog.contains(activeElement))) {
        event.preventDefault();
        last.focus();
      } else if (!event.shiftKey && activeElement === last) {
        event.preventDefault();
        first.focus();
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => {
      if (animationFrame !== undefined) {
        window.cancelAnimationFrame(animationFrame);
      }
      document.removeEventListener('keydown', handleKeyDown);
      const previous = restoreFocusRef.current;
      restoreFocusRef.current = null;
      if (previous && document.contains(previous)) {
        previous.focus();
      }
    };
  }, [isOpen]);

  return (
    <AnimatePresence>
      {isOpen && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          transition={{ duration: 0.15 }}
          className="fixed inset-0 z-[100] flex items-center justify-center bg-black/50 backdrop-blur-sm"
          onClick={onClose}
        >
          <motion.div
            initial={{ opacity: 0, scale: 0.95, y: 10 }}
            animate={{ opacity: 1, scale: 1, y: 0 }}
            exit={{ opacity: 0, scale: 0.95, y: 10 }}
          transition={{ duration: 0.2, ease: "easeOut" }}
          onClick={e => e.stopPropagation()}
          ref={dialogRef}
          role="dialog"
          aria-modal="true"
          aria-labelledby={titleId}
          tabIndex={-1}
          className={`bg-[#2b2b2d] border border-white/10 rounded-xl shadow-2xl overflow-hidden flex flex-col ${width} ${height ?? 'max-h-[80vh]'} max-w-[calc(100vw-32px)] max-h-[calc(100vh-40px)]`}
        >
            <div className="flex items-center justify-between px-5 py-4 border-b border-white/5 shrink-0">
              <h3 id={titleId} className="text-gray-200 font-medium">{title}</h3>
              <button
                type="button"
                onClick={onClose}
                aria-label={closeLabel}
                title={closeLabel}
                className="text-gray-400 hover:text-gray-200 transition-colors"
              >
                <X size={20} />
              </button>
            </div>
            <div className="flex-1 min-h-0 overflow-y-auto custom-scrollbar p-5">
              {children}
            </div>
            {footer && (
              <div className="px-5 py-4 border-t border-white/5 bg-[#222] shrink-0 flex justify-end gap-3">
                {footer}
              </div>
            )}
          </motion.div>
        </motion.div>
      )}
    </AnimatePresence>
  );
};
