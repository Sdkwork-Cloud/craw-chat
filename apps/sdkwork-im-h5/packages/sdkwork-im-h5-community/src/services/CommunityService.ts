import { useTranslation } from "react-i18next";
import { Community, Post, Resource } from '../types';

let MOCK_COMMUNITIES: Community[] = [
  {
    id: "comm_1",
    name: "AI 开发者联盟",
    description: "专注人工智能、大模型、AIGC技术交流与落地应用的实战社区。",
    coverImage: "https://picsum.photos/seed/comm_1/400/200",
    memberCount: 12500,
    postCount: 3420,
    tags: ["AI", "大模型", "AIGC", "开发"],
    isJoined: true
  },
  {
    id: "comm_2",
    name: "产品经理交流圈",
    description: "分享产品方法论、行业洞察、好书推荐。致力于培养顶尖产品经理。",
    coverImage: "https://picsum.photos/seed/comm_2/400/200",
    memberCount: 8430,
    postCount: 1280,
    tags: ["产品", "商业", "增长"],
    isJoined: false,
    isPaid: true,
    price: 99
  },
  {
    id: "comm_3",
    name: "独立开发者聚集地",
    description: "Indie Hackers, 分享一人公司的开发经验、出海经验和变现思路。",
    coverImage: "https://picsum.photos/seed/comm_3/400/200",
    memberCount: 5200,
    postCount: 890,
    tags: ["独立开发", "出海", "搞钱"],
    isPaid: true,
    price: 199
  }
];

let MOCK_POSTS: Record<string, Post[]> = {
  "comm_1": [
    {
      id: "post_1",
      communityId: "comm_1",
      authorId: "user_1",
      authorName: "AI 极客",
      authorAvatar: "https://i.pravatar.cc/150?u=user_1",
      content: "今天开源了一个基于本地大模型的RAG问答项目，支持LangChain，可以直接平替各种昂贵的API，大家去我的Github支持下求个Star！",
      createdAt: "2026-05-28T01:30:00Z",
      likes: 124,
      comments: 2,
      commentsList: [
        { id: "cm_1", authorName: "飞翔的企鹅", content: "必须支持！期待更新~", createdAt: "2026-05-28T02:10:00Z" },
        { id: "cm_2", authorName: "Alex", content: "太棒了，已经star了", createdAt: "2026-05-28T03:45:00Z" }
      ],
      isLiked: true
    },
    {
      id: "post_2",
      communityId: "comm_1",
      authorId: "user_2",
      authorName: "算法打工人",
      authorAvatar: "https://i.pravatar.cc/150?u=user_2",
      content: "今年大模型在自动驾驶方向有没有搞头？感觉年底几家大厂又要卷出天际了...",
      images: ["https://picsum.photos/seed/post2_img/300/200"],
      createdAt: "2026-05-27T14:15:00Z",
      likes: 45,
      comments: 0
    }
  ]
};

let MOCK_RESOURCES: Record<string, Resource[]> = {
  "comm_1": [
    {
      id: "res_1",
      communityId: "comm_1",
      title: "2026年AI行业发展白皮书.pdf",
      type: "pdf",
      size: "4.5MB",
      url: "#",
      uploadedBy: "Admin",
      createdAt: "2026-05-25T10:00:00Z"
    },
    {
      id: "res_2",
      communityId: "comm_1",
      title: "斯坦福深度学习课程笔记.md",
      type: "doc",
      size: "1.2MB",
      url: "#",
      uploadedBy: "LearnBot",
      createdAt: "2026-05-20T12:00:00Z"
    }
  ]
};

import { CommunityGroup, CommunityMember } from '../types';

let MOCK_GROUPS: Record<string, CommunityGroup[]> = {
  "comm_1": [
    {
      id: "grp_1",
      communityId: "comm_1",
      name: "AI 开发者交流微信1群",
      platform: "wechat",
      description: "日常交流AI前沿资讯、开发心得",
      memberCount: 450,
      qrCodes: [{ url: "https://picsum.photos/seed/wxqr/200/200", description: "扫码加入1群" }],
      createdAt: "2026-01-10T10:00:00Z"
    },
    {
      id: "grp_2",
      communityId: "comm_1",
      name: "LangChain 开源贡献者群",
      platform: "dingtalk",
      description: "仅限代码贡献者加入",
      memberCount: 120,
      qrCodes: [{ url: "https://picsum.photos/seed/dingqr/200/200", description: "扫码加入开源群" }],
      createdAt: "2026-02-15T12:00:00Z"
    },
    {
      id: "grp_3",
      communityId: "comm_1",
      name: "Discord 高频讨论组",
      platform: "discord",
      description: "海内外开发者无缝交流",
      memberCount: 2200,
      qrCodes: [{ url: "https://picsum.photos/seed/discordqr/200/200", description: "Discord 群入口" }],
      createdAt: "2026-03-20T14:00:00Z"
    }
  ]
};

