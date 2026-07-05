#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';

const appRoot = path.resolve(import.meta.dirname, '..');
const repoRoot = path.resolve(appRoot, '..', '..');
const courseRepoRoot = path.resolve(repoRoot, '..', 'sdkwork-course');

function readText(...segments) {
  return fs.readFileSync(path.join(appRoot, ...segments), 'utf8');
}

function readJson(...segments) {
  return JSON.parse(readText(...segments));
}

function readRepoText(...segments) {
  return fs.readFileSync(path.join(repoRoot, ...segments), 'utf8');
}

function readCourseText(...segments) {
  return fs.readFileSync(path.join(courseRepoRoot, ...segments), 'utf8');
}

const packageJson = readJson('packages', 'sdkwork-im-pc-core', 'package.json');
const tsconfig = readJson('tsconfig.json');
const viteConfigSource = readText('vite.config.ts');
const pnpmWorkspaceSource = readRepoText('pnpm-workspace.yaml');
const courseBackendIntegrationSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'courseBackendPcIntegration.ts',
);
const courseBackendShimSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'courseBackendSdkClient.ts',
);
const consoleLayoutSource = readText(
  'packages',
  'sdkwork-im-console-core',
  'src',
  'ConsoleLayout.tsx',
);
const consoleCourseSource = readCourseText(
  'apps',
  'sdkwork-course-pc',
  'packages',
  'sdkwork-course-pc-console',
  'src',
  'ConsoleCourse.tsx',
);
const consoleCourseServiceSource = readCourseText(
  'apps',
  'sdkwork-course-pc',
  'packages',
  'sdkwork-course-pc-console',
  'src',
  'services',
  'CourseConsoleService.ts',
);
const courseConsoleHostSource = readCourseText(
  'apps',
  'sdkwork-course-pc',
  'packages',
  'sdkwork-course-pc-console',
  'src',
  'courseConsoleHost.ts',
);
const courseConsoleBootstrapSource = readText('src', 'bootstrap', 'courseConsolePc.ts');
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
  'Chat PC core must consume sdkwork-course through the workspace backend SDK package.',
);

assert.match(
  pnpmWorkspaceSource,
  /sdkwork-course-pc-console/u,
  'pnpm workspace must register sdkwork-course-pc-console package.',
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

assert.match(
  viteConfigSource,
  /@sdkwork\/course-pc-console/u,
  'Vite must alias @sdkwork/course-pc-console to canonical course console package.',
);

assert.deepEqual(
  tsconfig.compilerOptions?.paths?.['@sdkwork/course-pc-console'],
  [
    '../../../sdkwork-course/apps/sdkwork-course-pc/packages/sdkwork-course-pc-console/src/index.ts',
  ],
  'tsconfig must map @sdkwork/course-pc-console for console integration.',
);

assert.match(
  courseBackendIntegrationSource,
  /from ['"]@sdkwork\/course-backend-sdk['"]/u,
  'Course backend PC integration must import the composed course backend SDK package.',
);

assert.match(
  courseBackendIntegrationSource,
  /tokenManager:\s*getSdkworkChatGlobalTokenManager\(\)/u,
  'Course backend PC integration must share the Sdkwork IM global token manager.',
);

assert.doesNotMatch(
  courseBackendIntegrationSource,
  /fetch\(|axios|Authorization|Access-Token/u,
  'Course backend PC integration must not assemble raw HTTP or auth headers.',
);

assert.match(
  courseBackendShimSource,
  /from '\.\/courseBackendPcIntegration'/u,
  'Legacy courseBackendSdkClient path must re-export courseBackendPcIntegration.',
);

assert.match(
  consoleLayoutSource,
  /from '@sdkwork\/course-pc-console'/u,
  'IM console layout must consume canonical course console package.',
);

assert.doesNotMatch(
  consoleLayoutSource,
  /\.\/ConsoleCourse/u,
  'IM console layout must not keep local ConsoleCourse implementation.',
);

assert.match(
  consoleCourseServiceSource,
  /getCourseConsolePcHost\(\)\.getBackendClientWithSession/u,
  'Course console service must consume the IM-wired backend SDK host port.',
);

assert.match(
  consoleCourseServiceSource,
  /client\.courses\.(list|create|publish|unpublish)/u,
  'Course console service must call generated backend course mutations.',
);

assert.match(
  consoleCourseServiceSource,
  /client\.courseCategories\.(list|create)/u,
  'Course console service must call generated backend category mutations.',
);

assert.match(
  consoleCourseServiceSource,
  /client\.courseSections\.(list|create)/u,
  'Course console service must call generated backend section mutations.',
);

assert.match(
  consoleCourseServiceSource,
  /client\.courseLessons\.(list|create)/u,
  'Course console service must call generated backend lesson mutations.',
);

assert.match(
  consoleCourseSource,
  /courseConsoleService\.createCourse/u,
  'Course console surface must expose course creation.',
);

assert.match(
  consoleCourseSource,
  /courseConsoleService\.publishCourse/u,
  'Course console surface must expose course publish workflow.',
);

assert.match(
  consoleCourseSource,
  /courseConsoleService\.createCategory/u,
  'Course console surface must expose category creation.',
);

assert.match(
  consoleCourseSource,
  /courseConsoleService\.createSection/u,
  'Course console surface must expose section creation.',
);

assert.match(
  consoleCourseSource,
  /courseConsoleService\.createLesson/u,
  'Course console surface must expose lesson creation.',
);

assert.match(
  consoleCourseSource,
  /onClick=\{\(\) => setShowCreateForm[\s\S]*?创建课程/u,
  'Course console header action must expose enabled create workflow.',
);

assert.match(
  courseConsoleHostSource,
  /configureCourseConsolePcHost/u,
  'Course console package must expose host port configuration.',
);

assert.match(
  courseConsoleBootstrapSource,
  /bootstrapImCourseConsolePcIntegration/u,
  'IM bootstrap must wire course console host ports.',
);

assert.match(
  courseConsoleBootstrapSource,
  /getCourseBackendSdkClientWithSession/u,
  'IM bootstrap must inject session-scoped course backend SDK client.',
);

assert.match(
  appAuthRuntimeSource,
  /resetCourseBackendPcIntegration/u,
  'IM auth runtime must reset course backend integration on session changes.',
);

assert.match(
  appAuthRuntimeSource,
  /syncImSessionToCourseBackendPc/u,
  'IM auth runtime must sync course backend session on IAM session changes.',
);

console.log('sdkwork im course backend SDK integration contract passed.');
