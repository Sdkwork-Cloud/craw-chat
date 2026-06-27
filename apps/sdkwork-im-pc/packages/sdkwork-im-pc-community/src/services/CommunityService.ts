import type { CommunityAppSdkClient, CommunityCategory, CommunityEntry } from '@sdkwork/community-app-sdk';
import {
  communityCategoryRecord,
  communityEntryRecord,
  extractCommunityEntity,
  extractCommunityItems,
  readNumber,
  readOptionalString,
  readString,
} from '@sdkwork/im-pc-core/sdk/communityApiHelpers';
import { getCommunityAppSdkClientWithSession } from '@sdkwork/im-pc-core/sdk/communityAppSdkClient';

export interface Community {
  id: string;
  name: string;
  description: string;
  avatar: string;
  cover: string;
  membersCount: number;
  tags: string[];
  tabs?: string[];
}

export interface Post {
  id: string;
  communityId: string;
  author: {
    id: string;
    name: string;
    avatar: string;
  };
  content: string;
  images?: string[];
  likes: number;
  comments: number;
  createdAt: string;
}

export interface ResourceItem {
  id: string;
  name: string;
  size: string;
  type: string;
  uploader: string;
  uploadTime: string;
}

export type PlatformType =
  | "wechat"
  | "qq"
  | "feishu"
  | "dingtalk"
  | "telegram"
  | "discord"
  | "other";

export interface GroupQRCode {
  url: string;
  description: string;
}

export interface ChatGroup {
  id: string;
  name: string;
  platform: PlatformType;
  qrCodes: GroupQRCode[];
  memberCount: number;
  description: string;
}

export interface NewsItem {
  id: string;
  title: string;
  summary: string;
  content: string;
  source: string;
  time: string;
  views: number;
  comments: number;
  cover?: string;
}

export interface DocOutlineNode {
  id: string;
  title: string;
  children?: DocOutlineNode[];
}

export interface RepoItem {
  id: string;
  name: string;
  lang: string;
  desc: string;
  stars: string;
  forks: string;
  updated: string;
  color: string;
}

export interface SoftwareItem {
  id: string;
  name: string;
  desc: string;
  icon: string;
  cat: string;
}

export interface CommunityMember {
  id: string;
  name: string;
  role: 'admin' | 'member';
  joinedAt: string;
  avatar: string;
}

export interface CommunityService {
  getCommunities(): Promise<Community[]>;
  getCommunity(id: string): Promise<Community | undefined>;
  getPosts(communityId: string): Promise<Post[]>;
  getResources(communityId: string): Promise<ResourceItem[]>;
  getGroups(communityId: string): Promise<ChatGroup[]>;
  getNews(communityId: string): Promise<NewsItem[]>;
  getDocsOutline(communityId: string): Promise<DocOutlineNode[]>;
  getRepos(communityId: string): Promise<RepoItem[]>;
  getSoftware(communityId: string): Promise<SoftwareItem[]>;
  getMembers(communityId: string): Promise<CommunityMember[]>;
  updateCommunity(id: string, updates: Partial<Community>): Promise<Community>;
  createPost(communityId: string, content: string, images?: string[]): Promise<Post>;
  createComment(_communityId: string, _postId: string, _content: string): Promise<void>;
  toggleLikePost(postId: string): Promise<boolean>;
  createGroup(communityId: string, group: Omit<ChatGroup, "id">): Promise<ChatGroup>;
  updateGroup(communityId: string, groupId: string, group: Partial<ChatGroup>): Promise<void>;
  deleteGroup(communityId: string, groupId: string): Promise<void>;
  deletePost(communityId: string, postId: string): Promise<void>;
  uploadResource(
    communityId: string,
    resource: Omit<ResourceItem, "id" | "uploadTime">,
  ): Promise<ResourceItem>;
  deleteResource(communityId: string, resourceId: string): Promise<void>;
}

