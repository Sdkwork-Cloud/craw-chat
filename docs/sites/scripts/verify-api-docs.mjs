import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import {
  apiReferenceOperationLinks,
  groupedPages,
  markdownPathFor,
  operationMarkdownPath,
  operationPageLink,
  readOperations,
} from "../.vitepress/api-reference-sidebar.mjs";

const currentDir = path.dirname(fileURLToPath(import.meta.url));
const docsRoot = path.resolve(currentDir, "..");
const issues = [];
const sourceOperationLinks = [];

function marker(...parts) {
  return parts.join("");
}

const forbiddenPublicPortalCredentialMarkers = [
  "ops_demo",
  "Portal#2026",
  "acct_portal_demo",
];

const forbiddenApiPathMarkers = [
  ["/api", "/v1"].join(""),
  ["/auth", "/login"].join(""),
  ["/auth", "/me"].join(""),
  ["/portal", "/auth"].join(""),
  ["/device", "-sessions"].join(""),
  ["/chat", "-runtime"].join(""),
];

const forbiddenMechanicalSessionRenames = [
  "RtcRealtime Presence",
  "StreamRealtime Presence",
  "durableRealtime Presence",
  "Realtime PresenceRequest",
  "Realtime PresenceId",
  "operationId: createRtcRealtime Presence",
  "operationId: inviteRtcRealtime Presence",
  "operationId: acceptRtcRealtime Presence",
  "operationId: rejectRtcRealtime Presence",
  "operationId: endRtcRealtime Presence",
  "operationId: disconnectRealtime Presence",
];

const forbiddenRetiredSdkMarkers = [
  marker("sdkwork", "-control", "-plane", "-sdk"),
  marker("sdkwork", "-im", "-admin", "-sdk"),
  marker("@sdkwork", "/control", "-plane", "-sdk"),
  marker("@sdkwork", "/im", "-admin", "-sdk"),
  marker("/sdk", "/control", "-plane", "-sdk"),
  marker("/sdk", "/control", "-plane", "-typescript", "-sdk"),
  marker("/sdk", "/control", "-plane", "-flutter", "-sdk"),
  marker("/sdk", "/im", "-admin", "-sdk"),
];

const blockPattern =
  /<a id="([^"]+)"><\/a>[\s\S]*?<section class="api-op">([\s\S]*?)(?=<a id="|$)/g;

function read(relativePath) {
  return fs.readFileSync(path.join(docsRoot, relativePath), "utf8");
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
  const entries = fs.readdirSync(root, { withFileTypes: true });
  const files = [];

  for (const entry of entries) {
    if (entry.name === "node_modules") {
      continue;
    }

    const entryPath = path.join(root, entry.name);

    if (entry.isDirectory()) {
      files.push(...collectMarkdownFiles(entryPath));
      continue;
    }

    if (entry.isFile() && path.extname(entry.name) === ".md") {
      files.push(entryPath);
    }
  }

  return files;
}

function sourceFamily(relativePath) {
  if (relativePath.startsWith("api-reference/im/")) {
    return "im";
  }
  if (relativePath.startsWith("api-reference/app/")) {
    return "app";
  }
  if (
    relativePath.startsWith("api-reference/backend/") ||
    relativePath.startsWith("api-reference/control-plane/")
  ) {
    return "backend";
  }
  return "other";
}

function routeAllowedForFamily(relativePath, family, route) {
  if (family === "im") {
    return route.startsWith("/im/v3/api/") || route === "/healthz" || route === "/readyz";
  }
  if (family === "app") {
    return route.startsWith("/app/v3/api/");
  }
  if (family === "backend") {
    return (
      route.startsWith("/backend/v3/api/") ||
      (relativePath === "api-reference/control-plane/protocol.md" && route === "/healthz")
    );
  }
  return true;
}

