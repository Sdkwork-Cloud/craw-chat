import React from 'react';
import type { KnowledgebasePcRuntimeConfigurator } from '@sdkwork/im-pc-core';

export type { DriveOpenRequest } from '@sdkwork/drive-pc-drive';

import {
  COMMERCIAL_RUNTIME_MODULES,
  type AppModuleId,
} from './moduleRegistry';

export type CapabilityModuleLoader = () => Promise<{ default: React.ComponentType<any> }>;

/** Lazy loaders for embed-capable modules. Core tabs (chat/workspace/contacts/…) are not lazy-loaded here. */
const CAPABILITY_MODULE_LOADERS: Record<string, CapabilityModuleLoader> = {
  orders: () => import('@sdkwork/im-pc-orders').then((module) => ({ default: module.OrdersView })),
  shop: () => import('@sdkwork/im-pc-shop').then((module) => ({ default: module.ShopView })),
  notary: () => import('@sdkwork/notary-pc-notary').then((module) => ({ default: module.NotaryView })),
  drive: () => import('@sdkwork/drive-pc-drive').then((module) => ({ default: module.DriveView })),
  knowledge: async () => {
    const [knowledgebaseModule, imCore] = await Promise.all([
      import('@sdkwork/knowledgebase-pc-knowledge'),
      import('@sdkwork/im-pc-core'),
    ]);
    imCore.ensureKnowledgebasePcRuntimeOnModule(
      knowledgebaseModule.configureKnowledgebasePcRuntime as KnowledgebasePcRuntimeConfigurator,
    );
    const KnowledgebaseCapability: React.FC = () => React.createElement(
      knowledgebaseModule.KnowledgebaseHostSurface,
      {
        presentationMode: knowledgebaseModule.resolveKnowledgebaseHostPresentationMode(),
      },
    );
    return { default: KnowledgebaseCapability };
  },
  community: () => import('@sdkwork/im-pc-community').then((module) => ({ default: module.CommunityView })),
  enterprise: async () => {
    const [companyModule, imCore] = await Promise.all([
      import('@sdkwork/im-pc-company'),
      import('@sdkwork/im-pc-core'),
    ]);
    const adapter = companyModule.createImCompanyPcHostAdapter({
      toast: (message) => {
        console.info(message);
      },
    });
    imCore.ensureCompanyPcRuntimeOnModule(companyModule.configureCompanyPcHost, adapter);
    return { default: companyModule.CompanyView };
  },
  voice: async () => {
    const [voiceModule, imCore] = await Promise.all([
      import('@sdkwork/voice-pc-market'),
      import('@sdkwork/im-pc-core'),
      import('@sdkwork/voice-pc-speech'),
    ]);
    imCore.ensureVoicePcRuntimeOnModule(voiceModule.configureVoicePcRuntime);
    return { default: voiceModule.VoiceMarketView };
  },
};

const CORE_SHELL_MODULE_IDS = new Set<AppModuleId>([
  'chat',
  'workspace',
  'contacts',
  'favorites',
  'agent',
]);

function isCommercialCapabilityModule(moduleId: string): moduleId is AppModuleId {
  return (
    COMMERCIAL_RUNTIME_MODULES.has(moduleId as AppModuleId)
    && !CORE_SHELL_MODULE_IDS.has(moduleId as AppModuleId)
    && Object.prototype.hasOwnProperty.call(CAPABILITY_MODULE_LOADERS, moduleId)
  );
}

/** Production navigation only exposes commercial runtime capability modules. */
export const SHELL_CAPABILITY_MODULE_LOADERS: Record<string, CapabilityModuleLoader> =
  Object.fromEntries(
    Object.entries(CAPABILITY_MODULE_LOADERS).filter(([moduleId]) =>
      isCommercialCapabilityModule(moduleId),
    ),
  );

const lazyModuleCache = new Map<string, React.LazyExoticComponent<React.ComponentType<any>>>();

export function isShellCapabilityModule(moduleId: string): boolean {
  return Object.prototype.hasOwnProperty.call(SHELL_CAPABILITY_MODULE_LOADERS, moduleId);
}

export function resolveLazyCapabilityModule(
  moduleId: string,
): React.LazyExoticComponent<React.ComponentType<any>> | null {
  const loader = SHELL_CAPABILITY_MODULE_LOADERS[moduleId];
  if (!loader) {
    return null;
  }
  const cached = lazyModuleCache.get(moduleId);
  if (cached) {
    return cached;
  }
  const lazyModule = React.lazy(loader);
  lazyModuleCache.set(moduleId, lazyModule);
  return lazyModule;
}
