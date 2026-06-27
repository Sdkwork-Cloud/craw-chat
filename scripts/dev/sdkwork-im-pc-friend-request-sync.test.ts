import assert from 'node:assert/strict';
import type {
  FriendRequest,
  ImConnectOptions,
  ImLiveConnection,
  ImRealtimeEventContext,
  ImSdkClient,
} from '@sdkwork/im-sdk';
import { createSdkworkContactService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ContactService';

type FriendRequestDirection = 'incoming' | 'outgoing';

type FriendRequestListParams = {
  cursor?: string;
  direction: FriendRequestDirection;
  limit?: number;
  status?: 'pending' | 'accepted' | 'declined' | 'canceled' | 'expired' | 'all';
};

const friendRequestCalls: FriendRequestListParams[] = [];
const userSearchCalls: Array<{ limit?: number; q?: string }> = [];
let incomingPendingCount = 2;
let realtimeConnectOptions: ImConnectOptions | undefined;
let realtimeEventHandler:
  | ((event: Record<string, unknown>, context: ImRealtimeEventContext) => void)
  | undefined;

function createFriendRequest(
  requestId: string,
  direction: FriendRequestDirection,
  status: FriendRequest['status'] = 'pending',
): FriendRequest {
  const currentUserId = 'current-user';
  const peerUserId = `${direction}-peer-${requestId}`;
  return {
    createdAt: '2026-06-04T00:00:00.000Z',
    requestId,
    requestMessage: `request ${requestId}`,
    requesterUserId: direction === 'incoming' ? peerUserId : currentUserId,
    status,
    targetUserId: direction === 'incoming' ? currentUserId : peerUserId,
    tenantId: '100001',
    updatedAt: '2026-06-04T00:00:00.000Z',
  };
}

function pageFriendRequests(params: FriendRequestListParams): {
  items: FriendRequest[];
  nextCursor?: string;
} {
  if (params.direction === 'incoming' && incomingPendingCount === 1) {
    return {
      items: [createFriendRequest('incoming-1', 'incoming')],
    };
  }
  const page = params.cursor ?? '0';
  if (page === '0') {
    return {
      items: [createFriendRequest(`${params.direction}-1`, params.direction)],
      nextCursor: '1',
    };
  }
  if (page === '1') {
    return {
      items: [createFriendRequest(`${params.direction}-2`, params.direction)],
    };
  }
  return { items: [] };
}

const fakeClient = {
  async connect(options?: ImConnectOptions) {
    realtimeConnectOptions = options;
    return {
      disconnect() {
        realtimeEventHandler = undefined;
      },
      events: {
        onConversation() {
          return () => undefined;
        },
        onScope(scopeType: string, scopeId: string, handler: (event: Record<string, unknown>, context: ImRealtimeEventContext) => void) {
          assert.equal(scopeType, 'user', 'friend request realtime must subscribe to the current user scope');
          assert.equal(scopeId, 'current-user', 'friend request realtime must use the current authenticated user id');
          realtimeEventHandler = handler;
          return () => {
            if (realtimeEventHandler === handler) {
              realtimeEventHandler = undefined;
            }
          };
        },
      },
      lifecycle: {
        onError() {
          return () => undefined;
        },
        onStateChange() {
          return () => undefined;
        },
      },
      messages: {
        onConversation() {
          return () => undefined;
        },
      },
      subscriptions: {
        syncConversations() {
          return undefined;
        },
        syncScopes() {
          return undefined;
        },
      },
    } satisfies ImLiveConnection;
  },
  social: {
    users: {
      async list(params: { limit?: number; q?: string }) {
        userSearchCalls.push(params);
        const userId = params.q;
        if (!userId) {
          return { items: [], hasMore: false };
        }
        return {
          items: [
            {
              avatarUrl: `https://cdn.example.test/${encodeURIComponent(userId)}.png`,
              displayName: `Profile ${userId}`,
              relationshipState: 'none',
              tenantId: '100001',
              userId,
            },
          ],
          hasMore: false,
        };
      },
    },
    friendRequests: {
      async accept() {
        incomingPendingCount = 1;
        return {
          friendship: {
            friendshipId: 'friendship-1',
            initiatorUserId: 'incoming-peer-incoming-1',
            tenantId: '100001',
            userHighId: 'incoming-peer-incoming-1',
            userLowId: 'current-user',
          },
        };
      },
      async decline() {
        incomingPendingCount = 1;
        return {
          friendRequest: createFriendRequest('incoming-1', 'incoming', 'declined'),
        };
      },
      async list(params: FriendRequestListParams) {
        friendRequestCalls.push(params);
        return pageFriendRequests(params);
      },
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkContactService(() => fakeClient);
  const requests = await service.getFriendRequests();

  const incomingCalls = friendRequestCalls.filter((call) => call.direction === 'incoming');
  const outgoingCalls = friendRequestCalls.filter((call) => call.direction === 'outgoing');

  assert.deepEqual(
    incomingCalls,
    [
      { direction: 'incoming', status: 'pending', limit: 100 },
      { direction: 'incoming', status: 'pending', limit: 100, cursor: '1' },
    ],
    'incoming friend request sync must continue until nextCursor is exhausted',
  );
  assert.deepEqual(
    outgoingCalls,
    [
      { direction: 'outgoing', status: 'pending', limit: 100 },
      { direction: 'outgoing', status: 'pending', limit: 100, cursor: '1' },
    ],
    'outgoing friend request sync must continue until nextCursor is exhausted',
  );
  assert.deepEqual(
    requests.map((request) => request.name),
    [
      'Profile incoming-peer-incoming-1',
      'Profile incoming-peer-incoming-2',
      'Profile outgoing-peer-outgoing-1',
      'Profile outgoing-peer-outgoing-2',
    ],
    'friend request list must resolve peer names through the real social user search endpoint',
  );
  assert.deepEqual(
    requests.map((request) => request.avatar),
    [
      'https://cdn.example.test/incoming-peer-incoming-1.png',
      'https://cdn.example.test/incoming-peer-incoming-2.png',
      'https://cdn.example.test/outgoing-peer-outgoing-1.png',
      'https://cdn.example.test/outgoing-peer-outgoing-2.png',
    ],
    'friend request list must resolve peer avatars through the real social user search endpoint',
  );
  assert.deepEqual(
    userSearchCalls,
    [
      { q: 'incoming-peer-incoming-1', limit: 20 },
      { q: 'incoming-peer-incoming-2', limit: 20 },
      { q: 'outgoing-peer-outgoing-1', limit: 20 },
      { q: 'outgoing-peer-outgoing-2', limit: 20 },
    ],
  );

  const pendingCounts: number[] = [];
  const unsubscribePendingCount = service.subscribePendingFriendRequestCount((count) => {
    pendingCounts.push(count);
  });
  const pendingCount = await service.getPendingFriendRequestCount();
  assert.equal(
    pendingCount,
    2,
    'pending friend request red dot count must include only incoming pending requests',
  );
  assert.deepEqual(
    realtimeConnectOptions?.subscriptions?.scopes,
    [
      {
        scopeType: 'user',
        scopeId: 'current-user',
        eventTypes: [
          'friend_request.submitted',
          'friend_request.accepted',
          'friend_request.declined',
          'friend_request.canceled',
        ],
      },
    ],
    'pending friend request red dot count must subscribe to user-scope realtime friend request events',
  );
  assert.equal(typeof realtimeEventHandler, 'function', 'friend request realtime handler must be registered');
  incomingPendingCount = 1;
  realtimeEventHandler?.({
    eventType: 'friend_request.submitted',
  }, {
    ack: () => Promise.resolve(),
    eventId: 'event-friend-request-1',
    eventType: 'friend_request.submitted',
    payload: {
      friendRequest: createFriendRequest('incoming-3', 'incoming'),
    },
    receivedAt: '2026-06-04T00:00:01.000Z',
    scopeId: 'current-user',
    scopeType: 'user',
    sequence: 9,
  });
  await new Promise((resolve) => setTimeout(resolve, 0));
  assert.equal(
    pendingCounts.at(-1),
    1,
    'friend request red dot count must refresh immediately after user-scope realtime events',
  );
  assert.equal(
    friendRequestCalls.filter((call) => call.direction === 'incoming').length >= 4,
    true,
    'friend request realtime refresh must reload incoming pending requests without waiting for the interval',
  );
  await service.handleFriendRequest(requests[0].id, 'accept');
  assert.equal(
    pendingCounts.at(-1),
    1,
    'friend request red dot count must refresh after accepting a request',
  );
  unsubscribePendingCount();

  console.log('sdkwork-im-pc friend request sync contract passed');
}

void main();
