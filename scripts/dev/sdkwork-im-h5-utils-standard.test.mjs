#!/usr/bin/env node
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..", "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

const corePackage = JSON.parse(read("apps/sdkwork-im-h5/packages/sdkwork-im-h5-core/package.json"));
assert.equal(
  corePackage.dependencies?.["@sdkwork/utils"],
  "workspace:*",
  "apps/sdkwork-im-h5-core must depend on @sdkwork/utils",
);

const viteConfig = read("apps/sdkwork-im-h5/vite.config.ts");
assert.match(
  viteConfig,
  /@sdkwork\/utils/u,
  "apps/sdkwork-im-h5/vite.config.ts must alias @sdkwork/utils",
);

process.stdout.write("sdkwork-im H5 utils standard passed\n");
