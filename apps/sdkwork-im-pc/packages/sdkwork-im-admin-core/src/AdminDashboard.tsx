import React from 'react';
import { 
  Activity, Users, Server, AlertTriangle, 
  Database, Network, Globe, Lock 
} from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';

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

const AdminMetricCard = ({ title, value, trend, isUp, icon: Icon, color }: any) => {
  const colorMap: Record<string, string> = {
    indigo: "text-indigo-400 bg-indigo-500/10",
    emerald: "text-emerald-400 bg-emerald-500/10",
    blue: "text-blue-400 bg-blue-500/10",
    amber: "text-amber-400 bg-amber-500/10"
  };

  return (
    <div className="bg-admin-bg-panel p-5 rounded-2xl border border-admin-border shadow-lg flex flex-col relative overflow-hidden group">
      <div className={cn("absolute -right-6 -top-6 w-24 h-24 rounded-full blur-[40px] opacity-20 group-hover:opacity-40 transition-opacity", colorMap[color].split(' ')[1])} />
      
      <div className="flex justify-between items-start mb-4 relative z-10">
        <div className={cn("p-2.5 rounded-xl border border-admin-border", colorMap[color])}>
          <Icon size={20} />
        </div>
        {trend !== "0" && (
          <div className={cn(
            "px-2 py-1 rounded-md text-[10px] font-mono tracking-wider",
            isUp ? "bg-emerald-500/10 text-emerald-400 border border-emerald-500/20" : "bg-rose-500/10 text-rose-400 border border-rose-500/20"
          )}>
            {trend}
          </div>
        )}
      </div>
      <div className="flex flex-col relative z-10">
        <span className="text-[28px] font-bold text-admin-text-main leading-none mb-1 tracking-tight">{value}</span>
        <div className="flex justify-between items-baseline mt-1">
          <span className="text-xs text-admin-text-muted font-medium tracking-wide">{title}</span>
          <span className="text-[10px] text-admin-text-muted font-mono">{trend}</span>
        </div>
      </div>
    </div>
  );
};

const AnomalyItem = ({ type, tenant, message, time }: any) => {
  const typeStyles = {
    critical: "text-rose-400 bg-rose-500/10 border-rose-500/20",
    warning: "text-amber-400 bg-amber-500/10 border-amber-500/20",
    info: "text-blue-400 bg-blue-500/10 border-blue-500/20"
  };
  
  return (
    <div className="flex items-center gap-3 p-3 rounded-xl bg-admin-bg-root border border-admin-border hover:bg-admin-bg-hover transition-colors cursor-pointer">
      <div className={cn("px-1.5 py-0.5 rounded font-mono text-[9px] border break-keep shrink-0", typeStyles[type as keyof typeof typeStyles])}>
        {tenant}
      </div>
      <div className="flex-1 min-w-0">
        <div className="text-xs text-admin-text-main leading-tight truncate font-medium">{message}</div>
        <div className="text-[10px] text-admin-text-muted mt-1">{time}</div>
      </div>
    </div>
  );
};
