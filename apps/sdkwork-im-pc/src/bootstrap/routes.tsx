import { lazy, Suspense, useMemo } from 'react';
import { Route, Routes, useNavigate } from 'react-router-dom';
import { AuthGate } from '../AuthGate';
import { useTauriTrayNavigationBridge } from './trayNavigation';

const ChatLayout = lazy(() =>
  import('@sdkwork/im-pc-chat').then((module) => ({ default: module.ChatLayout })),
);
const ConsoleLayout = lazy(() =>
  import('@sdkwork/im-console-core').then((module) => ({ default: module.ConsoleLayout })),
);
const AdminLayout = lazy(() =>
  import('@sdkwork/im-admin-core').then((module) => ({ default: module.AdminLayout })),
);
const ConsoleDashboard = lazy(() =>
  import('@sdkwork/im-console-dashboard').then((module) => ({ default: module.ConsoleDashboard })),
);
const TenantUsers = lazy(() =>
  import('@sdkwork/im-console-users').then((module) => ({ default: module.TenantUsers })),
);
const ConsoleRoles = lazy(() =>
  import('@sdkwork/im-console-roles').then((module) => ({ default: module.ConsoleRoles })),
);
const ConsoleGroups = lazy(() =>
  import('@sdkwork/im-console-communications').then((module) => ({ default: module.ConsoleGroups })),
);
const ConsoleMessages = lazy(() =>
  import('@sdkwork/im-console-communications').then((module) => ({ default: module.ConsoleMessages })),
);
const ConsoleAnnouncements = lazy(() =>
  import('@sdkwork/im-console-communications').then((module) => ({ default: module.ConsoleAnnouncements })),
);
const ConsoleIntegrations = lazy(() =>
  import('@sdkwork/im-console-integrations').then((module) => ({ default: module.ConsoleIntegrations })),
);
const ConsoleSecurity = lazy(() =>
  import('@sdkwork/im-console-security').then((module) => ({ default: module.ConsoleSecurity })),
);
const ConsoleAnalytics = lazy(() =>
  import('@sdkwork/im-console-security').then((module) => ({ default: module.ConsoleAnalytics })),
);
const ConsoleSettings = lazy(() =>
  import('@sdkwork/im-console-settings').then((module) => ({ default: module.ConsoleSettings })),
);
const ConsoleStores = lazy(() =>
  import('@sdkwork/im-console-shop').then((module) => ({ default: module.ConsoleStores })),
);
const ConsoleProducts = lazy(() =>
  import('@sdkwork/im-console-product').then((module) => ({ default: module.ConsoleProducts })),
);
const AdminDashboard = lazy(() =>
  import('@sdkwork/im-admin-dashboard').then((module) => ({ default: module.AdminDashboard })),
);
const PlatformTenants = lazy(() =>
  import('@sdkwork/im-admin-tenants').then((module) => ({ default: module.PlatformTenants })),
);
const AdminUsers = lazy(() =>
  import('@sdkwork/im-admin-tenants').then((module) => ({ default: module.AdminUsers })),
);
const InfrastructureStatus = lazy(() =>
  import('@sdkwork/im-admin-infrastructure').then((module) => ({ default: module.InfrastructureStatus })),
);
const AdminBilling = lazy(() =>
  import('@sdkwork/im-admin-infrastructure').then((module) => ({ default: module.AdminBilling })),
);
const AdminAnnouncements = lazy(() =>
  import('@sdkwork/im-admin-operations').then((module) => ({ default: module.AdminAnnouncements })),
);
const AdminCompliance = lazy(() =>
  import('@sdkwork/im-admin-operations').then((module) => ({ default: module.AdminCompliance })),
);
const AdminSettings = lazy(() =>
  import('@sdkwork/im-admin-operations').then((module) => ({ default: module.AdminSettings })),
);

const ROUTE_FALLBACK = (
  <div className="flex h-screen w-screen items-center justify-center bg-[#1f1f1f] text-sm text-gray-400">
    Loading...
  </div>
);

function ConsoleApp() {
  const navigate = useNavigate();
  const routes = useMemo(() => ({
    analytics: <ConsoleAnalytics />,
    announcements: <ConsoleAnnouncements />,
    dashboard: <ConsoleDashboard />,
    groups: <ConsoleGroups />,
    integrations: <ConsoleIntegrations />,
    messages: <ConsoleMessages />,
    products: <ConsoleProducts />,
    roles: <ConsoleRoles />,
    security: <ConsoleSecurity />,
    settings: <ConsoleSettings />,
    stores: <ConsoleStores />,
    users: <TenantUsers />,
  }), []);
  return <ConsoleLayout onSwitchToClient={() => navigate('/')} routes={routes} />;
}

function AdminApp() {
  const navigate = useNavigate();
  const routes = useMemo(() => ({
    announcements: <AdminAnnouncements />,
    billing: <AdminBilling />,
    compliance: <AdminCompliance />,
    infrastructure: <InfrastructureStatus />,
    overview: <AdminDashboard />,
    settings: <AdminSettings />,
    tenants: <PlatformTenants />,
    users: <AdminUsers />,
  }), []);
  return <AdminLayout onSwitchToClient={() => navigate('/')} routes={routes} />;
}

export function AppRoutes() {
  useTauriTrayNavigationBridge();

  return (
    <AuthGate>
      <Suspense fallback={ROUTE_FALLBACK}>
        <Routes>
          <Route path="/console/*" element={<ConsoleApp />} />
          <Route path="/admin/*" element={<AdminApp />} />
          <Route path="/*" element={<ChatLayout />} />
        </Routes>
      </Suspense>
    </AuthGate>
  );
}
