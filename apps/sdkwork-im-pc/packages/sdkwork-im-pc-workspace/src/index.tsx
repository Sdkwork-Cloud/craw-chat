import React, { useState, useEffect } from 'react';
import { useTranslation, I18nextProvider } from 'react-i18next';
import i18n from './i18n';
import { Calendar, FileText, CheckSquare, Mail, PieChart, Clock, Cloud, ShieldCheck, Search, Plus, MoreHorizontal, Settings, ExternalLink, Server, Video, Image as ImageIcon, Mic, Music, PenTool } from 'lucide-react';
import { toast } from '@sdkwork/im-pc-chat';
import { workspaceService, AppItem, DocumentItem } from './services/WorkspaceService';
import { cn } from '@sdkwork/im-pc-commons';

export interface WorkspaceViewProps {
  onAppSelect?: (appId: string) => void;
}

export const WorkspaceViewComponent: React.FC<WorkspaceViewProps> = ({ onAppSelect }) => {
  const { t, i18n: i18nInstance } = useTranslation();
  const [apps, setApps] = useState<AppItem[]>([]);
  const [docs, setDocs] = useState<DocumentItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [greeting, setGreeting] = useState('');
  const [showAddAppModal, setShowAddAppModal] = useState(false);
  const [newAppName, setNewAppName] = useState(t('defaultCenter'));

  useEffect(() => {
    const hour = new Date().getHours();
    if (hour < 6) setGreeting(t('greeting.lateNight'));
    else if (hour < 12) setGreeting(t('greeting.morning'));
    else if (hour < 14) setGreeting(t('greeting.noon'));
    else if (hour < 18) setGreeting(t('greeting.afternoon'));
    else setGreeting(t('greeting.evening'));

    const loadData = async () => {
      setLoading(true);
      try {
        const [appsData, docsData] = await Promise.all([
          workspaceService.getApps(),
          workspaceService.getRecentDocuments()
        ]);
        setApps(appsData);
        setDocs(docsData);
      } catch (error) {
        toast(t('loadAppFailed'), 'error');
      } finally {
        setLoading(false);
      }
    };
    loadData();
  }, [t]);

  const getIcon = (iconName: string, className?: string) => {
    const props = { size: 24, className };
    switch (iconName) {
      case 'ShieldCheck': return <ShieldCheck {...props} />;
      case 'Calendar': return <Calendar {...props} />;
      case 'CheckSquare': return <CheckSquare {...props} />;
      case 'FileText': return <FileText {...props} />;
      case 'Mail': return <Mail {...props} />;
      case 'PieChart': return <PieChart {...props} />;
      case 'Clock': return <Clock {...props} />;
      case 'Cloud': return <Cloud {...props} />;
      case 'Server': return <Server {...props} />;
      case 'Video': return <Video {...props} />;
      case 'ImageIcon': return <ImageIcon {...props} />;
      case 'Mic': return <Mic {...props} />;
      case 'Music': return <Music {...props} />;
      case 'PenTool': return <PenTool {...props} />;
      default: return <FileText {...props} />;
    }
  };

  const formatTime = (timestamp: number) => {
    const date = new Date(timestamp);
    const now = new Date();
    const isZh = i18nInstance.language?.startsWith('zh');
    if (date.toDateString() === now.toDateString()) {
      return `${t('today')} ${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}`;
    }
    if (isZh) {
      return `${date.getMonth() + 1}${t('month')}${date.getDate()}${t('day')} ${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}`;
    }
    return `${date.getMonth() + 1}/${date.getDate()} ${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}`;
  };

  const isMac = typeof navigator !== 'undefined' && /Mac|iPhone|iPad|iPod/i.test(navigator.platform);

  return (
    <div className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0 min-h-0">
      
      {/* Workspace Header Dashboard */}
      <div className="bg-gradient-to-br from-indigo-900/40 via-[#181818] to-[#181818] border-b border-white/5 shrink-0 px-8 py-8 relative overflow-hidden">
         {/* Decorative circle */}
         <div className="absolute top-0 right-0 w-64 h-64 bg-indigo-500/10 rounded-full blur-[80px] -translate-y-1/2 translate-x-1/3"></div>
         <div className="flex items-center justify-between relative z-10">
            <div>
               <h1 className="text-2xl font-bold text-gray-100 mb-2">{greeting}{t('startWork')}</h1>
               <p className="text-sm text-gray-400">
                 {t('todayOverview', {
                   date: new Date().toLocaleDateString(i18nInstance.language || 'zh-CN', { month: 'long', day: 'numeric', weekday: 'long' }),
                 })}
               </p>
            </div>
         </div>
         
         <div className="mt-8 flex relative z-10">
            <div className="relative w-full max-w-xl group">
               <Search size={18} className="absolute left-4 top-1/2 -translate-y-1/2 text-gray-500 group-focus-within:text-indigo-400 transition-colors" />
               <input 
                 type="text" 
                 placeholder={t('searchPlaceholder')} 
                 className="w-full bg-[#2b2b2d]/80 backdrop-blur-sm border border-white/10 focus:border-indigo-500/50 rounded-xl pl-12 pr-4 py-3.5 text-sm text-gray-200 outline-none transition-all placeholder:text-gray-500 shadow-md focus:shadow-[0_0_0_2px_rgba(99,102,241,0.2)]"
               />
               <div className="absolute right-3 top-1/2 -translate-y-1/2">
                 <kbd className="bg-[#1e1e1e] border border-white/10 text-gray-400 px-2 py-1 rounded text-xs">{isMac ? '⌘K' : 'Ctrl K'}</kbd>
               </div>
            </div>
         </div>
      </div>

      <div className="flex-1 w-full h-full p-8 flex flex-col gap-10 min-h-0 overflow-y-auto custom-scrollbar">
        
        {/* Apps Section */}
        <div>
          <div className="flex items-center justify-between mb-6">
             <h2 className="text-lg font-semibold text-gray-200">{t('commonApps')}</h2>
             <button className="text-sm text-indigo-400 hover:text-indigo-300 font-medium flex items-center gap-1 transition-colors">
               {t('appManagement')} <Settings size={14} />
             </button>
          </div>
          <div className="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 xl:grid-cols-12 gap-x-6 gap-y-8">
            {loading ? (
              <div className="text-gray-500 text-sm col-span-full">{t('loading')}</div>
            ) : apps.map((app) => (
              <div
                key={app.id}
                className="flex flex-col items-center gap-3 cursor-pointer group relative"
                onClick={() => {
                  if (onAppSelect) {
                    onAppSelect(app.id);
                  } else {
                    toast(t('workspaceModuleUnavailable', { name: t(app.nameKey) }), 'error');
                  }
                }}
              >
                <div className={cn("w-14 h-14 rounded-2xl flex items-center justify-center transition-all duration-300 group-hover:scale-110 shadow-lg group-hover:shadow-xl", app.color)}>
                  {getIcon(app.iconName, "text-white")}
                </div>
                <span className="text-[13px] font-medium text-gray-400 group-hover:text-gray-200 transition-colors">{t(app.nameKey)}</span>
              </div>
            ))}
            <div 
              className="flex flex-col items-center gap-3 cursor-pointer group" 
              onClick={() => {
                 setShowAddAppModal(true);
                 setNewAppName(t('defaultCenter'));
              }}
            >
              <div className="w-14 h-14 rounded-2xl bg-[#2b2b2d] border border-white/10 flex items-center justify-center transition-all duration-300 group-hover:scale-110 group-hover:bg-[#343438] group-hover:border-white/20">
                <Plus size={24} className="text-gray-400" />
              </div>
              <span className="text-[13px] font-medium text-gray-400 group-hover:text-gray-200 transition-colors">{t('addApp')}</span>
            </div>
          </div>
        </div>
        
        {/* Recents Section */}
        <div className="flex flex-col flex-1">
          <div className="flex items-center justify-between mb-6">
             <h2 className="text-lg font-semibold text-gray-200">{t('recentDocs')}</h2>
             <button className="text-sm text-gray-400 hover:text-gray-200 font-medium flex items-center gap-1 transition-colors">
               {t('viewAll')} <ExternalLink size={14} />
             </button>
          </div>
          <div className="bg-[#2b2b2d] rounded-xl border border-white/5 flex flex-col flex-1 h-full shadow-lg overflow-hidden">
            {loading ? (
              <div className="p-3 text-gray-500 text-sm">{t('loading')}</div>
            ) : docs.length === 0 ? (
              <div className="p-10 flex flex-col items-center justify-center text-center">
                 <FileText size={48} className="text-gray-600 mb-4" />
                 <p className="text-gray-300 font-medium mb-1">{t('noRecentDocs')}</p>
                 <p className="text-gray-500 text-sm">{t('noRecentDocsDesc')}</p>
              </div>
            ) : (
              <div className="flex-1 flex flex-col overflow-y-auto custom-scrollbar">
                {docs.map(doc => (
                  <div key={doc.id} className="flex items-center justify-between p-4 px-6 hover:bg-white/5 rounded-none cursor-pointer transition-colors border-b border-white/5 last:border-0 group" onClick={() => {
                     if (onAppSelect) onAppSelect('drive');
                     else toast(t('openingDoc', { name: t(doc.nameKey) }), 'success');
                  }}>
                    <div className="flex items-center gap-4">
                      <div className="w-10 h-10 rounded-lg bg-indigo-500/10 border border-indigo-500/20 flex items-center justify-center group-hover:bg-indigo-500/20 transition-colors">
                        <FileText size={20} className="text-indigo-400" />
                      </div>
                      <div className="flex flex-col">
                        <span className="text-[15px] font-medium text-gray-200 group-hover:text-indigo-300 transition-colors">{t(doc.nameKey)}</span>
                        <span className="text-[12px] text-gray-500 mt-0.5">{t('sourcePersonalSpace')}</span>
                      </div>
                    </div>
                    <div className="flex items-center gap-4">
                      <div className="flex items-center gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                        <button className="p-1.5 text-gray-400 hover:text-gray-200 hover:bg-white/10 rounded-md transition-colors" title={t('openInNewTab')} onClick={(e) => { e.stopPropagation(); window.open(`about:blank?doc=${doc.id}`, '_blank'); toast(t('openedInNewTab'), 'success'); }}>
                           <ExternalLink size={16} />
                        </button>
                        <button className="p-1.5 text-gray-400 hover:text-gray-200 hover:bg-white/10 rounded-md transition-colors" title={t('more')}>
                           <MoreHorizontal size={16} />
                        </button>
                      </div>
                      <span className="text-[13px] text-gray-400 bg-black/20 border border-white/5 px-3 py-1 rounded-full font-mono">{formatTime(doc.timestamp)}</span>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
        
      </div>

      {showAddAppModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-md animate-in fade-in cursor-default">
           <div className="bg-[#1e1e1e] border border-white/10 rounded-2xl w-full max-w-sm shadow-2xl overflow-hidden animate-in zoom-in-95" onClick={e => e.stopPropagation()}>
              <div className="p-6">
                <h3 className="text-lg font-medium text-white mb-4">{t('addAppCenter')}</h3>
                <input 
                  type="text" 
                  value={newAppName}
                  onChange={e => setNewAppName(e.target.value)}
                  placeholder={t('appCenterNamePlaceholder')} 
                  className="w-full bg-[#181818] border border-white/10 rounded-xl px-4 py-2.5 text-sm text-gray-200 outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 mb-2 font-medium"
                  autoFocus
                  onKeyDown={e => {
                     if (e.key === 'Enter' && newAppName.trim()) {
                        setShowAddAppModal(false);
                        toast(t('workspaceAppCenterUnavailable'), 'error');
                     }
                  }}
                />
              </div>
              <div className="px-6 py-4 flex justify-end gap-3 bg-black/20 border-t border-white/5">
                 <button onClick={() => setShowAddAppModal(false)} className="px-5 py-2 text-sm text-gray-400 hover:text-white hover:bg-white/5 rounded-xl text-medium transition-colors">{t('cancel')}</button>
                 <button 
                    onClick={() => {
                       if (newAppName.trim()) {
                          setShowAddAppModal(false);
                          toast(t('workspaceAppCenterUnavailable'), 'error');
                       }
                    }} 
                    className="px-5 py-2 text-sm bg-indigo-600 hover:bg-indigo-500 text-white font-medium rounded-xl transition-colors disabled:opacity-50"
                    disabled={!newAppName.trim()}
                 >
                    {t('confirm')}
                 </button>
              </div>
           </div>
        </div>
      )}
    </div>
  );
};

export const WorkspaceView: React.FC<WorkspaceViewProps> = (props) => {
  return (
    <I18nextProvider i18n={i18n}>
      <WorkspaceViewComponent {...props} />
    </I18nextProvider>
  );
};
