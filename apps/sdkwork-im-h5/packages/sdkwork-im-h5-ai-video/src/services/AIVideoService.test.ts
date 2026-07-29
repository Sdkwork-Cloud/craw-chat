import assert from "node:assert/strict";
import test from "node:test";

import {
  AIVideoCapabilityUnavailableError,
  AIVideoService,
} from "./AIVideoService";

test("AI video operations fail closed without an owner SDK", async () => {
  let progressCalled = false;

  await assert.rejects(
    AIVideoService.generateVideo(
      { aspectRatio: "16:9", prompt: "test", style: "none" },
      () => {
        progressCalled = true;
      },
    ),
    AIVideoCapabilityUnavailableError,
  );
  await assert.rejects(
    AIVideoService.getHistory(),
    AIVideoCapabilityUnavailableError,
  );
  assert.throws(
    () => AIVideoService.deleteFromHistory("task-id"),
    AIVideoCapabilityUnavailableError,
  );
  assert.equal(progressCalled, false);
});
