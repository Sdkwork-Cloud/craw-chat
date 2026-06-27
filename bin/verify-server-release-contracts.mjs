#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

function parseArgs(argv) {
  const options = {
    format: "json",
    field: null,
    releaseGatePath: "",
  };

  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];
    if (argument === "--release-gate-path") {
      options.releaseGatePath = argv[index + 1] ?? "";
      index += 1;
      continue;
    }
    if (argument === "--format") {
      options.format = argv[index + 1] ?? "json";
      index += 1;
      continue;
    }
    if (argument === "--field") {
      options.field = argv[index + 1] ?? "";
      index += 1;
      continue;
    }
    if (argument === "-h" || argument === "--help") {
      options.help = true;
      continue;
    }
    throw new Error(`Unknown argument: ${argument}`);
  }

  return options;
}

function showHelp() {
  process.stdout.write(
    [
      "Usage: node bin/verify-server-release-contracts.mjs --release-gate-path <path> [--format <json|text>] [--field <path>]",
      "",
      "Validate sdkwork-im server release-gate bundle consistency across release-gate, release-execution, package-catalog, release-provenance, acceptance manifests, and checklist contracts.",
      "",
    ].join("\n"),
  );
}

function normalizeStringArray(value) {
  if (!Array.isArray(value)) {
    return [];
  }
  return value.filter((entry) => typeof entry === "string");
}

function normalizeObjectArray(value) {
  if (!Array.isArray(value)) {
    return [];
  }
  return value.filter((entry) => entry && typeof entry === "object");
}

function uniqueStrings(values) {
  return Array.from(new Set(normalizeStringArray(values)));
}

function sortStrings(values) {
  return uniqueStrings(values).sort((left, right) => left.localeCompare(right));
}

function sameStringSet(left, right) {
  const normalizedLeft = sortStrings(left);
  const normalizedRight = sortStrings(right);
  if (normalizedLeft.length !== normalizedRight.length) {
    return false;
  }
  return normalizedLeft.every((entry, index) => entry === normalizedRight[index]);
}

function resolveReleaseWorkspaceRoot(gatePath) {
  return path.resolve(path.dirname(gatePath), "../../../..");
}

function resolveReleaseContractPath(workspaceRoot, contractPath) {
  if (!contractPath || typeof contractPath !== "string") {
    return "";
  }
  if (path.isAbsolute(contractPath)) {
    return contractPath;
  }
  const normalizedPath = contractPath.split("/").join(path.sep);
  return path.resolve(workspaceRoot, normalizedPath);
}

function readJsonFile(absolutePath) {
  return JSON.parse(fs.readFileSync(absolutePath, "utf8"));
}

function addUnique(list, value) {
  if (!value || typeof value !== "string") {
    return;
  }
  if (!list.includes(value)) {
    list.push(value);
  }
}

function getValueAtPath(value, fieldPath) {
  return String(fieldPath)
    .split(".")
    .filter(Boolean)
    .reduce((current, segment) => {
      if (current == null) {
        return undefined;
      }
      return current[segment];
    }, value);
}

function serializeFieldValue(value) {
  if (value === undefined) {
    return "";
  }
  if (typeof value === "string") {
    return value;
  }
  return JSON.stringify(value);
}

function buildTextSummary(report) {
  const lines = [
    `releaseGate: ${report.gatePath ?? ""}`,
    `releaseBundle: ${report.bundleId ?? ""}`,
    `releaseDecisionStatus: ${report.decisionStatus ?? ""}`,
    `releasePlatforms: ${normalizeStringArray(report.platforms).join(", ")}`,
    `releaseContractsValid: ${String(report.contractsValid)}`,
    `releaseSemanticIssueCount: ${String(report.semanticIssueCount)}`,
  ];

  if (normalizeStringArray(report.missing).length > 0) {
    lines.push(`releaseMissing: ${normalizeStringArray(report.missing).join(", ")}`);
  }
  if (normalizeStringArray(report.semanticIssues).length > 0) {
    lines.push(
      `releaseSemanticIssues: ${normalizeStringArray(report.semanticIssues).join(", ")}`,
    );
  }

  return lines.join("\n");
}

