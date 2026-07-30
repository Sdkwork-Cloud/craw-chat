import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import { defineConfig } from 'vite';

const appReactEntry = path.resolve(__dirname, 'node_modules/react/index.js');
const appReactJsxRuntimeEntry = path.resolve(__dirname, 'node_modules/react/jsx-runtime.js');
const appReactJsxDevRuntimeEntry = path.resolve(__dirname, 'node_modules/react/jsx-dev-runtime.js');
const appReactDomEntry = path.resolve(__dirname, 'node_modules/react-dom/index.js');
const appReactDomClientEntry = path.resolve(__dirname, 'node_modules/react-dom/client.js');
const sdkCommonSourceRoot = path.resolve(
  __dirname,
  '../../../sdkwork-sdk-commons/sdkwork-sdk-common-typescript/src',
);
const sdkworkUtilsSourceRoot = path.resolve(
  __dirname,
  '../../../sdkwork-utils/packages/sdkwork-utils-typescript/src',
);

export default defineConfig({
  cacheDir: path.resolve(__dirname, 'node_modules', '.vite', 'sdkwork-im-h5'),
  plugins: [react(), tailwindcss()],
  define: {
    // Replaced define to avoid passing server secrets to client.
  },
  resolve: {
    dedupe: ['react', 'react-dom'],
    alias: [
      { find: 'react/jsx-runtime', replacement: appReactJsxRuntimeEntry },
      { find: 'react/jsx-dev-runtime', replacement: appReactJsxDevRuntimeEntry },
      { find: 'react-dom/client', replacement: appReactDomClientEntry },
      { find: /^react-dom$/, replacement: appReactDomEntry },
      { find: /^react$/, replacement: appReactEntry },
      { find: /^@\/(.*)/, replacement: path.resolve(__dirname, 'src/$1') },
      {
        find: /^@sdkwork\/im-h5-([^/]+)\/(.+)$/,
        replacement: path.resolve(__dirname, 'packages/sdkwork-im-h5-$1/src/$2'),
      },
      {
        find: /^@sdkwork\/im-h5-([^/]+)$/,
        replacement: path.resolve(__dirname, 'packages/sdkwork-im-h5-$1/src'),
      },
      {
        find: /^@sdkwork\/im-app-sdk$/,
        replacement: path.resolve(
          __dirname,
          '../../sdks/sdkwork-im-app-sdk/sdkwork-im-app-sdk-typescript/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/im-backend-sdk$/,
        replacement: path.resolve(
          __dirname,
          '../../sdks/sdkwork-im-backend-sdk/sdkwork-im-backend-sdk-typescript/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/im-sdk$/,
        replacement: path.resolve(
          __dirname,
          '../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/iam-app-sdk$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-iam/sdks/sdkwork-iam-app-sdk/sdkwork-iam-app-sdk-typescript/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/iam-backend-sdk$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-iam/sdks/sdkwork-iam-backend-sdk/sdkwork-iam-backend-sdk-typescript/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/drive-app-sdk$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-drive/sdks/sdkwork-drive-app-sdk/sdkwork-drive-app-sdk-typescript/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/rtc-sdk$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/appbase-pc-react$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-appbase/packages/pc-react/foundation/sdkwork-appbase-pc-react/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/auth-pc-react$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-iam/apps/sdkwork-iam-pc/packages/sdkwork-auth-pc-react/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/auth-runtime-pc-react$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-iam/apps/sdkwork-iam-pc/packages/sdkwork-auth-runtime-pc-react/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/i18n-pc-react$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-appbase/packages/pc-react/foundation/sdkwork-i18n-pc-react/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/core-pc-react$/,
        replacement: path.resolve(__dirname, '../../../sdkwork-core/sdkwork-core-pc-react/src/index.ts'),
      },
      {
        find: /^@sdkwork\/ui-pc-react$/,
        replacement: path.resolve(__dirname, '../../../sdkwork-ui/sdkwork-ui-pc-react/src/index.ts'),
      },
      {
        find: /^@sdkwork\/sdk-common\/(.+)$/,
        replacement: `${sdkCommonSourceRoot}/$1/index.ts`,
      },
      { find: /^@sdkwork\/sdk-common$/, replacement: `${sdkCommonSourceRoot}/index.ts` },
      { find: /^@sdkwork\/utils\/(.+)$/, replacement: `${sdkworkUtilsSourceRoot}/$1` },
      { find: /^@sdkwork\/utils$/, replacement: `${sdkworkUtilsSourceRoot}/index.ts` },
    ],
  },
  server: {
    host: '127.0.0.1',
    port: 4178,
  },
  build: {
    rollupOptions: {
      output: {
        manualChunks(id) {
          const normalizedId = id.replaceAll('\\', '/');
          if (
            normalizedId.includes('/sdks/') ||
            normalizedId.includes('/sdkwork-sdk-commons/') ||
            normalizedId.includes('/sdkwork-utils/')
          ) {
            return 'sdk-vendor';
          }
          if (
            normalizedId.includes('/sdkwork-iam/apps/') ||
            normalizedId.includes('/sdkwork-appbase/')
          ) {
            return 'auth-vendor';
          }
          if (
            id.includes('node_modules/react/') ||
            id.includes('node_modules/react-dom/') ||
            id.includes('node_modules/react-router/') ||
            id.includes('node_modules/react-router-dom/') ||
            id.includes('node_modules/scheduler/')
          ) {
            return 'react-vendor';
          }
          if (id.includes('node_modules/@tiptap/') || id.includes('node_modules/prosemirror-')) {
            return 'editor-vendor';
          }
          if (id.includes('node_modules/i18next') || id.includes('node_modules/react-i18next/')) {
            return 'i18n-vendor';
          }
          if (
            id.includes('node_modules/@radix-ui/') ||
            id.includes('node_modules/lucide-react/') ||
            id.includes('node_modules/motion/') ||
            id.includes('node_modules/zustand/')
          ) {
            return 'ui-vendor';
          }
          return undefined;
        },
      },
    },
  },
});
