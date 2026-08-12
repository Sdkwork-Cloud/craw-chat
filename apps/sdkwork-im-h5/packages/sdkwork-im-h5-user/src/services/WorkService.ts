/**
 * My Works — fail-closed (PRD).
 *
 * Audited as a pure client-side mock with no owner backend SDK. The fake
 * seed works and `sdkwork_im_h5_my_works` / legacy `clawchat_*` storage are
 * removed: every method throws a typed `UserCapabilityUnavailableError` so
 * the works pages surface a typed unavailable state instead of fabricated
 * works.
 */
import { UserCapabilityUnavailableError } from "./UserCapabilityUnavailableError";

export interface Work {
  id: string;
  type: "video" | "article" | "audio" | "ai_image";
  title: string;
  coverUrl: string;
  createdAt: string;
  views: number;
  likes: number;
  comments: number;
}

export class WorkService {
  static async getMyWorks(): Promise<Work[]> {
    throw new UserCapabilityUnavailableError("My works list");
  }

  static async deleteWork(_id: string): Promise<boolean> {
    throw new UserCapabilityUnavailableError("Work deletion");
  }

  static async updateWork(_id: string, _updates: Partial<Work>): Promise<Work> {
    throw new UserCapabilityUnavailableError("Work update");
  }

  static async addWork(_work: Work): Promise<void> {
    throw new UserCapabilityUnavailableError("Work creation");
  }
}
