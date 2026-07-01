import React from 'react';

export type CapabilityModuleLoader = () => Promise<{ default: React.ComponentType<any> }>;

export const SHELL_CAPABILITY_MODULE_LOADERS: Record<string, CapabilityModuleLoader> = {
  orders: () => import('@sdkwork/im-pc-orders').then((module) => ({ default: module.OrdersView })),
  shop: () => import('@sdkwork/im-pc-shop').then((module) => ({ default: module.ShopView })),
  notary: () => import('@sdkwork/notary-pc-notary').then((module) => ({ default: module.NotaryView })),
  mail: () => import('@sdkwork/im-pc-mail').then((module) => ({ default: module.MailView })),
  drive: () => import('@sdkwork/drive-pc-drive').then((module) => ({ default: module.DriveView })),
  calendar: () => import('@sdkwork/im-pc-calendar').then((module) => ({ default: module.CalendarView })),
  approval: () => import('@sdkwork/im-pc-approvals').then((module) => ({ default: module.ApprovalsView })),
  report: () => import('@sdkwork/im-pc-reports').then((module) => ({ default: module.ReportsView })),
  attendance: () => import('@sdkwork/im-pc-attendance').then((module) => ({ default: module.AttendanceView })),
  knowledge: async () => {
    const [knowledgebaseModule, imCore] = await Promise.all([
      import('@sdkwork/knowledgebase-pc-knowledge'),
      import('@sdkwork/im-pc-core'),
    ]);
    imCore.ensureKnowledgebasePcRuntimeOnModule(knowledgebaseModule.configureKnowledgebasePcRuntime);
    return { default: knowledgebaseModule.KnowledgeView };
  },
  course: async () => {
    const [courseModule, imCore] = await Promise.all([
      import('@sdkwork/course-pc-course'),
      import('@sdkwork/im-pc-core'),
    ]);
    imCore.ensureCoursePcRuntimeOnModule(courseModule.configureCoursePcRuntime);
    return { default: courseModule.CourseView };
  },
  enterprise: () => import('@sdkwork/im-pc-enterprise').then((module) => ({ default: module.EnterpriseView })),
  devices: () => import('@sdkwork/im-pc-devices').then((module) => ({ default: module.DevicesView })),
  community: () => import('@sdkwork/im-pc-community').then((module) => ({ default: module.CommunityView })),
  videogen: () => import('@sdkwork/im-pc-video-gen').then((module) => ({ default: module.VideoGenView })),
  imagegen: () => import('@sdkwork/im-pc-image-gen').then((module) => ({ default: module.ImageGenView })),
  voice: async () => {
    const [voiceModule, imCore] = await Promise.all([
      import('@sdkwork/voice-pc-market'),
      import('@sdkwork/im-pc-core'),
    ]);
    imCore.ensureVoicePcRuntimeOnModule(voiceModule.configureVoicePcRuntime);
    return { default: voiceModule.VoiceMarketView };
  },
  voicegen: async () => {
    const [speechModule, imCore] = await Promise.all([
      import('@sdkwork/voice-pc-speech'),
      import('@sdkwork/im-pc-core'),
    ]);
    imCore.ensureVoicePcRuntimeOnModule(speechModule.configureVoicePcRuntime);
    return { default: speechModule.VoiceSpeechView };
  },
  musicgen: () => import('@sdkwork/im-pc-music-gen').then((module) => ({ default: module.MusicGenView })),
  writing: () => import('@sdkwork/im-pc-writing').then((module) => ({ default: module.WritingView })),
};

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
