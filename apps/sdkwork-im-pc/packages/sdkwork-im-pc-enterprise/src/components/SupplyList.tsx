import React, { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Package, TrendingUp, Clock, Building2 } from 'lucide-react';
import {
  enterpriseMarketplaceService,
  type EnterpriseSupplyListing,
} from '../services/EnterpriseMarketplaceService';

export const SupplyList: React.FC = () => {
  const { t } = useTranslation('enterprise');
  const [supplies, setSupplies] = useState<EnterpriseSupplyListing[]>([]);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | null>(null);

  useEffect(() => {
    let mounted = true;

    const loadSupplies = async () => {
      setLoading(true);
      setLoadError(null);
      try {
        const items = await enterpriseMarketplaceService.getSupplies();
        if (mounted) {
          setSupplies(items);
        }
      } catch (error) {
        if (mounted) {
          setLoadError(error instanceof Error ? error.message : 'enterprise supply catalog load failed');
          setSupplies([]);
        }
      } finally {
        if (mounted) {
          setLoading(false);
        }
      }
    };

    void loadSupplies();
    return () => {
      mounted = false;
    };
  }, []);

  if (loading) {
    return (
      <div className="py-16 text-center text-sm text-gray-500 dark:text-gray-400">
        {t('loading', { defaultValue: 'Loading...' })}
      </div>
    );
  }

  if (loadError) {
    return (
      <div className="py-16 text-center text-sm text-red-500">
        {loadError}
      </div>
    );
  }

  if (supplies.length === 0) {
    return (
      <div className="py-16 text-center text-sm text-gray-500 dark:text-gray-400">
        {t('marketplace.emptySupplies', { defaultValue: 'No supply listings available yet.' })}
      </div>
    );
  }

  return (
    <div className="max-w-[1000px] mx-auto space-y-4">
      {supplies.map(supply => (
        <div key={supply.id} className="bg-white dark:bg-[#28282b] rounded-2xl p-6 border border-gray-200 dark:border-white/5 shadow-sm hover:shadow-lg transition-all group flex flex-col md:flex-row md:items-center gap-6 cursor-pointer">
          <div className="w-16 h-16 rounded-2xl bg-indigo-50 dark:bg-indigo-500/10 flex items-center justify-center shrink-0 text-indigo-500">
            <Package size={28} />
          </div>
          <div className="flex-1 min-w-0">
            <div className="flex items-center justify-between gap-4 mb-2">
              <h3 className="text-lg font-bold text-gray-900 dark:text-gray-100 truncate group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors">{supply.title}</h3>
              <span className="shrink-0 px-2.5 py-1 text-xs font-medium text-green-600 bg-green-50 dark:bg-green-500/10 border border-green-200 dark:border-green-500/20 rounded-md">
                {t('supply.status.supplying')}
              </span>
            </div>
            <p className="text-sm text-gray-500 dark:text-gray-400 line-clamp-2 mb-3 leading-relaxed">
              {supply.description}
            </p>
            <div className="flex flex-wrap items-center gap-x-6 gap-y-2 text-sm text-gray-500 dark:text-gray-400">
              <span className="flex items-center gap-1.5"><Building2 size={14} className="text-gray-400" /> {supply.company}</span>
              <span className="flex items-center gap-1.5"><TrendingUp size={14} className="text-gray-400" /> {supply.price}/{supply.unit} ({supply.minOrder})</span>
              <span className="flex items-center gap-1.5"><Clock size={14} className="text-gray-400" /> {t('supply.publishedAt')} {supply.date}</span>
            </div>
          </div>
          <div className="shrink-0 pt-4 md:pt-0 md:border-l border-gray-100 dark:border-white/5 md:pl-6">
             <button
               type="button"
               onClick={() => void enterpriseMarketplaceService.contactSupplier(supply.id)}
               className="w-full md:w-auto px-6 py-2.5 bg-indigo-50 hover:bg-indigo-100 dark:bg-indigo-500/10 dark:hover:bg-indigo-500/20 text-indigo-600 dark:text-indigo-400 rounded-xl font-medium transition-colors border border-indigo-100 dark:border-indigo-500/20"
             >
               {t('supply.contactSupplier')}
             </button>
          </div>
        </div>
      ))}
    </div>
  );
};
