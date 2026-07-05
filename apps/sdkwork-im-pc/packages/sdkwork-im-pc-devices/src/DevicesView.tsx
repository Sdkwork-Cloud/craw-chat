import React from 'react';
import { SdkworkDevicePage } from '@sdkwork/aiot-pc-console-device';

export interface DevicesViewProps {
  onEditAgent?: (agentId: string) => void;
}

function resolveAgentIdFromRoute(route: string): string | null {
  const match = route.match(/(?:^|\/)agents?\/([^/?#]+)/iu);
  return match?.[1] ?? null;
}

export const DevicesView: React.FC<DevicesViewProps> = ({ onEditAgent }) => {
  return (
    <SdkworkDevicePage
      onNavigate={(route) => {
        const agentId = resolveAgentIdFromRoute(route);
        if (agentId) {
          onEditAgent?.(agentId);
        }
      }}
    />
  );
};
