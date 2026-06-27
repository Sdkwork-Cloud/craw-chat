import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkFavoriteService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/FavoriteService';

const calls: unknown[] = [];

const fakeClient = {
  messages: {
    favorites: {
      async list(params?: Record<string, unknown>) {
        calls.push({ method: 'messages.favorites.list', params });
        return {
          items: [
            {
              favoriteId: 'fav-message-1',
              favoriteType: 'chat',
              conversationId: 'chat-1',
              messageId: 'message-1',
              messageSeq: 7,
              title: 'Pinned context',
              contentPreview: 'Please keep this context',
              sourceDisplayName: 'Sarah Connor',
              favoritedAt: '2026-06-04T08:00:00.000Z',
            },
          ],
          hasMore: false,
        };
      },
      async create(messageId: string, body: Record<string, unknown>) {
        calls.push({ method: 'messages.favorites.create', messageId, body });
        return {
          favoriteId: 'fav-message-2',
          favoriteType: body.favoriteType,
          conversationId: body.conversationId,
          messageId,
          messageSeq: 8,
          title: body.title,
          contentPreview: body.contentPreview,
          sourceDisplayName: body.sourceDisplayName,
          favoritedAt: '2026-06-04T08:01:00.000Z',
        };
      },
      async delete(favoriteId: string) {
        calls.push({ method: 'messages.favorites.delete', favoriteId });
        return {
          favoriteId,
          deleted: true,
        };
      },
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkFavoriteService(() => fakeClient);

  const favorites = await service.getFavorites('chat');
  assert.deepEqual(favorites, [
    {
      id: 'fav-message-1',
      type: 'chat',
      title: 'Pinned context',
      content: 'Please keep this context',
      source: 'Sarah Connor',
      timestamp: new Date('2026-06-04T08:00:00.000Z').getTime(),
      conversationId: 'chat-1',
      messageId: 'message-1',
    },
  ]);

  const created = await service.addFavorite({
    type: 'chat',
    title: 'Release decision',
    content: 'Ship after contract tests pass',
    source: 'Owner',
    conversationId: 'chat-1',
    messageId: 'message-2',
  });
  assert.equal(created.id, 'fav-message-2');
  assert.equal(created.timestamp, new Date('2026-06-04T08:01:00.000Z').getTime());

  await service.removeFavorite('fav-message-2');

  assert.deepEqual(calls, [
    {
      method: 'messages.favorites.list',
      params: {
        favoriteType: 'chat',
        limit: 100,
      },
    },
    {
      method: 'messages.favorites.create',
      messageId: 'message-2',
      body: {
        conversationId: 'chat-1',
        favoriteType: 'chat',
        title: 'Release decision',
        contentPreview: 'Ship after contract tests pass',
        sourceDisplayName: 'Owner',
      },
    },
    {
      method: 'messages.favorites.delete',
      favoriteId: 'fav-message-2',
    },
  ], 'favorite service must persist list/create/delete through the standard IM SDK favorite API');

  console.log('sdkwork-im-pc favorites sync contract passed');
}

void main();
