import React, { useCallback, useEffect, useState } from 'react';
import {
  Activity,
  AlertTriangle,
  CheckCircle2,
  Clock3,
  Gauge,
  Radio,
  RefreshCw,
  Send,
  XCircle,
} from 'lucide-react';
import { dashboardService, DashboardMetricKey, DashboardViewModel } from './services/DashboardService';

const metricIcons: Record<DashboardMetricKey, React.ComponentType<{ size?: number; className?: string }>> = {
  clientRouteWindows: Radio,
  pendingRealtimeEvents: Activity,
  laggingConversationScopes: Clock3,
  maxConversationLag: Gauge,
  pendingOutboxEvents: Send,
  failedOutboxAttempts: XCircle,
};

function errorMessage(error: unknown): string {
  return error instanceof Error && error.message.trim()
    ? error.message
    : '运营数据暂时不可用。';
}

function formatGeneratedAt(value: string): string {
  const date = new Date(value);
  return Number.isNaN(date.getTime()) ? value : date.toLocaleString();
}

export const ConsoleDashboard: React.FC = () => {
  const [view, setView] = useState<DashboardViewModel | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      setView(await dashboardService.retrieve());
    } catch (cause) {
      setError(errorMessage(cause));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void load();
  }, [load]);

  if (loading && !view) {
    return (
      <div className="flex h-[50vh] items-center justify-center text-console-text-muted">
        <RefreshCw aria-hidden="true" className="animate-spin" size={24} />
      </div>
    );
  }

  if (!view) {
    return <DashboardFailure message={error ?? '运营数据暂时不可用。'} onRetry={load} />;
  }

  const unavailable = view.state === 'unavailable';
  const partial = view.state === 'partial' || !view.complete;

  return (
    <div className="flex h-full flex-col gap-5 pb-6">
      <header className="flex flex-col gap-3 border-b border-console-border pb-4 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-xl font-semibold text-console-text-main">运营概览</h1>
          <p className="mt-1 text-sm text-console-text-muted">
            {formatGeneratedAt(view.generatedAt)}
          </p>
        </div>
        <button
          aria-label="刷新运营数据"
          className="inline-flex h-9 w-9 items-center justify-center rounded-lg border border-console-border bg-console-bg-panel text-console-text-main transition-colors hover:bg-console-bg-hover disabled:cursor-not-allowed disabled:opacity-50"
          disabled={loading}
          onClick={() => void load()}
          title="刷新运营数据"
          type="button"
        >
          <RefreshCw aria-hidden="true" className={loading ? 'animate-spin' : undefined} size={17} />
        </button>
      </header>

      {error && <StatusBand icon={AlertTriangle} tone="warning" title="刷新失败" detail={error} />}
      {unavailable && (
        <StatusBand
          icon={XCircle}
          tone="error"
          title="数据不可用"
          detail={view.reason ?? '服务端尚未上报可验证的运行指标。'}
        />
      )}
      {!unavailable && partial && (
        <StatusBand
          icon={AlertTriangle}
          tone="warning"
          title="数据不完整"
          detail={view.reason ?? '当前结果只包含部分运行数据。'}
        />
      )}
      {!unavailable && !partial && (
        <StatusBand
          icon={CheckCircle2}
          tone="success"
          title="数据已同步"
          detail={`来源：${view.source}，运行状态：${view.opsStatus}`}
        />
      )}

      {view.metrics.length > 0 ? (
        <section className="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-3" aria-label="运行指标">
          {view.metrics.map((metric) => {
            const Icon = metricIcons[metric.key];
            const isFailure = metric.key === 'failedOutboxAttempts';
            return (
              <article className="rounded-lg border border-console-border bg-console-bg-panel p-5" key={metric.key}>
                <div className="flex items-center justify-between gap-3">
                  <span className="text-sm text-console-text-muted">{metric.label}</span>
                  <Icon aria-hidden="true" className={isFailure ? 'text-red-500' : 'text-blue-500'} size={18} />
                </div>
                <div className="mt-4 break-all text-2xl font-semibold text-console-text-main">
                  {metric.value}
                </div>
              </article>
            );
          })}
        </section>
      ) : (
        <section className="flex min-h-[220px] flex-col items-center justify-center border-y border-console-border text-center">
          <Activity aria-hidden="true" className="mb-3 text-console-text-muted" size={28} />
          <p className="text-sm font-medium text-console-text-main">暂无可验证运行指标</p>
        </section>
      )}
    </div>
  );
};

function StatusBand({
  icon: Icon,
  tone,
  title,
  detail,
}: {
  icon: React.ComponentType<{ size?: number; className?: string }>;
  tone: 'success' | 'warning' | 'error';
  title: string;
  detail: string;
}) {
  const toneClass = {
    success: 'border-emerald-500/30 bg-emerald-500/5 text-emerald-700 dark:text-emerald-300',
    warning: 'border-amber-500/30 bg-amber-500/5 text-amber-700 dark:text-amber-300',
    error: 'border-red-500/30 bg-red-500/5 text-red-700 dark:text-red-300',
  }[tone];

  return (
    <section className={`flex items-start gap-3 border p-4 ${toneClass}`} role={tone === 'error' ? 'alert' : 'status'}>
      <Icon aria-hidden="true" className="mt-0.5 shrink-0" size={18} />
      <div className="min-w-0">
        <h2 className="text-sm font-semibold">{title}</h2>
        <p className="mt-1 break-words text-sm opacity-90">{detail}</p>
      </div>
    </section>
  );
}

function DashboardFailure({ message, onRetry }: { message: string; onRetry: () => Promise<void> }) {
  return (
    <section className="flex min-h-[320px] flex-col items-center justify-center gap-4 border-y border-console-border p-8 text-center" role="alert">
      <XCircle aria-hidden="true" className="text-red-500" size={30} />
      <h1 className="text-lg font-semibold text-console-text-main">运营数据不可用</h1>
      <p className="max-w-xl break-words text-sm text-console-text-muted">{message}</p>
      <button
        className="inline-flex h-9 items-center gap-2 rounded-lg border border-console-border bg-console-bg-panel px-3 text-sm font-medium text-console-text-main transition-colors hover:bg-console-bg-hover"
        onClick={() => void onRetry()}
        type="button"
      >
        <RefreshCw aria-hidden="true" size={16} />
        重试
      </button>
    </section>
  );
}
