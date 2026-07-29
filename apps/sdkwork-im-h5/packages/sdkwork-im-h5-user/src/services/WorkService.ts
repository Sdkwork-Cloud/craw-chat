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
    throw new UserCapabilityUnavailableError("User works");
  }

  static async deleteWork(_id: string): Promise<boolean> {
    throw new UserCapabilityUnavailableError("User works");
  }

  static async updateWork(_id: string, _updates: Partial<Work>): Promise<Work> {
    throw new UserCapabilityUnavailableError("User works");
  }

  static async addWork(_work: Work): Promise<void> {
    throw new UserCapabilityUnavailableError("User works");
  }
}
