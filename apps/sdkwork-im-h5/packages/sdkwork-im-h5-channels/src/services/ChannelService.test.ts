import assert from "node:assert/strict";
import test from "node:test";

import { ChannelCapabilityUnavailableError, ChannelService } from "./ChannelService";

test("channel operations fail closed until the owner SDK is composed", async () => {
  for (const operation of [
    () => ChannelService.getFeedWorks(),
    () => ChannelService.getWaterfallWorks(),
  ]) {
    await assert.rejects(operation, ChannelCapabilityUnavailableError);
  }
});
