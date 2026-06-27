import React, { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Search, Plus, Building2, Briefcase, Package, ShoppingCart, Filter, MessageSquare, Phone } from 'lucide-react';
import { EnterpriseList } from './components/EnterpriseList';
import { SupplyList } from './components/SupplyList';
import { RecruitList } from './components/RecruitList';
import { PurchaseList } from './components/PurchaseList';
import { EnterpriseDetail, EnterpriseData } from './components/EnterpriseDetail';
import { enterpriseMarketplaceService } from './services/EnterpriseMarketplaceService';

export interface EnterpriseViewProps {
  onStartChat?: (enterpriseId: string, enterpriseName: string) => void;
  onCall?: (enterpriseId: string, enterpriseName: string) => void;
}

export const EnterpriseView: React.FC<EnterpriseViewProps> = ({ onStartChat, onCall }) => {
  const { t } = useTranslation('enterprise');
  const [activeTab, setActiveTab] = useState<'enterprises' | 'supplies' | 'recruits' | 'purchases'>('enterprises');
  const [isPlusOpen, setIsPlusOpen] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedEnterprise, setSelectedEnterprise] = useState<EnterpriseData | null>(null);
  const [publishError, setPublishError] = useState<string | null>(null);

  const handlePublish = async () => {
    setPublishError(null);
    try {
      await enterpriseMarketplaceService.publishListing();
    } catch (error) {
      setPublishError(error instanceof Error ? error.message : 'enterprise publish contract is not available');
    }
  };

  const tabs = [
    { id: 'enterprises', label: t('tabs.enterprises'), icon: Building2 },
    { id: 'supplies', label: t('tabs.supplies'), icon: Package },
    { id: 'purchases', label: t('tabs.purchases'), icon: ShoppingCart },
    { id: 'recruits', label: t('tabs.recruits'), icon: Briefcase },
  ] as const;

  return (
    <div className="flex-1 flex flex-col min-w-0 bg-[#f8fafc] dark:bg-[#121214] relative">
      {selectedEnterprise ? (
        <EnterpriseDetail 
          enterprise={selectedEnterprise} 
          onBack={() => setSelectedEnterprise(null)} 
          onStartChat={(id) => onStartChat?.(id, selectedEnterprise.name)}
          onCall={(id) => onCall?.(id, selectedEnterprise.name)}
        />
      ) : (
        <>
          {/* Top Banner / Hero Section */}
          <div className="bg-white dark:bg-[#18181b] border-b border-gray-200 dark:border-white/5 pb-4">
            <div className="h-16 flex items-center justify-between px-8">
              <h2 className="text-xl font-bold tracking-tight text-gray-900 dark:text-white">{t('title')}</h2>
              <div className="relative">
                <button 
                  onClick={() => setIsPlusOpen(!isPlusOpen)}
                  className={`px-4 py-2 rounded-xl transition-colors flex items-center gap-2 font-medium text-sm ${
                    isPlusOpen 
                      ? 'bg-indigo-600 text-white' 
                      : 'bg-indigo-50 dark:bg-indigo-500/10 text-indigo-600 dark:text-indigo-400 hover:bg-indigo-100 dark:hover:bg-indigo-500/20'
                  }`}
                >
                  <Plus size={16} />
                  {t('publish')}
                </button>
                
                {isPlusOpen && (
                  <>
                    <div className="fixed inset-0 z-40" onClick={() => setIsPlusOpen(false)} />
                    <div className="absolute right-0 top-full mt-2 w-48 bg-white dark:bg-[#2b2b2d] border border-gray-200 dark:border-white/10 rounded-xl shadow-2xl overflow-hidden z-50 py-1">
                      {publishError ? (
                        <div className="px-4 py-2 text-xs text-red-500 border-b border-gray-100 dark:border-white/5">
                          {publishError}
                        </div>
                      ) : null}
                      <button type="button" onClick={() => void handlePublish()} className="w-full px-4 py-3 flex items-center gap-3 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-white/5 transition-colors">
                        <Building2 size={16} className="text-gray-400" />
                        <span>{t('applySettle')}</span>
                      </button>
                      <button type="button" onClick={() => void handlePublish()} className="w-full px-4 py-3 flex items-center gap-3 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-white/5 transition-colors">
                        <Briefcase size={16} className="text-gray-400" />
                        <span>{t('postRecruit')}</span>
                      </button>
                      <button type="button" onClick={() => void handlePublish()} className="w-full px-4 py-3 flex items-center gap-3 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-white/5 transition-colors">
                        <Package size={16} className="text-gray-400" />
                        <span>{t('postSupply')}</span>
                      </button>
                      <button type="button" onClick={() => void handlePublish()} className="w-full px-4 py-3 flex items-center gap-3 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-white/5 transition-colors">
                        <ShoppingCart size={16} className="text-gray-400" />
                        <span>{t('postPurchase')}</span>
                      </button>
                    </div>
                  </>
                )}
              </div>
            </div>

            {/* Search Bar */}
            <div className="px-8 mt-2 max-w-4xl mx-auto w-full">
              <div className="relative group flex items-center">
                <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                  <Search className="h-5 w-5 text-gray-400 group-focus-within:text-indigo-500 transition-colors" />
                </div>
                <input
                  type="text"
                  placeholder={t('searchPlaceholder')}
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="block w-full pl-11 pr-4 py-3.5 bg-gray-50 dark:bg-white/5 border border-gray-200 dark:border-white/10 rounded-2xl text-gray-900 dark:text-gray-100 placeholder:text-gray-500 focus:bg-white dark:focus:bg-[#1f1f22] focus:ring-2 focus:ring-indigo-500/50 focus:border-indigo-500 transition-all text-base shadow-sm"
                />
                <button className="absolute right-3 p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors">
                  <Filter size={18} />
                </button>
              </div>
            </div>

            {/* Horizontal Tabs */}
            <div className="px-8 mt-6">
              <div className="flex gap-1 border-b border-gray-200 dark:border-white/5 pb-[1px]">
                {tabs.map(tab => (
                  <button
                    key={tab.id}
                    onClick={() => setActiveTab(tab.id as any)}
                    className={`flex items-center gap-2 px-5 py-3 text-sm font-medium transition-all relative ${
                      activeTab === tab.id
                        ? 'text-indigo-600 dark:text-indigo-400'
                        : 'text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200'
                    }`}
                  >
                    <tab.icon size={16} />
                    {tab.label}
                    {activeTab === tab.id && (
                      <div className="absolute bottom-[-1px] left-0 right-0 h-[2px] bg-indigo-600 dark:bg-indigo-400 rounded-t-full shadow-[0_0_8px_rgba(79,70,229,0.5)]" />
                    )}
                  </button>
                ))}
              </div>
            </div>
          </div>

          {/* Dynamic List */}
          <div className="flex-1 overflow-y-auto custom-scrollbar p-6">
            <div className="max-w-6xl mx-auto w-full">
              {activeTab === 'enterprises' && <EnterpriseList onSelectEnterprise={setSelectedEnterprise} />}
              {activeTab === 'supplies' && <SupplyList />}
              {activeTab === 'recruits' && <RecruitList />}
              {activeTab === 'purchases' && <PurchaseList />}
            </div>
          </div>
        </>
      )}
    </div>
  );
};
