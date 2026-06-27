import React, { useState, useEffect } from 'react';
import { Settings, Shield, Globe, HardDrive, Mail, Webhook, Save } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { adminSettingsService, AdminSettingsData } from './services/AdminSettingsService';

export const AdminSettings = () => {
  const [data, setData] = useState<AdminSettingsData | null>(null);
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    const fetchData = async () => {
      setLoading(true);
      try {
        const res = await adminSettingsService.getSettings();
        setData(res);
      } finally {
        setLoading(false);
      }
    };
    fetchData();
  }, []);

  const handleSave = async () => {
    if (!data) return;
    setSaving(true);
    try {
      await adminSettingsService.updateSettings(data);
    } finally {
      setSaving(false);
    }
  };

  if (loading && !data) return <div className="p-8 text-center text-admin-text-muted">加载设置中...</div>;
  if (!data) return null;
  return (
    <div className="flex flex-col lg:flex-row gap-8">
      {/* Settings Navigation */}
      <div className="w-full lg:w-64 shrink-0">
        <div className="bg-admin-bg-panel border border-admin-border rounded-xl shadow-xl p-2 flex flex-col gap-1 sticky top-6">
          <SettingNav active icon={Globe} label="Platform Configurations" />
          <SettingNav icon={Shield} label="Authentication & Security" />
          <SettingNav icon={HardDrive} label="Infrastructure Quotas" />
          <SettingNav icon={Mail} label="SMTP & Communication" />
          <SettingNav icon={Webhook} label="Platform Webhooks" />
          <SettingNav icon={Settings} label="Advanced Options" />
        </div>
      </div>

      {/* Main Content Area */}
      <div className="flex-1 space-y-6">
        <div className="bg-admin-bg-panel border border-admin-border rounded-2xl shadow-xl overflow-hidden">
          <div className="p-6 border-b border-admin-border bg-admin-bg-root/30">
            <h3 className="text-lg font-semibold text-admin-text-main tracking-wide">Platform Configurations</h3>
            <p className="text-sm text-admin-text-muted mt-1">Global settings affecting all tenants and system behavior</p>
          </div>
          
          <div className="p-6 space-y-8">
            {/* Form Fields */}
            <div className="space-y-4">
              <h4 className="text-sm font-semibold text-admin-text-main border-b border-admin-border pb-2">Global System Identification</h4>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div className="space-y-2">
                  <label className="text-xs font-medium text-admin-text-muted uppercase tracking-wider">Platform Name</label>
                  <input 
                    type="text" 
                    value={data.platformName || ''}
                    onChange={(e) => setData({ ...data, platformName: e.target.value })}
                    className="w-full bg-admin-input-bg border border-admin-border rounded-lg py-2.5 px-3 text-sm text-admin-text-main focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-shadow"
                  />
                </div>
                <div className="space-y-2">
                  <label className="text-xs font-medium text-admin-text-muted uppercase tracking-wider">Primary Support Contact</label>
                  <input 
                    type="email" 
                    value={data.supportContact || ''}
                    onChange={(e) => setData({ ...data, supportContact: e.target.value })}
                    className="w-full bg-admin-input-bg border border-admin-border rounded-lg py-2.5 px-3 text-sm text-admin-text-main focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-shadow"
                  />
                </div>
              </div>
            </div>

            <div className="space-y-4">
              <h4 className="text-sm font-semibold text-admin-text-main border-b border-admin-border pb-2">Tenant Onboarding</h4>
              <div className="flex items-center justify-between p-4 rounded-xl border border-admin-border bg-admin-bg-root/50">
                <div>
                  <div className="text-sm font-medium text-admin-text-main">Allow Self-Service Registration</div>
                  <div className="text-xs text-admin-text-muted mt-1">If enabled, new customers can provision tenants without sales team approval.</div>
                </div>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input type="checkbox" className="sr-only peer" checked={data.allowSelfService} onChange={(e) => setData({ ...data, allowSelfService: e.target.checked })} />
                  <div className="w-11 h-6 bg-admin-bg-hover peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-500 border border-admin-border-subtle"></div>
                </label>
              </div>
            </div>

            <div className="space-y-4">
              <h4 className="text-sm font-semibold text-admin-text-main border-b border-admin-border pb-2">Maintenance Mode</h4>
              <div className="flex items-center justify-between p-4 rounded-xl border border-rose-500/20 bg-rose-500/5">
                <div>
                  <div className="text-sm font-medium text-rose-400">Lock Platform Global Access</div>
                  <div className="text-xs text-admin-text-muted mt-1">Prevent all non-root users from logging into any tenant. Useful for critical database migrations.</div>
                </div>
                <button className="px-4 py-2 bg-rose-500/10 text-rose-400 border border-rose-500/20 hover:bg-rose-500/20 rounded-lg text-sm font-medium transition-colors">
                  Enable Lock
                </button>
              </div>
            </div>
            
            <div className="pt-6 border-t border-admin-border flex justify-end">
              <button 
                onClick={handleSave}
                disabled={saving}
                className={cn("bg-indigo-600 hover:bg-indigo-500 text-white px-6 py-2.5 rounded-lg text-sm font-medium flex items-center gap-2 shadow-[0_0_15px_rgba(79,70,229,0.3)] transition-colors", saving && "opacity-70 cursor-not-allowed")}>
                <Save size={16} />
                {saving ? "Saving..." : "Save Configurations"}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

const SettingNav = ({ icon: Icon, label, active }: any) => (
  <button className={cn(
    "flex items-center gap-3 px-4 py-2.5 rounded-lg text-sm text-left transition-colors font-medium",
    active 
      ? "bg-indigo-500/10 text-indigo-400 border border-indigo-500/20" 
      : "text-admin-text-main hover:bg-admin-bg-hover border border-transparent"
  )}>
    <Icon size={16} className={cn(active ? "text-indigo-400" : "text-admin-text-muted")} />
    {label}
  </button>
);
