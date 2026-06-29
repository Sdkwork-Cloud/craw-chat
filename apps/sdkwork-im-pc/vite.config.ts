import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react';
import { createRequire } from 'node:module';
import path from 'path';
import {defineConfig, loadEnv, type Plugin} from 'vite';
import { handleSdkworkChatLocalApiRequest } from './local-api';

const repoRoot = path.resolve(__dirname, '../..');
const appRequire = createRequire(path.join(__dirname, 'package.json'));

function dependencyRoot(dependencyId: string): string {
  return path.resolve(repoRoot, '..', dependencyId);
}

const generatedImAppSdkEntry = path.resolve(
  __dirname,
  '../../sdks/sdkwork-im-app-sdk/sdkwork-im-app-sdk-typescript/generated/server-openapi/src/index.ts',
);
const generatedImBackendSdkEntry = path.resolve(
  __dirname,
  '../../sdks/sdkwork-im-backend-sdk/sdkwork-im-backend-sdk-typescript/generated/server-openapi/src/index.ts',
);
const generatedAppbaseAppSdkEntry = path.resolve(
  repoRoot,
  '../sdkwork-iam/sdks/sdkwork-iam-app-sdk/sdkwork-iam-app-sdk-typescript/generated/server-openapi/src/index.ts',
);
const generatedAppbaseBackendSdkEntry = path.resolve(
  repoRoot,
  '../sdkwork-iam/sdks/sdkwork-iam-backend-sdk/sdkwork-iam-backend-sdk-typescript/generated/server-openapi/src/index.ts',
);
const generatedAiotAppSdkEntry = path.resolve(
  dependencyRoot('sdkwork-aiot'),
  'sdks/sdkwork-aiot-app-sdk/sdkwork-aiot-app-sdk-typescript/src/index.ts',
);
const generatedAiotBackendSdkEntry = path.resolve(
  dependencyRoot('sdkwork-aiot'),
  'sdks/sdkwork-aiot-backend-sdk/sdkwork-aiot-backend-sdk-typescript/src/index.ts',
);
const generatedDriveAppSdkEntry = path.resolve(
  dependencyRoot('sdkwork-drive'),
  'sdks/sdkwork-drive-app-sdk/sdkwork-drive-app-sdk-typescript/src/index.ts',
);
const generatedCatalogAppSdkEntry = path.resolve(
  dependencyRoot('sdkwork-catalog'),
  'sdks/sdkwork-catalog-app-sdk/sdkwork-catalog-app-sdk-typescript/generated/server-openapi/src/index.ts',
);
const generatedShopAppSdkEntry = path.resolve(
  dependencyRoot('sdkwork-shop'),
  'sdks/sdkwork-shop-app-sdk/sdkwork-shop-app-sdk-typescript/generated/server-openapi/src/index.ts',
);
const generatedOrderAppSdkEntry = path.resolve(
  dependencyRoot('sdkwork-order'),
  'sdks/sdkwork-order-app-sdk/sdkwork-order-app-sdk-typescript/generated/server-openapi/src/index.ts',
);

const generatedMailAppSdkEntry = path.resolve(
  repoRoot,
  'sdks/sdkwork-mail-app-sdk-shim/src/index.ts',
);
const generatedMailAppSdkTransportEntry = path.resolve(
  dependencyRoot('sdkwork-mail'),
  'sdks/sdkwork-mail-app-sdk/sdkwork-mail-app-sdk-typescript/generated/server-openapi/src/index.ts',
);
const generatedCommunityAppSdkEntry = path.resolve(
  repoRoot,
  'sdks/sdkwork-community-app-sdk-shim/src/index.ts',
);
const generatedCommunityAppSdkTransportEntry = path.resolve(
  dependencyRoot('sdkwork-community'),
  'sdks/sdkwork-community-app-sdk/sdkwork-community-app-sdk-typescript/generated/server-openapi/src/index.ts',
);

const generatedCourseAppSdkEntry = path.resolve(
  dependencyRoot('sdkwork-course'),
  'sdks/sdkwork-course-app-sdk/sdkwork-course-app-sdk-typescript/generated/server-openapi/src/index.ts',
);
const generatedCourseBackendSdkEntry = path.resolve(
  dependencyRoot('sdkwork-course'),
  'sdks/sdkwork-course-backend-sdk/sdkwork-course-backend-sdk-typescript/generated/server-openapi/src/index.ts',
);

