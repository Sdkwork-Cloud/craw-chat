import React from 'react';
import { BellRing, Send, Plus, Calendar, Target, Clock } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';

export const AdminAnnouncements = () => {
  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h2 className="text-xl font-bold text-admin-text-main tracking-wide">Platform Announcements</h2>
          <p className="text-sm text-admin-text-muted mt-1">Broadcast messages to Tenant Administrators and App Maintainers</p>
        </div>
        <button className="bg-indigo-600 hover:bg-indigo-500 text-white px-5 py-2.5 rounded-lg text-sm font-medium transition-all shadow-[0_0_15px_rgba(79,70,229,0.3)] flex items-center gap-2">
          <Send size={16} />
          New Broadcast
        </button>
      </div>

      {/* Notice List */}
      <div className="bg-admin-bg-panel border border-admin-border rounded-2xl shadow-xl flex flex-col overflow-hidden min-h-[500px]">
         <div className="p-5 border-b border-admin-border flex items-center justify-between bg-admin-bg-root/30">
          <div className="flex gap-4 border-b border-admin-border/50">
            <button className="text-sm font-medium text-indigo-400 border-b-2 border-indigo-500 pb-2 px-1">Sent Broadcasts</button>
            <button className="text-sm font-medium text-admin-text-muted hover:text-admin-text-main pb-2 px-1 transition-colors">Scheduled</button>
            <button className="text-sm font-medium text-admin-text-muted hover:text-admin-text-main pb-2 px-1 transition-colors">Drafts (2)</button>
          </div>
        </div>

        <div className="flex-1 p-6 space-y-4">
          <BroadcastCard 
            title="Critical Server Maintenance Window: APAC Region"
            date="Oct 20, 2023"
            target="All paid tenants in APAC-1, APAC-2"
            views="1,402"
            status="delivered"
            tag="Ops"
          />
          <BroadcastCard 
            title="New Feature Release: AI Summarizations & DLP Enhanced"
            date="Oct 15, 2023"
            target="Enterprise Grid Tenants Only"
            views="420"
            status="delivered"
            tag="Release"
          />
          <BroadcastCard 
            title="Policy Update: API Rate Limit Adjustments for Free Tier"
            date="Oct 01, 2023"
            target="All Free Tier Tenants"
            views="4,105"
            status="delivered"
            tag="Policy"
          />
        </div>
      </div>
    </div>
  );
};

const BroadcastCard = ({ title, date, target, views, status, tag }: any) => {
  return (
    <div className="bg-admin-bg-root border border-admin-border rounded-xl p-5 hover:border-indigo-500/30 transition-all hover:bg-admin-bg-hover group">
      <div className="flex justify-between items-start">
        <div>
          <div className="flex items-center gap-2 mb-2">
            <span className="px-2 py-0.5 bg-admin-bg-panel border border-admin-border rounded text-[10px] font-mono text-admin-text-muted">{tag}</span>
            <h4 className="text-base font-semibold text-admin-text-main group-hover:text-indigo-400 transition-colors cursor-pointer">{title}</h4>
          </div>
          <div className="flex items-center gap-6 mt-3">
            <div className="flex items-center gap-1.5 text-xs text-admin-text-muted">
              <Calendar size={14} className="opacity-70" /> {date}
            </div>
            <div className="flex items-center gap-1.5 text-xs text-admin-text-muted">
              <Target size={14} className="opacity-70" /> Audience: {target}
            </div>
            <div className="flex items-center gap-1.5 text-xs text-admin-text-muted">
              <Clock size={14} className="opacity-70" /> Read by {views} Admins
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
