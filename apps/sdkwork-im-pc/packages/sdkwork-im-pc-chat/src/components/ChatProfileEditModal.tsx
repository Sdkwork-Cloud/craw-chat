import React from 'react';
import { useTranslation } from 'react-i18next';
import { motion, AnimatePresence } from 'motion/react';
import { X } from 'lucide-react';
import type { Chat } from '@sdkwork/im-pc-types';

export type ChatProfileEditModalType = 'editName' | 'editNotice';

export interface ChatProfileEditModalProps {
  activeChat: Chat | null;
  modalType: ChatProfileEditModalType | null;
  modalInput: string;
  emptyNoticePlaceholder?: string;
  onClose: () => void;
  onModalInputChange: (value: string) => void;
  onSaveName: () => Promise<void>;
  onSaveNotice: () => Promise<void>;
}

/**
 * P2-21: Extracted from ChatLayout.tsx (lines 1493-1636).
 *
 * Inline modal for editing group name, friend remark, or group notice.
 * The save logic (calling groupService/chatService, updating chats state,
 * showing toasts) is delegated to the parent via `onSaveName` /
 * `onSaveNotice` callbacks to keep this component purely presentational
 * and avoid coupling to service/state layers.
 */
export const ChatProfileEditModal: React.FC<ChatProfileEditModalProps> = ({
  activeChat,
  modalType,
  modalInput,
  emptyNoticePlaceholder,
  onClose,
  onModalInputChange,
  onSaveName,
  onSaveNotice,
}) => {
  const { t } = useTranslation();

  return (
    <AnimatePresence>
      {modalType && activeChat && (
        <div className="fixed inset-0 z-50 flex items-center justify-center">
          <div
            className="absolute inset-0 bg-black/60 backdrop-blur-sm"
            onClick={onClose}
          />
          <motion.div
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, scale: 0.95 }}
            className="relative bg-[#282828] border border-white/10 rounded-2xl w-full max-w-md shadow-2xl p-6"
          >
            <div className="flex justify-between items-center mb-4">
              <h3 className="text-lg font-medium text-gray-100">
                {modalType === 'editName' &&
                  (activeChat.type === 'group'
                    ? t('chat.modal.title.editGroupName')
                    : t('chat.modal.title.editRemark'))}
                {modalType === 'editNotice' && t('chat.modal.title.editNotice')}
              </h3>
              <button
                onClick={onClose}
                className="p-1 text-gray-400 hover:text-gray-100 transition-colors"
              >
                <X size={20} />
              </button>
            </div>

            {modalType === 'editName' && (
              <div>
                <input
                  type="text"
                  placeholder={
                    activeChat.type === 'group'
                      ? t('chat.modal.placeholder.groupName')
                      : t('chat.modal.placeholder.remarkName')
                  }
                  className="w-full bg-[#181818] border border-white/10 rounded-xl px-4 py-2.5 text-sm text-gray-200 outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 transition-all mb-4"
                  value={modalInput}
                  onChange={(e) => onModalInputChange(e.target.value)}
                />
                <div className="flex justify-end gap-3 mt-6">
                  <button
                    onClick={onClose}
                    className="px-5 py-2 text-sm text-gray-300 hover:bg-white/5 rounded-xl transition-colors"
                  >
                    {t('chat.modal.actions.cancel')}
                  </button>
                  <button
                    onClick={onSaveName}
                    className="px-5 py-2 text-sm bg-indigo-600 hover:bg-indigo-500 text-white rounded-xl transition-colors font-medium"
                  >
                    {t('chat.modal.actions.save')}
                  </button>
                </div>
              </div>
            )}

            {modalType === 'editNotice' && (
              <div>
                <textarea
                  placeholder={t('chat.modal.placeholder.groupNotice')}
                  className="w-full bg-[#181818] border border-white/10 rounded-xl px-4 py-2.5 text-sm text-gray-200 outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 transition-all mb-4 min-h-[120px] resize-none"
                  value={
                    emptyNoticePlaceholder && modalInput === emptyNoticePlaceholder
                      ? ''
                      : modalInput
                  }
                  onChange={(e) => onModalInputChange(e.target.value)}
                />
                <div className="flex justify-end gap-3 mt-6">
                  <button
                    onClick={onClose}
                    className="px-5 py-2 text-sm text-gray-300 hover:bg-white/5 rounded-xl transition-colors"
                  >
                    {t('chat.modal.actions.cancel')}
                  </button>
                  <button
                    onClick={onSaveNotice}
                    className="px-5 py-2 text-sm bg-indigo-600 hover:bg-indigo-500 text-white rounded-xl transition-colors font-medium"
                  >
                    {t('chat.modal.actions.publish')}
                  </button>
                </div>
              </div>
            )}
          </motion.div>
        </div>
      )}
    </AnimatePresence>
  );
};
