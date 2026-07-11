import React, { Suspense } from 'react';
import { hasAppSdkPermissionForCurrentSession } from '@sdkwork/im-pc-core';

import { isShellCapabilityModule, resolveLazyCapabilityModule } from './capabilityModuleLoaders';
import { isCommercialRuntimeModule, resolveModuleRequiredPermission } from './moduleRegistry';

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
    <div className="sdkwork-capability-embed-host flex flex-1 min-h-0 h-full w-full items-center justify-center bg-[#1e1e1e] text-gray-500">
      Loading module...
    </div>
  ),
  renderModule,
}) => {
  if (!isShellCapabilityModule(activeTab) || !isCommercialRuntimeModule(activeTab)) {
    return null;
  }

  // Route-level RBAC: refuse to mount a commercial module whose declared
  // `requiredPermission` is not granted by the current session token. The
  // check mirrors the backend `AppContext::has_permission` semantics so a
  // client without the permission claim can never load the module surface.
  const requiredPermission = resolveModuleRequiredPermission(activeTab);
  if (requiredPermission && !hasAppSdkPermissionForCurrentSession(requiredPermission)) {
    return null;
  }

  const LazyModule = resolveLazyCapabilityModule(activeTab);
  if (!LazyModule) {
    return null;
  }

  return <Suspense fallback={fallback}>{renderModule(activeTab, LazyModule)}</Suspense>;
};
