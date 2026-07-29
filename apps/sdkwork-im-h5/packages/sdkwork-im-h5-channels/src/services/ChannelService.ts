import type { CreativeWork } from "../types";

export class ChannelCapabilityUnavailableError extends Error {
  constructor() {
    super("Creative Channels are unavailable because their owner SDK is not composed.");
    this.name = "ChannelCapabilityUnavailableError";
  }
}

export class ChannelService {
  static async getFeedWorks(): Promise<CreativeWork[]> {
    throw new ChannelCapabilityUnavailableError();
  }

  static async getWaterfallWorks(): Promise<CreativeWork[]> {
    throw new ChannelCapabilityUnavailableError();
  }
}
