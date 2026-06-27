import React, { Suspense } from 'react';

import { isShellCapabilityModule, resolveLazyCapabilityModule } from './capabilityModuleLoaders';

export interface LazyCapabilityModuleRendererProps {
  activeTab: string;
  fallback?: React.ReactNode;
  renderModule: (
    moduleId: string,
    ModuleComponent: React.LazyExoticComponent<React.ComponentType<any>>,
  ) => React.ReactNode;
}

export const LazyCapabilityModuleRenderer: React.FC<LazyCapabilityModuleRendererProps> = ({
  activeTab,
  fallback = (
    <div className="flex-1 flex items-center justify-center bg-[#1e1e1e] text-gray-500">
      Loading module...
    </div>
  ),
  renderModule,
}) => {
  if (!isShellCapabilityModule(activeTab)) {
    return null;
  }

  const LazyModule = resolveLazyCapabilityModule(activeTab);
  if (!LazyModule) {
    return null;
  }

  return <Suspense fallback={fallback}>{renderModule(activeTab, LazyModule)}</Suspense>;
};
