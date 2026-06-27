import React from 'react';

import { AppErrorBoundary } from '@sdkwork/im-pc-commons';
import { isChatModule } from './moduleLayout';

export interface ModuleRenderHostProps {
  activeTab: string;
  chatSurface: React.ReactNode;
  capabilitySurface: React.ReactNode;
}

/**
 * Shell-level module router: chat IM surface vs capability modules.
 * Capability view composition stays in capability packages; shell only switches surfaces.
 */
export const ModuleRenderHost: React.FC<ModuleRenderHostProps> = ({
  activeTab,
  chatSurface,
  capabilitySurface,
}) => {
  return (
    <AppErrorBoundary>
      {isChatModule(activeTab) ? chatSurface : capabilitySurface}
    </AppErrorBoundary>
  );
};
