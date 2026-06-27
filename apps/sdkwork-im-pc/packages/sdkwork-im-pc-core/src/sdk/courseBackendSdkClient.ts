import {
  createClient,
  type SdkworkBackendClient,
  type SdkworkBackendConfig,
} from '@sdkwork/course-backend-sdk';
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

export type CourseBackendSdkClient = SdkworkBackendClient;
export type CourseBackendSdkClientConfig = SdkworkBackendConfig & {
  interceptors?: Interceptors;
};

let courseBackendSdkClient: CourseBackendSdkClient | null = null;

export function createCourseBackendSdkClientConfig(
  session?: SdkworkChatSession | null,
): CourseBackendSdkClientConfig {
  const currentSession = session ?? readAppSdkSessionTokens();
  return {
    baseUrl: resolveAppSdkBaseUrl(),
    accessToken: resolveAppSdkAccessToken(currentSession),
    authToken: resolveAppSdkAuthToken(currentSession),
    interceptors: createSdkworkChatRequestContextInterceptors(
      () => readAppSdkSessionTokens() ?? currentSession,
    ),
    platform: 'pc',
    tokenManager: getSdkworkChatGlobalTokenManager(),
  };
}

export function initCourseBackendSdkClient(
  config: CourseBackendSdkClientConfig = createCourseBackendSdkClientConfig(),
): CourseBackendSdkClient {
  courseBackendSdkClient = createClient(config);
  return courseBackendSdkClient;
}

export function getCourseBackendSdkClient(): CourseBackendSdkClient {
  return courseBackendSdkClient ?? initCourseBackendSdkClient();
}

export function getCourseBackendSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): CourseBackendSdkClient {
  return initCourseBackendSdkClient(createCourseBackendSdkClientConfig(session));
}

export function resetCourseBackendSdkClient(): void {
  courseBackendSdkClient = null;
}