let MOCK_MEMBERS: Record<string, CommunityMember[]> = {
  "comm_1": [
    {
      id: "mem_1",
      communityId: "comm_1",
      name: "Alice",
      avatar: "https://i.pravatar.cc/150?u=mem_1",
      role: "owner",
      joinDate: "2023-01-10T10:00:00Z",
      status: "active",
      bio: "Founder of AI 开发者联盟"
    },
    {
      id: "mem_2",
      communityId: "comm_1",
      name: "Bob",
      avatar: "https://i.pravatar.cc/150?u=mem_2",
      role: "admin",
      joinDate: "2023-02-15T12:00:00Z",
      status: "active",
      bio: "Fullstack AI Engineer"
    },
    {
      id: "mem_3",
      communityId: "comm_1",
      name: "Charlie",
      avatar: "https://i.pravatar.cc/150?u=mem_3",
      role: "member",
      joinDate: "2023-05-20T14:00:00Z",
      status: "active"
    },
    {
      id: "mem_4",
      communityId: "comm_1",
      name: "David",
      avatar: "https://i.pravatar.cc/150?u=mem_4",
      role: "member",
      joinDate: "2023-06-10T09:00:00Z",
      status: "muted"
    },
    {
      id: "mem_5",
      communityId: "comm_1",
      name: "Eve",
      avatar: "https://i.pravatar.cc/150?u=mem_5",
      role: "member",
      joinDate: "2023-08-01T16:00:00Z",
      status: "banned"
    }
  ]
};

