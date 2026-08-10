import React from 'react';
import { User, MapPin, Calendar } from 'lucide-react';
import { useTranslation } from 'react-i18next';

export const CandidateBaseInfo: React.FC = () => {
  const { t } = useTranslation();
return (
    <div className="bg-chat-other-bg rounded-xl p-4 mb-4 shadow-sm border border-border-color/30">
      <h3 className="text-[15px] font-bold text-text-main mb-4 border-l-4 border-primary-blue pl-2 leading-tight">
        {t('recruitment.detail.baseInfo')}
      </h3>
      <div className="space-y-4">
        <div className="flex items-center gap-3">
          <User className="w-5 h-5 text-text-sub" />
          <div className="flex-1 border-b border-border-color/50 pb-3 flex justify-between">
            <span className="text-text-main">{t('recruitment.detail.age')}</span>
            <span className="text-text-sub">{t('recruitment.detail.ageValue', { age: 28 })}</span>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <MapPin className="w-5 h-5 text-text-sub" />
          <div className="flex-1 border-b border-border-color/50 pb-3 flex justify-between">
            <span className="text-text-main">{t('recruitment.detail.residence')}</span>
            <span className="text-text-sub">{t('recruitment.detail.location')}</span>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <Calendar className="w-5 h-5 text-text-sub" />
          <div className="flex-1 border-border-color/50 flex justify-between">
            <span className="text-text-main">{t('recruitment.detail.availability')}</span>
            <span className="text-text-sub">{t('recruitment.detail.anytime')}</span>
          </div>
        </div>
      </div>
    </div>
  );
};
