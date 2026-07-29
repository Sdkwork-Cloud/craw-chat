declare module '@sdkwork/agents-pc-agents' {
  import type { ComponentType } from 'react';

  export interface AgentConfig {
    id?: string;
    name: string;
    description: string;
    avatar?: string;
    type: 'normal' | 'independent';
    systemPrompt?: string;
    knowledgeBaseIds?: string[];
    author?: string;
    users?: string;
    color?: string;
    iconName?: string;
    categoryId?: string;
    welcomeMessage?: string;
    debugMode?: boolean;
    jsonMode?: boolean;
    memoryEnabled?: boolean;
    model?: string;
    temperature?: number;
    suggestedPrompts?: string[];
    voiceIds?: string[];
    toolIds?: string[];
    skillIds?: string[];
  }

  export interface AgentService {
    createAgent(config: AgentConfig): Promise<AgentConfig>;
    updateAgent(id: string, config: Partial<AgentConfig>): Promise<AgentConfig>;
    publishAgent(id: string): Promise<void>;
    listAgentsPage(params?: {
      page?: number;
      pageSize?: number;
      scope?: 'market' | 'mine';
      q?: string;
    }): Promise<{
      items: AgentConfig[];
      pageInfo: { page: number; pageSize: number; hasMore: boolean };
    }>;
    getAgent(id: string): Promise<AgentConfig | null>;
    deleteAgent(id: string): Promise<void>;
  }

  export interface KnowledgeSelectionAdapter {
    getBasesPage(params?: { cursor?: string; pageSize?: number }): Promise<{
      items: unknown[];
      hasMore: boolean;
      nextCursor?: string;
    }>;
  }

  export interface Agent {
    id: string;
    name: string;
    desc: string;
    avatar: string;
    color: string;
    icon: unknown;
    author: string;
    users: string;
  }

  export const agentService: AgentService;
  export const DEFAULT_AGENT_CONFIG: Omit<AgentConfig, 'name' | 'description' | 'type'>;
  export const AgentView: ComponentType<{
    onStartChat(agent: Agent): void;
    onCreateAgent(): void;
    onEditAgent?(agentId: string): void;
  }>;
  export const CreateAgentModal: ComponentType<{
    isOpen: boolean;
    onClose(): void;
    onSuccess(agentId?: string): void;
  }>;
  export const CreateAgentView: ComponentType<{
    onBack(): void;
    initialAgentId?: string;
  }>;

  export function configureAgentService(getAgentClient?: () => unknown): AgentService;
  export function configureKnowledgeSelectionAdapter(adapter: KnowledgeSelectionAdapter): void;
  export function createKnowledgebaseSelectionAdapter(client: unknown): KnowledgeSelectionAdapter;
}
