import React from 'react';
import { isSdkworkChatDesktopRuntime } from '@sdkwork/im-pc-core';

import { isFullscreenModule } from './moduleLayout';

export { FULLSCREEN_MODULE_TABS, isChatModule, isFullscreenModule } from './moduleLayout';
export interface AppShellFrameProps {
  sidebar: React.ReactNode;
  header?: React.ReactNode;
  children: React.ReactNode;
  activeTab: string;
  desktopTitleBar?: React.ReactNode;
}

export const AppShellFrame: React.FC<AppShellFrameProps> = ({
  sidebar,
  header,
  children,
  activeTab,
  desktopTitleBar,
}) => {
  const shouldRenderDesktopAppHeader = isSdkworkChatDesktopRuntime();
  const shouldRenderHeader = header && !isFullscreenModule(activeTab);

  return (
    <div className="flex flex-col h-screen w-full overflow-hidden bg-[#1e1e1e] font-sans text-gray-200 print:overflow-visible print:h-auto print:min-h-0">
      {shouldRenderDesktopAppHeader && desktopTitleBar}
      <div className="flex flex-1 min-h-0 relative print:overflow-visible print:h-auto print:min-h-0 print:block">
        <div className="h-full shrink-0 flex flex-col z-20 print:hidden">{sidebar}</div>
        <div className="flex flex-col flex-1 min-w-0 min-h-0 relative print:overflow-visible print:h-auto print:min-h-0 print:block">
          {shouldRenderHeader ? (
            <div className="h-[52px] w-full flex items-center shrink-0 border-b border-white/5 bg-[#1e1e1e] relative print:hidden">
              {header}
            </div>
          ) : null}
          <div className="flex flex-row flex-1 min-h-0 min-w-0 relative print:overflow-visible print:h-auto print:min-h-0 print:block">
            {children}
          </div>
        </div>
      </div>
    </div>
  );
};
