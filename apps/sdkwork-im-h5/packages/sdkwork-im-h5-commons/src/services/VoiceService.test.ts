import assert from "node:assert/strict";
import test from "node:test";

import { VoiceService } from "./VoiceService";

test("voice catalog returns the initial categories", async () => {
  const categories = await VoiceService.getVoiceCategories();
  assert.ok(Array.isArray(categories));
  assert.ok(categories.length > 0);
  assert.ok(categories.some((category) => category.voices.length > 0));
});

test("adding a custom voice appends to the my category", async () => {
  const before = await VoiceService.getVoiceCategories();
  const myBefore = before.find((category) => category.id === "my");
  await VoiceService.addCustomVoice("custom-label", "custom-desc");
  const after = await VoiceService.getVoiceCategories();
  const myAfter = after.find((category) => category.id === "my");
  assert.ok(myBefore && myAfter);
  assert.ok(myAfter.voices.length >= myBefore.voices.length);
  assert.ok(myAfter.voices.some((voice) => voice.label === "custom-label"));
});
