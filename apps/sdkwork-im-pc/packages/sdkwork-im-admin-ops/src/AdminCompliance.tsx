import React, { useState } from 'react';
import { ShieldAlert, FileText, Search, Filter, AlertTriangle, Monitor, FileSearch } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';

export const AdminCompliance = () => {
  const [searchTerm, setSearchTerm] = useState('');

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h2 className="text-xl font-bold text-admin-text-main tracking-wide">Platform Compliance & Audit</h2>
          <p className="text-sm text-admin-text-muted mt-1">Global security events, legal holds, and data residency overview</p>
        </div>
        <div className="flex gap-2">
          <button className="bg-admin-bg-panel border border-admin-border hover:bg-admin-bg-hover text-admin-text-main px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2">
            <FileText size={16} /> Generate SOC2 Report
          </button>
        </div>
      </div>

      {/* Overview Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-admin-bg-panel border border-admin-border rounded-2xl p-6 shadow-xl flex items-center gap-5">
          <div className="p-4 bg-emerald-500/10 border border-emerald-500/20 rounded-xl text-emerald-400">
            <ShieldAlert size={28} />
          </div>
          <div>
            <h3 className="font-semibold text-admin-text-main text-lg">System Secure</h3>
            <p className="text-xs text-admin-text-muted mt-1">No critical vulnerabilities detected</p>
          </div>
        </div>
        <div className="bg-admin-bg-panel border border-admin-border rounded-2xl p-6 shadow-xl flex items-center gap-5">
          <div className="p-4 bg-amber-500/10 border border-amber-500/20 rounded-xl text-amber-400">
            <AlertTriangle size={28} />
          </div>
          <div>
            <h3 className="font-semibold text-admin-text-main text-lg">14 Active Legal Holds</h3>
            <p className="text-xs text-admin-text-muted mt-1">Across 8 different enterprise tenants</p>
          </div>
        </div>
        <div className="bg-admin-bg-panel border border-admin-border rounded-2xl p-6 shadow-xl flex items-center gap-5">
          <div className="p-4 bg-blue-500/10 border border-blue-500/20 rounded-xl text-blue-400">
            <Monitor size={28} />
          </div>
          <div>
            <h3 className="font-semibold text-admin-text-main text-lg">99.998% Uptime</h3>
            <p className="text-xs text-admin-text-muted mt-1">Exceeding SLA requirements</p>
          </div>
        </div>
      </div>

      {/* Detailed Content */}
      <div className="bg-admin-bg-panel border border-admin-border rounded-2xl shadow-xl flex flex-col min-h-[500px]">
        <div className="p-5 border-b border-admin-border flex items-center justify-between bg-admin-bg-root/30">
          <h3 className="text-base font-semibold text-admin-text-main">Global Audit Log</h3>
          <div className="flex items-center gap-3">
            <div className="relative">
              <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-admin-text-muted" />
              <input 
                type="text" 
                placeholder="Search events, IPs, admin IDs..." 
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="w-64 bg-admin-input-bg border border-admin-border rounded-lg py-1.5 pl-9 pr-4 text-sm text-admin-text-main focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-all placeholder:text-admin-text-muted"
              />
            </div>
            <button className="bg-admin-bg-root border border-admin-border text-admin-text-muted px-3 py-1.5 rounded-lg text-sm flex items-center gap-2 hover:bg-admin-bg-hover hover:text-admin-text-main transition-colors">
              <Filter size={14} /> Filter
            </button>
          </div>
        </div>

        <div className="flex-1 overflow-auto custom-scrollbar">
          <table className="w-full text-left border-collapse">
            <thead>
              <tr className="text-[11px] uppercase tracking-widest text-admin-text-muted border-b border-admin-border bg-admin-bg-root/80 sticky top-0 z-10 backdrop-blur-sm">
                <th className="px-6 py-4 font-semibold">Event Time (UTC)</th>
                <th className="px-6 py-4 font-semibold">Actor</th>
                <th className="px-6 py-4 font-semibold">Action</th>
                <th className="px-6 py-4 font-semibold">Resource</th>
                <th className="px-6 py-4 font-semibold">IP Address</th>
                <th className="px-6 py-4 font-semibold text-right">Details</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-admin-border text-sm">
              <AuditRow time="2023-10-24 14:22:10" actor="root_admin_1" action="Configure Platform Plan" resource="Plan: Enterprise Grid" ip="104.28.19.12" />
              <AuditRow time="2023-10-24 13:10:05" actor="sys_automated" action="Apply Security Patch" resource="Cluster: EU-West-1" ip="10.0.0.4" />
              <AuditRow time="2023-10-24 11:45:00" actor="root_admin_2" action="Grant Tenant Admin Access" resource="Tenant: T-1045" ip="192.168.1.150" />
              <AuditRow time="2023-10-23 09:30:12" actor="support_tier_2" action="Initiate Legal Hold" resource="Tenant: T-0982 (DataVault)" ip="35.190.22.4" />
              <AuditRow time="2023-10-23 08:15:22" actor="sys_automated" action="Daily Backup Completed" resource="Database: Main_US_East" ip="10.0.0.8" />
              <AuditRow time="2023-10-22 16:40:05" actor="root_admin_1" action="Modify Rate Limits" resource="API Gateway: Global" ip="104.28.19.12" />
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
};

const AuditRow = ({ time, actor, action, resource, ip }: any) => (
  <tr className="hover:bg-admin-bg-hover transition-colors group">
    <td className="px-6 py-4 text-[11px] font-mono text-admin-text-muted">{time}</td>
    <td className="px-6 py-4">
      <span className="font-mono text-xs text-indigo-400 bg-indigo-500/10 px-2.5 py-1 rounded-md border border-indigo-500/20">{actor}</span>
    </td>
    <td className="px-6 py-4 font-medium text-admin-text-main">{action}</td>
    <td className="px-6 py-4 text-xs text-admin-text-muted">{resource}</td>
    <td className="px-6 py-4 text-[11px] font-mono text-admin-text-muted">{ip}</td>
    <td className="px-6 py-4 text-right">
      <button className="p-1.5 text-admin-text-muted hover:text-indigo-400 hover:bg-admin-bg-root rounded-md transition-colors">
        <FileSearch size={16} />
      </button>
    </td>
  </tr>
);