export const CommunityService = {
  getMembersByCommunity: async (communityId: string): Promise<CommunityMember[]> => {
    return new Promise(resolve => setTimeout(() => resolve([...(MOCK_MEMBERS[communityId] || [])]), 200));
  },

  updateMemberRole: async (communityId: string, memberId: string, role: CommunityMember['role']): Promise<void> => {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        const members = MOCK_MEMBERS[communityId];
        const member = members?.find(m => m.id === memberId);
        if (member) {
          member.role = role;
          resolve();
        } else {
          reject(new Error("Member not found"));
        }
      }, 300);
    });
  },

  updateMemberStatus: async (communityId: string, memberId: string, status: CommunityMember['status']): Promise<void> => {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        const members = MOCK_MEMBERS[communityId];
        const member = members?.find(m => m.id === memberId);
        if (member) {
          member.status = status;
          resolve();
        } else {
          reject(new Error("Member not found"));
        }
      }, 300);
    });
  },

  removeMember: async (communityId: string, memberId: string): Promise<void> => {
    return new Promise(resolve => {
      setTimeout(() => {
        if (MOCK_MEMBERS[communityId]) {
          MOCK_MEMBERS[communityId] = MOCK_MEMBERS[communityId].filter(m => m.id !== memberId);
        }
        resolve();
      }, 400);
    });
  },

  createCommunity: async (community: Omit<Community, 'id' | 'memberCount' | 'postCount' | 'isJoined'>): Promise<Community> => {
    return new Promise(resolve => {
      setTimeout(() => {
        const newCommunity: Community = {
          ...community,
          id: `comm_${Date.now()}`,
          memberCount: 1,
          postCount: 0,
          isJoined: true
        };
        MOCK_COMMUNITIES.unshift(newCommunity);
        resolve({...newCommunity});
      }, 400);
    });
  },

  getCommunities: async (): Promise<Community[]> => {
    return new Promise(resolve => setTimeout(() => resolve([...MOCK_COMMUNITIES]), 300));
  },

  getCommunityById: async (id: string): Promise<Community | undefined> => {
    return new Promise(resolve => setTimeout(() => resolve(MOCK_COMMUNITIES.find(c => c.id === id)), 200));
  },

  joinCommunity: async (id: string): Promise<void> => {
    return new Promise(resolve => setTimeout(() => {
      const idx = MOCK_COMMUNITIES.findIndex(c => c.id === id);
      if (idx > -1) {
        MOCK_COMMUNITIES[idx].isJoined = true;
        MOCK_COMMUNITIES[idx].memberCount++;
        
        // Ensure there is a default document upon joining
        if (!MOCK_RESOURCES[id]) {
          MOCK_RESOURCES[id] = [];
        }
        if (MOCK_RESOURCES[id].length === 0) {
          MOCK_RESOURCES[id].push({
            id: `res_default_${Date.now()}`,
            communityId: id,
            title: "新手必看：圈子玩法指南与核心文档.pdf",
            type: "pdf",
            size: "2.5MB",
            url: "#",
            uploadedBy: "圈主",
            createdAt: new Date().toISOString()
          });
        }
      }
      resolve();
    }, 400));
  },

  getPostsByCommunity: async (communityId: string): Promise<Post[]> => {
    return new Promise(resolve => setTimeout(() => resolve([...(MOCK_POSTS[communityId] || [])]), 200));
  },

  createPost: async (communityId: string, content: string, images?: string[]): Promise<Post> => {
    return new Promise((resolve) => {
      setTimeout(() => {
        const newPost: Post = {
          id: `post_${Date.now()}`,
          communityId,
          authorId: "me",
          authorName: "我",
          authorAvatar: "https://i.pravatar.cc/150?u=me",
          content,
          images,
          createdAt: new Date().toISOString(),
          likes: 0,
          comments: 0,
          commentsList: []
        };
        if (!MOCK_POSTS[communityId]) {
          MOCK_POSTS[communityId] = [];
        }
        MOCK_POSTS[communityId].unshift(newPost);
        resolve({...newPost});
      }, 500);
    });
  },

  addComment: async (communityId: string, postId: string, text: string): Promise<void> => {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        const posts = MOCK_POSTS[communityId];
        const post = posts?.find(p => p.id === postId);
        if (post) {
          if (!post.commentsList) post.commentsList = [];
          post.commentsList.push({
            id: `cmt_${Date.now()}`,
            authorName: "我",
            content: text,
            createdAt: new Date().toISOString()
          });
          post.comments++;
          resolve();
        } else {
          reject(new Error("Post not found"));
        }
      }, 300);
    });
  },

  toggleLikePost: async (communityId: string, postId: string): Promise<void> => {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        const posts = MOCK_POSTS[communityId];
        const post = posts?.find(p => p.id === postId);
        if (post) {
          post.isLiked = !post.isLiked;
          post.likes += post.isLiked ? 1 : -1;
          resolve();
        } else {
          reject(new Error("Post not found"));
        }
      }, 200);
    });
  },

  getResourcesByCommunity: async (communityId: string): Promise<Resource[]> => {
    return new Promise(resolve => setTimeout(() => resolve([...(MOCK_RESOURCES[communityId] || [])]), 200));
  },

  getGroupsByCommunity: async (communityId: string): Promise<CommunityGroup[]> => {
    return new Promise(resolve => setTimeout(() => resolve([...(MOCK_GROUPS[communityId] || [])]), 200));
  },

  createGroup: async (communityId: string, group: Omit<CommunityGroup, 'id' | 'createdAt' | 'communityId'>): Promise<CommunityGroup> => {
    return new Promise(resolve => {
      setTimeout(() => {
        const newGroup: CommunityGroup = {
          ...group,
          id: `grp_${Date.now()}`,
          communityId,
          createdAt: new Date().toISOString(),
        };
        if (!MOCK_GROUPS[communityId]) MOCK_GROUPS[communityId] = [];
        MOCK_GROUPS[communityId].push(newGroup);
        resolve({...newGroup});
      }, 400);
    });
  },

  updateGroup: async (communityId: string, groupId: string, data: Partial<CommunityGroup>): Promise<CommunityGroup> => {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        const groups = MOCK_GROUPS[communityId];
        const idx = groups?.findIndex(g => g.id === groupId);
        if (idx !== undefined && idx > -1) {
          groups[idx] = { ...groups[idx], ...data };
          resolve({...groups[idx]});
        } else {
          reject(new Error("Group not found"));
        }
      }, 400);
    });
  },

  updateCommunity: async (communityId: string, updates: Partial<Community>): Promise<void> => {
    return new Promise(resolve => {
      setTimeout(() => {
        const idx = MOCK_COMMUNITIES.findIndex(c => c.id === communityId);
        if (idx > -1) {
          MOCK_COMMUNITIES[idx] = { ...MOCK_COMMUNITIES[idx], ...updates };
        }
        resolve();
      }, 400);
    });
  },

  deleteGroup: async (communityId: string, groupId: string): Promise<void> => {
    return new Promise(resolve => {
      setTimeout(() => {
        if (MOCK_GROUPS[communityId]) {
          MOCK_GROUPS[communityId] = MOCK_GROUPS[communityId].filter(g => g.id !== groupId);
        }
        resolve();
      }, 400);
    });
  }
};
