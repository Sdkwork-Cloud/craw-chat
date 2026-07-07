import assert from 'node:assert/strict';
import type {
  FriendRequest,
  ImConnectOptions,
  ImLiveConnection,
  ImRealtimeEventContext,
  ImSdkClient,
} from '@sdkwork/im-sdk';
import { createSdkworkContactService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ContactService';
import {
  applyAppSdkSessionTokens,
  clearAppSdkSessionTokens,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/session';

const storage = new Map<string, string>();

(globalThis as unknown as {
  window: Pick<Window, 'addEventListener' | 'dispatchEvent' | 'localStorage' | 'removeEventListener'>;
}).window = {
  addEventListener() {
    return undefined;
  },
  removeEventListener() {
    return undefined;
  },
  dispatchEvent() {
    return true;
  },
  localStorage: {
    clear() {
      storage.clear();
    },
    getItem(key: string) {
      return storage.get(key) ?? null;
    },
    key(index: number) {
      return Array.from(storage.keys())[index] ?? null;
    },
    get length() {
      return storage.size;
    },
    removeItem(key: string) {
      storage.delete(key);
    },
    setItem(key: string, value: string) {
      storage.set(key, value);
    },
  } as Storage,
};

(globalThis as unknown as {
  document: Pick<Document, 'addEventListener' | 'removeEventListener' | 'visibilityState'>;
}).document = {
  visibilityState: 'visible',
  addEventListener() {
    return undefined;
  },
  removeEventListener() {
    return undefined;
  },
};

type FriendRequestDirection = 'incoming' | 'outgoing' | 'all';

type FriendRequestListParams = {
  cursor?: string;
  direction: FriendRequestDirection;
  pageSize?: number;
  status?: 'pending' | 'accepted' | 'declined' | 'canceled' | 'expired' | 'all';
};

const friendRequestCalls: FriendRequestListParams[] = [];
const userSearchCalls: Array<{ pageSize?: number; q?: string }> = [];
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
  if (params.direction === 'all') {
    const page = params.cursor ?? '0';
    if (page === '0') {
      return {
        items: [
          createFriendRequest('incoming-1', 'incoming'),
          createFriendRequest('outgoing-1', 'outgoing'),
        ],
        nextCursor: '1',
      };
    }
    if (page === '1') {
      return {
        items: [
          createFriendRequest('incoming-2', 'incoming'),
          createFriendRequest('outgoing-2', 'outgoing'),
        ],
      };
    }
    return { items: [] };
  }
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
      async list(params: { pageSize?: number; q?: string }) {
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
      async pendingCount() {
        return { count: incomingPendingCount };
      },
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
  clearAppSdkSessionTokens();
  applyAppSdkSessionTokens({
    accessToken: 'access-token',
    authToken: 'auth-token',
    context: {
      tenantId: '100001',
      organizationId: '0',
      userId: 'current-user',
    },
    user: {
      id: 'current-user',
      userId: 'current-user',
      displayName: 'Current User',
    },
  });

  const service = createSdkworkContactService(() => fakeClient);
  const requests = await service.getFriendRequests();

  const allCalls = friendRequestCalls.filter((call) => call.direction === 'all');
  const incomingCalls = friendRequestCalls.filter((call) => call.direction === 'incoming');
  const outgoingCalls = friendRequestCalls.filter((call) => call.direction === 'outgoing');

  assert.deepEqual(
    incomingCalls,
    [
      { direction: 'incoming', status: 'pending', pageSize: 20 },
      { direction: 'incoming', status: 'pending', pageSize: 20, cursor: '1' },
    ],
    'friend request inbox sync must page through pending incoming requests',
  );
  assert.deepEqual(
    outgoingCalls,
    [
      { direction: 'outgoing', status: 'pending', pageSize: 20 },
      { direction: 'outgoing', status: 'pending', pageSize: 20, cursor: '1' },
    ],
    'friend request inbox sync must page through pending outgoing requests',
  );
  assert.equal(
    allCalls.length,
    0,
    'getFriendRequests must not rely on direction=all when the UI needs incoming/outgoing action semantics',
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
      { q: 'incoming-peer-incoming-1', pageSize: 20 },
      { q: 'incoming-peer-incoming-2', pageSize: 20 },
      { q: 'outgoing-peer-outgoing-1', pageSize: 20 },
      { q: 'outgoing-peer-outgoing-2', pageSize: 20 },
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
