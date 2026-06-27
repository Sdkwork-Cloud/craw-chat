import React, { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { ArrowLeft, Building2, MapPin, Users, Globe, Phone, MessageSquare, ShieldCheck, ShieldAlert } from 'lucide-react';
import { sanitizeEnterpriseWebsiteUrl } from '@sdkwork/im-pc-commons';
import { EnterpriseDetailAbout } from './EnterpriseDetailAbout';
import { EnterpriseDetailProducts } from './EnterpriseDetailProducts';
import { EnterpriseDetailRecruits } from './EnterpriseDetailRecruits';
import { EnterpriseDetailOverlay } from './EnterpriseDetailOverlay';

export interface EnterpriseData {
  id: string;
  name: string;
  logo: string;
  industry: string;
  location: string;
  size: string;
  description: string;
  tags: string[];
  website: string;
  isVerified: boolean;
}

interface EnterpriseDetailProps {
  enterprise: EnterpriseData;
  onBack: () => void;
  onStartChat: (enterpriseId: string) => void;
  onCall: (enterpriseId: string) => void;
}

export const EnterpriseDetail: React.FC<EnterpriseDetailProps> = ({ enterprise, onBack, onStartChat, onCall }) => {
  const { t } = useTranslation('enterprise');
  const [activeTab, setActiveTab] = useState<'detail' | 'products' | 'recruits'>('detail');
  const [selectedItem, setSelectedItem] = useState<{ type: 'product' | 'job', id: string, title?: string } | null>(null);
  const websiteUrl = sanitizeEnterpriseWebsiteUrl(enterprise.website);

  const tabs = [
    { id: 'detail', label: t('detail.about') },
    { id: 'products', label: t('detail.products') },
    { id: 'recruits', label: t('detail.recruiting') },
  ] as const;

  return (
    <div className="flex-1 flex flex-col bg-[#f8fafc] dark:bg-[#121214] min-h-0 min-w-0 absolute inset-0 z-10 overflow-hidden">
      {/* Header / Navbar */}
      <div className="h-16 flex items-center px-6 border-b border-gray-200 dark:border-white/5 bg-white dark:bg-[#18181b] shrink-0 z-20 shadow-sm relative">
        <button 
          onClick={onBack}
          className="p-2 hover:bg-gray-100 dark:hover:bg-white/5 rounded-xl transition-colors text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200 mr-4"
        >
          <ArrowLeft size={20} />
        </button>
        <h2 className="text-lg font-bold text-gray-900 dark:text-white line-clamp-1">{enterprise.name}</h2>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto custom-scrollbar relative">
        {/* Banner Section */}
        <div className="bg-white dark:bg-[#18181b] border-b border-gray-200 dark:border-white/5 relative">
          {/* Cover Background */}
          <div className="absolute top-0 left-0 right-0 h-32 bg-gradient-to-r from-blue-500/10 via-indigo-500/10 to-purple-500/10 overflow-hidden">
            <div className="absolute inset-0 bg-gradient-to-b from-transparent to-white dark:to-[#18181b]"></div>
          </div>
          
          <div className="px-8 pt-16 relative z-10">
            <div className="max-w-4xl mx-auto flex flex-col md:flex-row gap-8 items-start relative mt-4">
              <div className="w-32 h-32 rounded-2xl bg-white dark:bg-[#28282b] border border-gray-200 dark:border-white/10 flex items-center justify-center shrink-0 overflow-hidden shadow-xl shadow-black/5 relative z-10">
                <img src={enterprise.logo} alt={enterprise.name} className="w-20 h-20 object-contain drop-shadow-sm" />
              </div>
              <div className="flex-1 mt-2">
                <div className="flex items-center gap-3 mb-3">
                  <h1 className="text-3xl font-extrabold text-gray-900 dark:text-white tracking-tight">{enterprise.name}</h1>
                  {enterprise.isVerified ? (
                    <div className="flex items-center gap-1.5 px-2.5 py-1 bg-green-50 dark:bg-green-500/10 border border-green-200 dark:border-green-500/20 rounded-md">
                      <ShieldCheck className="text-green-500" size={14} />
                      <span className="text-xs font-bold text-green-600 dark:text-green-400">{t('detail.verified')}</span>
                    </div>
                  ) : (
                    <div className="flex items-center gap-1.5 px-2.5 py-1 bg-orange-50 dark:bg-orange-500/10 border border-orange-200 dark:border-orange-500/20 rounded-md">
                      <ShieldAlert className="text-orange-400" size={14} />
                      <span className="text-xs font-bold text-orange-600 dark:text-orange-400">{t('detail.unverified')}</span>
                    </div>
                  )}
                </div>
                <div className="flex flex-wrap items-center gap-5 text-sm text-gray-600 dark:text-gray-400 mt-4 font-medium">
                  <span className="flex items-center gap-1.5 bg-gray-50 dark:bg-white/5 px-2 py-1 rounded"><Building2 size={16} className="text-indigo-500" /> {enterprise.industry}</span>
                  <span className="flex items-center gap-1.5 bg-gray-50 dark:bg-white/5 px-2 py-1 rounded"><MapPin size={16} /> {enterprise.location}</span>
                  <span className="flex items-center gap-1.5 bg-gray-50 dark:bg-white/5 px-2 py-1 rounded"><Users size={16} /> {enterprise.size}</span>
                  {websiteUrl ? (
                    <a href={websiteUrl} target="_blank" rel="noopener noreferrer" className="flex items-center gap-1.5 text-indigo-600 dark:text-indigo-400 hover:text-indigo-700 transition-colors bg-indigo-50 dark:bg-indigo-500/10 px-2 py-1 rounded hover:underline">
                      <Globe size={16} /> {enterprise.website}
                    </a>
                  ) : (
                    <span className="flex items-center gap-1.5 bg-gray-50 dark:bg-white/5 px-2 py-1 rounded">
                      <Globe size={16} /> {enterprise.website}
                    </span>
                  )}
                </div>
                <div className="flex flex-wrap gap-2 mt-5 mb-4">
                  {enterprise.tags.map(tag => (
                    <span key={tag} className="px-3 py-1 text-xs font-bold text-gray-600 dark:text-gray-300 bg-white dark:bg-[#18181b] border border-gray-200 dark:border-white/10 rounded-full shadow-sm">
                      {tag}
                    </span>
                  ))}
                </div>
              </div>
              
              <div className="flex flex-col gap-3 mt-4 md:mt-6 w-full md:w-auto shrink-0 mb-4 items-stretch">
                <button 
                  onClick={() => onStartChat(enterprise.id)}
                  className="flex items-center justify-center gap-2 px-8 py-3.5 bg-gradient-to-r from-indigo-600 to-blue-600 hover:from-indigo-500 hover:to-blue-500 text-white rounded-xl shadow-lg shadow-indigo-500/25 transition-all font-bold active:scale-95"
                >
                  <MessageSquare size={18} />
                  {t('detail.consult')}
                </button>
                <button 
                  onClick={() => onCall(enterprise.id)}
                  className="flex items-center justify-center gap-2 px-8 py-3.5 bg-white dark:bg-[#28282b] hover:bg-gray-50 dark:hover:bg-white/5 text-gray-900 dark:text-gray-100 rounded-xl transition-all font-bold active:scale-95 border border-gray-200 dark:border-white/10 shadow-sm hover:shadow"
                >
                  <Phone size={18} />
                  {t('detail.getContact')}
                </button>
              </div>
            </div>

            {/* Horizontal Tabs */}
            <div className="mt-8 flex gap-2 max-w-4xl mx-auto border-t border-gray-100 dark:border-white/5 pt-1">
              {tabs.map(tab => (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id as any)}
                  className={`flex items-center gap-2 px-6 py-4 text-[15px] font-bold transition-all relative outline-none ${
                    activeTab === tab.id
                      ? 'text-indigo-600 dark:text-indigo-400'
                      : 'text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200'
                  }`}
                >
                  {tab.label}
                  {activeTab === tab.id && (
                    <div className="absolute bottom-[-1px] left-8 right-8 h-[3px] bg-indigo-600 dark:bg-indigo-400 rounded-t-full shadow-[0_0_8px_rgba(79,70,229,0.5)]" />
                  )}
                </button>
              ))}
            </div>
          </div>
        </div>

        {/* Detailed Info */}
        <div className="max-w-4xl mx-auto flex flex-col gap-8 py-8 px-8">
          {activeTab === 'detail' && (
            <EnterpriseDetailAbout enterprise={enterprise} />
          )}

          {activeTab === 'products' && (
            <EnterpriseDetailProducts 
              enterprise={enterprise} 
              onSelectProduct={setSelectedItem} 
              onStartChat={onStartChat} 
            />
          )}

          {activeTab === 'recruits' && (
            <EnterpriseDetailRecruits 
              enterprise={enterprise} 
              onSelectJob={setSelectedItem} 
              onStartChat={onStartChat} 
            />
          )}
        </div>
      </div>

      {/* Item Detail Overlay */}
      {selectedItem && (
        <EnterpriseDetailOverlay 
          enterprise={enterprise}
          selectedItem={selectedItem}
          onClose={() => setSelectedItem(null)}
          onStartChat={onStartChat}
        />
      )}
    </div>
  );
};