const PC_COMMUNITY_GROUPS_UNAVAILABLE = 'pc community groups contract is not available';
const PC_COMMUNITY_RESOURCES_UNAVAILABLE = 'pc community resources contract is not available';
const PC_COMMUNITY_NEWS_UNAVAILABLE = 'pc community news contract is not available';
const PC_COMMUNITY_DOCS_UNAVAILABLE = 'pc community docs contract is not available';
const PC_COMMUNITY_REPOS_UNAVAILABLE = 'pc community repos contract is not available';
const PC_COMMUNITY_SOFTWARE_UNAVAILABLE = 'pc community software contract is not available';
const PC_COMMUNITY_REACTIONS_UNAVAILABLE = 'pc community reactions contract is not available';
const PC_COMMUNITY_DELETE_UNAVAILABLE = 'pc community delete contract is not available';
const PC_COMMUNITY_SETTINGS_UNAVAILABLE = 'pc community settings contract is not available';
const PC_COMMUNITY_COMMENTS_UNAVAILABLE = 'pc community comments contract is not available';
const PC_COMMUNITY_MEMBERS_UNAVAILABLE = 'pc community members contract is not available';
const PC_COMMUNITY_MEDIA_UNAVAILABLE = 'pc community media contract is not available';

interface CommunityServiceOptions {
  client?: CommunityAppSdkClient;
}

function failClosed(message: string): never {
  throw new Error(message);
}

function mapCategoryToCommunity(category: CommunityCategory): Community {
  const record = communityCategoryRecord(category);
  return {
    id: readString(record, 'id'),
    name: readString(record, 'title', 'name'),
    description: readOptionalString(record, 'description') ?? '',
    avatar: '',
    cover: '',
    membersCount: 0,
    tags: [],
    tabs: ['posts'],
  };
}

function mapEntryToCommunity(entry: CommunityEntry): Community {
  const record = communityEntryRecord(entry);
  const authorRecord = asAuthorRecord(record.author);
  return {
    id: readString(record, 'id'),
    name: readString(record, 'title'),
    description: readOptionalString(record, 'excerpt', 'body') ?? '',
    avatar: readOptionalString(authorRecord, 'avatarUrl', 'avatar_url') ?? '',
    cover: '',
    membersCount: readNumber(asStatsRecord(record.stats), 'viewCount', 'view_count'),
    tags: Array.isArray(record.tags)
      ? record.tags.filter((tag): tag is string => typeof tag === 'string')
      : [],
    tabs: ['posts'],
  };
}

function mapEntryToPost(entry: CommunityEntry): Post {
  const record = communityEntryRecord(entry);
  const authorRecord = asAuthorRecord(record.author);
  const statsRecord = asStatsRecord(record.stats);
  return {
    id: readString(record, 'id'),
    communityId: readString(record, 'categoryId', 'category_id'),
    author: {
      id: readString(authorRecord, 'id'),
      name: readString(authorRecord, 'name'),
      avatar: readOptionalString(authorRecord, 'avatarUrl', 'avatar_url') ?? '',
    },
    content: readOptionalString(record, 'body', 'excerpt') ?? readString(record, 'title'),
    likes: readNumber(statsRecord, 'reactionCount', 'reaction_count'),
    comments: readNumber(statsRecord, 'commentCount', 'comment_count'),
    createdAt: readOptionalString(record, 'publishedAt', 'published_at', 'lastActivityAt', 'last_activity_at')
      ?? new Date().toISOString(),
  };
}

function asAuthorRecord(value: unknown): Record<string, unknown> {
  if (value != null && typeof value === 'object' && !Array.isArray(value)) {
    return value as Record<string, unknown>;
  }
  return {};
}

function asStatsRecord(value: unknown): Record<string, unknown> {
  if (value != null && typeof value === 'object' && !Array.isArray(value)) {
    return value as Record<string, unknown>;
  }
  return {};
}

class SdkworkCommunityService implements CommunityService {
  private readonly clientFactory: () => CommunityAppSdkClient;

  constructor(options: CommunityServiceOptions = {}) {
    this.clientFactory = () => options.client ?? getCommunityAppSdkClientWithSession();
  }

  private client(): CommunityAppSdkClient {
    return this.clientFactory();
  }

  async getCommunities(): Promise<Community[]> {
    const response = await this.client().community.categories.list();
    return extractCommunityItems<CommunityCategory>(response)
      .filter((category) => category.enabled !== false)
      .map(mapCategoryToCommunity);
  }

