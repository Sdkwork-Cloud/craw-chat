import React, { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Package, ChevronRight, ShoppingCart } from 'lucide-react';
import { EnterpriseData } from './EnterpriseDetail';
import {
  enterpriseMarketplaceService,
  type EnterpriseProductListing,
} from '../services/EnterpriseMarketplaceService';

interface EnterpriseDetailProductsProps {
  enterprise: EnterpriseData;
  onSelectProduct: (product: { type: 'product', id: string, title?: string }) => void;
  onStartChat: (enterpriseId: string) => void;
}

export const EnterpriseDetailProducts: React.FC<EnterpriseDetailProductsProps> = ({ enterprise, onSelectProduct, onStartChat }) => {
  const { t } = useTranslation('enterprise');
  const [products, setProducts] = useState<EnterpriseProductListing[]>([]);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | null>(null);

  useEffect(() => {
    let mounted = true;

    const loadProducts = async () => {
      setLoading(true);
      setLoadError(null);
      try {
        const items = await enterpriseMarketplaceService.getEnterpriseProducts(enterprise.id);
        if (mounted) {
          setProducts(items);
        }
      } catch (error) {
        if (mounted) {
          setLoadError(error instanceof Error ? error.message : 'enterprise product catalog load failed');
          setProducts([]);
        }
      } finally {
        if (mounted) {
          setLoading(false);
        }
      }
    };

    void loadProducts();
    return () => {
      mounted = false;
    };
  }, [enterprise.id]);

  return (
    <div className="flex flex-col gap-8">
      <section className="bg-white dark:bg-[#18181b] rounded-2xl p-8 border border-gray-200 dark:border-white/5 shadow-sm animate-in fade-in slide-in-from-bottom-4 duration-300">
        <h3 className="text-xl font-bold text-gray-900 dark:text-white mb-6 flex items-center gap-2">
          <Package className="text-indigo-500" size={22} />
          {t('detail.coreProducts')}
        </h3>
        {loading ? (
          <div className="py-12 text-center text-sm text-gray-500 dark:text-gray-400">
            {t('loading', { defaultValue: 'Loading...' })}
          </div>
        ) : loadError ? (
          <div className="py-12 text-center text-sm text-red-500">
            {loadError}
          </div>
        ) : products.length === 0 ? (
          <div className="py-12 text-center text-sm text-gray-500 dark:text-gray-400">
            {t('marketplace.emptyProducts', { defaultValue: 'No enterprise products are available yet.' })}
          </div>
        ) : (
          <div className="grid grid-cols-1 gap-6">
            {products.map((product, index) => (
              <div
                key={product.id}
                onClick={() => onSelectProduct({ type: 'product', id: product.id, title: product.title })}
                className="flex flex-col md:flex-row gap-8 p-6 rounded-2xl bg-gray-50/50 dark:bg-white/[0.02] border border-gray-200/60 dark:border-white/5 hover:border-indigo-200 dark:hover:border-indigo-500/30 transition-all hover:bg-white dark:hover:bg-[#1c1c1e] hover:shadow-xl hover:shadow-indigo-500/5 group cursor-pointer"
              >
                <div className="w-full md:w-56 h-36 bg-gradient-to-br from-indigo-50 dark:from-indigo-900/20 to-blue-50 dark:to-blue-900/20 rounded-xl shrink-0 flex items-center justify-center overflow-hidden border border-indigo-100/50 dark:border-indigo-500/10">
                  {index % 2 === 0 ? (
                    <Package className="text-indigo-400 dark:text-indigo-500 group-hover:scale-110 transition-transform duration-500" size={56} />
                  ) : (
                    <ShoppingCart className="text-blue-400 dark:text-blue-500 group-hover:scale-110 transition-transform duration-500" size={56} />
                  )}
                </div>
                <div className="flex flex-col justify-center flex-1">
                  <h4 className="text-xl font-bold text-gray-900 dark:text-gray-100 mb-3 group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors tracking-tight">{product.title}</h4>
                  <p className="text-[15px] text-gray-600 dark:text-gray-400 leading-relaxed mb-6">
                    {product.description}
                  </p>
                  <div className="flex items-center gap-4 mt-auto">
                    <button
                      type="button"
                      onClick={(e) => { e.stopPropagation(); onStartChat(enterprise.id); }}
                      className="px-5 py-2 bg-indigo-600 hover:bg-indigo-700 text-white text-sm font-bold rounded-lg shadow-sm transition-colors shadow-indigo-500/20"
                    >
                      {t('detail.contactForQuote')}
                    </button>
                    <button type="button" className="text-sm font-bold text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200 flex items-center gap-1 transition-colors">
                      {t('detail.learnMore')} <ChevronRight size={16} />
                    </button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </section>
    </div>
  );
};
