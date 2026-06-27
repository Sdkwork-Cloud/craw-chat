import React, { useState, useEffect } from 'react';
import { Save, LogOut, Code, Key, Webhook, BellRing, Monitor, ShieldCheck, HelpCircle } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { sysSettingsService, SysSettings } from './services/SysSettingsService';

export const ConsoleSettings = () => {
  const [settings, setSettings] = useState<SysSettings | null>(null);
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    const fetchSettings = async () => {
      setLoading(true);
      try {
        const res = await sysSettingsService.getSettings();
        setSettings(res);
      } finally {
        setLoading(false);
      }
    };
    fetchSettings();
  }, []);

  const handleSave = async () => {
    if (!settings) return;
    setSaving(true);
    try {
      const res = await sysSettingsService.updateSettings(settings);
      setSettings(res);
    } finally {
      setSaving(false);
    }
  };

  if (loading && !settings) {
    return <div className="p-8 text-center text-console-text-muted">加载配置中...</div>;
  }

  if (!settings) return null;

  return (
    <div className="flex flex-col lg:flex-row gap-8">
      {/* Settings Navigation */}
      <div className="w-full lg:w-64 shrink-0">
        <div className="bg-console-bg-panel border border-console-border rounded-xl shadow-sm p-2 flex flex-col gap-1 sticky top-6">
          <SettingNav active icon={Monitor} label="基础设施与基本信息" />
          <SettingNav icon={Code} label="API Keys 与集成" />
          <SettingNav icon={Webhook} label="Webhook 订阅" />
          <SettingNav icon={Key} label="单点登录 (SSO)" />
          <SettingNav icon={ShieldCheck} label="全局安全配置" />
          <SettingNav icon={BellRing} label="通知与推送设置" />
        </div>
      </div>

      {/* Main Content Area */}
      <div className="flex-1 space-y-6">
        <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm overflow-hidden">
          <div className="p-6 border-b border-console-border">
            <h3 className="text-lg font-semibold text-console-text-main">基础设施与基本信息</h3>
            <p className="text-sm text-console-text-muted mt-1">配置您的企业即时通信环境基本标识和环境</p>
          </div>
          
          <div className="p-6 space-y-6">
            {/* Form Fields */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div className="space-y-2">
                <label className="text-sm font-medium text-console-text-main">企业/组织名称</label>
                <input 
                  type="text" 
                  value={settings.orgName}
                  onChange={e => setSettings({...settings, orgName: e.target.value})}
                  className="w-full bg-console-input-bg border border-console-border rounded-lg py-2 px-3 text-sm text-console-text-main focus:ring-2 focus:ring-blue-500/50 outline-none transition-shadow"
                />
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium text-console-text-main">环境标志</label>
                <input 
                  type="text" 
                  value={settings.envFlag}
                  disabled
                  className="w-full bg-console-bg-root border border-console-border rounded-lg py-2 px-3 text-sm text-console-text-muted cursor-not-allowed"
                />
                <p className="text-xs text-console-text-muted">环境标志在租户创建后不可修改。</p>
              </div>
            </div>

            <div className="space-y-2">
              <label className="text-sm font-medium text-console-text-main">自定义域名 (Cname)</label>
              <div className="flex gap-2">
                <input 
                  type="text" 
                  placeholder="例如: chat.acmecorp.com"
                  value={settings.customDomain}
                  onChange={e => setSettings({...settings, customDomain: e.target.value})}
                  className="flex-1 bg-console-input-bg border border-console-border rounded-lg py-2 px-3 text-sm text-console-text-main focus:ring-2 focus:ring-blue-500/50 outline-none transition-shadow"
                />
                <button className="px-4 py-2 bg-console-bg-hover hover:bg-console-border-light border border-console-border text-console-text-main rounded-lg text-sm font-medium transition-colors">
                  验证
                </button>
              </div>
            </div>

            <div className="space-y-2 pt-4 border-t border-console-border">
              <label className="text-sm font-medium text-console-text-main">客户端品牌定制</label>
              <p className="text-xs text-console-text-muted mb-4">上传您企业的Logo，它将显示在员工的聊天客户端和登录界面中。</p>
              
              <div className="flex items-center gap-6">
                <div className="w-20 h-20 bg-blue-600 rounded-2xl flex items-center justify-center text-white text-2xl font-bold shadow-lg">
                  A
                </div>
                <div className="flex flex-col gap-2">
                  <div className="flex gap-2">
                    <button className="px-4 py-2 border border-console-border hover:bg-console-bg-hover text-console-text-main text-sm font-medium rounded-lg transition-colors">
                      上传新图片
                    </button>
                    <button className="px-4 py-2 text-rose-600 hover:bg-rose-50 dark:hover:bg-rose-500/10 text-sm font-medium rounded-lg transition-colors">
                      移除
                    </button>
                  </div>
                  <p className="text-xs text-console-text-muted">建议尺寸 512x512，PNG 格式</p>
                </div>
              </div>
            </div>
            
            <div className="pt-6 mt-6 border-t border-console-border flex justify-end">
              <button 
                onClick={handleSave}
                disabled={saving}
                className="bg-blue-600 hover:bg-blue-700 disabled:opacity-50 text-white px-6 py-2.5 rounded-lg text-sm font-medium flex items-center gap-2 shadow-sm transition-colors"
              >
                <Save size={16} />
                {saving ? '保存中...' : '保存配置修改'}
              </button>
            </div>
          </div>
        </div>

        {/* API Keys Preview */}
        <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm overflow-hidden">
          <div className="p-6 border-b border-console-border flex justify-between items-center">
            <div>
              <h3 className="text-lg font-semibold text-console-text-main">API Keys (生产环境)</h3>
              <p className="text-sm text-console-text-muted mt-1">用于服务器端集成与客户端初始化的鉴权凭证</p>
            </div>
            <button className="px-3 py-1.5 bg-console-bg-hover border border-console-border text-console-text-main text-sm font-medium rounded-lg">
              轮换密钥
            </button>
          </div>
          <div className="p-6 space-y-4">
            <div className="flex justify-between items-center p-3 rounded-xl border border-console-border bg-console-bg-root">
              <div>
                <div className="text-sm font-medium text-console-text-main mb-1">发布密钥 (Publishable Key)</div>
                <div className="font-mono text-xs text-console-text-muted">用于客户端 SDK 初始化</div>
              </div>
              <div className="flex items-center gap-3">
                <div className="font-mono text-sm bg-console-bg-panel px-3 py-1.5 rounded-md border border-console-border-light text-console-text-main">
                  {settings.publishableKey}
                </div>
                <button className="text-blue-600 text-sm font-medium hover:text-blue-700">复制</button>
              </div>
            </div>
            
            <div className="flex justify-between items-center p-3 rounded-xl border border-rose-200 dark:border-rose-500/20 bg-rose-50 dark:bg-rose-500/5">
              <div>
                <div className="text-sm font-medium text-rose-700 dark:text-rose-400 mb-1">私有密钥 (Secret Key)</div>
                <div className="font-mono text-xs text-rose-600/70 dark:text-rose-400/70">用于服务器端 API 调用，请勿在客户端暴露</div>
              </div>
              <div className="flex items-center gap-3">
                <div className="font-mono text-sm bg-white dark:bg-console-bg-root px-3 py-1.5 rounded-md border border-rose-200 dark:border-rose-500/30 text-console-text-main">
                  {settings.secretKey}
                </div>
                <button className="text-blue-600 text-sm font-medium hover:text-blue-700">显示</button>
              </div>
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
      ? "bg-console-active-bg text-console-active-text" 
      : "text-console-text-main hover:bg-console-bg-hover hover:text-console-text-main"
  )}>
    <Icon size={16} className={cn(active ? "text-console-active-text" : "text-console-text-muted")} />
    {label}
  </button>
);
