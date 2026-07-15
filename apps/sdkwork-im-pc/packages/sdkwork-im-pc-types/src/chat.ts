import { Message } from './message';

export interface ChatAgentAssignment {
  agentId: string;
  revisionId?: string;
  /** Catalog presentation cached by the PC client; never used for authorization. */
  name?: string;
  avatar?: string;
  /** Disabled assignments remain visible in history but are not valid mention targets. */
  enabled?: boolean;
}

export interface Chat {
  id: string;
  name: string;
  avatar?: string;
  type: 'single' | 'group';
  lastMessage?: Message;
  unreadCount: number;
  updatedAt: number;
  memberCount?: number;
  /** True when the API only proves that at least `memberCount` members exist. */
  memberCountIsLowerBound?: boolean;
  activeCount?: number;
  isPinned?: boolean;
  isMuted?: boolean;
  isMarkedUnread?: boolean;
  notice?: string;
  welcomeMessage?: string;
  members?: string[];
  /** Synthetic group participants. They are not ordinary conversation members. */
  agentAssignments?: ChatAgentAssignment[];
  /** Optimistic version required by structured agent mentions and assignment replacement. */
  agentAssignmentGeneration?: number;
  /** Present only for an explicit create-time Knowledgebase initialization attempt. */
  knowledgebaseInitialization?: 'active' | 'provisioning' | 'failed';
}