export function buildReleaseContractReport(releaseGatePath) {
  if (!releaseGatePath) {
    throw new Error("--release-gate-path is required");
  }

  const releaseGateAbsolutePath = path.resolve(releaseGatePath);
  const workspaceRoot = resolveReleaseWorkspaceRoot(releaseGateAbsolutePath);
  const missing = [];
  const semanticIssues = [];
  let semanticCheckCount = 0;

  const check = (condition, code) => {
    semanticCheckCount += 1;
    if (!condition) {
      addUnique(semanticIssues, code);
    }
  };

  const report = {
    enabled: true,
    gatePath: releaseGateAbsolutePath,
    bundleId: null,
    wave: null,
    state: null,
    decisionStatus: null,
    reviewDocCount: 0,
    gateCheckCount: 0,
    platformCount: 0,
    platforms: [],
    packageArtifactCount: 0,
    canonicalStartupCommand: null,
    contractPathCount: 0,
    semanticCheckCount: 0,
    semanticIssueCount: 0,
    semanticIssues: [],
    contractsValid: false,
    missing: [],
  };

  if (!fs.existsSync(releaseGateAbsolutePath)) {
    addUnique(missing, "release-gate.json");
    report.missing = missing;
    return report;
  }

  const releaseGate = readJsonFile(releaseGateAbsolutePath);
  const gateChecks = normalizeObjectArray(releaseGate.gateChecks);
  const platformGateChecks = normalizeObjectArray(releaseGate.platformGateChecks);
  const reviewDocPaths = normalizeStringArray(releaseGate.reviewDocPaths);

  report.bundleId = releaseGate.bundleId ?? null;
  report.wave = releaseGate.wave ?? null;
  report.state = releaseGate.state ?? null;
  report.decisionStatus = releaseGate.decisionStatus ?? null;
  report.reviewDocCount = reviewDocPaths.length;
  report.gateCheckCount = gateChecks.length;
  report.platformCount = platformGateChecks.length;
  report.platforms = platformGateChecks
    .map((entry) => entry.platform)
    .filter((entry) => typeof entry === "string");

  const contractPaths = [];
  const addContractPath = (contractPath) => {
    addUnique(contractPaths, contractPath);
  };

  addContractPath(releaseGate.releaseChecklistPath);
  addContractPath(releaseGate.packageCatalogPath);
  addContractPath(releaseGate.releaseExecutionPath);
  addContractPath(releaseGate.releaseProvenancePath);
  reviewDocPaths.forEach(addContractPath);
  gateChecks.forEach((entry) => addContractPath(entry.contractPath));
  platformGateChecks.forEach((entry) => addContractPath(entry.acceptanceManifestPath));

  const packageCatalogPath = resolveReleaseContractPath(workspaceRoot, releaseGate.packageCatalogPath);
  const releaseExecutionPath = resolveReleaseContractPath(
    workspaceRoot,
    releaseGate.releaseExecutionPath,
  );
  const releaseProvenancePath = resolveReleaseContractPath(
    workspaceRoot,
    releaseGate.releaseProvenancePath,
  );

  let packageCatalog = null;
  let releaseExecution = null;
  let releaseProvenance = null;
  let releaseChecklistContent = "";

  if (packageCatalogPath && fs.existsSync(packageCatalogPath)) {
    packageCatalog = readJsonFile(packageCatalogPath);
  }
  if (releaseExecutionPath && fs.existsSync(releaseExecutionPath)) {
    releaseExecution = readJsonFile(releaseExecutionPath);
  }
  if (releaseProvenancePath && fs.existsSync(releaseProvenancePath)) {
    releaseProvenance = readJsonFile(releaseProvenancePath);
  }

  const checklistAbsolutePath = resolveReleaseContractPath(
    workspaceRoot,
    releaseGate.releaseChecklistPath,
  );
  if (checklistAbsolutePath && fs.existsSync(checklistAbsolutePath)) {
    releaseChecklistContent = fs.readFileSync(checklistAbsolutePath, "utf8");
  }

  const packageArtifacts = packageCatalog
    ? normalizeObjectArray(packageCatalog.packageArtifacts)
    : [];
  const platformExecutions = releaseExecution
    ? normalizeObjectArray(releaseExecution.platformExecutions)
    : [];
  const provenancePlatformRoots = releaseProvenance
    ? normalizeObjectArray(releaseProvenance.platformArtifactRoots)
    : [];

  report.packageArtifactCount = packageArtifacts.length;
  report.canonicalStartupCommand = releaseExecution?.canonicalStartupCommand ?? null;

  if (releaseExecution) {
    addContractPath(releaseExecution.checksumManifestPath);
    addContractPath(releaseExecution.artifactFileListPath);
    platformExecutions.forEach((entry) => {
      addContractPath(entry.stagingReadmePath);
      addContractPath(entry.acceptanceManifestPath);
      addContractPath(entry.layoutTreePath);
    });
  }

  packageArtifacts.forEach((entry) => {
    addContractPath(entry.releaseChecklistPath);
    addContractPath(entry.checksumManifestPath);
    addContractPath(entry.stagingReadmePath);
    addContractPath(entry.layoutTreePath);
    addContractPath(entry.acceptanceManifestPath);
  });

  if (releaseProvenance) {
    normalizeStringArray(releaseProvenance.contractPaths).forEach(addContractPath);
    provenancePlatformRoots.forEach((entry) => addContractPath(entry.acceptanceManifestPath));
  }

  report.contractPathCount = contractPaths.length;

  for (const contractPath of contractPaths) {
    const resolvedPath = resolveReleaseContractPath(workspaceRoot, contractPath);
    if (!fs.existsSync(resolvedPath)) {
      addUnique(missing, contractPath);
    }
  }

  const gateCheckContractPaths = new Set(
    gateChecks
      .map((entry) => entry.contractPath)
      .filter((entry) => typeof entry === "string"),
  );
  const gatePlatformByName = new Map(
    platformGateChecks
      .filter((entry) => typeof entry.platform === "string")
      .map((entry) => [entry.platform, entry]),
  );
  const executionPlatformByName = new Map(
    platformExecutions
      .filter((entry) => typeof entry.platform === "string")
      .map((entry) => [entry.platform, entry]),
  );
  const provenancePlatformByName = new Map(
    provenancePlatformRoots
      .filter((entry) => typeof entry.platform === "string")
      .map((entry) => [entry.platform, entry]),
  );
  const packageArtifactById = new Map(
    packageArtifacts
      .filter((entry) => typeof entry.id === "string")
      .map((entry) => [entry.id, entry]),
  );

  const acceptanceManifestByPlatform = new Map();
  for (const [platform, gatePlatform] of gatePlatformByName.entries()) {
    const manifestPath = resolveReleaseContractPath(
      workspaceRoot,
      gatePlatform.acceptanceManifestPath,
    );
    if (manifestPath && fs.existsSync(manifestPath)) {
      acceptanceManifestByPlatform.set(platform, readJsonFile(manifestPath));
    }
  }

  if (packageCatalog) {
    check(
      packageCatalog.bundleId === releaseGate.bundleId,
      "package-catalog:bundle-id-mismatch",
    );
    check(packageCatalog.wave === releaseGate.wave, "package-catalog:wave-mismatch");
  }

  if (releaseExecution) {
    check(
      releaseExecution.bundleId === releaseGate.bundleId,
      "release-execution:bundle-id-mismatch",
    );
    check(releaseExecution.wave === releaseGate.wave, "release-execution:wave-mismatch");
    check(
      releaseExecution.packageCatalogPath === releaseGate.packageCatalogPath,
      "release-execution:package-catalog-path-mismatch",
    );
    check(
      releaseExecution.releaseChecklistPath === releaseGate.releaseChecklistPath,
      "release-execution:release-checklist-path-mismatch",
    );
    check(
      gateCheckContractPaths.has(releaseExecution.checksumManifestPath),
      "release-gate:missing-checksum-gate-check",
    );
    check(
      gateCheckContractPaths.has(releaseExecution.artifactFileListPath),
      "release-gate:missing-artifact-file-list-gate-check",
    );
  }

  if (releaseProvenance) {
    check(
      releaseProvenance.bundleId === releaseGate.bundleId,
      "release-provenance:bundle-id-mismatch",
    );
    check(
      releaseProvenance.wave === releaseGate.wave,
      "release-provenance:wave-mismatch",
    );
    check(
      releaseProvenance.canonicalBuildCommand === releaseExecution?.canonicalBuild?.command,
      "release-provenance:canonical-build-command-mismatch",
    );
  }

  const checklistContentLower = releaseChecklistContent.toLowerCase();
  if (releaseChecklistContent) {
    check(
      checklistContentLower.includes("artifact-file-list"),
      "release-checklist:missing-artifact-file-list",
    );
    check(
      checklistContentLower.includes("sha256sums"),
      "release-checklist:missing-sha256sums",
    );
    check(
      checklistContentLower.includes("staged artifacts")
        || checklistContentLower.includes("stage platform artifacts"),
      "release-checklist:missing-staged-artifacts-step",
    );
    check(
      checklistContentLower.includes("go / no-go")
        || checklistContentLower.includes("go/no-go"),
      "release-checklist:missing-go-no-go-step",
    );
    check(
      checklistContentLower.includes("acceptance-manifest"),
      "release-checklist:missing-acceptance-manifest",
    );
    check(
      checklistContentLower.includes("startup contract"),
      "release-checklist:missing-startup-contract",
    );
  }

  const gatePlatforms = report.platforms;
  const executionPlatforms = platformExecutions
    .map((entry) => entry.platform)
    .filter((entry) => typeof entry === "string");
  const provenancePlatforms = provenancePlatformRoots
    .map((entry) => entry.platform)
    .filter((entry) => typeof entry === "string");

  if (platformExecutions.length > 0) {
    check(
      sameStringSet(gatePlatforms, executionPlatforms),
      "platforms:gate-execution-mismatch",
    );
  }
  if (provenancePlatformRoots.length > 0) {
    check(
      sameStringSet(gatePlatforms, provenancePlatforms),
      "platforms:gate-provenance-mismatch",
    );
  }

  const expectedPackageIds = sortStrings(
    platformExecutions.flatMap((entry) => normalizeStringArray(entry.packageIds)),
  );
  if (packageArtifacts.length > 0 && expectedPackageIds.length > 0) {
    check(
      sameStringSet(
        packageArtifacts.map((entry) => entry.id),
        expectedPackageIds,
      ),
      "package-catalog:package-ids-mismatch",
    );
  }

  for (const platform of gatePlatforms) {
    const gatePlatform = gatePlatformByName.get(platform);
    const executionPlatform = executionPlatformByName.get(platform);
    const provenancePlatform = provenancePlatformByName.get(platform);
    const acceptanceManifest = acceptanceManifestByPlatform.get(platform);

    check(Boolean(executionPlatform), `platform:${platform}:missing-release-execution`);
    check(Boolean(acceptanceManifest), `platform:${platform}:missing-acceptance-manifest`);

    if (executionPlatform) {
      check(
        executionPlatform.acceptanceManifestPath === gatePlatform.acceptanceManifestPath,
        `platform:${platform}:acceptance-manifest-path-mismatch`,
      );
      check(
        sameStringSet(gatePlatform.requiredPackageIds, executionPlatform.packageIds),
        `platform:${platform}:required-package-ids-mismatch`,
      );
    }

    if (provenancePlatform) {
      check(
        provenancePlatform.acceptanceManifestPath === gatePlatform.acceptanceManifestPath,
        `platform:${platform}:provenance-acceptance-manifest-path-mismatch`,
      );
      if (executionPlatform) {
        check(
          provenancePlatform.artifactRoot === executionPlatform.stagingRoot,
          `platform:${platform}:artifact-root-mismatch`,
        );
      }
    }

    if (!acceptanceManifest) {
      continue;
    }

    check(
      acceptanceManifest.bundleId === releaseGate.bundleId,
      `platform:${platform}:bundle-id-mismatch`,
    );
    check(
      acceptanceManifest.platform === platform,
      `platform:${platform}:acceptance-platform-mismatch`,
    );
    if (executionPlatform) {
      check(
        acceptanceManifest.validationStatus === executionPlatform.status,
        `platform:${platform}:validation-status-mismatch`,
      );
    }

    const packageChecks = normalizeObjectArray(acceptanceManifest.packageChecks);
    const acceptancePackageById = new Map(
      packageChecks
        .filter((entry) => typeof entry.packageId === "string")
        .map((entry) => [entry.packageId, entry]),
    );

    if (executionPlatform) {
      check(
        sameStringSet(
          Array.from(acceptancePackageById.keys()),
          normalizeStringArray(executionPlatform.packageIds),
        ),
        `platform:${platform}:acceptance-package-ids-mismatch`,
      );
    }

    for (const packageId of normalizeStringArray(executionPlatform?.packageIds)) {
      const packageArtifact = packageArtifactById.get(packageId);
      const packageCheck = acceptancePackageById.get(packageId);

      check(Boolean(packageArtifact), `package:${packageId}:missing-package-catalog-entry`);
      check(Boolean(packageCheck), `package:${packageId}:missing-acceptance-entry`);

      if (!packageArtifact || !packageCheck) {
        continue;
      }

      check(
        packageArtifact.platform === platform,
        `package:${packageId}:platform-mismatch`,
      );
      check(
        packageArtifact.serviceManager === executionPlatform.serviceManager,
        `package:${packageId}:service-manager-mismatch`,
      );
      check(
        packageArtifact.startupCommand === releaseExecution?.canonicalStartupCommand,
        `package:${packageId}:startup-command-mismatch`,
      );
      check(
        packageArtifact.releaseChecklistPath === releaseGate.releaseChecklistPath,
        `package:${packageId}:release-checklist-path-mismatch`,
      );
      check(
        packageArtifact.checksumManifestPath === releaseExecution?.checksumManifestPath,
        `package:${packageId}:checksum-manifest-path-mismatch`,
      );
      check(
        packageArtifact.stagingReadmePath === executionPlatform.stagingReadmePath,
        `package:${packageId}:staging-readme-path-mismatch`,
      );
      check(
        packageArtifact.layoutTreePath === executionPlatform.layoutTreePath,
        `package:${packageId}:layout-tree-path-mismatch`,
      );
      check(
        packageArtifact.acceptanceManifestPath === gatePlatform.acceptanceManifestPath,
        `package:${packageId}:acceptance-manifest-path-mismatch`,
      );
      check(
        packageCheck.packageType === packageArtifact.packageType,
        `package:${packageId}:package-type-mismatch`,
      );
      check(
        packageCheck.artifactPath === packageArtifact.artifactPath,
        `package:${packageId}:artifact-path-mismatch`,
      );
      check(
        packageCheck.installModel === packageArtifact.installModel,
        `package:${packageId}:install-model-mismatch`,
      );
      check(
        packageCheck.serviceManager === packageArtifact.serviceManager,
        `package:${packageId}:acceptance-service-manager-mismatch`,
      );
      check(
        packageCheck.startupCommand === packageArtifact.startupCommand,
        `package:${packageId}:acceptance-startup-command-mismatch`,
      );
      check(
        Array.isArray(packageCheck.requiredEntries) && packageCheck.requiredEntries.length > 0,
        `package:${packageId}:required-entries-empty`,
      );
    }
  }

  if (releaseProvenance) {
    const provenanceContractPaths = new Set(normalizeStringArray(releaseProvenance.contractPaths));
    const requiredContractPaths = [
      releaseGate.packageCatalogPath,
      releaseGate.releaseExecutionPath,
      releaseGate.releaseChecklistPath,
      releaseExecution?.checksumManifestPath,
      releaseExecution?.artifactFileListPath,
      ...platformGateChecks.map((entry) => entry.acceptanceManifestPath),
    ].filter((entry) => typeof entry === "string");

    for (const contractPath of requiredContractPaths) {
      check(
        provenanceContractPaths.has(contractPath),
        `release-provenance:missing-contract-path:${contractPath}`,
      );
    }
  }

  report.semanticCheckCount = semanticCheckCount;
  report.semanticIssueCount = semanticIssues.length;
  report.semanticIssues = semanticIssues;
  report.missing = missing;
  report.contractsValid = missing.length === 0 && semanticIssues.length === 0;

  return report;
}

function main() {
  const options = parseArgs(process.argv.slice(2));
  if (options.help) {
    showHelp();
    return;
  }

  const report = buildReleaseContractReport(options.releaseGatePath);

  if (options.field) {
    process.stdout.write(serializeFieldValue(getValueAtPath(report, options.field)));
    return;
  }

  process.stdout.write(
    options.format === "text"
      ? `${buildTextSummary(report)}\n`
      : `${JSON.stringify(report, null, 2)}\n`,
  );
}

const executedPath = process.argv[1] ? path.resolve(process.argv[1]) : "";
const currentModulePath = fileURLToPath(import.meta.url);
if (executedPath === currentModulePath) {
  try {
    main();
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    process.stderr.write(`${message}\n`);
    process.exit(1);
  }
}
