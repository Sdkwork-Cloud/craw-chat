import React, { useState, useEffect } from 'react';
import { Users, MessageSquare, Network, Database, Shield, Activity, Plus, TrendingUp, Store, Package, ShieldCheck } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { dashboardService, DashboardMetrics, ActivityTrend, SecurityAlert } from './services/DashboardService';
import { MetricCard } from './components/MetricCard';
import { AlertItem } from './components/AlertItem';

export const ConsoleDashboard: React.FC = () => {
  const [metrics, setMetrics] = useState<DashboardMetrics | null>(null);
  const [trends, setTrends] = useState<ActivityTrend[]>([]);
  const [alerts, setAlerts] = useState<SecurityAlert[]>([]);
  const [loading, setLoading] = useState(true);
  const [period, setPeriod] = useState('过去7天');

  useEffect(() => {
    const fetchData = async () => {
      setLoading(true);
      try {
        const [mRes, tRes, aRes] = await Promise.all([
          dashboardService.getMetrics(),
          dashboardService.getActivityTrends(period),
          dashboardService.getSecurityAlerts()
        ]);
        setMetrics(mRes);
        setTrends(tRes);
        setAlerts(aRes);
      } finally {
        setLoading(false);
      }
    };
    fetchData();
  }, [period]);

  if (loading && !metrics) {
    return <div className="p-8 text-center text-console-text-muted flex justify-center items-center h-[50vh]">
      <div className="flex flex-col items-center">
        <Activity className="animate-spin text-blue-500 mb-4" size={32} />
        <p className="text-sm tracking-widest uppercase">Initializing Workspace...</p>
      </div>
    </div>;
  }

  return (
    <div className="flex flex-col h-full gap-6 pb-6">
      {/* Welcome Header */}
      <div className="flex flex-col md:flex-row md:items-end justify-between gap-4 shrink-0 bg-console-bg-panel p-6 rounded-2xl border border-console-border shadow-sm relative overflow-hidden">
        <div className="absolute right-0 top-0 w-64 h-64 bg-blue-500/10 blur-[80px] rounded-full pointer-events-none" />
        <div className="absolute left-1/4 bottom-0 w-48 h-48 bg-purple-500/10 blur-[80px] rounded-full pointer-events-none" />
        <div className="relative z-10">
          <h1 className="text-2xl font-bold text-console-text-main tracking-tight flex items-center gap-2">
            上午好，管理员 <span className="text-blue-500">👋</span>
          </h1>
          <p className="text-sm text-console-text-muted mt-2 max-w-xl leading-relaxed">
            欢迎回到 Sdkwork IM 企业管理控制台。今日共有 <strong className="text-console-text-main">{metrics?.dailyMessages.value || 0}</strong> 条新消息产生，您的整体租户系统运行健康，没有阻断级别的告警。
          </p>
        </div>
        <div className="flex items-center gap-3 relative z-10">
          <button className="bg-console-bg-root hover:bg-console-active-bg hover:text-console-active-text border border-console-border text-console-text-main px-4 py-2 rounded-lg text-sm font-medium transition-all shadow-sm">
            生成运行报告
          </button>
          <button className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg text-sm transition-colors flex items-center gap-2 font-medium shadow-md shadow-blue-600/20">
            <Plus size={16} />
            <span>快捷邀请</span>
          </button>
        </div>
      </div>

      {/* Key Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 shrink-0">
        {metrics && (
          <>
            <MetricCard title="总活跃用户 (30天)" value={metrics.totalUsers.value} trend={metrics.totalUsers.trend} isUp={metrics.totalUsers.isUp} icon={Users} color="blue" />
            <MetricCard title="今日消息量" value={metrics.dailyMessages.value} trend={metrics.dailyMessages.trend} isUp={metrics.dailyMessages.isUp} icon={MessageSquare} color="indigo" />
            <MetricCard title="活跃群组" value={metrics.activeGroups.value} trend={metrics.activeGroups.trend} isUp={metrics.activeGroups.isUp} icon={Network} color="emerald" />
            <MetricCard title="存储容量使用" value={metrics.storageUsage.value} trend={metrics.storageUsage.trend} isUp={metrics.storageUsage.isUp} icon={Database} color="amber" />
          </>
        )}
      </div>

      {/* Main Charts & Activity */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 flex-1 min-h-0">
        {/* Left Column: Charts */}
        <div className="lg:col-span-2 flex flex-col gap-6 h-full">
          {/* Main Chart */}
          <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm p-6 flex flex-col flex-1 relative overflow-hidden group">
            {/* Grid Pattern Background */}
            <div className="absolute inset-0 bg-[linear-gradient(rgba(128,128,128,0.05)_1px,transparent_1px),linear-gradient(90deg,rgba(128,128,128,0.05)_1px,transparent_1px)] bg-[size:24px_24px] pointer-events-none opacity-50" />
            
            <div className="flex justify-between items-center mb-8 relative z-10">
              <div>
                <h3 className="text-base font-bold text-console-text-main flex items-center gap-2">
                  <TrendingUp size={18} className="text-blue-500" />
                  应用通信活跃度
                </h3>
                <p className="text-xs text-console-text-muted mt-1">展示企业全体员工消息收发频次汇总趋势</p>
              </div>
              <div className="flex items-center gap-2 bg-console-bg-root p-1 rounded-lg border border-console-border">
                {['过去7天', '过去30天', '今年'].map((p) => (
                  <button 
                    key={p}
                    onClick={() => setPeriod(p)}
                    className={cn(
                      "px-3 py-1.5 text-[12px] font-medium rounded-md transition-all duration-200", 
                      period === p 
                        ? "bg-blue-500/10 text-blue-600 shadow-sm" 
                        : "text-console-text-muted hover:text-console-text-main hover:bg-console-bg-hover"
                    )}
                  >
                    {p}
                  </button>
                ))}
              </div>
            </div>
            
            <div className="flex-1 flex items-end justify-between gap-3 pb-2 relative z-10 mt-4">
              {trends.map((item, i) => (
                <div key={i} className="flex-1 flex flex-col items-center gap-3 relative group/bar">
                  <div className="w-full bg-blue-500/10 dark:bg-blue-500/5 rounded-t-md relative flex items-end overflow-hidden h-full min-h-[160px] border border-blue-500/10 transition-colors group-hover/bar:border-blue-500/30">
                    <div 
                      className="w-full bg-gradient-to-t from-blue-600 to-blue-400 rounded-t-md transition-all duration-500 ease-out shadow-[0_0_15px_rgba(59,130,246,0)] group-hover/bar:shadow-[0_0_15px_rgba(59,130,246,0.3)] relative" 
                      style={{ height: `${item.value}%` }}
                    >
                      <div className="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.2)_1px,transparent_1px)] bg-[size:100%_4px] opacity-20 pointer-events-none" />
                      <div className="absolute top-0 left-0 w-full h-[1px] bg-white/40" />
                    </div>
                  </div>
                  <div className="flex flex-col items-center">
                    <span className="text-[12px] font-medium text-console-text-muted group-hover/bar:text-console-text-main transition-colors">周{item.day}</span>
                  </div>
                  {/* Tooltip */}
                  <div className="absolute -top-10 left-1/2 -translate-x-1/2 bg-console-bg-root border border-console-border px-3 py-1.5 rounded-lg shadow-lg opacity-0 group-hover/bar:opacity-100 transition-all duration-200 scale-95 group-hover/bar:scale-100 pointer-events-none whitespace-nowrap z-20">
                    <div className="text-[11px] text-console-text-muted mb-0.5">活跃度</div>
                    <div className="text-sm font-bold text-console-text-main">{item.value}%</div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* Right Column: Security & Actions */}
        <div className="flex flex-col gap-6 h-full">
          {/* Action Row */}
          <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm p-4 grid grid-cols-2 gap-3 shrink-0">
            <button className="flex flex-col items-start p-4 hover:bg-emerald-500/5 border border-transparent hover:border-emerald-500/20 rounded-xl transition-all group/btn text-left">
              <div className="p-2 bg-emerald-500/10 text-emerald-600 rounded-lg mb-3">
                <Store size={18} className="group-hover/btn:scale-110 transition-transform" />
              </div>
              <div className="font-semibold text-console-text-main text-sm">部署门店</div>
              <div className="text-xs text-console-text-muted mt-1 leading-tight line-clamp-2">上线新的数字微店</div>
            </button>
            <button className="flex flex-col items-start p-4 hover:bg-purple-500/5 border border-transparent hover:border-purple-500/20 rounded-xl transition-all group/btn text-left">
              <div className="p-2 bg-purple-500/10 text-purple-600 rounded-lg mb-3">
                <Package size={18} className="group-hover/btn:scale-110 transition-transform" />
              </div>
              <div className="font-semibold text-console-text-main text-sm">商品发布</div>
              <div className="text-xs text-console-text-muted mt-1 leading-tight line-clamp-2">上新/管理SKU</div>
            </button>
          </div>

          <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm p-6 flex flex-col flex-1 min-h-0">
            <div className="flex justify-between items-center mb-6">
              <h3 className="text-base font-bold text-console-text-main flex items-center gap-2">
                <ShieldCheck size={18} className="text-emerald-500" />
                安全与基线监控
              </h3>
              <span className="flex h-2 w-2">
                <span className="animate-ping absolute inline-flex h-2 w-2 rounded-full bg-emerald-400 opacity-75"></span>
                <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
              </span>
            </div>
            
            <div className="flex-1 flex flex-col gap-4 overflow-y-auto custom-scrollbar pr-2">
              {alerts.length > 0 ? alerts.map(alert => (
                <AlertItem key={alert.id} type={alert.type} message={alert.message} time={alert.time} />
              )) : (
                <div className="flex-1 flex flex-col items-center justify-center text-center p-4">
                  <Shield size={32} className="text-emerald-500/50 mb-3" />
                  <p className="text-sm font-medium text-console-text-main">系统安全健康状态良好</p>
                  <p className="text-xs text-console-text-muted mt-1">过去 24 小时未发现高危入侵告警</p>
                </div>
              )}
            </div>
            
            {alerts.length > 0 && (
              <button className="mt-4 w-full py-2.5 bg-console-bg-root hover:bg-console-active-bg hover:text-console-active-text text-sm text-console-text-muted font-medium rounded-xl transition-colors border border-console-border shadow-sm">
                查看监控大盘
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
