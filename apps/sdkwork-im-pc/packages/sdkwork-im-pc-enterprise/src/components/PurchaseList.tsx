import React, { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { ShoppingCart, Clock, Building2, MapPin } from 'lucide-react';
import {
  enterpriseMarketplaceService,
  type EnterprisePurchaseListing,
} from '../services/EnterpriseMarketplaceService';

export const PurchaseList: React.FC = () => {
  const { t } = useTranslation('enterprise');
  const [purchases, setPurchases] = useState<EnterprisePurchaseListing[]>([]);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | null>(null);

  useEffect(() => {
    let mounted = true;

    const loadPurchases = async () => {
      setLoading(true);
      setLoadError(null);
      try {
        const items = await enterpriseMarketplaceService.getPurchases();
        if (mounted) {
          setPurchases(items);
        }
      } catch (error) {
        if (mounted) {
          setLoadError(error instanceof Error ? error.message : 'enterprise purchase catalog load failed');
          setPurchases([]);
        }
      } finally {
        if (mounted) {
          setLoading(false);
        }
      }
    };

    void loadPurchases();
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

  if (purchases.length === 0) {
    return (
      <div className="py-16 text-center text-sm text-gray-500 dark:text-gray-400">
        {t('marketplace.emptyPurchases', { defaultValue: 'No purchase listings available yet.' })}
      </div>
    );
  }

  return (
    <div className="max-w-[1000px] mx-auto space-y-4">
      {purchases.map(item => (
        <div key={item.id} className="bg-white dark:bg-[#28282b] rounded-2xl p-6 border border-gray-200 dark:border-white/5 shadow-sm hover:shadow-lg transition-all group flex flex-col md:flex-row md:items-center gap-6 cursor-pointer">
          <div className="w-16 h-16 rounded-2xl bg-orange-50 dark:bg-orange-500/10 flex items-center justify-center shrink-0 text-orange-500">
            <ShoppingCart size={28} />
          </div>
          <div className="flex-1 min-w-0">
            <div className="flex items-center justify-between gap-4 mb-2">
              <h3 className="text-lg font-bold text-gray-900 dark:text-gray-100 truncate group-hover:text-orange-600 dark:group-hover:text-orange-400 transition-colors">{item.title}</h3>
              <span className="shrink-0 px-2.5 py-1 text-xs font-medium text-orange-600 bg-orange-50 dark:bg-orange-500/10 border border-orange-200 dark:border-orange-500/20 rounded-md">
                {t('purchase.status.purchasing')}
              </span>
            </div>
            <p className="text-sm text-gray-500 dark:text-gray-400 line-clamp-2 mb-3 leading-relaxed">
              {item.description}
            </p>
            <div className="flex flex-wrap items-center gap-x-6 gap-y-2 text-sm text-gray-500 dark:text-gray-400">
              <span className="flex items-center gap-1.5"><Building2 size={14} className="text-gray-400" /> {item.company}</span>
              <span className="flex items-center gap-1.5 text-orange-600 dark:text-orange-500 font-medium">{t('purchase.budget')}: {item.budget}</span>
              <span className="flex items-center gap-1.5"><MapPin size={14} className="text-gray-400" /> {item.location}</span>
              <span className="flex items-center gap-1.5"><Clock size={14} className="text-gray-400" /> {t('purchase.deadline')}: {item.deadline}</span>
            </div>
          </div>
          <div className="shrink-0 pt-4 md:pt-0 md:border-l border-gray-100 dark:border-white/5 md:pl-6">
             <button
               type="button"
               onClick={() => void enterpriseMarketplaceService.contactPurchaseBuyer(item.id)}
               className="w-full md:w-auto px-6 py-2.5 bg-orange-50 hover:bg-orange-100 dark:bg-orange-500/10 dark:hover:bg-orange-500/20 text-orange-600 dark:text-orange-400 rounded-xl font-medium transition-colors border border-orange-100 dark:border-orange-500/20"
             >
               {t('detail.contactForQuote')}
             </button>
          </div>
        </div>
      ))}
    </div>
  );
};
