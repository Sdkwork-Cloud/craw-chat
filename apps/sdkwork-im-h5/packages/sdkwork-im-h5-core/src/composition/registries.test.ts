import assert from "node:assert/strict";
import test from "node:test";

import {
  createImH5CoreModuleRegistry,
  createImH5HostRegistry,
  createImH5SdkRegistry,
} from "./index";

test("composition registries resolve independently injected building blocks", () => {
  const sdkRegistry = createImH5SdkRegistry();
  const moduleRegistry = createImH5CoreModuleRegistry([]);
  const hostRegistry = createImH5HostRegistry();
  const client = { kind: "im" };
  const host = { openExternalUrl: async () => undefined };

  sdkRegistry.register({ id: "im", client });
  moduleRegistry.register({
    id: "chat",
    packageName: "@sdkwork/im-h5-chat",
    enabledByDefault: true,
  });
  hostRegistry.register({ id: "browser", adapter: host });

  assert.equal(sdkRegistry.resolve("im"), client);
  assert.equal(moduleRegistry.resolve("chat")?.packageName, "@sdkwork/im-h5-chat");
  assert.equal(hostRegistry.resolve("browser"), host);
});

test("composition registries reject duplicate identities", () => {
  const sdkRegistry = createImH5SdkRegistry([{ id: "im", client: {} }]);
  assert.throws(
    () => sdkRegistry.register({ id: "im", client: {} }),
    /already exists/u,
  );
});