function verifySdkFamilyMeta(location, relativePath, route, metaText) {
  const family = sourceFamily(relativePath);
  const isProbe = route === "/healthz" || route === "/readyz";

  if (!routeAllowedForFamily(relativePath, family, route)) {
    issues.push(`${location}: route ${route} is outside the ${family} API authority`);
  }

  if (family === "im") {
    if (!isProbe && !metaText.includes("@sdkwork/im-sdk")) {
      issues.push(`${location}: IM Standard API operation must reference @sdkwork/im-sdk`);
    }
    if (metaText.includes("sdkwork-im-app-sdk") || metaText.includes("sdkwork-im-backend-sdk")) {
      issues.push(`${location}: IM Standard API operation must not reference app or backend SDK families`);
    }
  }

  if (family === "app") {
    if (!metaText.includes("sdkwork-im-app-sdk")) {
      issues.push(`${location}: App API operation must reference sdkwork-im-app-sdk`);
    }
    if (metaText.includes("@sdkwork/im-sdk") || metaText.includes("sdkwork-im-backend-sdk")) {
      issues.push(`${location}: App API operation must not reference IM or backend SDK families`);
    }
  }

  if (family === "backend") {
    if (!isProbe && !metaText.includes("sdkwork-im-backend-sdk")) {
      issues.push(`${location}: Backend API operation must reference sdkwork-im-backend-sdk`);
    }
    if (metaText.includes("@sdkwork/im-sdk") || metaText.includes("sdkwork-im-app-sdk")) {
      issues.push(`${location}: Backend API operation must not reference IM or app SDK families`);
    }
  }
}

