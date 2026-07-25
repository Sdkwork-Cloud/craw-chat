export interface Community {
  id: string;
  name: string;
  description: string;
  coverImage: string;
  avatar?: string;
  memberCount: number;
  postCount: number;
  tags: string[];
  tabs?: string[];
  isJoined?: boolean;
  isPaid?: boolean;
  price?: number;
}

export interface PostComment {
  id: string;
  authorName: string;
  content: string;
  createdAt: string;
}

export interface Post {
  id: string;
  communityId: string;
  authorId: string;
  authorName: string;
  authorAvatar: string;
  content: string;
  images?: string[];
  createdAt: string;
  likes: number;
  comments: number;
  commentsList?: PostComment[];
  isLiked?: boolean;
}

export interface QRCodeItem {
  url: string;
  description: string;
}

export interface CommunityGroup {
  id: string;
  communityId: string;
  name: string;
  platform: 'wechat' | 'qq' | 'feishu' | 'dingtalk' | 'telegram' | 'discord' | 'whatsapp' | 'other';
  description?: string;
  memberCount: number;
  qrCodeUrl?: string; // Legacy
  qrCodes?: QRCodeItem[];
  createdAt: string;
}

export interface Resource {
  id: string;
  communityId: string;
  title: string;
  type: string; // 'pdf', 'doc', 'link', 'video'
  size?: string;
  url: string;
  uploadedBy: string;
  createdAt: string;
}

export interface CommunityMember {
  id: string;
  communityId: string;
  name: string;
  avatar: string;
  role: 'owner' | 'admin' | 'member';
  joinDate: string;
  status: 'active' | 'muted' | 'banned';
  bio?: string;
}
