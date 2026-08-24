import { resolveBrowserDistOutDir } from '../../../sdkwork-specs/tools/browser-dist-layout.mjs';
function resolveViteEnvironment(mode, processEnv = process.env) {
  const profileMatch = /^(standalone|cloud)\.(development|test|staging|production)$/u.exec(mode ?? '');
  return profileMatch?.[2]
    ?? (['development', 'test', 'staging', 'production'].includes(processEnv.SDKWORK_ENVIRONMENT ?? '')
      ? processEnv.SDKWORK_ENVIRONMENT
      : 'production');
}
import tailwindcss from '@tailwindcss/vite';
import { createSdkworkCredentialEntryBootstrapVitePlugin } from '@sdkwork/iam-credential-entry/vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import { defineConfig } from 'vite';
import { wechatCssCompatPlugin } from './scripts/dev/wechat-css-compat.mjs';
import { wechatJsCompatPlugin } from './scripts/dev/wechat-js-compat.mjs';

const appReactEntry = path.resolve(__dirname, 'node_modules/react/index.js');
const appReactJsxRuntimeEntry = path.resolve(__dirname, 'node_modules/react/jsx-runtime.js');
const appReactJsxDevRuntimeEntry = path.resolve(__dirname, 'node_modules/react/jsx-dev-runtime.js');
const appReactDomEntry = path.resolve(__dirname, 'node_modules/react-dom/index.js');
const appReactDomClientEntry = path.resolve(__dirname, 'node_modules/react-dom/client.js');
const appClsxEntry = path.resolve(__dirname, 'node_modules/clsx/dist/clsx.mjs');
const appTailwindMergeEntry = path.resolve(
  __dirname,
  'node_modules/tailwind-merge/dist/bundle-mjs.mjs',
);
const appZustandEntry = path.resolve(__dirname, 'node_modules/zustand/esm/index.mjs');
const appLucideReactEntry = path.resolve(
  __dirname,
  'node_modules/lucide-react/dist/esm/lucide-react.js',
);
const appMotionReactEntry = path.resolve(__dirname, 'node_modules/motion/dist/es/react.mjs');
const appReactI18nextEntry = path.resolve(
  __dirname,
  'node_modules/react-i18next/dist/es/index.js',
);
const appReactRouterEntry = path.resolve(
  __dirname,
  'node_modules/react-router/dist/development/index.mjs',
);
const appReactQrCodeEntry = path.resolve(__dirname, 'node_modules/react-qr-code/lib/index.mjs');
const appReactSignatureCanvasEntry = path.resolve(
  __dirname,
  'node_modules/react-signature-canvas/dist/index.mjs',
);
const sdkCommonSourceRoot = path.resolve(
  __dirname,
  '../../../sdkwork-sdk-commons/sdkwork-sdk-common-typescript/src',
);

