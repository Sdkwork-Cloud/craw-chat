import React, { createContext, useContext, useState, useEffect } from 'react';

type Language = 'en' | 'zh';

interface I18nContextType {
  language: Language;
  setLanguage: (lang: Language) => void;
  t: (key: string, defaultText?: string) => string;
}

const I18nContext = createContext<I18nContextType>({
  language: 'zh',
  setLanguage: () => {},
  t: (key) => key
});

const translations: Record<string, Record<string, string>> = {
  en: {
    // AdminLayout
    'admin.title': 'Sdkwork IM Admin',
    'admin.subtitle': 'Super Platform',
    'admin.nav.operations': 'Platform Operations',
    'admin.nav.overview': 'Platform Overview',
    'admin.nav.tenants': 'Tenant Management',
    'admin.nav.users': 'Global Users',
    'admin.nav.infrastructure': 'Infrastructure & Nodes',
    'admin.nav.billing': 'Billing & Subscriptions',
    'admin.nav.announcements': 'Platform Announcements',
    'admin.nav.compliance': 'Global Compliance',
    'admin.nav.settings': 'Platform Configuration',
    'admin.nav.switch': 'Switch to Client',
    'admin.header.search': 'Search resources, tenants, IDs...',
    'admin.header.root': 'Root Administrator',
    'admin.header.system': 'System',
    // AdminUsers
    'admin.users.title': 'Global Users & Identities',
    'admin.users.subtitle': 'Cross-tenant user tracing, global bans, and identity verification',
    'admin.users.export': 'Export Global Roster',
    'admin.users.search': 'Search by exact UIN, email, phone...',
    'admin.users.filter.all': 'All Global Statuses',
    'admin.users.filter.active': 'Active Accounts',
    'admin.users.filter.banned': 'Banned Globally',
    'admin.users.filter.pending': 'Pending Verification',
    'admin.users.col.identity': 'User Identity',
    'admin.users.col.tenant': 'Primary Tenant',
    'admin.users.col.security': 'Security Level',
    'admin.users.col.status': 'Global Status',
    'admin.users.col.actions': 'Admin Actions',
    'admin.users.footer': 'Showing 4 of 12.8M platform identities',
    'admin.users.status.active': 'Active',
    'admin.users.status.banned': 'Global Ban',
    'admin.users.status.warning': 'Flagged',
    // PlatformTenants
    'admin.tenants.title': 'Tenant Management',
    'admin.tenants.subtitle': 'Manage platform tenants, subscription plans, and resource limits',
    'admin.tenants.provision': 'Provision Tenant',
    'admin.tenants.search': 'Search tenant ID, name, region...',
    'admin.tenants.filters': 'Filters',
    'admin.tenants.healthy': 'System Healthy',
    'admin.tenants.col.info': 'Tenant Info',
    'admin.tenants.col.plan': 'Plan Level',
    'admin.tenants.col.users': 'Active Users',
    'admin.tenants.col.region': 'Region',
    'admin.tenants.col.mrr': 'MRR',
    'admin.tenants.col.actions': 'Actions',
    'admin.tenants.pagination.prev': 'Previous',
    'admin.tenants.pagination.next': 'Next',
    'admin.tenants.loading': 'Loading...',
    'admin.tenants.empty': 'No tenants found',
    // Others
    'admin.under_construction': 'Module Under Construction',
    'admin.lang.zh': '中文',
    'admin.lang.en': 'English'
  },
  zh: {
    // AdminLayout
    'admin.title': 'Sdkwork IM 控制台',
    'admin.subtitle': '超级平台',
    'admin.nav.operations': '平台运营',
    'admin.nav.overview': '平台总览',
    'admin.nav.tenants': '租户管理',
    'admin.nav.users': '全局用户管理',
    'admin.nav.infrastructure': '基础设施与节点',
    'admin.nav.billing': '计费与订阅',
    'admin.nav.announcements': '平台公告',
    'admin.nav.compliance': '全球合规性',
    'admin.nav.settings': '平台配置',
    'admin.nav.switch': '切换到客户端',
    'admin.header.search': '搜索资源、租户、ID...',
    'admin.header.root': '超级管理员',
    'admin.header.system': '系统',
    // AdminUsers
    'admin.users.title': '全局用户与身份',
    'admin.users.subtitle': '跨租户用户追踪、全局封禁和身份验证',
    'admin.users.export': '导出全部人员',
    'admin.users.search': '通过 UIN、邮箱、手机号精确搜索...',
    'admin.users.filter.all': '所有状态',
    'admin.users.filter.active': '正常账户',
    'admin.users.filter.banned': '已全平台封禁',
    'admin.users.filter.pending': '等待验证',
    'admin.users.col.identity': '用户身份',
    'admin.users.col.tenant': '主要租户',
    'admin.users.col.security': '安全等级',
    'admin.users.col.status': '全局状态',
    'admin.users.col.actions': '管理操作',
    'admin.users.footer': '显示 1280 万 平台身份中的 4 个',
    'admin.users.status.active': '正常',
    'admin.users.status.banned': '全局封禁',
    'admin.users.status.warning': '标记为异常',
    // PlatformTenants
    'admin.tenants.title': '租户管理',
    'admin.tenants.subtitle': '管理平台租户、订阅计划和资源限制',
    'admin.tenants.provision': '配置新租户',
    'admin.tenants.search': '搜索租户ID、名称、地区...',
    'admin.tenants.filters': '过滤',
    'admin.tenants.healthy': '系统健康',
    'admin.tenants.col.info': '租户信息',
    'admin.tenants.col.plan': '服务计划',
    'admin.tenants.col.users': '活跃用户',
    'admin.tenants.col.region': '可用区',
    'admin.tenants.col.mrr': '主营收入',
    'admin.tenants.col.actions': '操作',
    'admin.tenants.pagination.prev': '上一页',
    'admin.tenants.pagination.next': '下一页',
    'admin.tenants.loading': '加载中...',
    'admin.tenants.empty': '未找到相关租户',
    // Others
    'admin.under_construction': '模块建设中',
    'admin.lang.zh': '中文',
    'admin.lang.en': 'English'
  }
};

export const I18nProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [language, setLanguage] = useState<Language>('zh');

  useEffect(() => {
    const storedLang = localStorage.getItem('admin_lang') as Language;
    if (storedLang && (storedLang === 'en' || storedLang === 'zh')) {
      setLanguage(storedLang);
    }
  }, []);

  const handleSetLanguage = (lang: Language) => {
    setLanguage(lang);
    localStorage.setItem('admin_lang', lang);
  };

  const t = (key: string, defaultText?: string) => {
    return translations[language]?.[key] || translations['en']?.[key] || defaultText || key;
  };

  return (
    <I18nContext.Provider value={{ language, setLanguage: handleSetLanguage, t }}>
      {children}
    </I18nContext.Provider>
  );
};

export const useTranslation = () => useContext(I18nContext);
