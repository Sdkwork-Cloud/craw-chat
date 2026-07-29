import assert from "node:assert/strict";
import test from "node:test";

import {
  VoiceCapabilityUnavailableError,
  VoiceService,
} from "./VoiceService";

test("voice catalog operations fail closed until the owner SDK is composed", async () => {
  await assert.rejects(
    VoiceService.getVoiceCategories(),
    VoiceCapabilityUnavailableError,
  );
  await assert.rejects(
    VoiceService.addCustomVoice("name", "description"),
    VoiceCapabilityUnavailableError,
  );
});
