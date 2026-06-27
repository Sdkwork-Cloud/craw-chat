import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const currentDir = path.dirname(fileURLToPath(import.meta.url));
const docsRoot = path.resolve(currentDir, "..");
const repoRoot = path.resolve(docsRoot, "..", "..");
const issues = [];

function marker(...parts) {
  return parts.join("");
}

const retiredPublicSdkMarkers = [
  marker("sdkwork", "-control", "-plane", "-sdk"),
  marker("sdkwork", "-im", "-admin", "-sdk"),
  marker("@sdkwork", "/control", "-plane", "-sdk"),
  marker("@sdkwork", "/im", "-admin", "-sdk"),
  marker("control", "_plane", "_sdk"),
  marker("im", "_admin", "_sdk"),
  marker("/sdk", "/control", "-plane", "-sdk"),
  marker("/sdk", "/control", "-plane", "-typescript", "-sdk"),
  marker("/sdk", "/control", "-plane", "-flutter", "-sdk"),
  marker("/sdk", "/im", "-admin", "-sdk"),
  marker("Control", "-Plane", " SDK"),
  marker("IM", " Admin", " SDK"),
  "sdkwork-sdkwork-im-sdk-management",
  "/sdk/management-sdk",
];

const forbiddenGeneratedTypeScriptPackageMarkers = [
  "@sdkwork-internal/im-sdk-generated",
  "@sdkwork/im-sdk-generated",
];

function read(relativePath) {
  return fs.readFileSync(path.join(docsRoot, relativePath), "utf8");
}

function readRequired(relativePath) {
  const absolutePath = path.join(docsRoot, relativePath);
  if (!fs.existsSync(absolutePath)) {
    issues.push(`${relativePath}: file is required`);
    return "";
  }
  return fs.readFileSync(absolutePath, "utf8");
}

function requireIncludes(source, relativePath, needle, message) {
  if (!source.includes(needle)) {
    issues.push(`${relativePath}: ${message}`);
  }
}

function requireExcludes(source, relativePath, needle, message) {
  if (source.includes(needle)) {
    issues.push(`${relativePath}: ${message}`);
  }
}

function collectMarkdownFiles(root) {
  const files = [];
  for (const entry of fs.readdirSync(root, { withFileTypes: true })) {
    const entryPath = path.join(root, entry.name);
    if (entry.isDirectory()) {
      if (entry.name === "node_modules" || entry.name === ".vitepress") {
        continue;
      }
      files.push(...collectMarkdownFiles(entryPath));
      continue;
    }
    if (entry.isFile() && entry.name.endsWith(".md")) {
      files.push(entryPath);
    }
  }
  return files;
}

for (const removedPage of [
  "sdk/control-plane-sdk.md",
  "sdk/control-plane-typescript-sdk.md",
  "sdk/control-plane-flutter-sdk.md",
  "sdk/im-admin-sdk.md",
  "sdk/management-sdk.md",
]) {
  if (fs.existsSync(path.join(docsRoot, removedPage))) {
    issues.push(`${removedPage}: retired SDK page must not exist in current public docs`);
  }
}

for (const markdownFile of collectMarkdownFiles(docsRoot)) {
  const source = fs.readFileSync(markdownFile, "utf8");
  const relativePath = path.relative(docsRoot, markdownFile).replaceAll("\\", "/");
  for (const marker of retiredPublicSdkMarkers) {
    requireExcludes(
      source,
      relativePath,
      marker,
      `must not publish retired SDK marker ${marker}`,
    );
  }
}

