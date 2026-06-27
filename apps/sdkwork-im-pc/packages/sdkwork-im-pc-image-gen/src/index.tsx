import React, { useState } from 'react';
import { Image as ImageIcon } from 'lucide-react';
import { useTranslation, I18nextProvider } from 'react-i18next';
import i18n from './i18n';
import { imageGenService, PC_IMAGEGEN_CONTRACT_UNAVAILABLE } from './services/ImageGenService';

const ImageGenViewComponent: React.FC = () => {
  const { t } = useTranslation();
  const [error, setError] = useState<string | null>(null);

  const handleGenerate = async () => {
    setError(null);
    try {
      await imageGenService.generate('');
    } catch (generateError) {
      setError(
        generateError instanceof Error
          ? generateError.message
          : PC_IMAGEGEN_CONTRACT_UNAVAILABLE,
      );
    }
  };

  return (
    <div className="flex-1 flex flex-col items-center justify-center bg-[#1e1e1e] min-w-0">
      <div className="bg-[#2b2b2d] p-8 rounded-2xl border border-white/5 flex flex-col items-center max-w-md w-full">
        <div className="w-16 h-16 bg-blue-500/10 rounded-2xl flex items-center justify-center mb-6 border border-blue-500/20">
          <ImageIcon className="text-blue-400" size={32} />
        </div>
        <h2 className="text-xl font-bold text-gray-200 mb-2">{t('title')}</h2>
        <p className="text-gray-400 text-center text-sm leading-relaxed mb-4">
          {t('desc')}
        </p>
        {error ? (
          <p className="text-red-400 text-center text-sm mb-4">{error}</p>
        ) : null}
        <button
          type="button"
          onClick={() => void handleGenerate()}
          className="w-full bg-blue-600 hover:bg-blue-500 text-white font-medium py-2.5 rounded-xl transition-colors"
        >
          {t('button')}
        </button>
      </div>
    </div>
  );
};

export const ImageGenView: React.FC = () => {
  return (
    <I18nextProvider i18n={i18n}>
      <ImageGenViewComponent />
    </I18nextProvider>
  );
};
