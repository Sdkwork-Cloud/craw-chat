#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import { buildReleaseContractReport } from "./verify-server-release-contracts.mjs";

function parseArgs(argv) {
  const options = {
    releaseGatePath: "",
    platform: "all",
    format: "json",
    field: null,
    help: false,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];
    if (argument === "--release-gate-path") {
      options.releaseGatePath = argv[index + 1] ?? "";
      index += 1;
      continue;
    }
    if (argument === "--platform") {
      options.platform = argv[index + 1] ?? "all";
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
      "Usage: node bin/plan-release-server-contracts.mjs --release-gate-path <path> [--platform <all|linux|macos|windows>] [--format <json|text>] [--field <path>]",
      "",
      "Emit the sdkwork-im server release execution plan from the machine-readable release-gate bundle.",
      "",
    ].join("\n"),
  );
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
  return path.resolve(workspaceRoot, contractPath.split("/").join(path.sep));
}

function normalizeObjectArray(value) {
  if (!Array.isArray(value)) {
    return [];
  }
  return value.filter((entry) => entry && typeof entry === "object");
}

function normalizeStringArray(value) {
  if (!Array.isArray(value)) {
    return [];
  }
  return value.filter((entry) => typeof entry === "string");
}

function uniqueStrings(values) {
  return Array.from(new Set(values.filter((entry) => typeof entry === "string")));
}

function toOperatorChecksumCommandExample(checksumCommandExample) {
  if (typeof checksumCommandExample !== "string") {
    return checksumCommandExample ?? null;
  }

  // The release-execution manifest freezes package-line examples, while the emitted
  // operator plan is executed from the per-platform staging root under `artifacts/`.
  return checksumCommandExample.replace(/\.\.\/SHA256SUMS/g, "../../SHA256SUMS");
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
    "sdkwork-im-server release plan",
    `bundle: ${report.bundleId ?? ""}`,
    `wave: ${report.wave ?? ""}`,
    `selectedPlatform: ${report.selectedPlatform ?? ""}`,
    `canonicalBuildCommand: ${report.canonicalBuildCommand ?? ""}`,
    `canonicalStartupCommand: ${report.canonicalStartupCommand ?? ""}`,
    `checksumManifestPath: ${report.checksumManifestPath ?? ""}`,
    `artifactFileListPath: ${report.artifactFileListPath ?? ""}`,
    `packageArtifactCount: ${report.packageArtifactCount ?? 0}`,
    `gateCheckCount: ${report.gateCheckCount ?? 0}`,
    `platformPlanCount: ${report.platformPlanCount ?? 0}`,
    `contractsValid: ${String(report.contractsValid)}`,
  ];

  for (const platformPlan of normalizeObjectArray(report.platformPlans)) {
    lines.push(
      `platform[${platformPlan.platform}]: packages=${normalizeStringArray(
        platformPlan.packageIds,
      ).join(", ")} staging=${platformPlan.stagingRoot} checksum=${
        platformPlan.checksumCommandExample
      }`,
    );
  }

  const semanticIssues = normalizeStringArray(report.semanticIssues);
  const missing = normalizeStringArray(report.missing);
  if (missing.length > 0) {
    lines.push(`missing: ${missing.join(", ")}`);
  }
  if (semanticIssues.length > 0) {
    lines.push(`semanticIssues: ${semanticIssues.join(", ")}`);
  }

  return lines.join("\n");
}