function verifySourceOperationBlock(filePath, anchor, block) {
  const titleMatch = block.match(/## `([^`]+)`/);
  const title = titleMatch?.[1] ?? anchor;
  const relativePath = path.relative(docsRoot, filePath).replaceAll("\\", "/");
  const location = `${relativePath}#${anchor}`;
  const metaMatch = block.match(/<div class="api-meta-grid">([\s\S]*?)<\/div>\s*(?=\n### )/);
  const route = title.replace(/^[A-Z]+\s+/, "");
  const isPost = title.startsWith("POST ");
  const hasPathParams = title.includes("{");
  const isOpenProbe = route === "/healthz" || route === "/readyz";

  if (!block.includes("operationId:")) {
    issues.push(`${location}: missing operationId`);
  }

  if (!/### Response `\d+`/.test(block)) {
    issues.push(`${location}: missing explicit response section`);
  }

  if (hasPathParams && !block.includes("### Path Parameters")) {
    issues.push(`${location}: missing path parameter table`);
  }

  if (
    isPost &&
    !block.includes("### Request Body") &&
    !/does not require a JSON request body/i.test(block) &&
    !/does not accept a JSON request body/i.test(block)
  ) {
    issues.push(`${location}: missing request body section or explicit no-body note`);
  }

  if (!metaMatch) {
    issues.push(`${location}: missing api-meta-grid`);
  } else {
    const metaText = metaMatch[1];
    for (const label of ["Security", "SDK", "Permission", "Success"]) {
      if (!metaText.includes(`<strong>${label}</strong>`)) {
        issues.push(`${location}: missing api-meta-grid label "${label}"`);
      }
    }

    verifySdkFamilyMeta(location, relativePath, route, metaText);

    if (
      (relativePath.startsWith("api-reference/control-plane/social") ||
        relativePath.startsWith("api-reference/control-plane/social-runtime")) &&
      metaText.includes("Authenticated principal.")
    ) {
      issues.push(
        `${location}: control-plane social docs must use explicit control-plane permissions, not generic authenticated principal text`,
      );
    }

    if (!isOpenProbe && !metaText.includes("SDKWork dual token + AppContext")) {
      issues.push(
        `${location}: public Security metadata must describe SDKWork dual-token validation plus AppContext projection`,
      );
    }
  }

  if (!isOpenProbe && !block.includes("### Error Responses")) {
    issues.push(`${location}: missing error responses section`);
  }
}

for (const group of groupedPages) {
  for (const page of group.pages) {
    const filePath = markdownPathFor(page.link);
    if (!fs.existsSync(filePath)) {
      issues.push(`${page.link}: missing source overview page`);
      continue;
    }

    const content = fs.readFileSync(filePath, "utf8");
    let match;
    while ((match = blockPattern.exec(content))) {
      const [, anchor, block] = match;
      verifySourceOperationBlock(filePath, anchor, block);
      sourceOperationLinks.push(operationPageLink(page.link, anchor));
    }
  }
}

const expectedSidebarLinks = new Set(sourceOperationLinks);
const sidebarLinks = [...apiReferenceOperationLinks];
const sidebarLinkSet = new Set(sidebarLinks);

for (const link of sidebarLinks) {
  const markdownPath = path.join(docsRoot, `${link.replace(/^\//, "")}.md`);
  if (!fs.existsSync(markdownPath)) {
    issues.push(`sidebar ${link}: missing operation markdown page`);
    continue;
  }

  const markdownContent = fs.readFileSync(markdownPath, "utf8");
  if (!markdownContent.includes("<section class=\"api-op")) {
    issues.push(`sidebar ${link}: missing operation section wrapper`);
  }
  if (!/### Response `\d+`/.test(markdownContent)) {
    issues.push(`sidebar ${link}: missing explicit response section`);
  }
  if (
    link.startsWith("/api-reference/operations/im/") &&
    !markdownContent.includes("@sdkwork/im-sdk") &&
    !markdownContent.includes("Direct HTTP probe")
  ) {
    issues.push(`${link}: IM operation page must reference @sdkwork/im-sdk`);
  }
  if (link.startsWith("/api-reference/operations/app/") && !markdownContent.includes("sdkwork-im-app-sdk")) {
    issues.push(`${link}: app operation page must reference sdkwork-im-app-sdk`);
  }
  if (
    (link.startsWith("/api-reference/operations/backend/") ||
      link.startsWith("/api-reference/operations/control-plane/")) &&
    !markdownContent.includes("sdkwork-im-backend-sdk") &&
    !markdownContent.includes("Direct HTTP probe")
  ) {
    issues.push(`${link}: backend operation page must reference sdkwork-im-backend-sdk`);
  }
  for (const legacyAuthMarker of [
    ["/auth", "/login"].join(""),
    ["/auth", "/me"].join(""),
    ["/portal", "/auth"].join(""),
    ["sdk", "auth", "login"].join("."),
    ["sdk", ".auth.me"].join(""),
  ]) {
    if (markdownContent.includes(legacyAuthMarker)) {
      issues.push(`${link}: operation page must not document sdkwork-im-owned identity marker ${legacyAuthMarker}`);
    }
  }
}

for (const link of expectedSidebarLinks) {
  if (!sidebarLinkSet.has(link)) {
    issues.push(`${link}: missing sidebar entry`);
  }
}

for (const group of groupedPages) {
  if (group.text === "Platform API" || group.text === "IoT API") {
    issues.push(`sidebar group ${group.text}: legacy API groups must not be active`);
  }
}

for (const markdownFile of collectMarkdownFiles(docsRoot)) {
  const content = fs.readFileSync(markdownFile, "utf8");
  const relativePath = path.relative(docsRoot, markdownFile).replaceAll("\\", "/");

  for (const marker of forbiddenPublicPortalCredentialMarkers) {
    if (content.includes(marker)) {
      issues.push(`${relativePath}: contains retired public portal credential marker "${marker}"`);
    }
  }
  for (const marker of forbiddenApiPathMarkers) {
    if (content.includes(marker)) {
      issues.push(`${relativePath}: contains retired API path marker "${marker}"`);
    }
  }
  for (const marker of forbiddenMechanicalSessionRenames) {
    if (content.includes(marker)) {
      issues.push(`${relativePath}: contains mechanical session rename residue "${marker}"`);
    }
  }
  for (const marker of forbiddenRetiredSdkMarkers) {
    if (content.includes(marker)) {
      issues.push(`${relativePath}: contains retired public SDK marker "${marker}"`);
    }
  }
}

for (const group of groupedPages) {
  for (const page of group.pages) {
    for (const operation of readOperations(page.link)) {
      const operationPath = operationMarkdownPath(page.link, operation.anchor);
      if (!fs.existsSync(operationPath)) {
        issues.push(`${operationPath}: missing generated operation page`);
        continue;
      }

      const content = fs.readFileSync(operationPath, "utf8");
      if (!content.includes(`# \`${operation.operationTitle}\``)) {
        issues.push(`${operationPath}: missing operation title heading`);
      }
      if (!content.includes(`href="${page.link}"`)) {
        issues.push(`${operationPath}: missing overview backlink`);
      }
    }
  }
}

const sidebarSource = read(".vitepress/api-reference-sidebar.mjs");
requireIncludes(sidebarSource, ".vitepress/api-reference-sidebar.mjs", 'text: "IM Standard API"', "must publish IM Standard API group");
requireIncludes(sidebarSource, ".vitepress/api-reference-sidebar.mjs", 'text: "App API"', "must publish App API group");
requireIncludes(sidebarSource, ".vitepress/api-reference-sidebar.mjs", 'text: "Backend API"', "must publish Backend API group");
requireExcludes(sidebarSource, ".vitepress/api-reference-sidebar.mjs", 'text: "Platform API"', "must not publish standalone Platform API group");
requireExcludes(sidebarSource, ".vitepress/api-reference-sidebar.mjs", 'text: "IoT API"', "must not publish standalone IoT API group");

const indexPath = "api-reference/index.md";
const indexSource = read(indexPath);
for (const marker of ["IM Standard API", "App API", "Backend API"]) {
  requireIncludes(indexSource, indexPath, marker, `must include ${marker}`);
}
requireExcludes(indexSource, indexPath, "Open Platform API overview", "must not publish legacy Platform API overview");
requireExcludes(indexSource, indexPath, "Open IoT API overview", "must not publish legacy IoT API overview");

const imApiPath = "api-reference/im-api.md";
const imApiSource = read(imApiPath);
for (const marker of ["/im/v3/api/*", "sdkwork-im-sdk", "@sdkwork/im-sdk", "im_sdk"]) {
  requireIncludes(imApiSource, imApiPath, marker, `must include ${marker}`);
}
for (const marker of ["sdkwork-im-app-sdk", "sdkwork-im-backend-sdk"]) {
  requireIncludes(imApiSource, imApiPath, marker, `must route ${marker} away from IM Standard API`);
}

const appApiPath = "api-reference/app-api.md";
const appApiSource = read(appApiPath);
for (const marker of [
  "/app/v3/api/*",
  "sdkwork-im-app-sdk",
  "Portal Access",
  "Notifications",
  "Automation",
  "Provider Health",
]) {
  requireIncludes(appApiSource, appApiPath, marker, `must include ${marker}`);
}
for (const marker of ["Device Twin", "IoT Protocol"]) {
  requireExcludes(appApiSource, appApiPath, marker, `must not include retired AIoT-owned domain ${marker}`);
}
for (const marker of ["Conversation Runtime", "Media and Streams", "Realtime Presence"]) {
  requireExcludes(appApiSource, appApiPath, marker, `must not include IM Standard API domain ${marker}`);
}

const backendApiPath = "api-reference/backend-api.md";
const backendApiSource = read(backendApiPath);
for (const marker of [
  "/backend/v3/api/*",
  "sdkwork-im-backend-sdk",
  "/backend/v3/api/control/*",
  "/backend/v3/api/admin/*",
  "no standalone admin SDK family",
  "no standalone",
]) {
  requireIncludes(backendApiSource, backendApiPath, marker, `must include ${marker}`);
}

for (const relativePath of [
  "api-reference/im/conversations.md",
  "api-reference/im/rooms.md",
  "api-reference/im/messages.md",
  "api-reference/im/media.md",
  "api-reference/im/calls.md",
  "api-reference/im/session-and-realtime.md",
  "api-reference/im/streams.md",
  "api-reference/im/membership-and-read-state.md",
]) {
  const source = read(relativePath);
  requireIncludes(source, relativePath, "@sdkwork/im-sdk", "must reference the official TypeScript root package");
  requireIncludes(source, relativePath, "im_sdk", "must reference the official Flutter consumer package");
  requireExcludes(source, relativePath, "sdkwork-im-app-sdk", "must not route IM Standard API through app SDK");
}

for (const relativePath of [
  "api-reference/app/portal-access.md",
  "api-reference/app/notifications.md",
  "api-reference/app/automation.md",
  "api-reference/app/provider-health.md",
]) {
  const source = read(relativePath);
  requireIncludes(source, relativePath, "sdkwork-im-app-sdk", "must use the app SDK family");
  requireExcludes(source, relativePath, "@sdkwork/im-sdk", "must not route App API through IM SDK");
  requireExcludes(source, relativePath, "/im/v3/api/", "must not document IM Standard API routes");
}

const portalAccessPath = "api-reference/app/portal-access.md";
const portalAccessSource = read(portalAccessPath);
requireIncludes(portalAccessSource, portalAccessPath, "/app/v3/api/portal/access", "must document app portal path");
requireIncludes(portalAccessSource, portalAccessPath, "client.portal.access.retrieve()", "must document generated app portal method");
requireExcludes(portalAccessSource, portalAccessPath, "sdk.portal.getAccess()", "must not document retired IM portal helper");

const automationPath = "api-reference/app/automation.md";
const automationSource = read(automationPath);
for (const marker of [
  "/app/v3/api/automation/agent_responses",
  "/app/v3/api/automation/agent_responses/{streamId}/frames",
  "/app/v3/api/automation/agent_responses/{streamId}/complete",
  "/app/v3/api/automation/agent_tool_calls",
  "/app/v3/api/automation/executions/{executionId}/agent_tool_calls/{toolCallId}/complete",
  "client.automation.agentResponses.create(body)",
  "client.automation.agentToolCalls.complete(executionId, toolCallId, body)",
]) {
  requireIncludes(automationSource, automationPath, marker, `must include ${marker}`);
}

for (const [relativePath, requiredSdkLabel] of [
  ["api-reference/backend/audit.md", "sdkwork-im-backend-sdk"],
  ["api-reference/backend/ops.md", "sdkwork-im-backend-sdk"],
  ["api-reference/control-plane/social.md", "sdkwork-im-backend-sdk"],
  ["api-reference/control-plane/social-runtime.md", "sdkwork-im-backend-sdk"],
  ["api-reference/control-plane/protocol.md", "sdkwork-im-backend-sdk"],
  ["api-reference/control-plane/providers.md", "sdkwork-im-backend-sdk"],
  ["api-reference/control-plane/nodes.md", "sdkwork-im-backend-sdk"],
]) {
  const source = read(relativePath);
  requireIncludes(source, relativePath, requiredSdkLabel, "must use the backend SDK family");
  requireExcludes(source, relativePath, "`sdkwork-im-sdk`", "must not claim the IM SDK family");
  requireExcludes(source, relativePath, "sdkwork-im-app-sdk", "must not claim the app SDK family");
}

if (issues.length > 0) {
  console.error(issues.join("\n"));
  process.exit(1);
}

console.log(
  `Verified ${groupedPages.reduce((sum, group) => sum + group.pages.length, 0)} source API pages, ${sourceOperationLinks.length} operation pages, and ${sidebarLinks.length} sidebar entries.`,
);
