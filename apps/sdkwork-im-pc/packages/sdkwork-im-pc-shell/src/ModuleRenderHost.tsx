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
      {isChatModule(activeTab) ? (
        chatSurface
      ) : (
        <div className="sdkwork-capability-embed-host flex flex-1 min-h-0 h-full w-full min-w-0 flex-col overflow-hidden">
          {capabilitySurface}
        </div>
      )}
    </AppErrorBoundary>
  );
};
