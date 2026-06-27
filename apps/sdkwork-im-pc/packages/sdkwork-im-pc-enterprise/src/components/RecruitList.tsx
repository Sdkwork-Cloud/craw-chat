import React, { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Briefcase, MapPin, Building2, CircleDollarSign } from 'lucide-react';
import {
  enterpriseMarketplaceService,
  type EnterpriseRecruitListing,
} from '../services/EnterpriseMarketplaceService';

export const RecruitList: React.FC = () => {
  const { t } = useTranslation('enterprise');
  const [recruits, setRecruits] = useState<EnterpriseRecruitListing[]>([]);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | null>(null);

  useEffect(() => {
    let mounted = true;

    const loadRecruits = async () => {
      setLoading(true);
      setLoadError(null);
      try {
        const items = await enterpriseMarketplaceService.getRecruits();
        if (mounted) {
          setRecruits(items);
        }
      } catch (error) {
        if (mounted) {
          setLoadError(error instanceof Error ? error.message : 'enterprise recruit catalog load failed');
          setRecruits([]);
        }
      } finally {
        if (mounted) {
          setLoading(false);
        }
      }
    };

    void loadRecruits();
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

  if (recruits.length === 0) {
    return (
      <div className="py-16 text-center text-sm text-gray-500 dark:text-gray-400">
        {t('marketplace.emptyRecruits', { defaultValue: 'No recruit listings available yet.' })}
      </div>
    );
  }

  return (
    <div className="max-w-[1000px] mx-auto space-y-4">
      {recruits.map(job => (
        <div key={job.id} className="bg-white dark:bg-[#28282b] rounded-2xl p-6 border border-gray-200 dark:border-white/5 shadow-sm hover:shadow-lg transition-all group cursor-pointer">
          <div className="flex justify-between items-start mb-3">
            <div>
              <h3 className="text-lg font-bold text-gray-900 dark:text-gray-100 group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors">{job.title}</h3>
              <div className="flex items-center gap-4 mt-2 text-sm text-gray-500">
                <span className="flex items-center gap-1.5 font-medium text-orange-500"><CircleDollarSign size={14} /> {job.salary}</span>
                <span className="flex items-center gap-1.5"><MapPin size={14} /> {job.location}</span>
                <span className="flex items-center gap-1.5"><Briefcase size={14} /> {job.exp}</span>
                <span className="bg-gray-100 dark:bg-white/5 px-2 py-0.5 rounded text-xs">{job.edu}</span>
              </div>
            </div>
            <button
              type="button"
              onClick={() => void enterpriseMarketplaceService.applyRecruit(job.id)}
              className="px-5 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-xl text-sm font-medium transition-colors shadow-sm"
            >
              {t('recruit.apply')}
            </button>
          </div>
          <div className="flex flex-wrap gap-2 mb-4">
            {job.tags.map(tag => (
              <span key={tag} className="px-2.5 py-1 text-xs text-gray-600 dark:text-gray-400 bg-gray-50 dark:bg-white/5 border border-gray-100 dark:border-white/5 rounded-lg">
                {tag}
              </span>
            ))}
          </div>
          <div className="flex items-center justify-between pt-4 border-t border-gray-100 dark:border-white/5 text-sm">
            <div className="flex items-center gap-2 text-gray-600 dark:text-gray-400">
              <Building2 size={16} />
              <span>{job.company}</span>
            </div>
            <span className="text-gray-400 text-xs">{job.updated}</span>
          </div>
        </div>
      ))}
    </div>
  );
};
