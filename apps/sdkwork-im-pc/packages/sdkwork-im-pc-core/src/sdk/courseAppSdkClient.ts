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
  type SdkworkChatSession,
} from './session';

export type CourseAppSdkClient = SdkworkAppClient;
export type CourseAppSdkClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let courseAppSdkClient: CourseAppSdkClient | null = null;

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
