import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import { defineConfig, loadEnv } from 'vite';

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, '.', '');
  return {
    cacheDir: path.resolve(__dirname, 'node_modules', '.vite', 'sdkwork-im-h5'),
    plugins: [react(), tailwindcss()],
    define: {
      // Replaced define to avoid passing server secrets to client.
    },
    resolve: {
      alias: [
        { find: /^@\/(.*)/, replacement: path.resolve(__dirname, 'src/$1') },
        { find: /^@sdkwork\/im-h5-(.*)/, replacement: path.resolve(__dirname, 'packages/sdkwork-im-h5-$1/src') },
        { find: /^@sdkwork\/im-h5-(.*)\/(.*)/, replacement: path.resolve(__dirname, 'packages/sdkwork-im-h5-$1/src/$2') },
        { find: /^@sdkwork\/im-app-sdk$/, replacement: path.resolve(__dirname, '../../sdks/sdkwork-im-app-sdk/sdkwork-im-app-sdk-typescript/src/index.ts') },
        { find: /^@sdkwork\/im-backend-sdk$/, replacement: path.resolve(__dirname, '../../sdks/sdkwork-im-backend-sdk/sdkwork-im-backend-sdk-typescript/src/index.ts') },
        { find: /^@sdkwork\/im-sdk$/, replacement: path.resolve(__dirname, '../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/index.ts') },
        { find: /^@sdkwork\/iam-app-sdk$/, replacement: path.resolve(__dirname, '../../../sdkwork-iam/sdks/sdkwork-iam-app-sdk/sdkwork-iam-app-sdk-typescript/src/index.ts') },
        { find: /^@sdkwork\/iam-backend-sdk$/, replacement: path.resolve(__dirname, '../../../sdkwork-iam/sdks/sdkwork-iam-backend-sdk/sdkwork-iam-backend-sdk-typescript/src/index.ts') },
        { find: /^@sdkwork\/drive-app-sdk$/, replacement: path.resolve(__dirname, '../../../sdkwork-drive/sdks/sdkwork-drive-app-sdk/sdkwork-drive-app-sdk-typescript/src/index.ts') },
        { find: /^@sdkwork\/rtc-sdk$/, replacement: path.resolve(__dirname, '../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/index.ts') },
        { find: /^@sdkwork\/appbase-pc-react$/, replacement: path.resolve(__dirname, '../../../sdkwork-appbase/packages/pc-react/foundation/sdkwork-appbase-pc-react/src/index.ts') },
        { find: /^@sdkwork\/auth-pc-react$/, replacement: path.resolve(__dirname, '../../../sdkwork-iam/apps/sdkwork-iam-pc/packages/sdkwork-auth-pc-react/src/index.ts') },
        { find: /^@sdkwork\/auth-runtime-pc-react$/, replacement: path.resolve(__dirname, '../../../sdkwork-iam/apps/sdkwork-iam-pc/packages/sdkwork-auth-runtime-pc-react/src/index.ts') },
        { find: /^@sdkwork\/i18n-pc-react$/, replacement: path.resolve(__dirname, '../../../sdkwork-appbase/packages/pc-react/foundation/sdkwork-i18n-pc-react/src/index.ts') },
        { find: /^@sdkwork\/core-pc-react$/, replacement: path.resolve(__dirname, '../../../sdkwork-core/sdkwork-core-pc-react/src/index.ts') },
        { find: /^@sdkwork\/ui-pc-react$/, replacement: path.resolve(__dirname, '../../../sdkwork-ui/sdkwork-ui-pc-react/src/index.ts') },
      ],
    },
    server: {
      host: '127.0.0.1',
      port: 4178,
    },
  };
});
