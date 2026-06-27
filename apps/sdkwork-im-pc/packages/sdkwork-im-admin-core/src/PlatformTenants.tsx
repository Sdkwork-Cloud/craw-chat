import React from 'react';
import { Building2, Search, Filter, MoreVertical, CreditCard, Activity, PackageCheck, Zap } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';

const tenants = [
  { id: 'T-8491', name: 'Acme Corporation', plan: 'Enterprise', users: '12.4K', status: 'active', revenue: '$4,200', region: 'US-East' },
  { id: 'T-8492', name: 'Global Tech Inc.', plan: 'Business', users: '2.8K', status: 'active', revenue: '$850', region: 'EU-West' },
  { id: 'T-8493', name: 'Startup Hub', plan: 'Pro', users: '84', status: 'active', revenue: '$99', region: 'AP-South' },
  { id: 'T-8494', name: 'Design Co.', plan: 'Business', users: '420', status: 'warning', revenue: '$250', region: 'US-West' },
  { id: 'T-8495', name: 'Stark Industries', plan: 'Enterprise', users: '45.2K', status: 'active', revenue: '$18,500', region: 'Global' },
];

export const PlatformTenants = () => {
  return (
    <div className="bg-admin-bg-panel border border-admin-border rounded-2xl shadow-xl flex flex-col min-h-[600px] relative overflow-hidden">
      <div className="absolute top-0 right-1/4 w-96 h-96 bg-indigo-500/5 blur-[100px] rounded-full pointer-events-none" />
      
      {/* Header */}
      <div className="relative z-10 flex items-center justify-between p-6 border-b border-admin-border">
        <div>
          <h2 className="text-xl font-bold text-admin-text-main tracking-wide">Tenant Management</h2>
          <p className="text-sm text-admin-text-muted mt-1">Manage platform tenants, subscription plans, and resource limits</p>
        </div>
        <button className="bg-indigo-600 hover:bg-indigo-500 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2 shadow-[0_0_15px_rgba(79,70,229,0.3)]">
          <Building2 size={16} />
          Provision Tenant
        </button>
      </div>

      {/* Toolbar */}
      <div className="relative z-10 p-5 flex items-center justify-between bg-admin-bg-root/50 border-b border-admin-border">
        <div className="flex items-center gap-4">
          <div className="relative group">
            <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500 group-focus-within:text-indigo-400 transition-colors" />
            <input 
              type="text" 
              placeholder="Search tenant ID, name, region..." 
              className="w-80 bg-admin-input-bg border border-admin-border rounded-lg py-2 pl-9 pr-4 text-sm text-admin-text-main focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-all placeholder:text-admin-text-muted"
            />
          </div>
          <button className="bg-admin-bg-root border border-admin-border text-admin-text-muted px-4 py-2 rounded-lg text-sm flex items-center gap-2 hover:bg-admin-bg-hover hover:text-admin-text-main transition-colors">
            <Filter size={14} />
            Filters
          </button>
        </div>
        
        <div className="flex gap-3">
          <span className="flex items-center gap-2 text-sm text-gray-400 px-3 py-1 bg-emerald-500/10 text-emerald-400 rounded-md border border-emerald-500/20 font-medium">
            <Activity size={14} /> System Healthy
          </span>
        </div>
      </div>

      {/* Table */}
      <div className="relative z-10 flex-1 overflow-auto custom-scrollbar">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="border-b border-admin-border text-[11px] uppercase tracking-widest text-admin-text-muted bg-admin-bg-root/80 backdrop-blur-sm sticky top-0 z-20">
              <th className="px-6 py-4 font-semibold w-16 text-center">
                <input type="checkbox" className="rounded border-admin-border bg-transparent text-indigo-500 focus:ring-indigo-500/50" />
              </th>
              <th className="px-6 py-4 font-semibold">Tenant Info</th>
              <th className="px-6 py-4 font-semibold">Plan Level</th>
              <th className="px-6 py-4 font-semibold">Active Users</th>
              <th className="px-6 py-4 font-semibold">Region</th>
              <th className="px-6 py-4 font-semibold">MRR</th>
              <th className="px-6 py-4 font-semibold text-right">Actions</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-white/5 text-sm">
            {tenants.map((tenant) => (
              <tr key={tenant.id} className="hover:bg-white/[0.02] transition-colors group">
                <td className="px-6 py-4 text-center">
                  <input type="checkbox" className="rounded border-white/20 bg-transparent text-indigo-500 focus:ring-indigo-500/50" />
                </td>
                <td className="px-6 py-4">
                  <div className="flex items-center gap-3">
                    <div className="w-10 h-10 rounded-xl bg-gradient-to-tr from-gray-800 to-gray-700 border border-admin-border flex items-center justify-center font-bold text-gray-300 shadow-inner">
                      {tenant.name.charAt(0)}
                    </div>
                    <div>
                      <div className="font-semibold text-admin-text-main group-hover:text-indigo-400 transition-colors cursor-pointer">{tenant.name}</div>
                      <div className="text-xs font-mono text-admin-text-muted mt-0.5">{tenant.id}</div>
                    </div>
                  </div>
                </td>
                <td className="px-6 py-4">
                  {tenant.plan === 'Enterprise' && <span className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md text-[11px] font-bold bg-indigo-500/10 text-indigo-400 border border-indigo-500/20 tracking-wide"><Zap size={12}/> {tenant.plan}</span>}
                  {tenant.plan === 'Business' && <span className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md text-[11px] font-bold bg-blue-500/10 text-blue-400 border border-blue-500/20 tracking-wide"><PackageCheck size={12}/> {tenant.plan}</span>}
                  {tenant.plan === 'Pro' && <span className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md text-[11px] font-bold bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 tracking-wide">{tenant.plan}</span>}
                </td>
                <td className="px-6 py-4">
                  <span className="font-mono text-gray-300">{tenant.users}</span>
                </td>
                <td className="px-6 py-4">
                  <span className="text-gray-400 text-xs">{tenant.region}</span>
                </td>
                <td className="px-6 py-4">
                  <span className="font-mono text-gray-300 flex items-center gap-1.5">
                    <CreditCard size={14} className="text-gray-500" />
                    {tenant.revenue}
                  </span>
                </td>
                <td className="px-6 py-4 text-right">
                  <button className="p-2 text-gray-500 hover:text-white hover:bg-white/10 rounded-lg transition-colors border border-transparent hover:border-white/10">
                    <MoreVertical size={16} />
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Pagination */}
      <div className="relative z-10 p-4 border-t border-admin-border flex items-center justify-between text-xs text-admin-text-muted bg-admin-bg-root/50">
        <div>Showing 1 to 5 of 8,240 tenants</div>
        <div className="flex gap-1">
          <button className="px-3 py-1.5 border border-admin-border rounded-md text-admin-text-muted cursor-not-allowed bg-admin-bg-root">Previous</button>
          <button className="px-3 py-1.5 border border-indigo-500 rounded-md bg-indigo-500/20 text-indigo-400 font-medium">1</button>
          <button className="px-3 py-1.5 border border-admin-border rounded-md text-admin-text-main hover:bg-admin-bg-hover">2</button>
          <button className="px-3 py-1.5 border border-admin-border rounded-md text-admin-text-main hover:bg-admin-bg-hover">3</button>
          <span className="px-2 py-1.5">...</span>
          <button className="px-3 py-1.5 border border-admin-border rounded-md text-admin-text-main hover:bg-admin-bg-hover">Next</button>
        </div>
      </div>
    </div>
  );
};
