import React, {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
} from 'react';
import {
  AlertCircle,
  ArrowRight,
  BookOpen,
  Calendar,
  CheckSquare,
  Clock,
  Cloud,
  FileImage,
  FileSpreadsheet,
  FileText,
  FolderClock,
  LayoutGrid,
  Mail,
  Mic,
  Music,
  PenTool,
  PieChart,
  ReceiptText,
  RefreshCw,
  Search,
  SearchX,
  Server,
  Settings2,
  ShieldCheck,
  Store,
  Users,
  Video,
  X,
  type LucideIcon,
} from 'lucide-react';
import { I18nextProvider, useTranslation } from 'react-i18next';

import { cn, toast } from '@sdkwork/im-pc-commons';

import i18n from './i18n';
import { WorkspaceShortcutDialog } from './components/WorkspaceShortcutDialog';
import {
  workspaceService,
  type AppItem,
  type DocumentItem,
  type WorkspaceDocumentOpenTarget,
  type WorkspaceDataSource,
  type WorkspaceService,
} from './services/WorkspaceService';

export interface WorkspaceViewProps {
  onAppSelect?: (appId: string) => void;
  onDocumentOpen?: (target: WorkspaceDocumentOpenTarget) => void;
  service?: WorkspaceService;
}

const APP_ICONS: Record<string, LucideIcon> = {
  BookOpen,
  Calendar,
  CheckSquare,
  Clock,
  Cloud,
  FileText,
  ImageIcon: FileImage,
  Mail,
  Mic,
  Music,
  PenTool,
  PieChart,
  ReceiptText,
  Server,
  ShieldCheck,
  Store,
  Users,
  Video,
};

function resolveGreetingKey(hour: number): string {
  if (hour < 6) return 'greeting.lateNight';
  if (hour < 12) return 'greeting.morning';
  if (hour < 14) return 'greeting.noon';
  if (hour < 18) return 'greeting.afternoon';
  return 'greeting.evening';
}

function resolveLocale(language: string | undefined): 'zh-CN' | 'en-US' {
  return language?.toLowerCase().startsWith('en') ? 'en-US' : 'zh-CN';
}

function isSameCalendarDay(left: Date, right: Date): boolean {
  return left.getFullYear() === right.getFullYear()
    && left.getMonth() === right.getMonth()
    && left.getDate() === right.getDate();
}

function getDocumentIcon(type: string): LucideIcon {
  if (type === 'excel') return FileSpreadsheet;
  if (type === 'image') return FileImage;
  return FileText;
}

function resolveWorkspaceDataStatusKey(sources: WorkspaceDataSource[]): string {
  if (sources.includes('permission-denied')) return 'workspaceDataPermissionDenied';
  if (sources.includes('offline')) return 'workspaceDataOffline';
  if (sources.includes('unavailable')) return 'workspaceDataUnavailable';
  return 'workspaceDataFallback';
}

