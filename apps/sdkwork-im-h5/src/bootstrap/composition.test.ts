import assert from "node:assert/strict";
import test from "node:test";

import {
  parseImH5ModuleSelection,
  resolveConfiguredImH5ModuleIds,
} from "./composition";

test("keeps the default application composition unchanged", () => {
  assert.deepEqual(resolveConfiguredImH5ModuleIds(), ["chat", "notary", "orders"]);
  assert.deepEqual(parseImH5ModuleSelection(), ["chat", "notary", "orders"]);
});

test("accepts an explicit composition of SDK-backed modules", () => {
  assert.deepEqual(
    parseImH5ModuleSelection("chat, notary, contacts, drive, orders"),
    ["chat", "notary", "contacts", "drive", "orders"],
  );
  assert.deepEqual(parseImH5ModuleSelection("drive"), ["drive"]);
});

test("rejects unknown, duplicate, empty, and contract-pending modules", () => {
  assert.throws(() => parseImH5ModuleSelection("chat,unknown"), /unknown module unknown/u);
  assert.throws(() => parseImH5ModuleSelection("chat,chat"), /duplicate module chat/u);
  assert.throws(() => parseImH5ModuleSelection("chat,,drive"), /empty module id/u);
  assert.throws(() => parseImH5ModuleSelection("chat,shop"), /does not have a composed runtime contract/u);
});
