import React, { useState } from 'react';
import { Video } from 'lucide-react';
import { useTranslation, I18nextProvider } from 'react-i18next';
import i18n from './i18n';
import { PC_VIDEOGEN_CONTRACT_UNAVAILABLE, videoGenService } from './services/VideoGenService';

const VideoGenViewComponent: React.FC = () => {
  const { t } = useTranslation();
  const [error, setError] = useState<string | null>(null);

  const handleGenerate = async () => {
    setError(null);
    try {
      await videoGenService.generate('');
    } catch (generateError) {
      setError(
        generateError instanceof Error
          ? generateError.message
          : PC_VIDEOGEN_CONTRACT_UNAVAILABLE,
      );
    }
  };

  return (
    <div className="flex-1 flex flex-col items-center justify-center bg-[#1e1e1e] min-w-0">
      <div className="bg-[#2b2b2d] p-8 rounded-2xl border border-white/5 flex flex-col items-center max-w-md w-full">
        <div className="w-16 h-16 bg-indigo-500/10 rounded-2xl flex items-center justify-center mb-6 border border-indigo-500/20">
          <Video className="text-indigo-400" size={32} />
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
          className="w-full bg-indigo-600 hover:bg-indigo-500 text-white font-medium py-2.5 rounded-xl transition-colors"
        >
          {t('button')}
        </button>
      </div>
    </div>
  );
};

export const VideoGenView: React.FC = () => {
  return (
    <I18nextProvider i18n={i18n}>
      <VideoGenViewComponent />
    </I18nextProvider>
  );
};
