import { useTranslation } from "react-i18next";
import { ChatService } from "@sdkwork/im-h5-chat";

export interface Agent {
  id: string;
  name: string;
  desc: string;
  users: string;
  author: string;
  iconName?: string;
  color?: string;
  isOfficial?: boolean;
  avatar?: string | null;
  kbId?: string;
  prompt?: string;
}

const INITIAL_AGENTS: Agent[] = [
  {
    id: "a1",
    name: "学习小帮手",
    desc: "可以解答从小学到大学的各科题目，包括数学、语文、英语、思政、经济学...",
    users: "2262.7 万人聊过",
    author: "@豆包爱学",
    iconName: "Bot",
    color: "bg-[#E8F3FF] text-[#2B5CE7]",
    isOfficial: true,
    avatar: "https://picsum.photos/seed/agent1/100/100",
  },
  {
    id: "a2",
    name: "超爱聊天的小宁",
    desc: "你的专属AI好友，善解人意，可以分享、交流生活中的一切喜怒哀乐",
    users: "1962.5 万人聊过",
    author: "@豆包官方",
    iconName: "Bot",
    color: "bg-[#FFF0F0] text-[#F53F3F]",
    isOfficial: true,
    avatar: "https://picsum.photos/seed/agent2/100/100",
  },
  {
    id: "a3",
    name: "英语外教 Owen",
    desc: "Passionate and open-minded English foreign teacher",
    users: "1909.9 万人聊过",
    author: "@豆包官方",
    iconName: "Bot",
    color: "bg-[#F0FDF4] text-[#00B42A]",
    isOfficial: true,
    avatar: "https://picsum.photos/seed/agent3/100/100",
  },
  {
    id: "a4",
    name: "全能写作助手",
    desc: "提供多种文案创作选择，轻松完成各种文案任务。",
    users: "1330.5 万人聊过",
    author: "@豆包官方",
    iconName: "PenTool",
    color: "bg-[#FFF7E8] text-[#FF7D00]",
    isOfficial: true,
    avatar: null,
  },
];

export let MOCK_AGENTS: Agent[] = [];

const STORAGE_KEY = "clawchat_agents";

const loadAgents = () => {
  if (MOCK_AGENTS.length > 0) return MOCK_AGENTS;
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      MOCK_AGENTS = JSON.parse(data);
    } else {
      MOCK_AGENTS = [...INITIAL_AGENTS];
    }
  } catch (e) {
    MOCK_AGENTS = [...INITIAL_AGENTS];
  }
  return MOCK_AGENTS;
};

const saveAgents = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(MOCK_AGENTS));
  } catch (e) {
    console.error("Failed to save agents", e);
  }
};

const MY_AGENTS_STORAGE_KEY = "clawchat_my_agents";

const loadMyAgents = (): Agent[] => {
  try {
    const data = localStorage.getItem(MY_AGENTS_STORAGE_KEY);
    if (data) {
      return JSON.parse(data);
    }
  } catch (e) {
    console.error("Failed to load my agents", e);
  }
  return [];
};

const saveMyAgents = (agents: Agent[]) => {
  try {
    localStorage.setItem(MY_AGENTS_STORAGE_KEY, JSON.stringify(agents));
  } catch (e) {
    console.error("Failed to save my agents", e);
  }
};

export const AgentService = {
  async getAgents(): Promise<Agent[]> {
    return [...loadAgents()];
  },
  async getMyAgents(): Promise<Agent[]> {
    return loadMyAgents();
  },
  async createAgent(data: Partial<Agent>): Promise<Agent> {
    const agents = loadMyAgents();
    const newAgent: Agent = {
      id: `agent_u_${Date.now()}`,
      name: data.name || "Untitled Agent",
      desc: data.desc || "",
      users: "0 人聊过",
      author: "@我",
      iconName: data.iconName || "Bot",
      color: "bg-[#F0FDF4] text-[#00B42A]",
      isOfficial: false,
      avatar: data.avatar || `https://picsum.photos/seed/${Date.now()}/200/200`,
      kbId: data.kbId,
      prompt: data.prompt
    } as Agent;
    agents.push(newAgent);
    saveMyAgents(agents);
    return newAgent;
  },
  async updateAgent(id: string, data: Partial<Agent>): Promise<Agent> {
    const agents = loadMyAgents();
    const index = agents.findIndex(a => a.id === id);
    if (index !== -1) {
      agents[index] = { ...agents[index], ...data };
      saveMyAgents(agents);
      return agents[index];
    }
    throw new Error("Agent not found");
  },
  async deleteAgent(id: string): Promise<void> {
    const agents = loadMyAgents();
    const filtered = agents.filter(a => a.id !== id);
    saveMyAgents(filtered);
  },
  async getAgentById(id: string): Promise<Agent | undefined> {
    const allAgents = [...loadAgents(), ...loadMyAgents()];
    return allAgents.find((a) => a.id === id);
  },
  async getCategories(): Promise<string[]> {
    return [
      "精选",
      "拍照问",
      "学习",
      "工作",
      "创作",
      "生活",
      "娱乐",
      "情感",
      "游戏",
      "编程",
      "绘画",
    ];
  },
  async getHotSearches(): Promise<string[]> {
    return [
      "小红书文案",
      "英语口语教练",
      "塔罗牌占卜",
      "简历优化",
      "Midjourney 提示词",
      "代码解释器",
    ];
  },
  async createAgentChat(agentName: string, greeting: string): Promise<any> {
    const agentUser: import("@sdkwork/im-h5-types").User = {
      id: `agent_${Date.now()}`,
      name: agentName,
      avatar: `https://picsum.photos/seed/${agentName}/200/200`,
      status: "online",
    };
    return ChatService.createDirectChat(
      agentUser,
      greeting || `你好！我是${agentName}，很高兴为你服务。`,
    );
  },
};
