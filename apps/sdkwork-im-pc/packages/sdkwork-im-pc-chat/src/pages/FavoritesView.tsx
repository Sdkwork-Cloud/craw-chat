import React, { useState, useEffect, useMemo } from 'react';
import { Star, Image as ImageIcon, Link2, FileText, MessageSquare, LayoutGrid, List } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { cn } from '@sdkwork/im-pc-commons';
import { toast } from '../components/Toast';
import { favoriteService } from '../services/FavoriteService';
import type { FavoriteItem } from '../services/FavoriteService';

export const FavoritesView: React.FC<{ searchQuery?: string }> = ({ searchQuery = '' }) => {
  const { t } = useTranslation();
  const filters = useMemo(() => ([
    { id: 'all', name: t('favorites.filters.all'), icon: <Star size={18} /> },
    { id: 'links', name: t('favorites.filters.links'), icon: <Link2 size={18} /> },
    { id: 'images', name: t('favorites.filters.images'), icon: <ImageIcon size={18} /> },
    { id: 'files', name: t('favorites.filters.files'), icon: <FileText size={18} /> },
    { id: 'chat', name: t('favorites.filters.chat'), icon: <MessageSquare size={18} /> },
  ]), [t]);
  const [activeFilter, setActiveFilter] = useState('all');
  const [favorites, setFavorites] = useState<FavoriteItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [viewMode, setViewMode] = useState<'list' | 'grid'>('list');
  const [localSearch, setLocalSearch] = useState('');

  useEffect(() => {
    const loadData = async () => {
      setLoading(true);
      try {
        const data = await favoriteService.getFavorites(activeFilter);
        setFavorites(data);
      } catch {
        toast(t('favorites.toast.loadFailed'), 'error');
      } finally {
        setLoading(false);
      }
    };
    void loadData();
  }, [activeFilter, t]);

  const filteredFavorites = favorites.filter(fav => {
    const q = localSearch.trim() || searchQuery.trim();
    if (!q) return true;
    return fav.title.toLowerCase().includes(q.toLowerCase()) || fav.content?.toLowerCase().includes(q.toLowerCase());
  });

  const getIcon = (type: string) => {
    switch (type) {
      case 'link': return <Link2 size={12} />;
      case 'image': return <ImageIcon size={12} />;
      case 'file': return <FileText size={12} />;
      case 'chat': return <MessageSquare size={12} />;
      default: return <Star size={12} />;
    }
  };

  const formatTime = (timestamp: number) => {
    const date = new Date(timestamp);
    return `${date.getMonth() + 1}/${date.getDate()} ${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}`;
  };

  return (
    <div className="flex flex-1 min-h-0">
      <div className="flex w-[280px] shrink-0 flex-col bg-[#202020] border-r border-white/5 min-h-0">
        <div className="flex-1 overflow-y-auto custom-scrollbar py-2">
          {filters.map(item => (
            <div
              key={item.id}
              onClick={() => setActiveFilter(item.id)}
              className={cn(
                "flex items-center px-4 py-3 cursor-pointer transition-colors hover:bg-white/5",
                activeFilter === item.id && "bg-white/10 hover:bg-white/10"
              )}
            >
              <div className="w-[32px] h-[32px] flex items-center justify-center text-gray-400 shrink-0 mr-2">
                {item.icon}
              </div>
              <span className="text-[14px] text-gray-200">{item.name}</span>
            </div>
          ))}
        </div>
      </div>

      <div className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0 p-6 lg:p-8 overflow-y-auto custom-scrollbar">
        <div className="w-full h-full flex flex-col">
          <div className="flex items-center justify-between mb-8 shrink-0">
            <div>
              <h2 className="text-xl font-medium text-gray-200 mb-1">{t('favorites.title')}</h2>
              <p className="text-sm text-gray-500">{t('favorites.description')}</p>
            </div>

            <div className="flex items-center gap-3">
              <div className="flex bg-[#181818] border border-white/10 rounded-lg p-0.5">
                <button
                  onClick={() => setViewMode('list')}
                  className={cn("p-1.5 rounded-md transition-colors", viewMode === 'list' ? 'bg-[#2b2b2d] text-gray-200 shadow-sm' : 'text-gray-500 hover:text-gray-300')}
                  title={t('favorites.viewList')}
                >
                  <List size={16} />
                </button>
                <button
                  onClick={() => setViewMode('grid')}
                  className={cn("p-1.5 rounded-md transition-colors", viewMode === 'grid' ? 'bg-[#2b2b2d] text-gray-200 shadow-sm' : 'text-gray-500 hover:text-gray-300')}
                  title={t('favorites.viewGrid')}
                >
                  <LayoutGrid size={16} />
                </button>
              </div>
              <div className="relative">
                <input
                  type="text"
                  placeholder={t('favorites.searchPlaceholder')}
                  value={localSearch}
                  onChange={(e) => setLocalSearch(e.target.value)}
                  className="w-64 bg-[#181818] border border-white/10 rounded-lg pl-3 pr-4 py-1.5 text-sm text-gray-200 outline-none focus:border-blue-500"
                />
              </div>
            </div>
          </div>

          <div className={cn(
            viewMode === 'grid'
              ? "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-5"
              : "flex flex-col gap-3"
          )}>
            {loading ? (
              <div className="text-gray-500 text-sm col-span-full">{t('favorites.loading')}</div>
            ) : filteredFavorites.length === 0 ? (
              <div className="text-gray-500 text-sm col-span-full flex justify-center py-20">{t('favorites.empty')}</div>
            ) : (
              filteredFavorites.map(item => (
                <div
                  key={item.id}
                  className={cn(
                    "bg-[#2b2b2d] border border-white/5 hover:border-blue-500/30 transition-all cursor-pointer group shadow-sm hover:shadow-md hover:-translate-y-0.5 relative",
                    viewMode === 'grid'
                      ? "rounded-xl p-5 flex flex-col"
                      : "rounded-xl p-4 flex flex-row items-center gap-4"
                  )}
                  onClick={() => toast(t('favorites.toast.opening', { title: item.title }), 'success')}
                >
                  <button
                    onClick={async (e) => {
                       e.stopPropagation();
                       try {
                         await favoriteService.removeFavorite(item.id);
                         setFavorites(favorites.filter(f => f.id !== item.id));
                         toast(t('favorites.toast.removeSucceeded'), 'success');
                       } catch {
                         toast(t('favorites.toast.removeFailed'), 'error');
                       }
                    }}
                    className="absolute top-2 right-2 p-1.5 bg-[#181818] border border-white/10 rounded-md text-gray-500 hover:text-red-400 opacity-0 group-hover:opacity-100 transition-opacity z-10"
                    title={t('favorites.remove')}
                  >
                    <Star size={14} className="fill-current" />
                  </button>

                  {viewMode === 'grid' ? (
                    <>
                      <div className="flex items-center gap-3 mb-3 shrink-0">
                        <div className="w-8 h-8 rounded-lg bg-blue-500/10 flex items-center justify-center text-blue-400 group-hover:bg-blue-500 group-hover:text-white transition-colors shadow-sm">
                          {getIcon(item.type)}
                        </div>
                        <div className="flex-1 min-w-0">
                          <div className="text-[12px] text-gray-400 truncate">{t('favorites.fromSource', { source: item.source })}</div>
                          <div className="text-[11px] text-gray-500">{formatTime(item.timestamp)}</div>
                        </div>
                      </div>
                      <div className="text-[15px] font-medium text-gray-200 mb-3 truncate shrink-0 group-hover:text-blue-400 transition-colors">
                        {item.title}
                      </div>
                      <div className="text-[13px] text-gray-400 bg-[#1e1e1e] p-3 rounded-lg flex-1 line-clamp-3 leading-relaxed border border-white/5">
                        {item.content}
                      </div>
                    </>
                  ) : (
                    <>
                      <div className="w-10 h-10 shrink-0 rounded-lg bg-blue-500/10 flex items-center justify-center text-blue-400 group-hover:bg-blue-500 group-hover:text-white transition-colors shadow-sm">
                        {getIcon(item.type)}
                      </div>
                      <div className="flex-1 min-w-0 flex flex-col justify-center">
                        <div className="text-[15px] font-medium text-gray-200 truncate group-hover:text-blue-400 transition-colors">
                          {item.title}
                        </div>
                        <div className="text-[13px] text-gray-400 truncate mt-1">
                          {item.content}
                        </div>
                      </div>
                      <div className="shrink-0 flex flex-col items-end justify-center min-w-[120px]">
                        <div className="text-[12px] text-gray-400">{t('favorites.fromSource', { source: item.source })}</div>
                        <div className="text-[11px] text-gray-500 mt-1">{formatTime(item.timestamp)}</div>
                      </div>
                    </>
                  )}
                </div>
              ))
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
