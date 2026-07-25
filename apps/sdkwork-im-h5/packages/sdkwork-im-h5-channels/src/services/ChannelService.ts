import { useTranslation } from "react-i18next";
import { CreativeWork } from "../types";
import { CREATIVE_WORKS } from "../data/mockData";

export class ChannelService {
  static async getFeedWorks(): Promise<CreativeWork[]> {
    return new Promise((resolve) => {
      setTimeout(() => resolve(CREATIVE_WORKS), 500);
    });
  }

  static async getWaterfallWorks(): Promise<CreativeWork[]> {
    return new Promise((resolve) => {
      const items = [...CREATIVE_WORKS, ...CREATIVE_WORKS, ...CREATIVE_WORKS].map((w, i) => ({
        ...w,
        id: `wf-${i}`,
      }));
      setTimeout(() => resolve(items), 500);
    });
  }
}
