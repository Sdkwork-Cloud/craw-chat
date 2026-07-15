import type { ConversationMember } from '@sdkwork/im-sdk';

export interface GroupKnowledgebaseMemberAccessPolicy {
  canInitialize: boolean;
  canOpen: boolean;
}

export function resolveCurrentGroupKnowledgebaseMemberAccess(
  member: Pick<ConversationMember, 'role' | 'state'>,
): GroupKnowledgebaseMemberAccessPolicy {
  if (String(member.state).toLowerCase() !== 'joined') {
    return { canInitialize: false, canOpen: false };
  }

  switch (String(member.role).toLowerCase()) {
    case 'owner':
      return { canInitialize: true, canOpen: true };
    case 'admin':
    case 'member':
      return { canInitialize: false, canOpen: true };
    case 'guest':
    default:
      return { canInitialize: false, canOpen: false };
  }
}

export function isCurrentGroupOwnerMember(
  member: Pick<ConversationMember, 'role' | 'state'>,
): boolean {
  return resolveCurrentGroupKnowledgebaseMemberAccess(member).canInitialize;
}
