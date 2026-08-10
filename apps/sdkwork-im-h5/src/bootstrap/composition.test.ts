import assert from "node:assert/strict";
import test from "node:test";

import {
  parseImH5ModuleSelection,
  resolveConfiguredImH5ModuleIds,
} from "./composition";

test("keeps the full application composition (original UI)", () => {
  assert.deepEqual(resolveConfiguredImH5ModuleIds(), [
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
  assert.deepEqual(parseImH5ModuleSelection(), [
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

test("accepts an explicit composition of SDK-backed modules", () => {
  assert.deepEqual(
    parseImH5ModuleSelection("chat, user, contacts, drive, orders"),
    ["chat", "user", "contacts", "drive", "orders"],
  );
  assert.deepEqual(parseImH5ModuleSelection("drive"), ["drive"]);
});

test("rejects unknown, duplicate, empty, and contract-pending modules", () => {
  assert.throws(() => parseImH5ModuleSelection("chat,unknown"), /unknown module unknown/u);
  assert.throws(() => parseImH5ModuleSelection("chat,chat"), /duplicate module chat/u);
  assert.throws(() => parseImH5ModuleSelection("chat,,drive"), /empty module id/u);
  assert.throws(() => parseImH5ModuleSelection("chat,channels"), /does not have a composed runtime contract/u);
});
