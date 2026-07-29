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

export class AgentCapabilityUnavailableError extends Error {
  constructor() {
    super("Agent catalog and lifecycle operations are not exposed by an approved owner SDK.");
    this.name = "AgentCapabilityUnavailableError";
  }
}

function unavailable<T>(): Promise<T> {
  return Promise.reject(new AgentCapabilityUnavailableError());
}

export const AgentService = {
  getAgents(): Promise<Agent[]> {
    return unavailable();
  },
  getMyAgents(): Promise<Agent[]> {
    return unavailable();
  },
  createAgent(_data: Partial<Agent>): Promise<Agent> {
    return unavailable();
  },
  updateAgent(_id: string, _data: Partial<Agent>): Promise<Agent> {
    return unavailable();
  },
  deleteAgent(_id: string): Promise<void> {
    return unavailable();
  },
  getAgentById(_id: string): Promise<Agent | undefined> {
    return unavailable();
  },
  getCategories(): Promise<string[]> {
    return unavailable();
  },
  getHotSearches(): Promise<string[]> {
    return unavailable();
  },
  createAgentChat(_agentName: string, _greeting: string): Promise<never> {
    return unavailable();
  },
};
