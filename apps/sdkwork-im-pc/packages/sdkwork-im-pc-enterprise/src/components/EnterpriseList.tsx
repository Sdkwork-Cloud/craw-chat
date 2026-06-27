import React, { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Building2, MapPin, Users, Globe, ExternalLink, ShieldCheck, ShieldAlert } from 'lucide-react';
import { EnterpriseData } from './EnterpriseDetail';
import { enterpriseService } from '../services/EnterpriseService';

interface EnterpriseListProps {
  onSelectEnterprise?: (ent: EnterpriseData) => void;
}

export const EnterpriseList: React.FC<EnterpriseListProps> = ({ onSelectEnterprise }) => {
  const { t } = useTranslation('enterprise');
  const [filter, setFilter] = useState<'all' | 'verified' | 'unverified'>('all');
  const [enterprises, setEnterprises] = useState<EnterpriseData[]>([]);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | null>(null);

  useEffect(() => {
    let mounted = true;

    const loadEnterprises = async () => {
      setLoading(true);
      setLoadError(null);
      try {
        const items = await enterpriseService.getEnterprises();
        if (mounted) {
          setEnterprises(items);
        }
      } catch (error) {
        if (mounted) {
          setLoadError(error instanceof Error ? error.message : 'enterprise catalog load failed');
          setEnterprises([]);
        }
      } finally {
        if (mounted) {
          setLoading(false);
        }
      }
    };

    void loadEnterprises();
    return () => {
      mounted = false;
    };
  }, []);

  const filteredEnterprises = enterprises.filter(ent => {
    if (filter === 'verified') return ent.isVerified;
    if (filter === 'unverified') return !ent.isVerified;
    return true;
  });

  return (
    <div className="max-w-[1200px] mx-auto space-y-6">
      <div className="flex items-center gap-2 mb-8 bg-gray-100 dark:bg-[#28282b] p-1 rounded-xl w-fit">
        <button
          onClick={() => setFilter('all')}
          className={`px-6 py-2.5 rounded-lg text-sm font-bold transition-all ${
            filter === 'all'
              ? 'bg-white dark:bg-[#38383b] text-gray-900 dark:text-gray-100 shadow-sm'
              : 'text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-200 dark:hover:bg-white/5'
          }`}
        >
          {t('list.allEnterprises')}
        </button>
        <button
          onClick={() => setFilter('verified')}
          className={`px-6 py-2.5 rounded-lg text-sm font-bold transition-all flex items-center gap-2 ${
            filter === 'verified'
              ? 'bg-white dark:bg-[#38383b] text-indigo-600 dark:text-indigo-400 shadow-sm'
              : 'text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-200 dark:hover:bg-white/5'
          }`}
        >
          <ShieldCheck size={16} className={filter === 'verified' ? '' : 'text-green-500'} />
          {t('list.verified')}
        </button>
        <button
          onClick={() => setFilter('unverified')}
          className={`px-6 py-2.5 rounded-lg text-sm font-bold transition-all flex items-center gap-2 ${
            filter === 'unverified'
              ? 'bg-white dark:bg-[#38383b] text-orange-600 dark:text-orange-400 shadow-sm'
              : 'text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-200 dark:hover:bg-white/5'
          }`}
        >
          <ShieldAlert size={16} className={filter === 'unverified' ? '' : 'text-orange-400'} />
          {t('list.unverified')}
        </button>
      </div>

      {loading ? (
        <div className="py-16 text-center text-sm text-gray-500 dark:text-gray-400">
          {t('loading', { defaultValue: 'Loading...' })}
        </div>
      ) : loadError ? (
        <div className="py-16 text-center text-sm text-red-500">
          {loadError}
        </div>
      ) : filteredEnterprises.length === 0 ? (
        <div className="py-16 text-center text-sm text-gray-500 dark:text-gray-400">
          {t('empty', { defaultValue: 'No enterprises found' })}
        </div>
      ) : (
        <div className="grid grid-cols-1 xl:grid-cols-2 gap-6">
          {filteredEnterprises.map(ent => (
          <div 
            key={ent.id} 
            onClick={() => onSelectEnterprise?.(ent)}
            className="bg-white dark:bg-[#28282b] rounded-2xl p-6 border border-gray-200 dark:border-white/5 shadow-sm hover:shadow-xl dark:hover:shadow-2xl hover:border-indigo-200 dark:hover:border-indigo-500/30 transition-all group flex flex-col h-full cursor-pointer relative overflow-hidden"
          >
            {/* Verification Badge */}
            <div className="absolute top-6 right-6">
              {ent.isVerified ? (
                <div className="flex items-center gap-1.5 px-3 py-1 bg-green-50 dark:bg-green-500/10 border border-green-200 dark:border-green-500/20 rounded-full">
                  <ShieldCheck className="text-green-500" size={14} />
                  <span className="text-xs font-bold text-green-600 dark:text-green-400">{t('list.verified')}</span>
                </div>
              ) : (
                <div className="flex items-center gap-1.5 px-3 py-1 bg-orange-50 dark:bg-orange-500/10 border border-orange-200 dark:border-orange-500/20 rounded-full">
                  <ShieldAlert className="text-orange-400" size={14} />
                  <span className="text-xs font-bold text-orange-600 dark:text-orange-400">{t('list.unverified')}</span>
                </div>
              )}
            </div>

            <div className="flex gap-5 mb-5 pr-24">
              <div className="w-20 h-20 rounded-xl bg-gray-50 dark:bg-[#1a1a1c] border border-gray-100 dark:border-white/5 flex items-center justify-center shrink-0 overflow-hidden">
                <img src={ent.logo} alt={ent.name} className="w-12 h-12 object-contain" />
              </div>
              <div className="flex-1 min-w-0">
                <h3 className="text-xl font-bold text-gray-900 dark:text-gray-100 truncate group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors">{ent.name}</h3>
                <div className="flex items-center gap-4 mt-2 text-sm text-gray-500 dark:text-gray-400">
                  <span className="flex items-center gap-1.5"><Building2 size={14} className="text-indigo-400" /> {ent.industry}</span>
                  <span className="flex items-center gap-1.5"><MapPin size={14} /> {ent.location}</span>
                  <span className="flex items-center gap-1.5"><Users size={14} /> {ent.size}</span>
                </div>
                <div className="flex flex-wrap gap-2 mt-3">
                  {ent.tags.map(tag => (
                    <span key={tag} className="px-2.5 py-1 text-xs font-medium bg-gray-100 dark:bg-white/5 text-gray-600 dark:text-gray-300 rounded-md border border-gray-200 dark:border-white/5 group-hover:border-indigo-100 dark:group-hover:border-indigo-500/20 group-hover:bg-indigo-50 dark:group-hover:bg-indigo-500/10 group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors">
                      {tag}
                    </span>
                  ))}
                </div>
              </div>
            </div>
            <p className="text-sm text-gray-600 dark:text-gray-400 leading-relaxed mb-6 line-clamp-3 flex-1 pt-2">
              {ent.description}
            </p>
            <div className="flex items-center justify-between pt-4 border-t border-gray-100 dark:border-white/5 mt-auto">
              <a href="#" className="flex items-center gap-1.5 text-sm text-gray-500 hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors">
                <Globe size={14} /> {ent.website} <ExternalLink size={12} className="opacity-50" />
              </a>
              <button 
                onClick={(e) => {
                  e.stopPropagation();
                  onSelectEnterprise?.(ent);
                }}
                className="px-4 py-1.5 bg-gray-900 hover:bg-gray-800 dark:bg-white dark:hover:bg-gray-200 text-white dark:text-gray-900 rounded-lg text-sm font-medium transition-colors"
              >
                {t('list.visitHome')}
              </button>
            </div>
          </div>
          ))}
        </div>
      )}
    </div>
  );
};
