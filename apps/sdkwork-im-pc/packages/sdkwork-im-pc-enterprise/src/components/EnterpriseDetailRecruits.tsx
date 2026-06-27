import React, { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Briefcase, MapPin, Send } from 'lucide-react';
import { EnterpriseData } from './EnterpriseDetail';
import {
  enterpriseMarketplaceService,
  type EnterpriseJobListing,
} from '../services/EnterpriseMarketplaceService';

interface EnterpriseDetailRecruitsProps {
  enterprise: EnterpriseData;
  onSelectJob: (job: { type: 'job', id: string, title?: string }) => void;
  onStartChat: (enterpriseId: string) => void;
}

export const EnterpriseDetailRecruits: React.FC<EnterpriseDetailRecruitsProps> = ({ enterprise, onSelectJob, onStartChat }) => {
  const { t } = useTranslation('enterprise');
  const [jobs, setJobs] = useState<EnterpriseJobListing[]>([]);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | null>(null);

  useEffect(() => {
    let mounted = true;

    const loadJobs = async () => {
      setLoading(true);
      setLoadError(null);
      try {
        const items = await enterpriseMarketplaceService.getEnterpriseRecruits(enterprise.id);
        if (mounted) {
          setJobs(items);
        }
      } catch (error) {
        if (mounted) {
          setLoadError(error instanceof Error ? error.message : 'enterprise recruit catalog load failed');
          setJobs([]);
        }
      } finally {
        if (mounted) {
          setLoading(false);
        }
      }
    };

    void loadJobs();
    return () => {
      mounted = false;
    };
  }, [enterprise.id]);

  return (
    <div className="flex flex-col gap-8">
      <section className="bg-white dark:bg-[#18181b] rounded-2xl p-8 border border-gray-200 dark:border-white/5 shadow-sm animate-in fade-in slide-in-from-bottom-4 duration-300">
        <div className="flex items-center justify-between mb-6">
          <h3 className="text-xl font-bold text-gray-900 dark:text-white flex items-center gap-2">
            <Briefcase className="text-indigo-500" size={22} />
            {t('detail.allJobs')}
            {jobs.length > 0 ? (
              <span className="ml-2 text-xs font-bold text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-500/10 px-2.5 py-1 rounded-full">{t('detail.hotJobs', { count: jobs.length })}</span>
            ) : null}
          </h3>
        </div>
        {loading ? (
          <div className="py-12 text-center text-sm text-gray-500 dark:text-gray-400">
            {t('loading', { defaultValue: 'Loading...' })}
          </div>
        ) : loadError ? (
          <div className="py-12 text-center text-sm text-red-500">
            {loadError}
          </div>
        ) : jobs.length === 0 ? (
          <div className="py-12 text-center text-sm text-gray-500 dark:text-gray-400">
            {t('marketplace.emptyRecruits', { defaultValue: 'No recruit listings available yet.' })}
          </div>
        ) : (
          <div className="flex flex-col gap-6 mt-6">
            {jobs.map(job => (
              <div
                key={job.id}
                onClick={() => onSelectJob({ type: 'job', id: job.id, title: job.title })}
                className="flex flex-col sm:flex-row items-start sm:items-center justify-between p-6 rounded-2xl bg-white dark:bg-[#1c1c1e] border border-gray-200/60 dark:border-white/5 hover:border-indigo-300 dark:hover:border-indigo-500/40 transition-all cursor-pointer group hover:shadow-lg hover:shadow-indigo-500/5"
              >
                <div className="flex flex-col gap-3">
                  <h4 className="text-xl font-bold text-gray-900 dark:text-gray-100 group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors">{job.title}</h4>
                  <div className="flex flex-wrap items-center gap-4 text-sm font-medium text-gray-600 dark:text-gray-400">
                    <span className="flex items-center gap-1 bg-gray-50 dark:bg-white/5 px-2 py-1 rounded"><MapPin size={16}/> {job.location}</span>
                    <span className="flex items-center gap-1 bg-gray-50 dark:bg-white/5 px-2 py-1 rounded"><Briefcase size={16} /> {job.exp}</span>
                    <span className="flex items-center gap-1 bg-gray-50 dark:bg-white/5 px-2 py-1 rounded">{job.edu}</span>
                  </div>
                  {job.tags.length > 0 ? (
                    <div className="flex flex-wrap gap-2 mt-1">
                      {job.tags.map(tag => (
                        <span key={tag} className="px-2.5 py-1 text-xs font-semibold bg-gray-100 dark:bg-white/10 text-gray-600 dark:text-gray-300 rounded border border-gray-200 dark:border-white/10">{tag}</span>
                      ))}
                    </div>
                  ) : null}
                </div>
                <div className="flex flex-col items-end mt-5 sm:mt-0">
                  <span className="text-2xl font-extrabold text-indigo-600 dark:text-indigo-400 mb-4 tracking-tight">{job.salary}</span>
                  <button
                    type="button"
                    onClick={(e) => { e.stopPropagation(); onStartChat(enterprise.id); }}
                    className="flex items-center gap-1.5 px-6 py-2.5 bg-indigo-50 dark:bg-indigo-500/10 hover:bg-indigo-600 hover:text-white text-indigo-600 dark:text-indigo-400 font-bold rounded-xl transition-colors active:scale-95 shadow-sm"
                  >
                    <Send size={16} />
                    {t('detail.applyJob')}
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </section>
    </div>
  );
};
