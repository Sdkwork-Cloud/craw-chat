import type { SdkworkAppClient } from '@sdkwork/course-app-sdk';
import { configureCoursePcRuntime, type CoursePcSdkPorts } from '@sdkwork/course-pc-course';
import { createImPcHostLanguageBridge } from '@sdkwork/im-pc-commons';

import { getCourseAppSdkClient } from './courseAppSdkClient';
import {
  readAppSdkSessionTokens,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  type SdkworkChatSession,
} from './session';

let coursePcRuntimeBootstrapped = false;
let imCoursePcPorts: CoursePcSdkPorts | null = null;

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

function createImCoursePcSdkPorts(): CoursePcSdkPorts {
  const hostLanguageBridge = createImPcHostLanguageBridge();
  return {
    getCourseClient: getCourseAppSdkClient as () => SdkworkAppClient,
    readHostSession: () => mapImSessionToCourseSnapshot(readAppSdkSessionTokens()),
    subscribeHostSession(listener) {
      const handler = () => listener();
      window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, handler);
      return () => window.removeEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, handler);
    },
    resolveHostLanguage: hostLanguageBridge.resolveInitialLanguage,
    subscribeHostLanguage: hostLanguageBridge.onLanguageChange,
  };
}

export function bootstrapCoursePcForIm(): void {
  imCoursePcPorts = createImCoursePcSdkPorts();
  configureCoursePcRuntime({
    sdkPorts: imCoursePcPorts,
  });
  coursePcRuntimeBootstrapped = true;
}

export function rebootstrapCoursePcRuntimeForIm(): void {
  if (!imCoursePcPorts) {
    return;
  }
  configureCoursePcRuntime({
    sdkPorts: imCoursePcPorts,
  });
}

export function isCoursePcRuntimeBootstrapped(): boolean {
  return coursePcRuntimeBootstrapped;
}

export function resetCoursePcRuntime(): void {
  coursePcRuntimeBootstrapped = false;
}
