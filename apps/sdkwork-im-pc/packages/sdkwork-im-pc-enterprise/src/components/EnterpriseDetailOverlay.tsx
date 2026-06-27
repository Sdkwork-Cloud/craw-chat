import React from 'react';
import { useTranslation } from 'react-i18next';
import { Package, Briefcase, MessageSquare } from 'lucide-react';
import { EnterpriseData } from './EnterpriseDetail';

interface EnterpriseDetailOverlayProps {
  enterprise: EnterpriseData;
  selectedItem: { type: 'product' | 'job', id: string, title?: string };
  onClose: () => void;
  onStartChat: (enterpriseId: string) => void;
}

export const EnterpriseDetailOverlay: React.FC<EnterpriseDetailOverlayProps> = ({ enterprise, selectedItem, onClose, onStartChat }) => {
  const { t } = useTranslation('enterprise');

  return (
    <div className="absolute inset-0 z-50 bg-[#f8fafc] dark:bg-[#121214] flex flex-col animate-in slide-in-from-right-8 duration-300">
      <div className="h-16 flex items-center px-6 border-b border-gray-200 dark:border-white/5 bg-white dark:bg-[#18181b] shrink-0">
        <button
          type="button"
          onClick={onClose}
          className="p-2 hover:bg-gray-100 dark:hover:bg-white/5 rounded-xl transition-colors text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200 mr-4"
        >
          <span className="font-bold">{t('detail.back')}</span>
        </button>
        <h2 className="text-lg font-bold text-gray-900 dark:text-white line-clamp-1">{selectedItem.title}</h2>
        <div className="ml-auto flex gap-3">
          <button
            type="button"
            onClick={() => onStartChat(enterprise.id)}
            className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white text-sm font-bold rounded-lg shadow-sm transition-colors flex items-center gap-2"
          >
            <MessageSquare size={16} /> {t('detail.exclusiveConsult')}
          </button>
        </div>
      </div>
      <div className="flex-1 overflow-y-auto custom-scrollbar p-8">
        <div className="max-w-4xl mx-auto bg-white dark:bg-[#18181b] rounded-2xl p-8 border border-gray-200 dark:border-white/5 shadow-sm">
          {selectedItem.type === 'product' ? (
            <div className="flex flex-col gap-8">
              <div className="aspect-video bg-gradient-to-br from-indigo-50 dark:from-indigo-900/20 to-blue-50 dark:to-blue-900/20 rounded-xl flex items-center justify-center border border-indigo-100/50 dark:border-indigo-500/10">
                <Package className="text-indigo-400 dark:text-indigo-500" size={80} />
              </div>
              <div>
                <h3 className="text-2xl font-bold text-gray-900 dark:text-white mb-4">{selectedItem.title}</h3>
                <p className="text-gray-600 dark:text-gray-400 leading-relaxed text-[15px]">
                  {t('marketplace.profileUnavailable', { defaultValue: 'Extended enterprise profile data is not available yet.' })}
                </p>
              </div>
            </div>
          ) : (
            <div className="flex flex-col gap-8">
              <div className="pb-6 border-b border-gray-100 dark:border-white/5">
                <div className="flex items-center gap-3 mb-4">
                  <div className="p-2.5 bg-indigo-50 dark:bg-indigo-500/10 rounded-xl text-indigo-600 dark:text-indigo-400">
                    <Briefcase size={24} />
                  </div>
                  <h3 className="text-2xl font-bold text-gray-900 dark:text-white">{selectedItem.title}</h3>
                </div>
              </div>
              <p className="text-sm text-gray-600 dark:text-gray-400 leading-relaxed font-medium">
                {t('marketplace.profileUnavailable', { defaultValue: 'Extended enterprise profile data is not available yet.' })}
              </p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
