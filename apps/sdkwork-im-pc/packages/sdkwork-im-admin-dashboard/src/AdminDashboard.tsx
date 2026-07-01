import React, { useEffect, useState } from 'react';
import { Activity, Users, Network, Globe, Loader2 } from 'lucide-react';
import { AdminMetricCard } from './components/AdminMetricCard';
import { AnomalyItem } from './components/AnomalyItem';
import { adminDashboardService, type AdminDashboardData } from './services/AdminDashboardService';

const FALLBACK_THROUGHPUT = Array.from({ length: 12 }, () => ({ egress: 0, ingress: 0 }));

export const AdminDashboard: React.FC = () => {
  const [data, setData] = useState<AdminDashboardData | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let cancelled = false;

    void (async () => {
      setLoading(true);
      setError(null);
      try {
        const dashboard = await adminDashboardService.getDashboardData();
        if (!cancelled) {
          setData(dashboard);
        }
      } catch (fetchError) {
        if (!cancelled) {
          const message =
            fetchError instanceof Error ? fetchError.message : 'Failed to load admin dashboard';
          setError(message);
          setData(null);
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    })();

    return () => {
      cancelled = true;
    };
  }, []);

  const metrics = data?.metrics;
  const throughput = data?.throughput.length ? data.throughput : FALLBACK_THROUGHPUT;
  const anomalies = data?.anomalies ?? [];

  return (
    <>
      {loading && (
        <div className="mb-6 flex items-center gap-2 text-sm text-admin-text-muted" role="status">
          <Loader2 className="h-4 w-4 animate-spin" aria-hidden="true" />
          Loading platform overview…
        </div>
      )}

      {error && (
        <div
          className="mb-6 rounded-xl border border-rose-500/30 bg-rose-500/10 px-4 py-3 text-sm text-rose-200"
          role="alert"
        >
          {error}
        </div>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-6">
        <AdminMetricCard
          title="System Load"
          value={metrics?.systemLoad.value ?? '—'}
          trend={metrics?.systemLoad.trend ?? ''}
          isUp={metrics?.systemLoad.isUp ?? false}
          icon={Activity}
          color="indigo"
        />
        <AdminMetricCard
          title="Active Tenants"
          value={metrics?.activeTenants.value ?? '—'}
          trend={metrics?.activeTenants.trend ?? ''}
          isUp={metrics?.activeTenants.isUp ?? true}
          icon={Users}
          color="emerald"
        />
        <AdminMetricCard
          title="Active Connections"
          value={metrics?.activeConnections.value ?? '—'}
          trend={metrics?.activeConnections.trend ?? ''}
          isUp={metrics?.activeConnections.isUp ?? true}
          icon={Network}
          color="blue"
        />
        <AdminMetricCard
          title="Global Nodes"
          value={metrics?.globalNodes.value ?? '—'}
          trend={metrics?.globalNodes.trend ?? ''}
          isUp={metrics?.globalNodes.isUp ?? true}
          icon={Globe}
          color="amber"
        />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2 bg-admin-bg-panel border border-admin-border rounded-2xl p-6 shadow-xl flex flex-col relative overflow-hidden">
          <div className="absolute top-0 right-0 w-64 h-64 bg-indigo-500/5 blur-[80px] rounded-full pointer-events-none" />

          <div className="flex justify-between items-center mb-8 relative z-10">
            <h3 className="text-base font-semibold text-admin-text-main tracking-wide">Network Throughput</h3>
            <div className="flex gap-2">
              <span className="flex items-center gap-1.5 text-xs text-admin-text-muted">
                <div className="w-2 h-2 rounded-full bg-indigo-500" />
                Egress
              </span>
              <span className="flex items-center gap-1.5 text-xs text-admin-text-muted">
                <div className="w-2 h-2 rounded-full bg-emerald-500" />
                Ingress
              </span>
            </div>
          </div>

          <div className="flex-1 flex items-end gap-2 relative z-10 h-[200px]">
            {throughput.map((sample, index) => (
              <div key={index} className="flex-1 flex flex-col justify-end gap-1 group">
                <div
                  className="w-full bg-indigo-500/80 rounded-sm transition-all group-hover:bg-indigo-400 group-hover:shadow-[0_0_10px_rgba(99,102,241,0.5)]"
                  style={{ height: `${Math.max(4, sample.egress)}%` }}
                />
                <div
                  className="w-full bg-emerald-500/80 rounded-sm transition-all group-hover:bg-emerald-400 group-hover:shadow-[0_0_10px_rgba(16,185,129,0.5)]"
                  style={{ height: `${Math.max(4, sample.ingress)}%` }}
                />
              </div>
            ))}
          </div>
        </div>

        <div className="bg-admin-bg-panel border border-admin-border rounded-2xl p-6 shadow-xl flex flex-col relative overflow-hidden">
          <div className="absolute top-0 right-0 w-32 h-32 bg-rose-500/5 blur-[50px] rounded-full pointer-events-none" />
          <h3 className="text-base font-semibold text-admin-text-main tracking-wide mb-1 relative z-10">
            System Anomalies
          </h3>
          <p className="text-xs text-admin-text-muted mb-6 relative z-10">
            Recent audit-backed platform events
          </p>

          <div className="flex-1 flex flex-col gap-4 relative z-10">
            {anomalies.length === 0 && !loading && (
              <p className="text-sm text-admin-text-muted">No recent anomalies detected.</p>
            )}
            {anomalies.map((anomaly) => (
              <AnomalyItem
                key={anomaly.id}
                type={anomaly.type}
                tenant={anomaly.tenant}
                message={anomaly.message}
                time={anomaly.time || '—'}
              />
            ))}
          </div>
        </div>
      </div>
    </>
  );
};
