import { createImPcHostLanguageBridge } from '@sdkwork/im-pc-commons';

import { getCourseAppSdkClient } from './courseAppSdkClient';
import {
  readAppSdkSessionTokens,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  type SdkworkChatSession,
} from './session';

interface CourseCapabilitySdkPorts {
  getCourseClient: () => unknown;
  readHostSession: () => unknown;
  subscribeHostSession?: (listener: () => void) => () => void;
  resolveHostLanguage?: () => string;
  subscribeHostLanguage?: (listener: (language: string) => void) => () => void;
}

let coursePcRuntimeBootstrapped = false;
let imCoursePcPorts: CourseCapabilitySdkPorts | null = null;

function mapImSessionToCourseSnapshot(session: SdkworkChatSession | null) {
  if (!session?.user) {
    return null;
  }

  return {
    user: {
      displayName: session.user.displayName,
      nickname: session.user.nickname,
      name: session.user.name,
      avatar: session.user.avatar,
    },
  };
}

function createImCoursePcSdkPorts(): CourseCapabilitySdkPorts {
  const hostLanguageBridge = createImPcHostLanguageBridge();
  return {
    getCourseClient: getCourseAppSdkClient,
    readHostSession: () => mapImSessionToCourseSnapshot(readAppSdkSessionTokens()),
    subscribeHostSession(listener: () => void) {
      const handler = () => listener();
      window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, handler);
      return () => window.removeEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, handler);
    },
    resolveHostLanguage: hostLanguageBridge.resolveInitialLanguage,
    subscribeHostLanguage: hostLanguageBridge.onLanguageChange,
  };
}

export type CoursePcRuntimeConfigurator = (options: {
  sdkPorts: CourseCapabilitySdkPorts;
}) => void;

function resolveImCoursePcSdkPorts(): CourseCapabilitySdkPorts {
  imCoursePcPorts ??= createImCoursePcSdkPorts();
  return imCoursePcPorts;
}

export function ensureCoursePcRuntimeOnModule(
  configureRuntime: CoursePcRuntimeConfigurator,
): void {
  configureRuntime({
    sdkPorts: resolveImCoursePcSdkPorts() as never,
  });
  coursePcRuntimeBootstrapped = true;
}

export async function bootstrapCoursePcForIm(): Promise<void> {
  const { configureCoursePcRuntime } = await import('@sdkwork/course-pc-course');
  ensureCoursePcRuntimeOnModule(configureCoursePcRuntime as CoursePcRuntimeConfigurator);
}

export async function rebootstrapCoursePcRuntimeForIm(): Promise<void> {
  if (!imCoursePcPorts) {
    return;
  }
  const { configureCoursePcRuntime } = await import('@sdkwork/course-pc-course');
  configureCoursePcRuntime({
    sdkPorts: imCoursePcPorts as never,
  });
}

export function isCoursePcRuntimeBootstrapped(): boolean {
  return coursePcRuntimeBootstrapped;
}

export function resetCoursePcRuntime(): void {
  coursePcRuntimeBootstrapped = false;
  imCoursePcPorts = null;
}
