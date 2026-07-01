#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';

const appRoot = path.resolve(import.meta.dirname, '..');
const repoRoot = path.resolve(appRoot, '..', '..');

function readText(...segments) {
  return fs.readFileSync(path.join(appRoot, ...segments), 'utf8');
}

function readJson(...segments) {
  return JSON.parse(readText(...segments));
}

function readRepoText(...segments) {
  return fs.readFileSync(path.join(repoRoot, ...segments), 'utf8');
}

const packageJson = readJson('package.json');
const tsconfig = readJson('tsconfig.json');
const viteConfigSource = readText('vite.config.ts');
const pnpmWorkspaceSource = readRepoText('pnpm-workspace.yaml');
const courseBackendClientSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'courseBackendSdkClient.ts',
);
const consoleCourseSource = readText(
  'packages',
  'sdkwork-im-console-core',
  'src',
  'ConsoleCourse.tsx',
);
const consoleCourseServiceSource = readText(
  'packages',
  'sdkwork-im-console-core',
  'src',
  'services',
  'CourseConsoleService.ts',
);
const appAuthRuntimeSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'appAuthRuntime.ts',
);

assert.equal(
  packageJson.dependencies?.['@sdkwork/course-backend-sdk'],
  'workspace:*',
  'Chat PC must consume sdkwork-course through the workspace backend SDK package.',
);

assert.match(
  pnpmWorkspaceSource,
  /sdkwork-course-backend-sdk\/sdkwork-course-backend-sdk-typescript\/generated\/server-openapi/u,
  'pnpm workspace must register sdkwork-course-backend-sdk generated transport.',
);

assert.match(
  viteConfigSource,
  /@sdkwork\/course-backend-sdk/u,
  'Vite must alias @sdkwork/course-backend-sdk to generated course backend transport.',
);

assert.deepEqual(
  tsconfig.compilerOptions?.paths?.['@sdkwork/course-backend-sdk'],
  [
    '../../../sdkwork-course/sdks/sdkwork-course-backend-sdk/sdkwork-course-backend-sdk-typescript/generated/server-openapi/src/index.ts',
  ],
  'tsconfig must map @sdkwork/course-backend-sdk for console integration.',
);

assert.match(
  courseBackendClientSource,
  /from ['"]@sdkwork\/course-backend-sdk['"]/u,
  'Core course backend client must import the composed course backend SDK package.',
);

assert.match(
  courseBackendClientSource,
  /tokenManager:\s*getSdkworkChatGlobalTokenManager\(\)/u,
  'Core course backend client must share the Sdkwork IM global token manager.',
);

assert.doesNotMatch(
  courseBackendClientSource,
  /fetch\(|axios|Authorization|Access-Token/u,
  'Core course backend client must not assemble raw HTTP or auth headers.',
);

assert.match(
  consoleCourseServiceSource,
  /getCourseBackendSdkClientWithSession/u,
  'Console course service must consume the composed backend SDK client.',
);

assert.match(
  consoleCourseServiceSource,
  /client\.courses\.(list|create|publish|unpublish)/u,
  'Console course service must call generated backend course mutations.',
);

assert.match(
  consoleCourseServiceSource,
  /client\.courseCategories\.(list|create)/u,
  'Console course service must call generated backend category mutations.',
);

assert.match(
  consoleCourseServiceSource,
  /client\.courseSections\.(list|create)/u,
  'Console course service must call generated backend section mutations.',
);

assert.match(
  consoleCourseServiceSource,
  /client\.courseLessons\.(list|create)/u,
  'Console course service must call generated backend lesson mutations.',
);

assert.match(
  consoleCourseSource,
  /courseConsoleService\.createCourse/u,
  'Console course surface must expose course creation.',
);

assert.match(
  consoleCourseSource,
  /courseConsoleService\.publishCourse/u,
  'Console course surface must expose course publish workflow.',
);

assert.match(
  consoleCourseSource,
  /courseConsoleService\.createCategory/u,
  'Console course surface must expose category creation.',
);

assert.match(
  consoleCourseSource,
  /courseConsoleService\.createSection/u,
  'Console course surface must expose section creation.',
);

assert.match(
  consoleCourseSource,
  /courseConsoleService\.createLesson/u,
  'Console course surface must expose lesson creation.',
);

assert.match(
  consoleCourseSource,
  /onClick=\{\(\) => setShowCreateForm[\s\S]*?创建课程/u,
  'Console course header action must expose enabled create workflow.',
);

assert.match(
  appAuthRuntimeSource,
  /resetCourseBackendSdkClient/u,
  'IM auth runtime must reset course backend SDK client on session changes.',
);

console.log('sdkwork im course backend SDK integration contract passed.');