const sdkIndexPath = "sdk/index.md";
const sdkIndexSource = readRequired(sdkIndexPath);
for (const marker of [
  "sdkwork-im-sdk",
  "sdkwork-im-app-sdk",
  "sdkwork-im-backend-sdk",
  "sdkwork-rtc-sdk",
  "/im/v3/api/*",
  "/app/v3/api/*",
  "/backend/v3/api/*",
  "/backend/v3/api/control/*",
  "/backend/v3/api/admin/*",
  "Control-plane and admin APIs are backend modules",
  "[Backend SDK](/sdk/backend-sdk)",
  "[RTC SDK](/sdk/rtc-sdk)",
]) {
  requireIncludes(sdkIndexSource, sdkIndexPath, marker, `must include ${marker}`);
}

const appSdkPath = "sdk/app-sdk.md";
const appSdkSource = readRequired(appSdkPath);
for (const marker of [
  "sdkwork-im-app-sdk",
  "/app/v3/api",
  "SdkworkAppClient",
  "sdkwork-im-app-api.openapi.yaml",
  "must not contain backend, admin, or control routes",
]) {
  requireIncludes(appSdkSource, appSdkPath, marker, `must include ${marker}`);
}
for (const marker of ["/backend/v3/api/*`; use `sdkwork-im-backend-sdk`", "/backend/v3/api/control/*", "/backend/v3/api/admin/*"]) {
  requireIncludes(appSdkSource, appSdkPath, marker, `must route ${marker} away from app SDK`);
}

const backendSdkPath = "sdk/backend-sdk.md";
const backendSdkSource = readRequired(backendSdkPath);
for (const marker of [
  "sdkwork-im-backend-sdk",
  "/backend/v3/api",
  "SdkworkBackendClient",
  "/backend/v3/api/control/*",
  "/backend/v3/api/admin/*",
  "Do not introduce a new admin SDK family",
]) {
  requireIncludes(backendSdkSource, backendSdkPath, marker, `must include ${marker}`);
}

const rtcSdkPath = "sdk/rtc-sdk.md";
const rtcSdkSource = readRequired(rtcSdkPath);
for (const marker of [
  "sdkwork-rtc-sdk",
  "not generated from OpenAPI",
  "provider package",
  "native driver",
  "node ../../../../sdkwork-rtc\\sdks\\sdkwork-rtc-sdk\\bin\\verify-sdk.mjs",
]) {
  requireIncludes(rtcSdkSource, rtcSdkPath, marker, `must include ${marker}`);
}

const languageSupportPath = "sdk/language-support.md";
const languageSupportSource = readRequired(languageSupportPath);
for (const marker of [
  "sdkwork-im-sdk",
  "sdkwork-im-app-sdk",
  "sdkwork-im-backend-sdk",
  "sdkwork-rtc-sdk",
  "generated/server-openapi",
  ".sdkwork-assembly.json",
  "SdkworkAppClient",
  "SdkworkBackendClient",
  "There are no current standalone admin or control-plane SDK families",
]) {
  requireIncludes(languageSupportSource, languageSupportPath, marker, `must include ${marker}`);
}

const generatorBoundaryPath = "sdk/generator-boundary.md";
const generatorBoundarySource = readRequired(generatorBoundaryPath);
for (const marker of [
  "generated/server-openapi",
  "composed",
  "/im/v3/openapi.json",
  "/sdk/app-sdk",
  "/sdk/backend-sdk",
  "/sdk/language-support",
]) {
  requireIncludes(generatorBoundarySource, generatorBoundaryPath, marker, `must include ${marker}`);
}

const typescriptDocPath = "sdk/typescript-sdk.md";
const typescriptDocSource = readRequired(typescriptDocPath);
for (const marker of [
  "@sdkwork/im-sdk",
  "ImSdkClient",
  "generated/server-openapi",
  "new ImSdkClient({",
  "sdk.connect(...)",
  "sdk.createTextMessage(...)",
  "sdkwork-drive",
  "ContentPart.drive",
]) {
  requireIncludes(typescriptDocSource, typescriptDocPath, marker, `must include ${marker}`);
}
for (const marker of forbiddenGeneratedTypeScriptPackageMarkers) {
  requireExcludes(
    typescriptDocSource,
    typescriptDocPath,
    marker,
    `must not expose unsupported generated TypeScript package ${marker}`,
  );
}

