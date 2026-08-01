import assert from "node:assert/strict";
import test from "node:test";

import type { ImH5CapabilityModule } from "./contracts";
import {
  COMPOSABLE_IM_H5_MODULES,
  CONTRACT_PENDING_IM_H5_MODULES,
  DEFAULT_IM_H5_MODULES,
} from "./moduleCatalog";
import {
  requireImH5ShellModule,
  validateImH5ShellModules,
} from "./moduleValidation";
import { resolveImH5ShellHomePath } from "./moduleNavigation";

test("keeps the default H5 product composition unchanged", () => {
  assert.deepEqual(DEFAULT_IM_H5_MODULES, ["chat", "notary", "orders"]);
});

test("classifies the SDK-backed contacts module as optional and composable", () => {
  assert.equal(COMPOSABLE_IM_H5_MODULES.has("contacts"), true);
  assert.equal(CONTRACT_PENDING_IM_H5_MODULES.has("contacts"), false);
});

test("classifies the SDK-backed Drive module as optional and composable", () => {
  assert.equal(COMPOSABLE_IM_H5_MODULES.has("drive"), true);
  assert.equal(CONTRACT_PENDING_IM_H5_MODULES.has("drive"), false);
});

test("classifies the SDK-backed Order module as composable", () => {
  assert.equal(COMPOSABLE_IM_H5_MODULES.has("orders"), true);
  assert.equal(CONTRACT_PENDING_IM_H5_MODULES.has("orders"), false);
});

test("keeps modules without an owner runtime contract pending", () => {
  assert.equal(CONTRACT_PENDING_IM_H5_MODULES.has("shop"), true);
  assert.throws(
    () => requireImH5ShellModule("shop", {}),
    /H5 module shop is not composed/u,
  );
});

test("rejects duplicate module, route, path, and navigation identities", () => {
  const chat: ImH5CapabilityModule = {
    id: "chat",
    navigation: [
      {
        id: "chat",
        moduleId: "chat",
        path: "/",
        labelKey: "common.tabs.chat",
        icon: () => null,
      },
    ],
    routes: [
      {
        id: "app.communication.chat.inbox",
        moduleId: "chat",
        domain: "communication",
        capability: "chat",
        screen: "inbox",
        path: "/",
        titleKey: "common.tabs.chat",
        surface: "app",
        auth: "required",
        presentation: { h5Mobile: "tab" },
        render: () => null,
      },
    ],
  };
  assert.throws(() => validateImH5ShellModules([chat, chat]), /Duplicate H5 module id/u);

  const conflictingModule: ImH5CapabilityModule = {
    id: "contacts",
    navigation: [
      { ...chat.navigation![0]!, moduleId: "contacts" },
    ],
    routes: [
      { ...chat.routes[0], moduleId: "contacts", render: () => null },
    ],
  };
  assert.throws(
    () => validateImH5ShellModules([chat, conflictingModule]),
    /Duplicate H5 route id/u,
  );

  const pathConflict: ImH5CapabilityModule = {
    ...conflictingModule,
    routes: [
      {
        ...chat.routes[0],
        id: "app.communication.contacts.index",
        moduleId: "contacts",
        render: () => null,
      },
    ],
  };
  assert.throws(
    () => validateImH5ShellModules([chat, pathConflict]),
    /Duplicate H5 route path/u,
  );
});

test("rejects an empty composition and derives a home path from selected modules", () => {
  assert.throws(
    () => validateImH5ShellModules([]),
    /must contain at least one module/u,
  );
  const contacts: ImH5CapabilityModule = {
    id: "contacts",
    navigation: [{
      id: "contacts",
      moduleId: "contacts",
      path: "/workspace/contacts",
      labelKey: "common.tabs.contacts",
      icon: () => null,
    }],
    routes: [],
  };
  const drive: ImH5CapabilityModule = {
    id: "drive",
    navigation: [{
      id: "drive",
      moduleId: "drive",
      path: "/workspace/drive",
      labelKey: "common.tabs.drive",
      icon: () => null,
    }],
    routes: [],
  };
  assert.equal(resolveImH5ShellHomePath([contacts, drive]), "/workspace/contacts");
  assert.equal(resolveImH5ShellHomePath([drive]), "/workspace/drive");
});
