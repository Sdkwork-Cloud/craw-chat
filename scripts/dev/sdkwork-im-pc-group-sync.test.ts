import assert from 'node:assert/strict';
import type { ConversationMember, ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkGroupService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/GroupService';

type ListMembersParams = {
  cursor?: string;
  limit?: number;
};

type GroupSyncCall =
  | {
      conversationId: string;
      method: 'listMembers';
      params?: ListMembersParams;
    }
  | {
      body: {
        memberId: string;
      };
      conversationId: string;
      method: 'removeMember';
    };

const calls: GroupSyncCall[] = [];
const removedMemberIds = new Set<string>();

function createMember(principalId: string, memberId = `member-${principalId}`): ConversationMember {
  return {
    attributes: {},
    conversationId: 'group-1',
    joinedAt: '2026-06-04T00:00:00.000Z',
    memberId,
    principalId,
    principalKind: 'user',
    role: principalId === 'u_owner' ? 'owner' : 'member',
    state: 'joined',
    tenantId: '100001',
  };
}

const baseMembers = [
  createMember('u_owner'),
  ...Array.from({ length: 99 }, (_value, index) => createMember(`u_member_${String(index + 1).padStart(3, '0')}`)),
  createMember('u_target', 'member-target'),
  createMember('u_after_target'),
];

function currentMembers(): ConversationMember[] {
  return baseMembers.filter((member) => !removedMemberIds.has(member.memberId));
}

function pageMembers(params: ListMembersParams | undefined): {
  hasMore: boolean;
  items: ConversationMember[];
  nextCursor?: string;
} {
  const limit = params?.limit ?? 100;
  const offset = params?.cursor ? Number(params.cursor) : 0;
  const members = currentMembers();
  const nextOffset = offset + limit;
  const hasMore = nextOffset < members.length;
  return {
    items: members.slice(offset, nextOffset),
    hasMore,
    ...(hasMore ? { nextCursor: String(nextOffset) } : {}),
  };
}

const fakeClient = {
  conversations: {
    async listMembers(
      conversationId: string,
      params?: ListMembersParams,
    ) {
      calls.push({ method: 'listMembers', conversationId, params });
      return pageMembers(params);
    },
    async removeMember(
      conversationId: string,
      body: { memberId: string },
    ) {
      calls.push({ method: 'removeMember', conversationId, body });
      removedMemberIds.add(body.memberId);
      return {
        ...createMember('u_target', body.memberId),
        state: 'removed',
      };
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkGroupService(() => fakeClient);

  await service.removeMember('group-1', 'u_target');

  const listCalls = calls.filter((call) => call.method === 'listMembers');
  assert.equal(
    listCalls.length,
    4,
    'group member reads must page both the target lookup and the follow-up view-state sync',
  );
  assert.deepEqual(
    listCalls.map((call) => call.params),
    [
      { limit: 100 },
      { limit: 100, cursor: '100' },
      { limit: 100 },
      { limit: 100, cursor: '100' },
    ],
  );
  assert.deepEqual(
    calls.find((call) => call.method === 'removeMember'),
    {
      method: 'removeMember',
      conversationId: 'group-1',
      body: {
        memberId: 'member-target',
      },
    },
  );

  console.log('sdkwork-im-pc group member sync contract passed');
}

void main();
