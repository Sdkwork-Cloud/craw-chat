import assert from "node:assert/strict";
import test from "node:test";

import {
  parseImH5ModuleSelection,
  resolveConfiguredImH5ModuleIds,
} from "./composition";

test("keeps the real-SDK application composition (mock-only modules excluded)", () => {
  // approval / attendance / calendar / report were audited as pure
  // localStorage mocks; they are excluded from the default composition
  // (fail-closed, PRD). enterprise / recruitment now have the
  // sdkwork-company owner SDK and are composed by default.
  assert.deepEqual(resolveConfiguredImH5ModuleIds(), [
    "chat",
    "contacts",
    "user",
    "agents",
    "notary",
    "orders",
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
    "enterprise",
    "recruitment",
  ]);
  assert.deepEqual(parseImH5ModuleSelection(), [
    "chat",
    "contacts",
    "user",
    "agents",
    "notary",
    "orders",
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
      "enterprise",
    "recruitment",
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
  // Mock-only modules moved to contract-pending until a real owner SDK exists.
  assert.throws(
    () => parseImH5ModuleSelection("chat,approval"),
    /approval does not have a composed runtime contract/u,
  );
});
