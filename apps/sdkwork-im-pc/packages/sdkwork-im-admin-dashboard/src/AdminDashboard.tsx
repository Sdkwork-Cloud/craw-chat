import React from 'react';
import { 
  Activity, Users, Server, AlertTriangle, 
  Database, Network, Globe, Lock 
} from 'lucide-react';
import { AdminMetricCard } from './components/AdminMetricCard';
import { AnomalyItem } from './components/AnomalyItem';

export const AdminDashboard: React.FC = () => {
  return (
    <>
      {/* Metrics Row */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-6">
        <AdminMetricCard title="System Load" value="28%" trend="-2%" isUp={false} icon={Activity} color="indigo" />
        <AdminMetricCard title="Active Tenants" value="8,240" trend="+12" isUp={true} icon={Users} color="emerald" />
        <AdminMetricCard title="Active Connections" value="1.2M" trend="+45k" isUp={true} icon={Network} color="blue" />
        <AdminMetricCard title="Global Nodes" value="12" trend="0" isUp={true} icon={Globe} color="amber" />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Node Status Graph */}
        <div className="lg:col-span-2 bg-admin-bg-panel border border-admin-border rounded-2xl p-6 shadow-xl flex flex-col relative overflow-hidden">
          <div className="absolute top-0 right-0 w-64 h-64 bg-indigo-500/5 blur-[80px] rounded-full pointer-events-none" />
          
          <div className="flex justify-between items-center mb-8 relative z-10">
            <h3 className="text-base font-semibold text-admin-text-main tracking-wide">Network Throughput</h3>
            <div className="flex gap-2">
              <span className="flex items-center gap-1.5 text-xs text-admin-text-muted"><div className="w-2 h-2 rounded-full bg-indigo-500"></div> Egress</span>
              <span className="flex items-center gap-1.5 text-xs text-admin-text-muted"><div className="w-2 h-2 rounded-full bg-emerald-500"></div> Ingress</span>
            </div>
          </div>
          
          <div className="flex-1 flex items-end gap-2 relative z-10 h-[200px]">
            {[30, 45, 25, 60, 85, 40, 70, 90, 50, 65, 35, 80].map((h, i) => (
              <div key={i} className="flex-1 flex flex-col justify-end gap-1 group">
                <div 
                  className="w-full bg-indigo-500/80 rounded-sm transition-all group-hover:bg-indigo-400 group-hover:shadow-[0_0_10px_rgba(99,102,241,0.5)]" 
                  style={{ height: `${h}%` }}
                ></div>
                <div 
                  className="w-full bg-emerald-500/80 rounded-sm transition-all group-hover:bg-emerald-400 group-hover:shadow-[0_0_10px_rgba(16,185,129,0.5)]" 
                  style={{ height: `${h * 0.6}%` }}
                ></div>
              </div>
            ))}
          </div>
        </div>

        {/* Live System Logs / Anomalies */}
        <div className="bg-admin-bg-panel border border-admin-border rounded-2xl p-6 shadow-xl flex flex-col relative overflow-hidden">
          <div className="absolute top-0 right-0 w-32 h-32 bg-rose-500/5 blur-[50px] rounded-full pointer-events-none" />
          <h3 className="text-base font-semibold text-admin-text-main tracking-wide mb-1 relative z-10">System Anomalies</h3>
          <p className="text-xs text-admin-text-muted mb-6 relative z-10">Platform-wide auto-detected issues</p>
          
          <div className="flex-1 flex flex-col gap-4 relative z-10">
            <AnomalyItem type="critical" tenant="T-4829" message="Database connection pool exhausted" time="2m ago" />
            <AnomalyItem type="warning" tenant="T-9921" message="Spike in auth failures (120 req/s)" time="15m ago" />
            <AnomalyItem type="info" tenant="System" message="Routine backup completed successfully" time="1h ago" />
            <AnomalyItem type="warning" tenant="T-1021" message="Payment gateway latency > 2s" time="2.5h ago" />
          </div>

          <button className="mt-6 w-full py-2.5 bg-admin-bg-hover hover:bg-admin-border-subtle border border-admin-border hover:border-admin-border text-xs text-admin-text-main font-medium rounded-lg transition-all tracking-wide">
            View Audit Logs
          </button>
        </div>
      </div>
    </>
  );
};
