import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";
// Load the app vite config, override cssMinify, and build
const mod = await import(resolve("vite.config.ts").replace(/\/g, "/"));
const base = mod.default || mod;
