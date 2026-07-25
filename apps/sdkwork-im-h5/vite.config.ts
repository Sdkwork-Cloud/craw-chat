import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import {defineConfig, loadEnv} from 'vite';

const repoRoot = path.resolve(__dirname, '../..');

function dependencyRoot(dependencyId: string): string {
  return path.resolve(repoRoot, '..', dependencyId);
}

const sdkworkUtilsSourceRoot = path.resolve(
  dependencyRoot('sdkwork-utils'),
  'packages/sdkwork-utils-typescript/src',
);
const sdkworkUtilsEntry = path.resolve(sdkworkUtilsSourceRoot, 'index.ts');

export default defineConfig(({mode}) => {
  const env = loadEnv(mode, '.', '');
  return {
    plugins: [react(), tailwindcss()],
    define: {
// Replaced define to avoid passing server secrets to client
    },
    resolve: {
      alias: [
        { find: /^@\/(.*)/, replacement: path.resolve(__dirname, 'src/$1') },
        { find: /^@sdkwork\/im-h5-(.*)/, replacement: path.resolve(__dirname, 'packages/sdkwork-im-h5-$1/src') },
        { find: /^@sdkwork\/utils\/(.+)$/, replacement: `${sdkworkUtilsSourceRoot}/$1` },
        { find: /^@sdkwork\/utils$/, replacement: sdkworkUtilsEntry },
      ],
    },
    server: {
      // HMR is disabled in automated environments via DISABLE_HMR env var.
      hmr: process.env.DISABLE_HMR !== 'true',
    },
    optimizeDeps: {
      exclude: ['@sdkwork/utils'],
    },
  };
});