export const WorkspaceViewComponent: React.FC<WorkspaceViewProps> = ({
  onAppSelect,
  onDocumentOpen,
  service = workspaceService,
}) => {
  const { t, i18n: i18nInstance } = useTranslation();
  const [apps, setApps] = useState<AppItem[]>([]);
  const [docs, setDocs] = useState<DocumentItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState(false);
  const [fallbackSources, setFallbackSources] = useState<WorkspaceDataSource[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [showShortcutDialog, setShowShortcutDialog] = useState(false);
  const [draftPinnedIds, setDraftPinnedIds] = useState<Set<string>>(new Set());
  const [savingShortcuts, setSavingShortcuts] = useState(false);
  const searchInputRef = useRef<HTMLInputElement>(null);
  const loadRequestRef = useRef(0);

  const locale = resolveLocale(i18nInstance.resolvedLanguage ?? i18nInstance.language);

  const getAppLabel = useCallback(
    (app: AppItem) => t(app.nameKey, { defaultValue: app.id }),
    [t],
  );
  const getDocumentLabel = useCallback(
    (doc: DocumentItem) => doc.nameKey.startsWith('docs.')
      ? t(doc.nameKey, { defaultValue: doc.name })
      : doc.name,
    [t],
  );

  const loadWorkspaceData = useCallback(async () => {
    const requestId = ++loadRequestRef.current;
    setLoading(true);
    setLoadError(false);
    setFallbackSources([]);
    try {
      const workspaceData = await service.getWorkspaceData();
      if (loadRequestRef.current !== requestId) return;
      setApps(workspaceData.apps.items);
      setDocs(workspaceData.documents.items);
      setFallbackSources([
        workspaceData.apps.source,
        workspaceData.documents.source,
      ].filter((source): source is WorkspaceDataSource => source !== 'remote'));
    } catch {
      if (loadRequestRef.current !== requestId) return;
      setLoadError(true);
    } finally {
      if (loadRequestRef.current === requestId) {
        setLoading(false);
      }
    }
  }, [service]);

  useEffect(() => {
    void loadWorkspaceData();
    return () => {
      loadRequestRef.current += 1;
    };
  }, [loadWorkspaceData]);

  useEffect(() => {
    if (showShortcutDialog) {
      return undefined;
    }
    const handleShortcut = (event: KeyboardEvent) => {
      if ((event.metaKey || event.ctrlKey) && !event.altKey && event.key.toLowerCase() === 'k') {
        event.preventDefault();
        searchInputRef.current?.focus();
      }
    };
    window.addEventListener('keydown', handleShortcut);
    return () => window.removeEventListener('keydown', handleShortcut);
  }, [showShortcutDialog]);

  const normalizedQuery = searchQuery.trim().toLocaleLowerCase(locale);
  const queryActive = normalizedQuery.length > 0;
  const pinnedApps = useMemo(() => apps.filter((app) => app.pinned), [apps]);
  const matchingApps = useMemo(() => {
    const candidates = queryActive ? apps : pinnedApps;
    if (!queryActive) return candidates;
    return candidates.filter((app) => {
      const label = getAppLabel(app).toLocaleLowerCase(locale);
      return label.includes(normalizedQuery) || app.id.toLocaleLowerCase(locale).includes(normalizedQuery);
    });
  }, [apps, getAppLabel, locale, normalizedQuery, pinnedApps, queryActive]);
  const matchingDocs = useMemo(() => {
    if (!queryActive) return docs;
    return docs.filter((doc) => getDocumentLabel(doc).toLocaleLowerCase(locale).includes(normalizedQuery));
  }, [docs, getDocumentLabel, locale, normalizedQuery, queryActive]);

  const greeting = t(resolveGreetingKey(new Date().getHours()));
  const todayLabel = useMemo(
    () => new Intl.DateTimeFormat(locale, {
      day: 'numeric',
      month: 'long',
      weekday: 'long',
    }).format(new Date()),
    [locale],
  );

  const formatDocumentTime = useCallback((timestamp: number) => {
    const date = new Date(timestamp);
    if (!Number.isFinite(date.getTime())) return t('time.unknown');
    const now = new Date();
    if (isSameCalendarDay(date, now)) {
      const time = new Intl.DateTimeFormat(locale, {
        hour: '2-digit',
        minute: '2-digit',
      }).format(date);
      return t('time.todayAt', { time });
    }
    return new Intl.DateTimeFormat(locale, {
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      month: 'short',
    }).format(date);
  }, [locale, t]);

  const openApp = useCallback((appId: string, label: string) => {
    if (onAppSelect) {
      onAppSelect(appId);
      return;
    }
    toast(t('workspaceModuleUnavailable', { name: label }), 'error');
  }, [onAppSelect, t]);

  const openDocument = useCallback((doc: DocumentItem) => {
    if (doc.openTarget && onDocumentOpen) {
      onDocumentOpen(doc.openTarget);
      return;
    }
    openApp('drive', t('apps.drive'));
  }, [onDocumentOpen, openApp, t]);

  const openShortcutDialog = useCallback(() => {
    setDraftPinnedIds(new Set(apps.filter((app) => app.pinned).map((app) => app.id)));
    setShowShortcutDialog(true);
  }, [apps]);

  const closeShortcutDialog = useCallback(() => {
    setShowShortcutDialog(false);
  }, []);

  const toggleDraftShortcut = useCallback((appId: string) => {
    setDraftPinnedIds((current) => {
      const app = apps.find((candidate) => candidate.id === appId);
      if (!app || app.required) return current;
      const next = new Set(current);
      if (next.has(appId)) next.delete(appId);
      else next.add(appId);
      return next;
    });
  }, [apps]);

  const savePinnedAppIds = useCallback(async () => {
    setSavingShortcuts(true);
    try {
      await service.savePinnedAppIds(Array.from(draftPinnedIds));
      setApps((current) => current.map((app) => ({
        ...app,
        pinned: app.required || draftPinnedIds.has(app.id),
      })));
      setShowShortcutDialog(false);
      toast(t('shortcutsSaved'), 'success');
    } catch {
      toast(t('shortcutsSaveFailed'), 'error');
    } finally {
      setSavingShortcuts(false);
    }
  }, [draftPinnedIds, service, t]);

  const noSearchResults = queryActive && matchingApps.length === 0 && matchingDocs.length === 0;

  return (
    <div className="flex min-h-0 min-w-0 flex-1 flex-col bg-[#1e1e1e]">
      <header className="shrink-0 border-b border-white/10 bg-[#181818] px-4 py-5 sm:px-6 lg:px-8">
        <div className="flex flex-wrap items-start justify-between gap-4">
          <div className="min-w-0">
            <h1 className="text-xl font-semibold text-gray-100 sm:text-2xl">{greeting}</h1>
            <p className="mt-1 text-sm text-gray-400">{t('todayOverview', { date: todayLabel })}</p>
          </div>
          <div className="flex items-center gap-4 text-xs text-gray-400" aria-label={t('workspaceSummary')}>
            <span className="flex items-center gap-1.5">
              <LayoutGrid aria-hidden="true" className="text-cyan-400" size={15} />
              {t('summary.apps', { count: pinnedApps.length })}
            </span>
            <span className="h-4 w-px bg-white/10" aria-hidden="true" />
            <span className="flex items-center gap-1.5">
              <FolderClock aria-hidden="true" className="text-amber-400" size={15} />
              {t('summary.documents', { count: docs.length })}
            </span>
          </div>
        </div>

        <div className="relative mt-5 w-full max-w-2xl">
          <Search
            aria-hidden="true"
            className="pointer-events-none absolute left-3.5 top-1/2 -translate-y-1/2 text-gray-500"
            size={18}
          />
          <input
            ref={searchInputRef}
            aria-label={t('searchLabel')}
            className="h-11 w-full rounded-lg border border-white/10 bg-[#2b2b2d] pl-11 pr-11 text-sm text-gray-100 outline-none transition-colors placeholder:text-gray-500 focus:border-indigo-400 focus:ring-2 focus:ring-indigo-500/20"
            onChange={(event) => setSearchQuery(event.target.value)}
            placeholder={t('searchPlaceholder')}
            type="search"
            value={searchQuery}
          />
          {queryActive ? (
            <button
              aria-label={t('clearSearch')}
              className="absolute right-2 top-1/2 flex h-8 w-8 -translate-y-1/2 items-center justify-center rounded-md text-gray-500 transition-colors hover:bg-white/10 hover:text-gray-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-400"
              onClick={() => {
                setSearchQuery('');
                searchInputRef.current?.focus();
              }}
              title={t('clearSearch')}
              type="button"
            >
              <X size={16} />
            </button>
          ) : null}
        </div>
      </header>

      <main
        aria-busy={loading}
        className="custom-scrollbar flex min-h-0 flex-1 flex-col gap-8 overflow-y-auto p-4 sm:p-6 lg:p-8"
      >
        {loading ? <span className="sr-only" role="status">{t('loading')}</span> : null}

        {fallbackSources.length > 0 && !loading && !loadError ? (
          <div className="flex flex-wrap items-center justify-between gap-3 rounded-lg border border-amber-500/20 bg-amber-500/5 px-4 py-3" role="status">
            <span className="flex min-w-0 items-center gap-2 text-sm text-amber-200">
              <AlertCircle aria-hidden="true" className="shrink-0 text-amber-400" size={17} />
              {t(resolveWorkspaceDataStatusKey(fallbackSources))}
            </span>
            <button
              className="inline-flex shrink-0 items-center gap-1.5 rounded-md px-2.5 py-1.5 text-sm font-medium text-amber-300 transition-colors hover:bg-amber-400/10 hover:text-amber-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-amber-300"
              onClick={() => void loadWorkspaceData()}
              type="button"
            >
              <RefreshCw aria-hidden="true" size={14} />
              {t('retry')}
            </button>
          </div>
        ) : null}

        {loadError ? (
          <div className="flex flex-1 flex-col items-center justify-center py-16 text-center" role="alert">
            <AlertCircle className="mb-3 text-rose-400" size={34} />
            <h2 className="text-base font-semibold text-gray-200">{t('loadAppFailed')}</h2>
            <p className="mt-1 max-w-sm text-sm text-gray-500">{t('loadAppFailedDescription')}</p>
            <button
              className="mt-5 inline-flex items-center gap-2 rounded-md bg-indigo-600 px-3 py-2 text-sm font-medium text-white transition-colors hover:bg-indigo-500 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-300"
              onClick={() => void loadWorkspaceData()}
              type="button"
            >
              <RefreshCw aria-hidden="true" size={15} />
              {t('retry')}
            </button>
          </div>
        ) : noSearchResults && !loading ? (
          <div className="flex flex-1 flex-col items-center justify-center py-16 text-center" role="status">
            <SearchX className="mb-3 text-gray-600" size={36} />
            <h2 className="text-base font-medium text-gray-300">{t('searchNoResults')}</h2>
            <p className="mt-1 text-sm text-gray-500">{t('searchNoResultsDescription', { query: searchQuery.trim() })}</p>
          </div>
        ) : (
          <>
            {(!queryActive || matchingApps.length > 0 || loading) ? (
              <section aria-labelledby="workspace-apps-heading">
                <div className="mb-4 flex min-h-9 items-center justify-between gap-3">
                  <div className="flex min-w-0 items-center gap-2">
                    <h2 className="truncate text-base font-semibold text-gray-200" id="workspace-apps-heading">
                      {queryActive ? t('matchingApps') : t('commonApps')}
                    </h2>
                    {!loading ? (
                      <span className="text-xs text-gray-500" aria-live="polite">{matchingApps.length}</span>
                    ) : null}
                  </div>
                  {!queryActive && apps.length > 0 ? (
                    <button
                      className="inline-flex items-center gap-1.5 rounded-md px-2.5 py-2 text-sm font-medium text-gray-400 transition-colors hover:bg-white/5 hover:text-gray-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-400"
                      onClick={openShortcutDialog}
                      type="button"
                    >
                      <Settings2 aria-hidden="true" size={15} />
                      {t('manageShortcuts')}
                    </button>
                  ) : null}
                </div>

                {loading ? (
                  <div className="grid grid-cols-3 gap-x-3 gap-y-5 sm:grid-cols-5 lg:grid-cols-7 xl:grid-cols-9" aria-hidden="true">
                    {Array.from({ length: 7 }, (_, index) => (
                      <div className="flex min-h-24 animate-pulse flex-col items-center gap-2.5" key={index}>
                        <span className="h-12 w-12 rounded-lg bg-white/10" />
                        <span className="h-3 w-14 rounded bg-white/10" />
                      </div>
                    ))}
                  </div>
                ) : matchingApps.length === 0 ? (
                  <div className="flex min-h-32 flex-col items-center justify-center border-y border-white/5 py-8 text-center">
                    <LayoutGrid className="mb-2 text-gray-600" size={30} />
                    <p className="text-sm text-gray-400">{t('noApps')}</p>
                    <button
                      className="mt-3 rounded-md px-3 py-2 text-sm text-indigo-400 transition-colors hover:bg-white/5 hover:text-indigo-300 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-400"
                      onClick={openShortcutDialog}
                      type="button"
                    >
                      {t('manageShortcuts')}
                    </button>
                  </div>
                ) : (
                  <div className="grid grid-cols-3 gap-x-3 gap-y-5 sm:grid-cols-5 lg:grid-cols-7 xl:grid-cols-9 2xl:grid-cols-11">
                    {matchingApps.map((app) => {
                      const AppIcon = APP_ICONS[app.iconName] ?? FileText;
                      const label = getAppLabel(app);
                      return (
                        <button
                          aria-label={t('openApp', { name: label })}
                          className="group flex min-h-24 min-w-0 flex-col items-center gap-2.5 rounded-md px-1 py-2 text-center transition-colors hover:bg-white/5 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-400"
                          key={app.id}
                          onClick={() => openApp(app.id, label)}
                          title={label}
                          type="button"
                        >
                          <span className={cn(
                            'flex h-12 w-12 shrink-0 items-center justify-center rounded-lg border border-white/5 shadow-sm transition-transform group-hover:-translate-y-0.5',
                            app.color,
                          )}>
                            <AppIcon aria-hidden="true" size={23} />
                          </span>
                          <span className="line-clamp-2 min-h-8 w-full break-words text-[13px] font-medium leading-4 text-gray-400 transition-colors group-hover:text-gray-200">
                            {label}
                          </span>
                        </button>
                      );
                    })}
                  </div>
                )}
              </section>
            ) : null}

            {(!queryActive || matchingDocs.length > 0 || loading) ? (
              <section aria-labelledby="workspace-docs-heading" className="flex min-h-0 flex-col">
                <div className="mb-4 flex min-h-9 items-center justify-between gap-3">
                  <div className="flex min-w-0 items-center gap-2">
                    <h2 className="truncate text-base font-semibold text-gray-200" id="workspace-docs-heading">
                      {queryActive ? t('matchingDocuments') : t('recentDocs')}
                    </h2>
                    {!loading ? <span className="text-xs text-gray-500">{matchingDocs.length}</span> : null}
                  </div>
                  {!queryActive ? (
                    <button
                      className="inline-flex items-center gap-1.5 rounded-md px-2.5 py-2 text-sm font-medium text-gray-400 transition-colors hover:bg-white/5 hover:text-gray-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-400"
                      onClick={() => openApp('drive', t('apps.drive'))}
                      type="button"
                    >
                      {t('viewAll')}
                      <ArrowRight aria-hidden="true" size={15} />
                    </button>
                  ) : null}
                </div>

                <div className="overflow-hidden rounded-lg border border-white/10 bg-[#242424]">
                  {loading ? (
                    <div aria-hidden="true">
                      {Array.from({ length: 3 }, (_, index) => (
                        <div className="flex h-17 animate-pulse items-center gap-3 border-b border-white/5 px-4 last:border-0" key={index}>
                          <span className="h-9 w-9 rounded-md bg-white/10" />
                          <span className="h-3 w-48 max-w-[55%] rounded bg-white/10" />
                        </div>
                      ))}
                    </div>
                  ) : matchingDocs.length === 0 ? (
                    <div className="flex min-h-44 flex-col items-center justify-center px-4 py-10 text-center">
                      <FileText className="mb-3 text-gray-600" size={34} />
                      <p className="text-sm font-medium text-gray-300">{t('noRecentDocs')}</p>
                      <p className="mt-1 text-sm text-gray-500">{t('noRecentDocsDesc')}</p>
                    </div>
                  ) : (
                    <div className="divide-y divide-white/5">
                      {matchingDocs.map((doc) => {
                        const DocumentIcon = getDocumentIcon(doc.type);
                        const label = getDocumentLabel(doc);
                        return (
                          <button
                            aria-label={t('openDriveForDocument', { name: label })}
                            className="group grid min-h-17 w-full grid-cols-[auto_minmax(0,1fr)_auto] items-center gap-3 px-4 py-3 text-left transition-colors hover:bg-white/5 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-indigo-400 sm:gap-4 sm:px-5"
                            key={doc.id}
                            onClick={() => openDocument(doc)}
                            title={t('openDriveForDocument', { name: label })}
                            type="button"
                          >
                            <span className="flex h-9 w-9 items-center justify-center rounded-md bg-cyan-500/10 text-cyan-400">
                              <DocumentIcon aria-hidden="true" size={18} />
                            </span>
                            <span className="min-w-0">
                              <span className="block truncate text-sm font-medium text-gray-200 transition-colors group-hover:text-cyan-300">{label}</span>
                              <span className="mt-0.5 block truncate text-xs text-gray-500">{t('recentDocument')}</span>
                            </span>
                            <span className="flex shrink-0 items-center gap-2 pl-2 text-xs text-gray-500">
                              <span className="hidden sm:inline">{formatDocumentTime(doc.timestamp)}</span>
                              <ArrowRight aria-hidden="true" className="text-gray-600 transition-transform group-hover:translate-x-0.5 group-hover:text-gray-300" size={16} />
                            </span>
                          </button>
                        );
                      })}
                    </div>
                  )}
                </div>
              </section>
            ) : null}
          </>
        )}
      </main>

      {showShortcutDialog ? (
        <WorkspaceShortcutDialog
          apps={apps}
          isSaving={savingShortcuts}
          onCancel={closeShortcutDialog}
          onSave={() => void savePinnedAppIds()}
          onToggle={toggleDraftShortcut}
          pinnedIds={draftPinnedIds}
        />
      ) : null}
    </div>
  );
};

export const WorkspaceView: React.FC<WorkspaceViewProps> = (props) => (
  <I18nextProvider i18n={i18n}>
    <WorkspaceViewComponent {...props} />
  </I18nextProvider>
);
