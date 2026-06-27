import React, { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Building2, MapPin, Globe, Phone, Mail, Image as ImageIcon } from 'lucide-react';
import { sanitizeEnterpriseWebsiteUrl } from '@sdkwork/im-pc-commons';
import { EnterpriseData } from './EnterpriseDetail';

interface EnterpriseDetailAboutProps {
  enterprise: EnterpriseData;
}

export const EnterpriseDetailAbout: React.FC<EnterpriseDetailAboutProps> = ({ enterprise }) => {
  const { t } = useTranslation('enterprise');
  const websiteUrl = sanitizeEnterpriseWebsiteUrl(enterprise.website);
  const hasContactInfo = Boolean(websiteUrl || enterprise.location);

  return (
    <div className="flex flex-col gap-8">
      <section className="bg-white dark:bg-[#18181b] rounded-2xl p-8 border border-gray-200 dark:border-white/5 shadow-sm animate-in fade-in slide-in-from-bottom-4 duration-300">
        <h3 className="text-xl font-bold text-gray-900 dark:text-white mb-6 flex items-center gap-2">
          <Building2 className="text-indigo-500" size={22} />
          {t('detail.about')}
        </h3>
        <p className="text-gray-700 dark:text-gray-300 leading-[1.8] text-[15px] whitespace-pre-wrap">
          {enterprise.description || t('marketplace.profileUnavailable', { defaultValue: 'Extended enterprise profile data is not available yet.' })}
        </p>
      </section>

      <section className="bg-white dark:bg-[#18181b] rounded-2xl p-8 border border-gray-200 dark:border-white/5 shadow-sm animate-in fade-in slide-in-from-bottom-4 duration-300 delay-75">
        <h3 className="text-xl font-bold text-gray-900 dark:text-white mb-6 flex items-center gap-2">
          <ImageIcon className="text-indigo-500" size={22} />
          {t('detail.album')}
        </h3>
        <div className="py-8 text-center text-sm text-gray-500 dark:text-gray-400">
          {t('detail.albumEmpty', { defaultValue: 'No enterprise album media is available yet.' })}
        </div>
      </section>

      {hasContactInfo ? (
        <section className="bg-white dark:bg-[#18181b] rounded-2xl p-8 border border-gray-200 dark:border-white/5 shadow-sm animate-in fade-in slide-in-from-bottom-4 duration-300 delay-150">
          <h3 className="text-xl font-bold text-gray-900 dark:text-white mb-6 flex items-center gap-2">
            <MapPin className="text-indigo-500" size={22} />
            {t('detail.contactInfo')}
          </h3>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            {websiteUrl ? (
              <div className="flex items-start gap-4">
                <div className="w-12 h-12 rounded-xl bg-gray-50 dark:bg-white/5 border border-gray-100 dark:border-white/5 flex items-center justify-center shrink-0">
                  <Globe className="text-gray-500 dark:text-gray-400" size={20} />
                </div>
                <div className="flex flex-col justify-center h-12">
                  <div className="text-sm font-medium text-gray-500 dark:text-gray-400 mb-0.5">{t('detail.website')}</div>
                  <a href={websiteUrl} target="_blank" rel="noopener noreferrer" className="text-[15px] font-bold text-gray-900 dark:text-gray-100 hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors">
                    {enterprise.website}
                  </a>
                </div>
              </div>
            ) : null}
            {enterprise.location ? (
              <div className="flex items-start gap-4">
                <div className="w-12 h-12 rounded-xl bg-gray-50 dark:bg-white/5 border border-gray-100 dark:border-white/5 flex items-center justify-center shrink-0">
                  <MapPin className="text-gray-500 dark:text-gray-400" size={20} />
                </div>
                <div className="flex flex-col justify-center h-12">
                  <div className="text-sm font-medium text-gray-500 dark:text-gray-400 mb-0.5">{t('detail.address')}</div>
                  <div className="text-[15px] font-bold text-gray-900 dark:text-gray-100">{enterprise.location}</div>
                </div>
              </div>
            ) : null}
            <div className="flex items-start gap-4 opacity-60">
              <div className="w-12 h-12 rounded-xl bg-gray-50 dark:bg-white/5 border border-gray-100 dark:border-white/5 flex items-center justify-center shrink-0">
                <Phone className="text-gray-500 dark:text-gray-400" size={20} />
              </div>
              <div className="flex flex-col justify-center h-12">
                <div className="text-sm font-medium text-gray-500 dark:text-gray-400 mb-0.5">{t('detail.phone')}</div>
                <div className="text-[15px] font-bold text-gray-900 dark:text-gray-100">{t('marketplace.unavailable', { defaultValue: 'Unavailable' })}</div>
              </div>
            </div>
            <div className="flex items-start gap-4 opacity-60">
              <div className="w-12 h-12 rounded-xl bg-gray-50 dark:bg-white/5 border border-gray-100 dark:border-white/5 flex items-center justify-center shrink-0">
                <Mail className="text-gray-500 dark:text-gray-400" size={20} />
              </div>
              <div className="flex flex-col justify-center h-12">
                <div className="text-sm font-medium text-gray-500 dark:text-gray-400 mb-0.5">{t('detail.email')}</div>
                <div className="text-[15px] font-bold text-gray-900 dark:text-gray-100">{t('marketplace.unavailable', { defaultValue: 'Unavailable' })}</div>
              </div>
            </div>
          </div>
        </section>
      ) : null}
    </div>
  );
};
