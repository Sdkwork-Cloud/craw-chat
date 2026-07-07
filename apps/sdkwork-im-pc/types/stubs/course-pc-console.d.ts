declare module '@sdkwork/course-pc-console' {
  export interface CourseConsolePcHostConfig {
    getBackendClientWithSession: () => unknown;
  }

  export function configureCourseConsolePcHost(
    config: CourseConsolePcHostConfig,
  ): void;
}
