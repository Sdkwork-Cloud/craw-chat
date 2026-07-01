/** Host session snapshot passed into sibling capability PC packages at runtime. */
export interface HostCapabilitySessionSnapshot {
  authToken?: string;
  accessToken?: string;
  refreshToken?: string;
  sessionId?: string;
  user?: {
    id: string;
    displayName?: string;
    avatarUrl?: string;
    email?: string;
  };
  context?: {
    tenantId: string;
    userId: string;
    organizationId?: string;
    sessionId?: string;
    appId?: string;
    environment?: string;
    deploymentMode?: string;
    iamDeploymentMode?: string;
    authLevel?: string;
    dataScope?: string[];
    permissionScope?: string[];
    actorId?: string;
    actorKind?: string;
    deviceId?: string;
  };
  updatedAt?: string;
}

export interface DriveCapabilitySdkPorts {
  getDriveClient: () => unknown;
  readHostSession: () => HostCapabilitySessionSnapshot | null;
  subscribeHostSession?: (listener: () => void) => () => void;
  resolveHostLanguage?: () => string;
  subscribeHostLanguage?: (listener: (language: string) => void) => () => void;
}

export interface KnowledgebaseCapabilitySdkPorts {
  getKnowledgebaseClient: () => unknown;
  getDriveClient: () => unknown;
  readHostSession: () => HostCapabilitySessionSnapshot | null;
  subscribeHostSession?: (listener: () => void) => () => void;
  resolveHostLanguage?: () => string;
  subscribeHostLanguage?: (listener: (language: string) => void) => () => void;
}

export interface VoiceCapabilitySdkPorts {
  getVoiceClient: () => unknown;
  readHostSession: () => HostCapabilitySessionSnapshot | null;
  subscribeHostSession?: (listener: () => void) => () => void;
  resolveHostLanguage?: () => string;
  subscribeHostLanguage?: (listener: (language: string) => void) => () => void;
}
