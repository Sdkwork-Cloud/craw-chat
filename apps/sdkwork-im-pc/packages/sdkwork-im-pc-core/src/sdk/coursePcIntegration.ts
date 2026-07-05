import { createImPcHostLanguageBridge } from '@sdkwork/im-pc-commons';
import {
  createClient,
  type SdkworkAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/course-app-sdk';
import type { Interceptors } from '@sdkwork/sdk-common';

import { resolveAppSdkBaseUrl } from './appSdkClient';
import {
  createSdkworkChatRequestContextInterceptors,
  getSdkworkChatGlobalTokenManager,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  type SdkworkChatSession,
} from './session';

export type CourseAppSdkClient = SdkworkAppClient;
export type CourseAppSdkClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

interface CourseCapabilitySdkPorts {
  getCourseClient: () => unknown;
  readHostSession: () => unknown;
  subscribeHostSession?: (listener: () => void) => () => void;
  resolveHostLanguage?: () => string;
  subscribeHostLanguage?: (listener: (language: string) => void) => () => void;
}

let courseAppSdkClient: CourseAppSdkClient | null = null;
let coursePcRuntimeBootstrapped = false;
let courseSessionListenerRegistered = false;
let imCoursePcPorts: CourseCapabilitySdkPorts | null = null;

export function createCourseAppSdkClientConfig(
  session?: SdkworkChatSession | null,
): CourseAppSdkClientConfig {
  const currentSession = session ?? readAppSdkSessionTokens();
  return {
    baseUrl: resolveAppSdkBaseUrl(),
    accessToken: resolveAppSdkAccessToken(currentSession),
    authToken: resolveAppSdkAuthToken(currentSession),
    interceptors: createSdkworkChatRequestContextInterceptors(() => readAppSdkSessionTokens() ?? currentSession),
    platform: 'pc',
    tokenManager: getSdkworkChatGlobalTokenManager(),
  };
}

export function initCourseAppSdkClient(
  config: CourseAppSdkClientConfig = createCourseAppSdkClientConfig(),
): CourseAppSdkClient {
  courseAppSdkClient = createClient(config);
  return courseAppSdkClient;
}

export function getCourseAppSdkClient(): CourseAppSdkClient {
  return courseAppSdkClient ?? initCourseAppSdkClient();
}

export function getCourseAppSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): CourseAppSdkClient {
  return initCourseAppSdkClient(createCourseAppSdkClientConfig(session));
}

export function resetCourseAppSdkClient(): void {
  courseAppSdkClient = null;
}

export function syncImSessionToCoursePc(session = readAppSdkSessionTokens()): void {
  if (!session?.authToken || !session.accessToken) {
    resetCourseAppSdkClient();
    return;
  }

  resetCourseAppSdkClient();
  void resolveAppSdkBaseUrl();
}

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
  syncImSessionToCoursePc();
  const { configureCoursePcRuntime } = await import('@sdkwork/course-pc-course');
  ensureCoursePcRuntimeOnModule(configureCoursePcRuntime as CoursePcRuntimeConfigurator);

  if (!courseSessionListenerRegistered && typeof window !== 'undefined') {
    window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, () => {
      syncImSessionToCoursePc();
    });
    courseSessionListenerRegistered = true;
  }
}

export async function rebootstrapCoursePcRuntimeForIm(): Promise<void> {
  syncImSessionToCoursePc();
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

export function resetCoursePcIntegration(): void {
  coursePcRuntimeBootstrapped = false;
  courseSessionListenerRegistered = false;
  imCoursePcPorts = null;
  resetCourseAppSdkClient();
}
