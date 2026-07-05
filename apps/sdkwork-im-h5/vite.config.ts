import path from "node:path";
import { fileURLToPath } from "node:url";
import react from "@vitejs/plugin-react";
import { defineConfig, loadEnv } from "vite";

const imH5Root = path.dirname(fileURLToPath(import.meta.url));
const imRoot = path.resolve(imH5Root, "../..");
const appbaseRoot = path.resolve(imRoot, "../sdkwork-appbase");
const iamRoot = path.resolve(imRoot, "../sdkwork-iam");
const uiRoot = path.resolve(imRoot, "../sdkwork-ui/sdkwork-ui-pc-react");
const driveRoot = path.resolve(imRoot, "../sdkwork-drive");
const utilsRoot = path.resolve(imRoot, "../sdkwork-utils");
const sdkCommonRoot = path.resolve(imRoot, "../sdkwork-sdk-commons/sdkwork-sdk-common-typescript");
const sdkCommonSourceRoot = path.resolve(sdkCommonRoot, "src");

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, imH5Root, "");
  return {
    define: {
      "process.env.SDKWORK_ACCESS_TOKEN": JSON.stringify(env.SDKWORK_ACCESS_TOKEN ?? ""),
    },
    plugins: [react()],
    resolve: {
      alias: {
        "@sdkwork/auth-pc-react": path.resolve(
          iamRoot,
          "apps/sdkwork-iam-pc/packages/sdkwork-auth-pc-react/src/index.ts",
        ),
        "@sdkwork/ui-pc-react": path.resolve(uiRoot, "src/index.ts"),
        "@sdkwork/auth-runtime-pc-react": path.resolve(
          iamRoot,
          "apps/sdkwork-iam-pc/packages/sdkwork-auth-runtime-pc-react/src/index.ts",
        ),
        "@sdkwork/iam-app-sdk": path.resolve(
          iamRoot,
          "sdks/sdkwork-iam-app-sdk/sdkwork-iam-app-sdk-typescript/generated/server-openapi/src/index.ts",
        ),
        "@sdkwork/iam-contracts": path.resolve(
          iamRoot,
          "apps/sdkwork-iam-common/packages/sdkwork-iam-contracts/src/index.ts",
        ),
        "@sdkwork/iam-runtime": path.resolve(
          iamRoot,
          "apps/sdkwork-iam-common/packages/sdkwork-iam-runtime/src/index.ts",
        ),
        "@sdkwork/iam-sdk-ports": path.resolve(
          iamRoot,
          "apps/sdkwork-iam-common/packages/sdkwork-iam-sdk-ports/src/index.ts",
        ),
        "@sdkwork/iam-service": path.resolve(
          iamRoot,
          "apps/sdkwork-iam-common/packages/sdkwork-iam-service/src/index.ts",
        ),
        "@sdkwork/runtime-bootstrap": path.resolve(
          appbaseRoot,
          "packages/common/foundation/sdkwork-runtime-bootstrap/src/index.ts",
        ),
        "@sdkwork/im-sdk": path.resolve(
          imRoot,
          "sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/index.ts",
        ),
        "@sdkwork/im-sdk-generated": path.resolve(
          imRoot,
          "sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/generated/server-openapi/src/index.ts",
        ),
        "@sdkwork/drive-app-sdk": path.resolve(
          driveRoot,
          "sdks/sdkwork-drive-app-sdk/sdkwork-drive-app-sdk-typescript/src/index.ts",
        ),
        "@sdkwork/utils": path.resolve(
          utilsRoot,
          "packages/sdkwork-utils-typescript/src/index.ts",
        ),
        "@sdkwork/sdk-common/core": path.resolve(sdkCommonSourceRoot, "core/index.ts"),
        "@sdkwork/sdk-common/auth": path.resolve(sdkCommonSourceRoot, "auth/index.ts"),
        "@sdkwork/sdk-common/http": path.resolve(sdkCommonSourceRoot, "http/index.ts"),
        "@sdkwork/sdk-common/errors": path.resolve(sdkCommonSourceRoot, "errors/index.ts"),
        "@sdkwork/sdk-common/utils": path.resolve(sdkCommonSourceRoot, "utils/index.ts"),
        "@sdkwork/sdk-common": path.resolve(sdkCommonSourceRoot, "index.ts"),
        "@sdkwork/im-h5-commons": path.resolve(imH5Root, "packages/sdkwork-im-h5-commons/src"),
        "@sdkwork/im-h5-core": path.resolve(imH5Root, "packages/sdkwork-im-h5-core/src"),
        "@sdkwork/im-h5-shell": path.resolve(imH5Root, "packages/sdkwork-im-h5-shell/src"),
        "@sdkwork/im-h5-chat": path.resolve(imH5Root, "packages/sdkwork-im-h5-chat/src"),
      },
    },
    server: { port: 3010 },
  };
});
