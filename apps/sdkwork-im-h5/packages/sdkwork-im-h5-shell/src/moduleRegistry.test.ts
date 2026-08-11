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

test("keeps the full H5 product composition (original UI tabs)", () => {
  assert.deepEqual(DEFAULT_IM_H5_MODULES, [
    "chat",
    "contacts",
    "user",
    "agents",
    "notary",
    "orders",
    "approval",
    "attendance",
    "calendar",
    "report",
    "recruitment",
    "enterprise",
    "meeting",
    "moments",
    "music",
    "knowledge",
    "drive",
    "voice",
    "videogen",
    "imagegen",
    "musicgen",
    "writing",
    "devices",
    "membership",
    "course",
    "community",
    "shop",
  ]);
});

test("classifies every composed module as composable", () => {
  for (const moduleId of DEFAULT_IM_H5_MODULES) {
    assert.equal(COMPOSABLE_IM_H5_MODULES.has(moduleId), true);
    assert.equal(CONTRACT_PENDING_IM_H5_MODULES.has(moduleId), false);
  }
});

test("keeps the channels module pending until its surface is composed", () => {
  assert.equal(CONTRACT_PENDING_IM_H5_MODULES.has("channels"), true);
  assert.throws(
    () => requireImH5ShellModule("channels", {}),
    /H5 module channels is not composed/u,
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
      labelKey: "contacts.title",
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
      labelKey: "drive.title",
      icon: () => null,
    }],
    routes: [],
  };
  assert.equal(resolveImH5ShellHomePath([contacts, drive]), "/workspace/contacts");
  assert.equal(resolveImH5ShellHomePath([drive]), "/workspace/drive");
});