export default defineConfig(({ mode }) => ({
  // Keep the dependency cache outside `node_modules`: the workspace's
  // node_modules donor/junction machinery (`ensureLocalNodeModules` renames
  // node_modules wholesale to `*.__stale__donor*`) and pnpm reinstalls would
  // otherwise wipe the cache mid-session, forcing a full dep re-optimization
  // that invalidates every in-flight `?v=` dep URL. A root-level `.vite`
  // cache also serves dep URLs as `/.vite/deps/...` instead of
  // `/node_modules/.vite/<app>/...`, which the adaptive dev proxy treats as
  // stale (410 Gone) whenever the request is routed to another renderer.
  cacheDir: path.resolve(__dirname, '.vite'),
  plugins: [
    createSdkworkCredentialEntryBootstrapVitePlugin({
      accessToken: process.env.SDKWORK_ACCESS_TOKEN,
      environment: mode,
    }),
    wechatJsCompatPlugin(),
    react(),
    tailwindcss(),
    wechatCssCompatPlugin(),
  ],
  define: {
    // Replaced define to avoid passing server secrets to client.
  },
  esbuild: {
      outDir: resolveBrowserDistOutDir(resolveViteEnvironment(mode, process.env)),
    // Dev esbuild defaults to target `esnext`. Combined with
    // `useDefineForClassFields: false` (injected for SDK sources without a
    // tsconfig, or set explicitly in tsconfig.json), esbuild compiles class
    // static fields into ES2022 static blocks (`static {}`). WeChat DevTools'
    // embedded kernel (Chromium < 94) rejects those with
    // "Unexpected token '{'". Aligning the dev target with the build default
    // (ESBUILD_MODULES_TARGET) lowers static fields to plain assignments,
    // keeping the assign semantics while staying parseable by WeChat X5 /
    // older WKWebView.
    target: ['es2020', 'edge88', 'firefox78', 'chrome87', 'safari14'],
  },
  resolve: {
    dedupe: ['react', 'react-dom'],
    alias: [
      {
        find: '@sdkwork/iam-credential-entry/vite',
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-iam/apps/sdkwork-iam-common/packages/sdkwork-iam-credential-entry/src/vite.ts',
        ),
      },
      { find: 'react/jsx-runtime', replacement: appReactJsxRuntimeEntry },
      { find: 'react/jsx-dev-runtime', replacement: appReactJsxDevRuntimeEntry },
      { find: 'react-dom/client', replacement: appReactDomClientEntry },
      { find: /^react-dom$/, replacement: appReactDomEntry },
      { find: /^react$/, replacement: appReactEntry },
      { find: /^clsx$/, replacement: appClsxEntry },
      { find: /^tailwind-merge$/, replacement: appTailwindMergeEntry },
      { find: /^zustand$/, replacement: appZustandEntry },
      { find: /^lucide-react$/, replacement: appLucideReactEntry },
      { find: /^motion\/react$/, replacement: appMotionReactEntry },
      { find: /^react-i18next$/, replacement: appReactI18nextEntry },
      { find: /^react-router$/, replacement: appReactRouterEntry },
      { find: /^react-qr-code$/, replacement: appReactQrCodeEntry },
      { find: /^react-signature-canvas$/, replacement: appReactSignatureCanvasEntry },
      { find: /^@\/(.*)/, replacement: path.resolve(__dirname, 'src/$1') },
      {
        find: /^@sdkwork\/notary-h5-commons$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-notary/apps/sdkwork-notary-h5/packages/sdkwork-notary-h5-commons/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/notary-h5-core$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-notary/apps/sdkwork-notary-h5/packages/sdkwork-notary-h5-core/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/notary-h5-notary$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-notary/apps/sdkwork-notary-h5/packages/sdkwork-notary-h5-notary/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/notary-h5-shell$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-notary/apps/sdkwork-notary-h5/packages/sdkwork-notary-h5-shell/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/order-mobile-react-orders$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-order/apps/sdkwork-order-common/packages/sdkwork-order-mobile-react-orders/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/ui-mobile-react$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-ui/sdkwork-ui-mobile-react/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/aiot-mobile-react-hardware$/,
        replacement: path.resolve(__dirname, '../../../sdkwork-aiot/apps/sdkwork-aiot-shared/packages/sdkwork-aiot-mobile-react-hardware/src/index.ts'),
      },
      {
        find: /^@sdkwork\/agents-h5-agents$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-agents/apps/sdkwork-agents-h5/packages/sdkwork-agents-h5-agents/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/agents-h5-commons$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-agents/apps/sdkwork-agents-h5/packages/sdkwork-agents-h5-commons/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/agents-h5-core$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-agents/apps/sdkwork-agents-h5/packages/sdkwork-agents-h5-core/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/community-mobile-react-community$/,
        replacement: path.resolve(__dirname, '../../../sdkwork-community/apps/sdkwork-community-common/packages/sdkwork-community-mobile-react-community/src/index.ts'),
      },
      {
        find: /^@sdkwork\/course-mobile-react-courses$/,
        replacement: path.resolve(__dirname, '../../../sdkwork-course/apps/sdkwork-course-common/packages/sdkwork-course-mobile-react-courses/src/index.ts'),
      },
      {
        find: /^@sdkwork\/drive-mobile-react-drive$/,
        replacement: path.resolve(__dirname, '../../../sdkwork-drive/apps/sdkwork-drive-common/packages/sdkwork-drive-mobile-react-drive/src/index.ts'),
      },
      {
        find: /^@sdkwork\/image-mobile-react-generation$/,
        replacement: path.resolve(__dirname, '../../../sdkwork-image/apps/sdkwork-image-common/packages/sdkwork-image-mobile-react-generation/src/index.ts'),
      },
      {
        find: /^@sdkwork\/knowledgebase-mobile-react-knowledge$/,
        replacement: path.resolve(__dirname, '../../../sdkwork-knowledgebase/apps/sdkwork-knowledgebase-common/packages/sdkwork-knowledgebase-mobile-react-knowledge/src/index.ts'),
      },
      {
        find: /^@sdkwork\/membership-mobile-react-subscription$/,
        replacement: path.resolve(__dirname, '../../../sdkwork-membership/apps/sdkwork-membership-common/packages/sdkwork-membership-mobile-react-subscription/src/index.ts'),
      },
      {
        find: /^@sdkwork\/music-mobile-react-generation$/,
        replacement: path.resolve(__dirname, '../../../sdkwork-music/apps/sdkwork-music-common/packages/sdkwork-music-mobile-react-generation/src/index.ts'),
      },
      {
        find: /^@sdkwork\/rtc-mobile-react-meeting$/,
        replacement: path.resolve(__dirname, '../../../sdkwork-rtc/apps/sdkwork-rtc-h5/packages/sdkwork-rtc-mobile-react-meeting/src/index.ts'),
      },
      {
        find: /^@sdkwork\/shop-mobile-react-shopping$/,
        replacement: path.resolve(__dirname, '../../../sdkwork-shop/apps/sdkwork-shop-common/packages/sdkwork-shop-mobile-react-shopping/src/index.ts'),
      },
      {
        find: /^@sdkwork\/video-mobile-react-generation$/,
        replacement: path.resolve(__dirname, '../../../sdkwork-video/apps/sdkwork-video-common/packages/sdkwork-video-mobile-react-generation/src/index.ts'),
      },
      {
        find: /^@sdkwork\/voice-mobile-react-generation$/,
        replacement: path.resolve(__dirname, '../../../sdkwork-voice/apps/sdkwork-voice-common/packages/sdkwork-voice-mobile-react-generation/src/index.ts'),
      },
      {
        find: /^@sdkwork\/voice-mobile-my-voices$/,
        replacement: path.resolve(__dirname, '../../../sdkwork-voice/apps/sdkwork-voice-common/packages/sdkwork-voice-mobile-my-voices/src/index.ts'),
      },
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
        find: /^@sdkwork\/order-app-sdk$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-order/sdks/sdkwork-order-app-sdk/sdkwork-order-app-sdk-typescript/src/index.ts',
        ),
      },
      {
        find: /^@sdkwork\/knowledgebase-app-sdk$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-knowledgebase/sdks/sdkwork-knowledgebase-app-sdk/sdkwork-knowledgebase-app-sdk-typescript/src/index.ts',
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
        find: /^@sdkwork\/iam-h5-auth$/,
        replacement: path.resolve(
          __dirname,
          '../../../sdkwork-iam/apps/sdkwork-iam-h5/packages/sdkwork-iam-h5-auth/src/index.ts',
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
    ],
  },
  css: {
    // Lightning CSS lowers modern features (cascade layers, dvh, color-mix)
    // for old browser targets so WeChat X5 / older WKWebView keep the styles.
    transformer: 'lightningcss',
    lightningcss: {
      targets: {
        // X5 kernel ≈ Chromium 86-107; iOS WKWebView ≈ Safari 13+.
        chrome: 86 << 16,
        android: 86 << 16,
        ios_saf: 13 << 16,
        safari: 13 << 16,
      },
    },
  },
  server: {
    host: '127.0.0.1',
    port: 4178,
    // Sibling workspace sources (sdkwork-community, sdkwork-agents, ...)
    // are aliased into the module graph; allow the workspace collection root
    // so Vite's fs.allow does not 403 those /@fs requests in dev.
    fs: {
      allow: [path.resolve(__dirname, '../../..')],
    },
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
}));