function main() {
  const options = parseArgs(process.argv.slice(2));
  if (options.help) {
    showHelp();
    return;
  }

  if (!options.releaseGatePath) {
    throw new Error("--release-gate-path is required");
  }

  if (!["all", "linux", "macos", "windows"].includes(options.platform)) {
    throw new Error(`Unsupported platform: ${options.platform}`);
  }

  const releaseGateAbsolutePath = path.resolve(options.releaseGatePath);
  const workspaceRoot = resolveReleaseWorkspaceRoot(releaseGateAbsolutePath);
  const releaseGate = JSON.parse(fs.readFileSync(releaseGateAbsolutePath, "utf8"));
  const packageCatalogPath = resolveReleaseContractPath(
    workspaceRoot,
    releaseGate.packageCatalogPath,
  );
  const releaseExecutionPath = resolveReleaseContractPath(
    workspaceRoot,
    releaseGate.releaseExecutionPath,
  );
  const packageCatalog = JSON.parse(fs.readFileSync(packageCatalogPath, "utf8"));
  const releaseExecution = JSON.parse(fs.readFileSync(releaseExecutionPath, "utf8"));

  const verifyReport = buildReleaseContractReport(releaseGateAbsolutePath);

  const selectedPlatformExecutions =
    options.platform === "all"
      ? normalizeObjectArray(releaseExecution.platformExecutions)
      : normalizeObjectArray(releaseExecution.platformExecutions).filter(
          (entry) => entry.platform === options.platform,
        );

  const missing = [...normalizeStringArray(verifyReport.missing)];
  if (options.platform !== "all" && selectedPlatformExecutions.length === 0) {
    missing.push(`platform:${options.platform}`);
  }

  const packageArtifactById = new Map(
    normalizeObjectArray(packageCatalog.packageArtifacts)
      .filter((entry) => typeof entry.id === "string")
      .map((entry) => [entry.id, entry]),
  );

  const platformPlans = selectedPlatformExecutions.map((platformExecution) => {
    const packageIds = normalizeStringArray(platformExecution.packageIds);
    for (const packageId of packageIds) {
      if (!packageArtifactById.has(packageId)) {
        missing.push(`packageArtifact:${packageId}`);
      }
    }

    return {
      platform: platformExecution.platform,
      stagingRoot: platformExecution.stagingRoot,
      stagingReadmePath: platformExecution.stagingReadmePath,
      acceptanceManifestPath: platformExecution.acceptanceManifestPath,
      layoutTreePath: platformExecution.layoutTreePath,
      packageIds,
      checksumCommandExample: toOperatorChecksumCommandExample(
        platformExecution.checksumCommandExample,
      ),
      serviceManager: platformExecution.serviceManager,
      status: platformExecution.status,
    };
  });

  const result = {
    product: "sdkwork-im-server",
    gatePath: releaseGateAbsolutePath,
    bundleId: releaseGate.bundleId,
    wave: releaseGate.wave,
    selectedPlatform: options.platform,
    canonicalBuildCommand: releaseExecution?.canonicalBuild?.command ?? null,
    canonicalStartupCommand: releaseExecution.canonicalStartupCommand ?? null,
    checksumManifestPath: releaseExecution.checksumManifestPath ?? null,
    artifactFileListPath: releaseExecution.artifactFileListPath ?? null,
    packageArtifactCount: normalizeObjectArray(packageCatalog.packageArtifacts).length,
    gateCheckCount: normalizeObjectArray(releaseGate.gateChecks).length,
    platformPlanCount: platformPlans.length,
    contractsValid:
      verifyReport.contractsValid && uniqueStrings(missing).length === 0,
    semanticCheckCount: verifyReport.semanticCheckCount ?? 0,
    semanticIssueCount: verifyReport.semanticIssueCount ?? 0,
    semanticIssues: normalizeStringArray(verifyReport.semanticIssues),
    missing: uniqueStrings(missing),
    platformPlans,
  };

  if (options.field) {
    process.stdout.write(serializeFieldValue(getValueAtPath(result, options.field)));
    return;
  }

  process.stdout.write(
    options.format === "text"
      ? `${buildTextSummary(result)}\n`
      : `${JSON.stringify(result, null, 2)}\n`,
  );
}

try {
  main();
} catch (error) {
  const message = error instanceof Error ? error.message : String(error);
  process.stderr.write(`${message}\n`);
  process.exit(1);
}
