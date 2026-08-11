/**
 * My Works — fail-closed (PRD).
 *
 * Audited as a pure localStorage mock with no owner backend SDK. The fake
 * seed works and `sdkwork_im_h5_my_works` / legacy `clawchat_*` storage are
 * removed: every method throws a typed `WorkCapabilityUnavailableError` so
 * the works pages surface a typed unavailable state instead of fabricated
 * works.
 */
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

export class WorkCapabilityUnavailableError extends Error {
  constructor(capability: string) {
    super(`${capability} is unavailable because its owner SDK is not composed.`);
    this.name = "WorkCapabilityUnavailableError";
  }
}

export class WorkService {
  static async getMyWorks(): Promise<Work[]> {
    throw new WorkCapabilityUnavailableError("My works list");
  }

  static async deleteWork(_id: string): Promise<boolean> {
    throw new WorkCapabilityUnavailableError("Work deletion");
  }

  static async updateWork(_id: string, _updates: Partial<Work>): Promise<Work> {
    throw new WorkCapabilityUnavailableError("Work update");
  }

  static async addWork(_work: Work): Promise<void> {
    throw new WorkCapabilityUnavailableError("Work creation");
  }
}
