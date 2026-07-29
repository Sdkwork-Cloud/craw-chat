import assert from "node:assert/strict";
import test from "node:test";

import {
  AIWritingCapabilityUnavailableError,
  AIWritingService,
} from "./AIWritingService";

test("AI writing operations fail closed without an owner SDK", async () => {
  let chunkCalled = false;

  await assert.rejects(
    AIWritingService.generateArticle(
      { language: "English", length: "short", style: "plain", topic: "test" },
      () => {
        chunkCalled = true;
      },
    ),
    AIWritingCapabilityUnavailableError,
  );
  await assert.rejects(
    AIWritingService.getHistory(),
    AIWritingCapabilityUnavailableError,
  );
  assert.throws(
    () => AIWritingService.deleteFromHistory("task-id"),
    AIWritingCapabilityUnavailableError,
  );
  assert.equal(chunkCalled, false);
});
