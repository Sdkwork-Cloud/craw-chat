import { chatService, type ChatOfflineSyncResult, type ChatService } from './ChatService';
import { contactService, type ContactService, type ContactSyncResult } from './ContactService';
import { groupService, type GroupMemberSyncChange, type GroupService } from './GroupService';

export interface ImStartupSyncOptions {}

export interface RecoveredRtcSessionResult {
  rtcSessionId: string;
  snapshot: unknown;
}

export interface ImStartupSyncError {
  stage: 'chat' | 'contacts' | 'groups';
  message: string;
}

export interface ImStartupSyncResult {
  chat?: ChatOfflineSyncResult;
  contacts?: ContactSyncResult;
  errors: ImStartupSyncError[];
  groups?: GroupMemberSyncChange[];
  recoveredRtcSessions: RecoveredRtcSessionResult[];
}

export interface ImSyncCoordinatorService {
  syncStartup(options?: ImStartupSyncOptions): Promise<ImStartupSyncResult>;
}

export interface ImSyncCoordinatorServiceDependencies {
  chatService?: Pick<ChatService, 'syncOfflineMessages'>;
  contactService?: Pick<ContactService, 'syncContacts'>;
  groupService?: Pick<GroupService, 'syncGroupMembers'>;
}

function toErrorMessage(error: unknown): string {
  if (error instanceof Error && error.message) {
    return error.message;
  }
  return typeof error === 'string' && error.trim().length > 0 ? error : 'IM startup sync failed';
}

class SdkworkImSyncCoordinatorService implements ImSyncCoordinatorService {
  private readonly chatService: Pick<ChatService, 'syncOfflineMessages'>;
  private readonly contactService: Pick<ContactService, 'syncContacts'>;
  private readonly groupService: Pick<GroupService, 'syncGroupMembers'>;

  constructor(dependencies: ImSyncCoordinatorServiceDependencies = {}) {
    this.chatService = dependencies.chatService ?? chatService;
    this.contactService = dependencies.contactService ?? contactService;
    this.groupService = dependencies.groupService ?? groupService;
  }

  async syncStartup(_options: ImStartupSyncOptions = {}): Promise<ImStartupSyncResult> {
    const result: ImStartupSyncResult = {
      errors: [],
      recoveredRtcSessions: [],
    };

    try {
      result.chat = await this.chatService.syncOfflineMessages();
    } catch (error) {
      result.errors.push({ stage: 'chat', message: toErrorMessage(error) });
    }

    try {
      result.contacts = await this.contactService.syncContacts();
    } catch (error) {
      result.errors.push({ stage: 'contacts', message: toErrorMessage(error) });
    }

    try {
      result.groups = await this.groupService.syncGroupMembers();
    } catch (error) {
      result.errors.push({ stage: 'groups', message: toErrorMessage(error) });
    }

    return result;
  }
}

export function createSdkworkImSyncCoordinatorService(
  dependencies?: ImSyncCoordinatorServiceDependencies,
): ImSyncCoordinatorService {
  return new SdkworkImSyncCoordinatorService(dependencies);
}

export const imSyncCoordinatorService = createSdkworkImSyncCoordinatorService();
