/**
 * 组合 social 表面单元测试。
 *
 * 回归场景：生成层把待处理好友申请数暴露为路径嵌套形态
 * `friendRequests.pending.count.retrieve()`，组合 facade 需将其扁平化为
 * `friendRequests.pendingCount()`（`ImTransportClientLike['social']` 声明
 * 的公开契约），其余表面原样透传。
 *
 * 使用 Node 内置 node:test 运行：
 *   node --test tests/social-module.test.mjs
 *
 * 前置条件：先执行 `npm run build` 生成 dist/ 输出。
 */
import { describe, it } from 'node:test';
import assert from 'node:assert/strict';

import { composeSocialSurface } from '../dist/social-module.js';

/** 构造与生成层同构的假 SocialApi，记录委托调用。 */
function fakeSocialApi() {
  const calls = [];
  const friendRequests = {
    pending: {
      count: {
        retrieve: async () => {
          calls.push('pending.count.retrieve');
          return { count: 7 };
        },
      },
    },
    list: async (params) => {
      calls.push(`list:${JSON.stringify(params)}`);
      return { items: [], pageInfo: { mode: 'cursor', hasMore: false } };
    },
    create: async (body) => {
      calls.push(`create:${body.targetUserId}`);
      return { ok: true };
    },
    accept: async (requestId) => {
      calls.push(`accept:${requestId}`);
      return { ok: true };
    },
    decline: async (requestId) => {
      calls.push(`decline:${requestId}`);
      return { ok: true };
    },
    cancel: async (requestId) => {
      calls.push(`cancel:${requestId}`);
      return { ok: true };
    },
  };
  const social = {
    users: { list: async () => ({ items: [] }) },
    contacts: { list: async () => ({ items: [] }) },
    friendships: { remove: async () => ({}) },
    userBlocks: { create: async () => ({}), delete: async () => undefined },
    friendRequests,
  };
  return { social, calls };
}

describe('composeSocialSurface', () => {
  it('flattens pending.count.retrieve into pendingCount', async () => {
    const { social, calls } = fakeSocialApi();
    const composed = composeSocialSurface(social);
    assert.equal(typeof composed.friendRequests.pendingCount, 'function');
    assert.deepEqual(await composed.friendRequests.pendingCount(), { count: 7 });
    assert.deepEqual(calls, ['pending.count.retrieve']);
  });

  it('keeps the remaining friend request mutations delegating to the generated surface', async () => {
    const { social, calls } = fakeSocialApi();
    const composed = composeSocialSurface(social);
    await composed.friendRequests.list({ direction: 'incoming', status: 'pending' });
    await composed.friendRequests.create({ targetUserId: 'u_bob', requestMessage: 'hi' });
    await composed.friendRequests.accept('fr_1');
    await composed.friendRequests.decline('fr_2');
    await composed.friendRequests.cancel('fr_3');
    assert.deepEqual(calls, [
      'list:{"direction":"incoming","status":"pending"}',
      'create:u_bob',
      'accept:fr_1',
      'decline:fr_2',
      'cancel:fr_3',
    ]);
  });

  it('passes the remaining social surfaces through unmodified', () => {
    const { social } = fakeSocialApi();
    const composed = composeSocialSurface(social);
    assert.equal(composed.users, social.users);
    assert.equal(composed.contacts, social.contacts);
    assert.equal(composed.friendships, social.friendships);
    assert.equal(composed.userBlocks, social.userBlocks);
  });
});