const generatedNotaryAppSdkEntry = path.resolve(
  dependencyRoot('sdkwork-notary'),
  'sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/src/index.ts',
);
const notaryPcPackageRoot = path.resolve(
  dependencyRoot('sdkwork-notary'),
  'apps/sdkwork-notary-pc/packages',
);
const drivePcPackageRoot = path.resolve(
  dependencyRoot('sdkwork-drive'),
  'apps/sdkwork-drive-pc/packages',
);
const knowledgebasePcPackageRoot = path.resolve(
  dependencyRoot('sdkwork-knowledgebase'),
  'apps/sdkwork-knowledgebase-pc',
);
const coursePcPackageRoot = path.resolve(
  dependencyRoot('sdkwork-course'),
  'apps/sdkwork-course-pc/packages',
);
const agentsPcPackageRoot = path.resolve(
  dependencyRoot('sdkwork-agents'),
  'apps/sdkwork-agents-pc/packages',
);
const generatedAgentsAppSdkEntry = path.resolve(
  dependencyRoot('sdkwork-agents'),
  'sdks/sdkwork-agents-app-sdk/sdkwork-agents-app-sdk-typescript/generated/server-openapi/src/index.ts',
);
const generatedKnowledgebaseAppSdkEntry = path.resolve(
  dependencyRoot('sdkwork-knowledgebase'),
  'sdks/sdkwork-knowledgebase-app-sdk/sdkwork-knowledgebase-app-sdk-typescript/src/index.ts',
);
const generatedImSdkEntry = path.resolve(
  __dirname,
  '../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/index.ts',
);
const generatedImSdkTransportEntry = path.resolve(
  __dirname,
  '../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/generated/server-openapi/src/index.ts',
);
const generatedRtcSdkEntry = path.resolve(
  dependencyRoot('sdkwork-rtc'),
  'sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/index.ts',
);
const generatedRtcVolcengineProviderEntry = path.resolve(
  dependencyRoot('sdkwork-rtc'),
  'sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/providers/rtc-sdk-provider-volcengine/index.js',
);
const appbasePcReactEntry = path.resolve(
  repoRoot,
  '../sdkwork-appbase/packages/pc-react/foundation/sdkwork-appbase-pc-react/src/index.ts',
);
const authPcReactEntry = path.resolve(
  repoRoot,
  '../sdkwork-iam/apps/sdkwork-iam-pc/packages/sdkwork-auth-pc-react/src/index.ts',
);
const authRuntimePcReactEntry = path.resolve(
  repoRoot,
  '../sdkwork-iam/apps/sdkwork-iam-pc/packages/sdkwork-auth-runtime-pc-react/src/index.ts',
);
const authPcReactAuthEntry = path.resolve(
  repoRoot,
  '../sdkwork-iam/apps/sdkwork-iam-pc/packages/sdkwork-auth-pc-react/src/auth.ts',
);
const iamContractsEntry = path.resolve(
  repoRoot,
  '../sdkwork-iam/apps/sdkwork-iam-common/packages/sdkwork-iam-contracts/src/index.ts',
);
const iamSdkPortsEntry = path.resolve(
  repoRoot,
  '../sdkwork-iam/apps/sdkwork-iam-common/packages/sdkwork-iam-sdk-ports/src/index.ts',
);
const i18nPcReactEntry = path.resolve(
  repoRoot,
  '../sdkwork-appbase/packages/pc-react/foundation/sdkwork-i18n-pc-react/src/index.ts',
);
const corePcReactEntry = path.resolve(
  dependencyRoot('sdkwork-core'),
  'sdkwork-core-pc-react/src',
);
const uiPcReactSourceRoot = path.resolve(
  dependencyRoot('sdkwork-ui'),
  'sdkwork-ui-pc-react/src',
);
const uiPcReactEntry = path.resolve(
  repoRoot,
  '../sdkwork-ui/sdkwork-ui-pc-react/src/index.ts',
);
const uiPcReactStylesEntry = path.resolve(
  repoRoot,
  '../sdkwork-ui/sdkwork-ui-pc-react/src/styles/sdkwork-ui.css',
);
const sdkCommonSourceRoot = path.resolve(
  repoRoot,
  '../sdkwork-sdk-commons/sdkwork-sdk-common-typescript/src',
);
const sdkCommonEntry = path.resolve(
  sdkCommonSourceRoot,
  'index.ts',
);
const sdkworkUtilsSourceRoot = path.resolve(
  dependencyRoot('sdkwork-utils'),
  'packages/sdkwork-utils-typescript/src',
);
const sdkworkUtilsEntry = path.resolve(sdkworkUtilsSourceRoot, 'index.ts');
const adminCoreSourceRoot = path.resolve(__dirname, './packages/sdkwork-im-admin-core/src');
const reactEntry = path.resolve(__dirname, 'node_modules/react/index.js');
const reactJsxRuntimeEntry = path.resolve(__dirname, 'node_modules/react/jsx-runtime.js');
const reactJsxDevRuntimeEntry = path.resolve(__dirname, 'node_modules/react/jsx-dev-runtime.js');
const reactDomEntry = path.resolve(__dirname, 'node_modules/react-dom/index.js');
const reactDomClientEntry = path.resolve(__dirname, 'node_modules/react-dom/client.js');
const reactRouterDomEntry = appRequire.resolve('react-router-dom');
const reactRouterEntry = appRequire.resolve('react-router');
const reactRouterDomExportEntry = appRequire.resolve('react-router/dom');

