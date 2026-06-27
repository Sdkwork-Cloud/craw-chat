import React from 'react';
import { Network, ServerIcon, Database, ShieldCheck, Activity, MemoryStick, Cpu, HardDrive } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';

const nodes = [
  { region: 'us-east-1', status: 'healthy', cpu: 42, mem: 60, connections: '124K' },
  { region: 'us-west-2', status: 'healthy', cpu: 38, mem: 55, connections: '98K' },
  { region: 'eu-central-1', status: 'warning', cpu: 85, mem: 92, connections: '180K' },
  { region: 'ap-southeast-1', status: 'healthy', cpu: 20, mem: 40, connections: '45K' },
];

export const InfrastructureStatus = () => {
  return (
    <div className="space-y-6">
      {/* Overview Head */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-xl font-bold text-admin-text-main tracking-wide">Infrastructure & Nodes</h2>
          <p className="text-sm text-admin-text-muted mt-1">Global edge network and real-time database clusters</p>
        </div>
        <div className="flex gap-3">
          <button className="bg-admin-bg-hover border border-admin-border hover:bg-admin-border-subtle text-admin-text-main px-4 py-2 rounded-lg text-sm font-medium transition-colors">
            Topology Map
          </button>
          <button className="bg-indigo-600 hover:bg-indigo-500 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors shadow-[0_0_15px_rgba(79,70,229,0.3)]">
            Provision Node
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <ServerMetric title="Global Connection Pool" value="842.5K" usage={65} icon={Network} color="indigo" />
        <ServerMetric title="Database IOPS (Avg)" value="42,050" usage={45} icon={Database} color="emerald" />
        <ServerMetric title="Redis Cache Hit Rate" value="98.2%" usage={98} icon={MemoryStick} color="amber" reverse />
      </div>

      <div className="bg-admin-bg-panel border border-admin-border rounded-2xl p-6 shadow-xl relative overflow-hidden">
        <div className="absolute top-0 right-0 w-64 h-64 bg-indigo-500/5 blur-[80px] rounded-full pointer-events-none" />
        <h3 className="text-base font-semibold text-admin-text-main tracking-wide mb-6 relative z-10 flex items-center gap-2">
          <ServerIcon size={18} className="text-indigo-400" /> Active Edge Nodes
        </h3>

        <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-4 gap-4 relative z-10">
          {nodes.map((node) => (
            <div key={node.region} className="bg-admin-bg-root border border-admin-border rounded-xl p-5 hover:bg-admin-bg-hover transition-colors group cursor-crosshair">
              <div className="flex justify-between items-start mb-4">
                <div>
                  <div className="text-sm font-bold text-admin-text-main font-mono tracking-wider">{node.region}</div>
                  <div className="text-[10px] text-gray-500 uppercase tracking-widest mt-1">WebSocket Gateway</div>
                </div>
                <div className={cn("w-2 h-2 rounded-full shadow-[0_0_8px_currentColor]", node.status === 'healthy' ? "bg-emerald-500 text-emerald-500" : "bg-amber-500 text-amber-500 animate-pulse")} />
              </div>
              
              <div className="space-y-4">
                <div>
                  <div className="flex justify-between text-[11px] mb-1">
                    <span className="text-gray-400 font-medium flex items-center gap-1"><Cpu size={10} /> CPU Load</span>
                    <span className={cn("font-mono", node.cpu > 80 ? "text-rose-400" : "text-gray-300")}>{node.cpu}%</span>
                  </div>
                  <div className="w-full bg-black/40 h-1.5 rounded-full overflow-hidden">
                    <div 
                      className={cn("h-full rounded-full transition-all duration-1000", node.cpu > 80 ? "bg-rose-500" : "bg-indigo-500")}
                      style={{ width: `${node.cpu}%` }}
                    />
                  </div>
                </div>

                <div>
                  <div className="flex justify-between text-[11px] mb-1">
                    <span className="text-gray-400 font-medium flex items-center gap-1"><MemoryStick size={10} /> Memory</span>
                    <span className={cn("font-mono", node.mem > 80 ? "text-rose-400" : "text-gray-300")}>{node.mem}%</span>
                  </div>
                  <div className="w-full bg-black/40 h-1.5 rounded-full overflow-hidden">
                    <div 
                      className={cn("h-full rounded-full transition-all duration-1000", node.mem > 80 ? "bg-amber-500" : "bg-emerald-500")}
                      style={{ width: `${node.mem}%` }}
                    />
                  </div>
                </div>

                <div className="pt-2 mt-2 border-t border-admin-border flex justify-between items-center text-xs">
                  <span className="text-admin-text-muted">Active Conns</span>
                  <span className="text-admin-text-main font-mono">{node.connections}</span>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

const ServerMetric = ({ title, value, usage, icon: Icon, color, reverse = false }: any) => {
  const colorMap: Record<string, string> = {
    indigo: "text-indigo-400 bg-indigo-400/10",
    emerald: "text-emerald-400 bg-emerald-400/10",
    amber: "text-amber-400 bg-amber-400/10",
  };
  const barColor = reverse ? (usage > 80 ? 'bg-emerald-500' : 'bg-amber-500') : (usage > 80 ? 'bg-rose-500' : 'bg-indigo-500');

  return (
    <div className="bg-admin-bg-panel border border-admin-border rounded-2xl p-5 relative overflow-hidden group">
      <div className="flex items-center gap-4 relative z-10">
        <div className={cn("p-3 rounded-xl border border-admin-border", colorMap[color])}>
          <Icon size={20} />
        </div>
        <div className="flex-1 min-w-0">
          <div className="text-xs text-admin-text-muted font-medium tracking-wide truncate">{title}</div>
          <div className="text-2xl font-bold text-admin-text-main font-mono tracking-tight">{value}</div>
        </div>
      </div>
      <div className="mt-5 relative z-10">
        <div className="flex justify-between text-[10px] text-gray-500 font-mono mb-1.5">
          <span>Usage</span>
          <span>{usage}%</span>
        </div>
        <div className="w-full bg-white/5 h-1 rounded-full overflow-hidden">
          <div className={cn("h-full rounded-full transition-all duration-1000", barColor)} style={{ width: `${usage}%` }} />
        </div>
      </div>
    </div>
  );
}
