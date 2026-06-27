import { spawn } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const currentDir = path.dirname(fileURLToPath(import.meta.url));
const docsRoot = path.resolve(currentDir, "..");
const cliPath = path.join(docsRoot, "node_modules", "vitepress", "dist", "node", "cli.js");
const esbuildPath = path.join(
  docsRoot,
  "node_modules",
  "@esbuild",
  "win32-x64",
  "esbuild.exe",
);
const esbuildPackagePath = path.join(docsRoot, "node_modules", "esbuild", "package.json");
const esbuildVersion = JSON.parse(fs.readFileSync(esbuildPackagePath, "utf8")).version;

async function assertSpawnAvailable(command) {
  const describeBlockedSpawn = () => {
    console.error(
      [
        `Cannot run \`vitepress ${command}\` in this environment.`,
        "",
        "Reason:",
        "- Node child process spawning is blocked with `EPERM`.",
        "- VitePress uses Vite + esbuild to load `.vitepress/config.ts` or `.vitepress/config.mjs`, and that path requires spawning a subprocess.",
        "",
        "What still works here:",
        "- `npm run docs:verify`",
        "- `node ./scripts/generate-operation-pages.mjs`",
        "",
        "What to do next:",
        "- Run `npm run docs:build` in a local shell or CI environment that allows Node subprocess execution.",
      ].join("\n"),
    );
    process.exit(1);
  };

  let child;
  try {
    child = spawn(esbuildPath, [`--service=${esbuildVersion}`, "--ping"], {
      cwd: docsRoot,
      stdio: ["pipe", "pipe", "inherit"],
      windowsHide: true,
    });
  } catch (error) {
    if (error?.code === "EPERM") {
      describeBlockedSpawn();
    }

    console.error(`Failed to probe subprocess execution: ${error.message}`);
    process.exit(1);
  }

  await new Promise((resolve, reject) => {
    child.once("error", reject);
    child.once("spawn", () => {
      child.stdin.destroy();
      child.stdout.destroy();
      child.kill();
      resolve();
    });
  }).catch((error) => {
    if (error?.code === "EPERM") {
      describeBlockedSpawn();
    }

    console.error(`Failed to probe subprocess execution: ${error.message}`);
    process.exit(1);
  });
}

const [command, ...args] = process.argv.slice(2);

if (!command) {
  console.error("Usage: node ./scripts/run-vitepress.mjs <dev|build|preview> [args...]");
  process.exit(1);
}

await assertSpawnAvailable(command);

process.argv = [process.execPath, cliPath, command, ...args];
await import(pathToFileURL(cliPath).href);