function sdkworkChatLocalApiPlugin(): Plugin {
  return {
    name: 'sdkwork-chat-local-api',
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        handleSdkworkChatLocalApiRequest(req, res)
          .then((handled) => {
            if (!handled) {
              next();
            }
          })
          .catch(next);
      });
    },
  };
}

export default defineConfig(({mode}) => {
  const env = loadEnv(mode, '.', '');
  return {
    define: {
      'process.env.SDKWORK_ACCESS_TOKEN': JSON.stringify(
        env.SDKWORK_ACCESS_TOKEN ?? process.env.SDKWORK_ACCESS_TOKEN ?? '',
      ),
    },
        plugins: [sdkworkChatLocalApiPlugin(), react(), tailwindcss()],
    resolve: {
      alias: [
        { find: '@', replacement: path.resolve(__dirname, '.') },
        { find: 'react/jsx-runtime', replacement: reactJsxRuntimeEntry },
        { find: 'react/jsx-dev-runtime', replacement: reactJsxDevRuntimeEntry },
        { find: 'react-dom/client', replacement: reactDomClientEntry },
        { find: /^react-dom$/, replacement: reactDomEntry },
        { find: /^react-router\/dom$/, replacement: reactRouterDomExportEntry },
        { find: /^react-router$/, replacement: reactRouterEntry },
        { find: /^react-router-dom$/, replacement: reactRouterDomEntry },
        { find: /^react$/, replacement: reactEntry },
        { find: '@sdkwork-internal/im-app-api-generated', replacement: generatedImAppSdkEntry },
        { find: '@sdkwork-internal/im-backend-api-generated', replacement: generatedImBackendSdkEntry },
        { find: '@sdkwork/agents-app-sdk', replacement: generatedAgentsAppSdkEntry },
        { find: '@sdkwork/agents-pc-agents', replacement: path.resolve(agentsPcPackageRoot, 'sdkwork-agents-pc-agents/src/index.ts') },
        { find: '@sdkwork/agents-pc-commons', replacement: path.resolve(agentsPcPackageRoot, 'sdkwork-agents-pc-commons/src/index.ts') },
        { find: '@sdkwork/agents-pc-core', replacement: path.resolve(agentsPcPackageRoot, 'sdkwork-agents-pc-core/src') },
        { find: '@sdkwork/agents-pc-core/sdk/agentsAppSdkClient', replacement: path.resolve(agentsPcPackageRoot, 'sdkwork-agents-pc-core/src/sdk/agentsAppSdkClient.ts') },
        { find: '@sdkwork/aiot-app-sdk', replacement: generatedAiotAppSdkEntry },
        { find: '@sdkwork/aiot-backend-sdk', replacement: generatedAiotBackendSdkEntry },
        { find: '@sdkwork/iam-app-sdk', replacement: generatedAppbaseAppSdkEntry },
        { find: '@sdkwork/iam-backend-sdk', replacement: generatedAppbaseBackendSdkEntry },
        { find: '@sdkwork/drive-app-sdk', replacement: generatedDriveAppSdkEntry },
        { find: '@sdkwork/knowledgebase-app-sdk', replacement: generatedKnowledgebaseAppSdkEntry },
        { find: '@sdkwork/catalog-app-sdk', replacement: generatedCatalogAppSdkEntry },
        { find: '@sdkwork/shop-app-sdk', replacement: generatedShopAppSdkEntry },
        { find: '@sdkwork/order-app-sdk', replacement: generatedOrderAppSdkEntry },
        { find: '@sdkwork/mail-app-sdk', replacement: generatedMailAppSdkEntry },
        { find: 'sdkwork-mail-app-sdk-generated-typescript', replacement: generatedMailAppSdkTransportEntry },
        { find: '@sdkwork/community-app-sdk', replacement: generatedCommunityAppSdkEntry },
        { find: 'sdkwork-community-app-sdk-generated-typescript', replacement: generatedCommunityAppSdkTransportEntry },
        { find: '@sdkwork/course-app-sdk', replacement: generatedCourseAppSdkEntry },
        { find: '@sdkwork/course-backend-sdk', replacement: generatedCourseBackendSdkEntry },
        { find: '@sdkwork/notary-app-sdk', replacement: generatedNotaryAppSdkEntry },
        { find: '@sdkwork/im-sdk', replacement: generatedImSdkEntry },
        { find: '@sdkwork/im-sdk-generated', replacement: generatedImSdkTransportEntry },
        { find: '@sdkwork/rtc-sdk', replacement: generatedRtcSdkEntry },
        { find: '@sdkwork/rtc-sdk-provider-volcengine', replacement: generatedRtcVolcengineProviderEntry },
        { find: '@sdkwork/appbase-pc-react', replacement: appbasePcReactEntry },
        { find: '@sdkwork/auth-pc-react/auth', replacement: authPcReactAuthEntry },
        { find: '@sdkwork/auth-runtime-pc-react', replacement: authRuntimePcReactEntry },
        { find: '@sdkwork/auth-pc-react', replacement: authPcReactEntry },
        { find: '@sdkwork/iam-contracts', replacement: iamContractsEntry },
        { find: '@sdkwork/iam-sdk-ports', replacement: iamSdkPortsEntry },
        { find: '@sdkwork/i18n-pc-react', replacement: i18nPcReactEntry },
        { find: '@sdkwork/core-pc-react', replacement: corePcReactEntry },
        { find: '@sdkwork/ui-pc-react/styles.css', replacement: uiPcReactStylesEntry },
        { find: /^@sdkwork\/ui-pc-react\/(.+)$/, replacement: `${uiPcReactSourceRoot}/$1` },
        { find: '@sdkwork/ui-pc-react', replacement: uiPcReactEntry },
        { find: '@sdkwork/sdk-common/core', replacement: path.resolve(sdkCommonSourceRoot, 'core/index.ts') },
        { find: '@sdkwork/sdk-common/auth', replacement: path.resolve(sdkCommonSourceRoot, 'auth/index.ts') },
        { find: '@sdkwork/sdk-common/http', replacement: path.resolve(sdkCommonSourceRoot, 'http/index.ts') },
        { find: '@sdkwork/sdk-common/errors', replacement: path.resolve(sdkCommonSourceRoot, 'errors/index.ts') },
        { find: '@sdkwork/sdk-common/utils', replacement: path.resolve(sdkCommonSourceRoot, 'utils/index.ts') },
        { find: '@sdkwork/sdk-common', replacement: sdkCommonEntry },
        { find: '@sdkwork/utils', replacement: sdkworkUtilsEntry },
        { find: '@sdkwork/im-pc-types', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-types/src') },
        { find: '@sdkwork/im-pc-commons', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-commons/src') },
        { find: '@sdkwork/im-pc-shell', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-shell/src') },
        { find: '@sdkwork/im-pc-core', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-core/src') },
        { find: '@sdkwork/im-pc-chat', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-chat/src') },
        { find: '@sdkwork/im-pc-agent', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-agent/src') },
        { find: '@sdkwork/im-pc-voice', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-voice/src') },
        { find: '@sdkwork/im-pc-workspace', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-workspace/src') },
        { find: '@sdkwork/im-pc-orders', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-orders/src') },
        { find: '@sdkwork/notary-pc-commons', replacement: path.resolve(notaryPcPackageRoot, 'sdkwork-notary-pc-commons/src/index.ts') },
        { find: '@sdkwork/notary-pc-core', replacement: path.resolve(notaryPcPackageRoot, 'sdkwork-notary-pc-core/src/index.ts') },
        { find: '@sdkwork/notary-pc-notary', replacement: path.resolve(notaryPcPackageRoot, 'sdkwork-notary-pc-notary/src/index.ts') },
        { find: '@sdkwork/drive-pc-drive', replacement: path.resolve(drivePcPackageRoot, 'sdkwork-drive-pc-drive/src/index.ts') },
        { find: 'sdkwork-drive-pc-core', replacement: path.resolve(drivePcPackageRoot, 'sdkwork-drive-pc-core/src') },
        { find: 'sdkwork-drive-pc-commons', replacement: path.resolve(drivePcPackageRoot, 'sdkwork-drive-pc-commons/src') },
        { find: 'sdkwork-drive-pc-file', replacement: path.resolve(drivePcPackageRoot, 'sdkwork-drive-pc-file/src') },
        { find: 'sdkwork-drive-pc-transfer', replacement: path.resolve(drivePcPackageRoot, 'sdkwork-drive-pc-transfer/src') },
        { find: 'sdkwork-drive-pc-types', replacement: path.resolve(drivePcPackageRoot, 'sdkwork-drive-pc-types/src') },
        { find: '@sdkwork/knowledgebase-pc-knowledge', replacement: path.resolve(knowledgebasePcPackageRoot, 'packages/sdkwork-knowledgebase-pc-knowledge/src/index.ts') },
        { find: '@sdkwork/course-pc-course', replacement: path.resolve(coursePcPackageRoot, 'sdkwork-course-pc-course/src/index.ts') },
        { find: '@packages', replacement: path.resolve(knowledgebasePcPackageRoot, 'packages') },
        { find: 'sdkwork-knowledgebase-pc-core', replacement: path.resolve(knowledgebasePcPackageRoot, 'packages/sdkwork-knowledgebase-pc-core/src') },
        { find: 'sdkwork-knowledgebase-pc-core/host/hostAdapter', replacement: path.resolve(knowledgebasePcPackageRoot, 'packages/sdkwork-knowledgebase-pc-core/src/host/hostAdapter.ts') },
        { find: '@sdkwork/sdkwork-knowledgebase-pc-commons/stringUtils', replacement: path.resolve(knowledgebasePcPackageRoot, 'packages/sdkwork-knowledgebase-pc-commons/src/stringUtils.ts') },
        { find: '@sdkwork/sdkwork-knowledgebase-pc-commons/reactKeyedProps', replacement: path.resolve(knowledgebasePcPackageRoot, 'packages/sdkwork-knowledgebase-pc-commons/src/reactKeyedProps.ts') },
        { find: '@sdkwork/sdkwork-knowledgebase-pc-commons/htmlSanitizer', replacement: path.resolve(knowledgebasePcPackageRoot, 'packages/sdkwork-knowledgebase-pc-commons/src/htmlSanitizer.ts') },
        { find: '@sdkwork/sdkwork-knowledgebase-pc-commons', replacement: path.resolve(knowledgebasePcPackageRoot, 'packages/sdkwork-knowledgebase-pc-commons/src/index.ts') },
        { find: '@sdkwork/im-pc-mail', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-mail/src') },
        { find: '@sdkwork/im-pc-contacts', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-contacts/src') },
        { find: '@sdkwork/im-pc-calendar', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-calendar/src') },
        { find: '@sdkwork/im-pc-shop', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-shop/src') },
        { find: '@sdkwork/im-pc-devices', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-devices/src') },
        { find: '@sdkwork/im-pc-community', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-community/src') },
        { find: '@sdkwork/im-pc-enterprise', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-enterprise/src') },
        { find: '@sdkwork/im-console-core', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-core/src') },
        { find: /^@sdkwork\/im-admin-core\/(.+)$/, replacement: `${adminCoreSourceRoot}/$1` },
        { find: '@sdkwork/im-admin-core', replacement: adminCoreSourceRoot },
        { find: '@sdkwork/im-pc-approvals', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-approvals/src') },
        { find: '@sdkwork/im-pc-reports', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-reports/src') },
        { find: '@sdkwork/im-pc-attendance', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-attendance/src') },
        { find: '@sdkwork/im-console-users', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-users/src') },
        { find: '@sdkwork/im-console-roles', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-roles/src') },
        { find: '@sdkwork/im-console-communications', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-communications/src') },
        { find: '@sdkwork/im-console-integrations', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-integrations/src') },
        { find: '@sdkwork/im-console-security', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-security/src') },
        { find: '@sdkwork/im-console-settings', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-settings/src') },
        { find: '@sdkwork/im-console-shop', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-shop/src') },
        { find: '@sdkwork/im-console-product', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-product/src') },
        { find: '@sdkwork/im-admin-tenants', replacement: path.resolve(__dirname, './packages/sdkwork-im-admin-tenants/src') },
        { find: '@sdkwork/im-admin-infrastructure', replacement: path.resolve(__dirname, './packages/sdkwork-im-admin-infrastructure/src') },
        { find: '@sdkwork/im-admin-operations', replacement: path.resolve(__dirname, './packages/sdkwork-im-admin-operations/src') },
        { find: '@sdkwork/im-console-dashboard', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-dashboard/src') },
        { find: '@sdkwork/im-admin-dashboard', replacement: path.resolve(__dirname, './packages/sdkwork-im-admin-dashboard/src') },
        { find: '@sdkwork/im-pc-video-gen', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-video-gen/src') },
        { find: '@sdkwork/im-pc-image-gen', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-image-gen/src') },
        { find: '@sdkwork/im-pc-voice-gen', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-voice-gen/src') },
        { find: '@sdkwork/im-pc-music-gen', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-music-gen/src') },
        { find: '@sdkwork/im-pc-writing', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-writing/src') },
      ],
      dedupe: ['react', 'react-dom'],
    },
    server: {
      hmr: process.env.DISABLE_HMR !== 'true',
    },
    optimizeDeps: {
      exclude: [
        '@sdkwork-internal/im-app-api-generated',
        '@sdkwork-internal/im-backend-api-generated',
        '@sdkwork/agents-app-sdk',
        '@sdkwork/aiot-app-sdk',
        '@sdkwork/aiot-backend-sdk',
        '@sdkwork/iam-app-sdk',
        '@sdkwork/iam-backend-sdk',
        '@sdkwork/drive-app-sdk',
        '@sdkwork/knowledgebase-app-sdk',
        '@sdkwork/drive-pc-drive',
        '@sdkwork/knowledgebase-pc-knowledge',
        '@sdkwork/course-pc-course',
        'sdkwork-drive-pc-core',
        'sdkwork-drive-pc-commons',
        'sdkwork-drive-pc-file',
        'sdkwork-drive-pc-transfer',
        'sdkwork-drive-pc-types',
        'sdkwork-knowledgebase-pc-core',
        '@sdkwork/catalog-app-sdk',
        '@sdkwork/shop-app-sdk',
        '@sdkwork/order-app-sdk',
        '@sdkwork/mail-app-sdk',
        'sdkwork-mail-app-sdk-generated-typescript',
        '@sdkwork/community-app-sdk',
        'sdkwork-community-app-sdk-generated-typescript',
        '@sdkwork/course-app-sdk',
        '@sdkwork/course-backend-sdk',
        '@sdkwork/notary-app-sdk',
        '@sdkwork/im-sdk',
        '@sdkwork/rtc-sdk',
        '@sdkwork/rtc-sdk-provider-volcengine',
        '@sdkwork/appbase-pc-react',
        '@sdkwork/auth-pc-react',
        '@sdkwork/auth-runtime-pc-react',
        '@sdkwork/auth-pc-react/auth',
        '@sdkwork/iam-contracts',
        '@sdkwork/iam-sdk-ports',
        '@sdkwork/i18n-pc-react',
        '@sdkwork/sdk-common',
        '@sdkwork/utils',
        '@sdkwork/core-pc-react',
        '@sdkwork/ui-pc-react',
      ],
    },
    build: {
      rollupOptions: {
        output: {
          manualChunks(id) {
            if (id.includes('/node_modules/react') || id.includes('/node_modules/react-dom')) {
              return 'react-vendor';
            }
            if (id.includes('/node_modules/@tiptap') || id.includes('/node_modules/prosemirror')) {
              return 'editor-vendor';
            }
            return undefined;
          },
        },
      },
      chunkSizeWarningLimit: 2000,
    },
  };
});