  async getCommunity(id: string): Promise<Community | undefined> {
    try {
      const entry = await this.client().community.entries.retrieve(id);
      if (entry?.id) {
        return mapEntryToCommunity(entry);
      }
    } catch {
      // Fall back to category lookup for legacy community ids.
    }
    const categories = await this.getCommunities();
    return categories.find((community) => community.id === id);
  }

  async getPosts(communityId: string): Promise<Post[]> {
    const response = await this.client().community.feed.list({ categoryId: communityId });
    return extractCommunityItems<CommunityEntry>(response).map(mapEntryToPost);
  }

  async getResources(_communityId: string): Promise<ResourceItem[]> {
    return [];
  }

  async getGroups(_communityId: string): Promise<ChatGroup[]> {
    return [];
  }

  async getNews(_communityId: string): Promise<NewsItem[]> {
    return [];
  }

  async getDocsOutline(_communityId: string): Promise<DocOutlineNode[]> {
    return [];
  }

  async getRepos(_communityId: string): Promise<RepoItem[]> {
    return [];
  }

  async getSoftware(_communityId: string): Promise<SoftwareItem[]> {
    return [];
  }

  async getMembers(_communityId: string): Promise<CommunityMember[]> {
    return [];
  }

  async updateCommunity(_id: string, _updates: Partial<Community>): Promise<Community> {
    failClosed(PC_COMMUNITY_SETTINGS_UNAVAILABLE);
  }

  async createPost(communityId: string, content: string, _images?: string[]): Promise<Post> {
    const trimmed = content.trim();
    if (!trimmed) {
      failClosed(PC_COMMUNITY_DELETE_UNAVAILABLE);
    }
    const created = await this.client().community.entries.create({
      categoryId: communityId,
      kind: 'discussion',
      title: trimmed.slice(0, 120) || 'Untitled',
      body: trimmed,
      excerpt: trimmed.slice(0, 240),
    });
    const entry = extractCommunityEntity<CommunityEntry>(created) ?? created;
    if (!entry?.id) {
      failClosed(PC_COMMUNITY_DELETE_UNAVAILABLE);
    }
    return mapEntryToPost(entry);
  }

  async createComment(
    _communityId: string,
    _postId: string,
    _content: string,
  ): Promise<void> {
    failClosed(PC_COMMUNITY_COMMENTS_UNAVAILABLE);
  }

  async toggleLikePost(_postId: string): Promise<boolean> {
    failClosed(PC_COMMUNITY_REACTIONS_UNAVAILABLE);
  }

  async createGroup(_communityId: string, _group: Omit<ChatGroup, "id">): Promise<ChatGroup> {
    failClosed(PC_COMMUNITY_GROUPS_UNAVAILABLE);
  }

  async updateGroup(
    _communityId: string,
    _groupId: string,
    _group: Partial<ChatGroup>,
  ): Promise<void> {
    failClosed(PC_COMMUNITY_GROUPS_UNAVAILABLE);
  }

  async deleteGroup(_communityId: string, _groupId: string): Promise<void> {
    failClosed(PC_COMMUNITY_GROUPS_UNAVAILABLE);
  }

  async deletePost(_communityId: string, _postId: string): Promise<void> {
    failClosed(PC_COMMUNITY_DELETE_UNAVAILABLE);
  }

  async uploadResource(
    _communityId: string,
    _resource: Omit<ResourceItem, "id" | "uploadTime">,
  ): Promise<ResourceItem> {
    failClosed(PC_COMMUNITY_RESOURCES_UNAVAILABLE);
  }

  async deleteResource(_communityId: string, _resourceId: string): Promise<void> {
    failClosed(PC_COMMUNITY_RESOURCES_UNAVAILABLE);
  }
}

export const communityService = new SdkworkCommunityService();

export {
  PC_COMMUNITY_COMMENTS_UNAVAILABLE,
  PC_COMMUNITY_MEDIA_UNAVAILABLE,
  PC_COMMUNITY_MEMBERS_UNAVAILABLE,
};
