import {
  ensureCourseBackendPcSessionBridge,
  getCourseBackendSdkClientWithSession,
} from '@sdkwork/im-pc-core/sdk/courseBackendPcIntegration';
import { configureCourseConsolePcHost } from '@sdkwork/course-pc-console';

export function bootstrapImCourseConsolePcIntegration(): void {
  ensureCourseBackendPcSessionBridge();
  configureCourseConsolePcHost({
    getBackendClientWithSession: () => getCourseBackendSdkClientWithSession(),
  });
}
