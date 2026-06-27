import { lazy, Suspense } from 'react';
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

const ROUTE_FALLBACK = (
  <div className="flex h-screen w-screen items-center justify-center bg-[#1f1f1f] text-sm text-gray-400">
    Loading...
  </div>
);

function ConsoleApp() {
  const navigate = useNavigate();
  return <ConsoleLayout onSwitchToClient={() => navigate('/')} />;
}

function AdminApp() {
  const navigate = useNavigate();
  return <AdminLayout onSwitchToClient={() => navigate('/')} />;
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
