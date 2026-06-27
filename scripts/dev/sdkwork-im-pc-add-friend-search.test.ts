import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkContactService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ContactService';

const calls: Array<{ body?: Record<string, unknown>; method: string; params?: Record<string, unknown> }> = [];

const fakeClient = {
  chat: {
    contacts: {
      async list() {
        calls.push({ method: 'chat.contacts.list' });
        return { items: [], hasMore: false };
      },
    },
  },
  social: {
    users: {
      async list(params: Record<string, unknown>) {
        calls.push({ method: 'social.users.list', params });
        if (params.q === 'current-user' || params.q === 'current-chat-id') {
          return {
            items: [
              {
                userId: 'current-user',
                chatId: 'current-chat-id',
                displayName: 'Current User',
                relationshipState: 'self',
              },
            ],
            hasMore: false,
          };
        }
        if (params.q === 'alice' || params.q === 'alice@example.com' || params.q === '+12025550100') {
          return {
            items: [
              {
                userId: 'u_alice',
                chatId: 'cc8k2m7q4x9p',
                displayName: 'Alice',
                avatarUrl: 'https://example.com/alice.png',
                email: 'alice@example.com',
                phone: '+12025550100',
                relationshipState: 'none',
              },
            ],
            hasMore: false,
          };
        }
        if (params.q === 'cc8k2m7q4x9p') {
          return {
            items: [
              {
                userId: 'u_alice',
                chatId: 'cc8k2m7q4x9p',
                displayName: 'Alice',
                relationshipState: 'none',
              },
            ],
            hasMore: false,
          };
        }
        return {
          items: [],
          hasMore: false,
        };
      },
    },
    contacts: {
      async list() {
        calls.push({ method: 'social.contacts.list' });
        return { items: [], hasMore: false };
      },
    },
    friendRequests: {
      async create(body: Record<string, unknown>) {
        calls.push({ method: 'social.friendRequests.create', body });
        return {
          friendRequest: {
            requestId: 'fr_alice',
            requesterUserId: 'current-user',
            targetUserId: body.targetUserId,
            status: 'pending',
          },
        };
      },
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkContactService(() => fakeClient);

  const results = await service.searchContacts(' alice ');

  assert.deepEqual(
    calls.at(-1),
    {
      method: 'social.users.list',
      params: { q: 'alice', limit: 20 },
    },
    'add-friend search must query the generated IM SDK social user search endpoint',
  );
  assert.deepEqual(
    results.map((user) => [user.id, user.chatId, user.name, user.email, user.phone]),
    [['u_alice', 'cc8k2m7q4x9p', 'Alice', 'alice@example.com', '+12025550100']],
    'add-friend search must map backend user search results into selectable contacts with a public chat id',
  );
  assert.notEqual(
    results[0]?.chatId,
    results[0]?.id,
    'public chat id must be distinct from the internal user id used for friend request targets',
  );

  const missing = await service.searchContacts('does-not-exist');
  assert.deepEqual(
    missing,
    [],
    'add-friend search must not synthesize mock users when the backend returns no matches',
  );
  assert.equal(
    await service.getUserById('does-not-exist'),
    null,
    'getUserById must not synthesize mock users when the backend user search has no match',
  );
  assert.equal(
    calls.filter((call) => call.method === 'social.friendRequests.create').length,
    0,
    'searching for a missing user must not create a friend request',
  );

  calls.length = 0;
  const selfResults = await service.searchContacts(' current-user ');
  assert.deepEqual(
    selfResults,
    [],
    'add-friend search must not return the current user as an addable friend target',
  );
  await assert.rejects(
    () => service.addFriend(' current-user '),
    /yourself|current user/i,
    'addFriend must reject the current user locally before creating a friend request',
  );
  await assert.rejects(
    () => service.addFriendBySearchQuery(' current-chat-id '),
    /not found|yourself|current user/i,
    'direct add-by-public-id must not submit a friend request when the search target is the current user',
  );
  assert.equal(
    calls.filter((call) => call.method === 'social.friendRequests.create').length,
    0,
    'current-user add attempts must not call social.friendRequests.create',
  );

  const added = await service.addFriendBySearchQuery(' alice ');
  assert.equal(added.id, 'u_alice', 'direct add-by-input must use the backend search result user id');
  assert.deepEqual(
    calls.slice(-2),
    [
      {
        method: 'social.users.list',
        params: { q: 'alice', limit: 20 },
      },
      {
        method: 'social.friendRequests.create',
        body: { targetUserId: 'u_alice' },
      },
    ],
    'direct add-by-input must search real users before creating a friend request',
  );

  await assert.rejects(
    () => service.addFriendBySearchQuery('does-not-exist'),
    /not found/i,
    'direct add-by-input must reject missing users instead of treating the raw input as a target user id',
  );

  calls.length = 0;
  const addedByChatId = await service.addFriendBySearchQuery(' cc8k2m7q4x9p ');
  assert.equal(addedByChatId.id, 'u_alice', 'direct add-by-public-id must resolve the real internal target user id');
  assert.equal(addedByChatId.chatId, 'cc8k2m7q4x9p', 'direct add-by-public-id must preserve the public chat id');
  assert.deepEqual(
    calls,
    [
      {
        method: 'social.users.list',
        params: { q: 'cc8k2m7q4x9p', limit: 20 },
      },
      {
        method: 'social.friendRequests.create',
        body: { targetUserId: 'u_alice' },
      },
    ],
    'direct add-by-public-id must search by chat id but submit the real internal target user id',
  );

  calls.length = 0;
  const addedByEmail = await service.addFriendBySearchQuery(' alice@example.com ');
  assert.equal(addedByEmail.id, 'u_alice', 'direct add-by-email must resolve the real internal target user id');
  assert.deepEqual(
    calls,
    [
      {
        method: 'social.users.list',
        params: { q: 'alice@example.com', limit: 20 },
      },
      {
        method: 'social.friendRequests.create',
        body: { targetUserId: 'u_alice' },
      },
    ],
    'direct add-by-email must search by email but submit the real internal target user id',
  );

  calls.length = 0;
  const addedByPhone = await service.addFriendBySearchQuery(' +12025550100 ');
  assert.equal(addedByPhone.id, 'u_alice', 'direct add-by-phone must resolve the real internal target user id');
  assert.deepEqual(
    calls,
    [
      {
        method: 'social.users.list',
        params: { q: '+12025550100', limit: 20 },
      },
      {
        method: 'social.friendRequests.create',
        body: { targetUserId: 'u_alice' },
      },
    ],
    'direct add-by-phone must search by phone but submit the real internal target user id',
  );

  console.log('sdkwork-im-pc add-friend search contract passed');
}

void main();
