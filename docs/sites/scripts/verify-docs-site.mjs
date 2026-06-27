import fs from "node:fs";
import path from "node:path";

const docsRoot = process.cwd();
const repoRoot = path.resolve(docsRoot, "..", "..");
const issues = [];

function marker(...parts) {
  return parts.join("");
}

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

function assertFileExists(relativePath, baseDir = docsRoot) {
  const absolutePath = path.join(baseDir, relativePath);
  if (!fs.existsSync(absolutePath)) {
    issues.push(`${relativePath}: missing file`);
    return null;
  }
  return absolutePath;
}

function assertContains(relativePath, expectedText, baseDir = docsRoot) {
  const absolutePath = assertFileExists(relativePath, baseDir);
  if (!absolutePath) {
    return;
  }

  const content = read(absolutePath);
  if (!content.includes(expectedText)) {
    issues.push(`${relativePath}: missing "${expectedText}"`);
  }
}

function assertContainsInFirstExisting(relativePaths, expectedText, baseDir = docsRoot) {
  for (const relativePath of relativePaths) {
    const absolutePath = path.join(baseDir, relativePath);
    if (!fs.existsSync(absolutePath)) {
      continue;
    }

    const content = read(absolutePath);
    if (!content.includes(expectedText)) {
      issues.push(`${relativePath}: missing "${expectedText}"`);
    }
    return;
  }

  issues.push(`${relativePaths.join(' or ')}: missing file`);
}

function assertDoesNotContain(relativePath, forbiddenText, baseDir = docsRoot) {
  const absolutePath = assertFileExists(relativePath, baseDir);
  if (!absolutePath) {
    return;
  }

  const content = read(absolutePath);
  if (content.includes(forbiddenText)) {
    issues.push(`${relativePath}: must not contain "${forbiddenText}"`);
  }
}

assertContains("reference/admin-storage-contract.md", "/backend/v3/api/admin/storage/providers");
assertContains(
  "reference/admin-storage-contract.md",
  "/backend/v3/api/admin/storage/config/tenants/{tenantId}",
);
assertContains("reference/admin-storage-contract.md", "whole-record override");
assertContains(
  "reference/admin-storage-contract.md",
  "SDKWORK_ADMIN_SANDBOX_STORAGE_FILE",
);
assertContains("reference/admin-storage-contract.md", "StorageConfigUpsertInput");
assertContains("reference/admin-storage-contract.md", "StorageSecretSummaryRecord");
assertContains(
  "reference/admin-storage-contract.md",
  "sdkwork-im-backend-sdk",
);
assertContains(
  "reference/admin-storage-contract.md",
  "must not be split into a standalone admin SDK family",
);
assertDoesNotContain(
  "reference/admin-storage-contract.md",
  marker("sdkwork", "-control", "-plane", "-sdk"),
);
assertDoesNotContain(
  "reference/admin-storage-contract.md",
  marker("sdkwork", "-im", "-admin", "-sdk"),
);

assertContainsInFirstExisting(
  [".vitepress/config.ts", ".vitepress/config.mjs"],
  "/reference/admin-storage-contract",
);
assertContains("index.md", "/reference/admin-storage-contract");
assertContains("architecture/storage-management.md", "/reference/admin-storage-contract");
assertContains("reference/cli-and-scripts.md", "npm run docs:verify");
assertContains("reference/cli-and-scripts.md", "sdkwork-im-sdk");
assertContains("reference/cli-and-scripts.md", "sdkwork-im-app-sdk");
assertContains("reference/cli-and-scripts.md", "sdkwork-im-backend-sdk");
assertContains("reference/cli-and-scripts.md", "sdkwork-rtc-sdk");
assertContains("reference/cli-and-scripts.md", "sdkwork-im-im.sdkgen.yaml");
assertContains("reference/cli-and-scripts.md", "sdkwork-im-im.flutter.sdkgen.yaml");
assertContains(
  "reference/cli-and-scripts.md",
  "node .\\sdks\\sdkwork-im-sdk\\bin\\verify-sdk.mjs",
);
assertContains("reference/cli-and-scripts.md", "prepare-openapi-source.mjs");
assertContains("reference/cli-and-scripts.md", "materialize-im-v3-openapi-boundaries.mjs");
assertContains(
  "reference/cli-and-scripts.md",
  "node .\\sdks\\sdkwork-im-app-sdk\\bin\\verify-sdk.mjs",
);
assertContains(
  "reference/cli-and-scripts.md",
  "node .\\sdks\\sdkwork-im-backend-sdk\\bin\\verify-sdk.mjs",
);
assertContains(
  "reference/cli-and-scripts.md",
  "node .\\sdks\\sdkwork-rtc-sdk\\bin\\verify-sdk.mjs",
);
assertContains("reference/cli-and-scripts.md", ".sdkwork-assembly.json");
assertContains("reference/cli-and-scripts.md", "Do not use a separate admin or control SDK family.");
assertDoesNotContain(
  "reference/cli-and-scripts.md",
  marker("sdkwork", "-control", "-plane", "-sdk"),
);
assertDoesNotContain(
  "reference/cli-and-scripts.md",
  marker("sdkwork", "-im", "-admin", "-sdk"),
);

const adminReadmePath = path.join(repoRoot, "apps/sdkwork-im-admin/README.md");
if (fs.existsSync(adminReadmePath)) {
  assertContains(
    "apps/sdkwork-im-admin/README.md",
    "../../docs/sites/reference/admin-storage-contract.md",
    repoRoot,
  );
}

if (issues.length > 0) {
  console.error(issues.join("\n"));
  process.exit(1);
}

console.log(
  "Verified admin storage reference page, docs navigation links, CLI docs, and optional admin README alignment.",
);
