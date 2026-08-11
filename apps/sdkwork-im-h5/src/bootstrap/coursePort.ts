/**
 * IM H5 course (课程) runtime port wiring.
 *
 * Binds the shared `@sdkwork/course-mobile-react-courses` package to the IM
 * host through `configureCourseRuntimePort` with the generated Course App
 * SDK port constructed from the IM gateway base URL and the shared H5 token
 * manager. The canonical package owns the UI and the course domain stays in
 * sdkwork-course; the IM host only injects the port (same pattern as the
 * community circles integration).
 */

import { configureCourseRuntimePort } from '@sdkwork/course-mobile-react-courses';
import { getSdkClients } from './sdkClients';

let bootstrapped = false;

export function bootstrapImCourseH5Port(): void {
  if (bootstrapped) {
    return;
  }
  bootstrapped = true;
  configureCourseRuntimePort(getSdkClients().courseAppSdkPort);
}

export function isImCourseH5PortBootstrapped(): boolean {
  return bootstrapped;
}
