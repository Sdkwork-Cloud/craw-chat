import type {
  Community,
  CommunityGroup,
  CommunityMember,
  Post,
  Resource,
} from "../types";

export class CommunityCapabilityUnavailableError extends Error {
  constructor() {
    super("Community is unavailable because its owner SDK and permission model are not composed.");
    this.name = "CommunityCapabilityUnavailableError";
  }
}

export const CommunityService = {
  async getMembersByCommunity(_communityId: string): Promise<CommunityMember[]> {
    throw new CommunityCapabilityUnavailableError();
  },

  async updateMemberRole(
    _communityId: string,
    _memberId: string,
    _role: CommunityMember["role"],
  ): Promise<void> {
    throw new CommunityCapabilityUnavailableError();
  },

  async updateMemberStatus(
    _communityId: string,
    _memberId: string,
    _status: CommunityMember["status"],
  ): Promise<void> {
    throw new CommunityCapabilityUnavailableError();
  },

  async removeMember(_communityId: string, _memberId: string): Promise<void> {
    throw new CommunityCapabilityUnavailableError();
  },

  async createCommunity(
    _community: Omit<Community, "id" | "memberCount" | "postCount" | "isJoined">,
  ): Promise<Community> {
    throw new CommunityCapabilityUnavailableError();
  },

  async getCommunities(): Promise<Community[]> {
    throw new CommunityCapabilityUnavailableError();
  },

  async getCommunityById(_id: string): Promise<Community | undefined> {
    throw new CommunityCapabilityUnavailableError();
  },

  async joinCommunity(_id: string): Promise<void> {
    throw new CommunityCapabilityUnavailableError();
  },

  async getPostsByCommunity(_communityId: string): Promise<Post[]> {
    throw new CommunityCapabilityUnavailableError();
  },

  async createPost(
    _communityId: string,
    _content: string,
    _images?: string[],
  ): Promise<Post> {
    throw new CommunityCapabilityUnavailableError();
  },

  async addComment(_communityId: string, _postId: string, _text: string): Promise<void> {
    throw new CommunityCapabilityUnavailableError();
  },

  async toggleLikePost(_communityId: string, _postId: string): Promise<void> {
    throw new CommunityCapabilityUnavailableError();
  },

  async getResourcesByCommunity(_communityId: string): Promise<Resource[]> {
    throw new CommunityCapabilityUnavailableError();
  },

  async getGroupsByCommunity(_communityId: string): Promise<CommunityGroup[]> {
    throw new CommunityCapabilityUnavailableError();
  },

  async createGroup(
    _communityId: string,
    _group: Omit<CommunityGroup, "id" | "createdAt" | "communityId">,
  ): Promise<CommunityGroup> {
    throw new CommunityCapabilityUnavailableError();
  },

  async updateGroup(
    _communityId: string,
    _groupId: string,
    _data: Partial<CommunityGroup>,
  ): Promise<CommunityGroup> {
    throw new CommunityCapabilityUnavailableError();
  },

  async updateCommunity(_communityId: string, _updates: Partial<Community>): Promise<void> {
    throw new CommunityCapabilityUnavailableError();
  },

  async deleteGroup(_communityId: string, _groupId: string): Promise<void> {
    throw new CommunityCapabilityUnavailableError();
  },
};
