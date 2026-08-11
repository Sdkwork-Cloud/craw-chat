import assert from "node:assert/strict";
import test from "node:test";

import { VoiceCapabilityUnavailableError, VoiceService } from "./VoiceService";

test("voice service fails closed until an owner SDK is composed", async () => {
  await assert.rejects(VoiceService.getVoiceCategories(), VoiceCapabilityUnavailableError);
  await assert.rejects(VoiceService.addCustomVoice("custom-label", "custom-desc"), VoiceCapabilityUnavailableError);
});
