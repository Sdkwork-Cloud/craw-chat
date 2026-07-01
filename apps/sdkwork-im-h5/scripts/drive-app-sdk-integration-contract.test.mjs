#!/usr/bin/env node

import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const appRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const repoRoot = path.resolve(appRoot, "..", "..");

function readText(...segments) {
  return fs.readFileSync(path.join(appRoot, ...segments), "utf8");
}

function readJson(...segments) {
  return JSON.parse(readText(...segments));
}

function readRepoText(...segments) {
  return fs.readFileSync(path.join(repoRoot, ...segments), "utf8");
}

const packageJson = readJson("package.json");
const corePackageJson = readJson("packages", "sdkwork-im-h5-core", "package.json");
const chatPackageJson = readJson("packages", "sdkwork-im-h5-chat", "package.json");
const viteConfigSource = readText("vite.config.ts");
const driveClientSource = readText(
  "packages",
  "sdkwork-im-h5-core",
  "src",
  "sdk",
  "driveAppSdkClient.ts",
);
const chatUploadSource = readText(
  "packages",
  "sdkwork-im-h5-chat",
  "src",
  "services",
  "chatMediaUploadService.ts",
);
const componentSpec = readRepoText("specs", "component.spec.json");

assert.equal(
  packageJson.scripts?.["test:drive-app-sdk-integration"],
  "node scripts/drive-app-sdk-integration-contract.test.mjs",
  "H5 app root must expose drive app SDK integration contract script.",
);

assert.equal(
  corePackageJson.dependencies?.["@sdkwork/drive-app-sdk"],
  "workspace:*",
  "H5 core must consume sdkwork-drive through the workspace app SDK package.",
);

assert.equal(
  corePackageJson.dependencies?.["@sdkwork/utils"],
  "workspace:*",
  "H5 core must consume @sdkwork/utils for shared utility standardization.",
);

assert.match(
  viteConfigSource,
  /@sdkwork\/drive-app-sdk/u,
  "H5 vite config must alias @sdkwork/drive-app-sdk to the sibling drive app SDK source.",
);

assert.match(
  viteConfigSource,
  /@sdkwork\/utils/u,
  "H5 vite config must alias @sdkwork/utils to the sibling utils TypeScript package.",
);

assert.match(
  driveClientSource,
  /createDriveAppClient/u,
  "H5 drive client must use the sdkwork-drive generated app SDK factory.",
);

assert.match(
  driveClientSource,
  /tokenManager:\s*getImH5GlobalTokenManager\(\)/u,
  "H5 drive client must share the IM H5 global token manager.",
);

assert.doesNotMatch(
  driveClientSource,
  /fetch\(|axios|Authorization|Access-Token/u,
  "H5 drive client must not assemble raw HTTP or auth headers.",
);

assert.match(
  chatUploadSource,
  /getDriveAppSdkClientWithSession/u,
  "H5 chat media upload service must route uploads through the drive app SDK client.",
);

assert.match(
  chatUploadSource,
  /uploadImage|uploadAudio|uploadVideo|uploadAttachment/u,
  "H5 chat media upload service must expose drive uploader profile methods.",
);

assert.match(
  chatUploadSource,
  /const CHAT_DRIVE_APP_RESOURCE_TYPE = "im_conversation"/u,
  "H5 chat media upload must bind files to im_conversation per im-app-api-sdk-integration.spec.md.",
);

assert.match(
  chatUploadSource,
  /const CHAT_DRIVE_SCENE = "im"/u,
  "H5 chat media upload must use scene=im per im-app-api-sdk-integration.spec.md.",
);

assert.match(
  chatUploadSource,
  /const CHAT_DRIVE_SOURCE = "chat_message"/u,
  "H5 chat media upload must tag source=chat_message per im-app-api-sdk-integration.spec.md.",
);

assert.doesNotMatch(
  chatUploadSource,
  /fetch\(|axios|FormData\(\)/u,
  "H5 chat media upload service must not bypass drive with raw multipart HTTP.",
);

const dependencySurface = JSON.parse(componentSpec).contracts?.dependencyApiSurfaces?.find(
  (surface) => surface.apiAuthority === "sdkwork-drive-app-api",
);
assert.ok(dependencySurface, "component.spec.json must declare sdkwork-drive-app-api dependency surface.");

assert.equal(
  chatPackageJson.dependencies?.["@sdkwork/im-h5-core"],
  "workspace:*",
  "H5 chat package must depend on H5 core for drive SDK wiring.",
);

process.stdout.write("sdkwork-im-h5 drive app SDK integration contract passed\n");
