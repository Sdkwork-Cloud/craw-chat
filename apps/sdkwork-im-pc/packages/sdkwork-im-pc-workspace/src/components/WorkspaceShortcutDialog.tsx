import React, { useEffect, useRef } from 'react';
import { Check, X } from 'lucide-react';
import { useTranslation } from 'react-i18next';

import type { AppItem } from '../services/WorkspaceService';

export interface WorkspaceShortcutDialogProps {
  apps: AppItem[];
  pinnedIds: ReadonlySet<string>;
  isSaving: boolean;
  onToggle: (appId: string) => void;
  onCancel: () => void;
  onSave: () => void;
}

export const WorkspaceShortcutDialog: React.FC<WorkspaceShortcutDialogProps> = ({
  apps,
  pinnedIds,
  isSaving,
  onToggle,
  onCancel,
  onSave,
}) => {
  const { t } = useTranslation();
  const dialogRef = useRef<HTMLDivElement>(null);
  const savingRef = useRef(isSaving);

  useEffect(() => {
    savingRef.current = isSaving;
  }, [isSaving]);

  useEffect(() => {
    const previousFocus = typeof document === 'undefined'
      ? null
      : document.activeElement instanceof HTMLElement
        ? document.activeElement
        : null;
    dialogRef.current?.focus();

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape' && !savingRef.current) {
        event.preventDefault();
        onCancel();
        return;
      }
      if (event.key === 'Tab') {
        const dialog = dialogRef.current;
        if (!dialog) return;
        const focusableElements = Array.from(dialog.querySelectorAll<HTMLElement>(
          'button:not([disabled]), input:not([disabled]), [href], [tabindex]:not([tabindex="-1"])',
        )).filter((element) => !element.hasAttribute('hidden'));
        const firstElement = focusableElements[0];
        const lastElement = focusableElements.at(-1);
        if (!firstElement || !lastElement) {
          event.preventDefault();
          dialog.focus();
          return;
        }
        if (event.shiftKey && (document.activeElement === firstElement || document.activeElement === dialog)) {
          event.preventDefault();
          lastElement.focus();
        } else if (!event.shiftKey && document.activeElement === lastElement) {
          event.preventDefault();
          firstElement.focus();
        }
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      previousFocus?.focus({ preventScroll: true });
    };
  }, [onCancel]);

  const selectedCount = apps.filter((app) => pinnedIds.has(app.id)).length;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm"
      onPointerDown={(event) => {
        if (event.target === event.currentTarget && !isSaving) {
          onCancel();
        }
      }}
    >
      <div
        ref={dialogRef}
        aria-labelledby="workspace-shortcut-dialog-title"
        aria-modal="true"
        className="flex max-h-[min(680px,calc(100vh-32px))] w-full max-w-lg flex-col overflow-hidden rounded-lg border border-white/10 bg-[#242529] shadow-2xl outline-none"
        role="dialog"
        tabIndex={-1}
      >
        <header className="flex shrink-0 items-center justify-between border-b border-white/10 px-5 py-4">
          <h2 className="text-base font-semibold text-gray-100" id="workspace-shortcut-dialog-title">
            {t('manageShortcutsTitle')}
          </h2>
          <button
            aria-label={t('close')}
            className="flex h-8 w-8 items-center justify-center rounded-md text-gray-400 transition-colors hover:bg-white/10 hover:text-gray-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-400"
            disabled={isSaving}
            onClick={onCancel}
            title={t('close')}
            type="button"
          >
            <X size={17} />
          </button>
        </header>

        <div className="min-h-0 flex-1 overflow-y-auto p-3 custom-scrollbar">
          <div className="grid gap-1 sm:grid-cols-2" role="group" aria-label={t('manageShortcutsTitle')}>
            {apps.map((app) => {
              const label = t(app.nameKey, { defaultValue: app.id });
              const checked = pinnedIds.has(app.id);
              return (
                <label
                  className="flex min-h-12 cursor-pointer items-center gap-3 rounded-md px-3 py-2 text-sm text-gray-200 transition-colors hover:bg-white/5 has-[:focus-visible]:ring-2 has-[:focus-visible]:ring-indigo-400"
                  key={app.id}
                >
                  <input
                    aria-label={label}
                    checked={checked}
                    className="h-4 w-4 accent-indigo-500"
                    disabled={app.required || isSaving}
                    onChange={() => onToggle(app.id)}
                    type="checkbox"
                  />
                  <span className="min-w-0 flex-1 truncate">{label}</span>
                  {app.required ? (
                    <span className="shrink-0 text-xs text-gray-500">{t('requiredShortcut')}</span>
                  ) : checked ? (
                    <Check aria-hidden="true" className="shrink-0 text-indigo-400" size={16} />
                  ) : null}
                </label>
              );
            })}
          </div>
        </div>

        <footer className="flex shrink-0 items-center justify-between gap-3 border-t border-white/10 bg-black/10 px-5 py-3">
          <span className="text-xs text-gray-500">{t('selectedShortcuts', { count: selectedCount })}</span>
          <div className="flex items-center gap-2">
            <button
              className="rounded-md px-3 py-2 text-sm text-gray-400 transition-colors hover:bg-white/5 hover:text-gray-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-400"
              disabled={isSaving}
              onClick={onCancel}
              type="button"
            >
              {t('cancel')}
            </button>
            <button
              aria-busy={isSaving}
              className="rounded-md bg-indigo-600 px-3 py-2 text-sm font-medium text-white transition-colors hover:bg-indigo-500 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-300 disabled:cursor-not-allowed disabled:opacity-60"
              disabled={isSaving}
              onClick={onSave}
              type="button"
            >
              {isSaving ? t('saving') : t('save')}
            </button>
          </div>
        </footer>
      </div>
    </div>
  );
};
