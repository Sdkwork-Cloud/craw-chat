import type { User } from "@sdkwork/im-h5-types";

import { UserCapabilityUnavailableError } from "./UserCapabilityUnavailableError";

export interface Moment {
  id: string;
  author: User;
  content: string;
  images?: string[];
  video?: string;
  timestamp: number;
  likes: string[];
  comments: { id: string; authorName: string; content: string }[];
}

export const MomentService = {
  async getMoments(): Promise<Moment[]> {
    throw new UserCapabilityUnavailableError("Moments");
  },

  async addMoment(
    _content: string,
    _images?: string[],
    _video?: string,
    _authorProfile?: Partial<User>,
  ): Promise<Moment> {
    throw new UserCapabilityUnavailableError("Moments");
  },

  async toggleLike(_momentId: string, _userId: string): Promise<void> {
    throw new UserCapabilityUnavailableError("Moments");
  },

  async addComment(
    _momentId: string,
    _authorName: string,
    _content: string,
  ): Promise<void> {
    throw new UserCapabilityUnavailableError("Moments");
  },

  async deleteMoment(_momentId: string): Promise<void> {
    throw new UserCapabilityUnavailableError("Moments");
  },
};
