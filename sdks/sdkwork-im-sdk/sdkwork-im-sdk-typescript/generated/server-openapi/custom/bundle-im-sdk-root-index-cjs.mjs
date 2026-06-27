#!/usr/bin/env node
import path from 'node:path';
import { stat } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { rollup } from 'rollup';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const inputFile = path.resolve(__dirname, '../../../dist/index.js');
const outputFile = path.resolve(__dirname, '../../../dist/index.cjs');

const relativeExtensionResolver = () => ({
  name: 'relative-extension-resolver',
  async resolveId(source, importer) {
    if (!importer || !source.startsWith('.')) {
      return null;
    }

    const base = path.resolve(path.dirname(importer), source);
    const candidates = [base, `${base}.js`, path.join(base, 'index.js')];

    for (const candidate of candidates) {
      try {
        const s = await stat(candidate);
        if (s.isFile()) {
          return candidate;
        }
      } catch {
        // Try next candidate.
      }
    }

    return null;
  },
});

const bundle = await rollup({
  input: inputFile,
  external: (source) => source.startsWith('@sdkwork/'),
  plugins: [relativeExtensionResolver()],
  onwarn(warning, warn) {
    // Keep compatibility with existing bundling behavior: fail on empty bundle only.
    if (warning.code === 'EMPTY_BUNDLE') {
      throw new Error(warning.message);
    }
    warn(warning);
  },
});

try {
  await bundle.write({
    file: outputFile,
    format: 'cjs',
    exports: 'named',
    interop: 'auto',
    sourcemap: false,
  });
} finally {
  await bundle.close();
}

console.log(`wrote ${outputFile}`);

