declare module '@sdkwork/course-pc-course' {
  import type { ComponentType } from 'react';

  export interface CoursePcSdkPorts {
    getCourseClient: () => unknown;
    readHostSession: () => unknown;
    subscribeHostSession?: (listener: () => void) => () => void;
    resolveHostLanguage?: () => string;
    subscribeHostLanguage?: (listener: (language: string) => void) => () => void;
  }

  export const CourseView: ComponentType<unknown>;
  export function configureCoursePcRuntime(options: { sdkPorts: CoursePcSdkPorts }): void;
}