const flutterDocPath = "sdk/flutter-sdk.md";
const flutterDocSource = readRequired(flutterDocPath);
for (const marker of [
  "im_sdk_generated",
  "ImTransportClient",
  "generated/server-openapi",
  "client.realtime.eventsList",
]) {
  requireIncludes(flutterDocSource, flutterDocPath, marker, `must include ${marker}`);
}

const apiReferenceIndexPath = "api-reference/index.md";
const apiReferenceIndexSource = readRequired(apiReferenceIndexPath);
for (const marker of [
  "`sdkwork-im-sdk` maps to `/im/v3/api`",
  "`sdkwork-im-app-sdk` maps to `/app/v3/api`",
  "`sdkwork-im-backend-sdk` maps to `/backend/v3/api`",
  "`sdkwork-rtc-sdk` maps to provider-runtime integration",
]) {
  requireIncludes(apiReferenceIndexSource, apiReferenceIndexPath, marker, `must include ${marker}`);
}

const controlPlanePath = "api-reference/control-plane-api.md";
const controlPlaneSource = readRequired(controlPlanePath);
for (const marker of [
  "sdkwork-im-backend-sdk",
  "control modules",
  "sdkwork-im-backend-api.openapi.yaml",
  "[Backend SDK](/sdk/backend-sdk)",
]) {
  requireIncludes(controlPlaneSource, controlPlanePath, marker, `must include ${marker}`);
}

const appApiPath = "api-reference/app-api.md";
const appApiSource = readRequired(appApiPath);
for (const marker of [
  "sdkwork-im-app-sdk",
  "/app/v3/api/*",
]) {
  requireIncludes(appApiSource, appApiPath, marker, `must include ${marker}`);
}
for (const marker of ["Device Twin", "IoT Protocol"]) {
  requireExcludes(appApiSource, appApiPath, marker, `must not include retired AIoT-owned domain ${marker}`);
}

const backendApiPath = "api-reference/backend-api.md";
const backendApiSource = readRequired(backendApiPath);
for (const marker of [
  "sdkwork-im-backend-sdk",
  "/backend/v3/api/*",
  "/backend/v3/api/control/*",
  "/backend/v3/api/admin/*",
]) {
  requireIncludes(backendApiSource, backendApiPath, marker, `must include ${marker}`);
}

const cliPath = "reference/cli-and-scripts.md";
const cliSource = readRequired(cliPath);
for (const marker of [
  "materialize-im-v3-openapi-boundaries.mjs",
  "sdkwork-im-app-sdk",
  "sdkwork-im-backend-sdk",
  "sdkwork-rtc-sdk",
  ".sdkwork-assembly.json",
]) {
  requireIncludes(cliSource, cliPath, marker, `must include ${marker}`);
}

const releaseCatalogPath = path.join(
  repoRoot,
  "artifacts",
  "releases",
  "wave-d-2026-04-08",
  "sdk-release-catalog.json",
);
if (fs.existsSync(releaseCatalogPath)) {
  const releaseCatalog = JSON.parse(fs.readFileSync(releaseCatalogPath, "utf8"));
  const audiences = new Set((releaseCatalog.sdkArtifacts ?? []).map((artifact) => artifact.audience));
  for (const audience of ["im", "app", "backend", "rtc"]) {
    if (!audiences.has(audience)) {
      issues.push(`sdk-release-catalog.json: must include ${audience} SDK artifacts`);
    }
  }
  for (const retiredAudience of ["admin", "im-admin"]) {
    if (audiences.has(retiredAudience)) {
      issues.push(`sdk-release-catalog.json: must not include retired ${retiredAudience} artifacts`);
    }
  }
}

if (issues.length > 0) {
  console.error(issues.join("\n"));
  process.exit(1);
}

console.log("Verified SDK documentation contract pages.");
