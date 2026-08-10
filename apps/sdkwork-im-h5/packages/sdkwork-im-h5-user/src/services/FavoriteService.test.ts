import assert from "node:assert/strict";
import test from "node:test";

import type {
  CmsAppSdkClient,
  CmsFavoriteType,
  CmsFavoriteView,
} from "@sdkwork/im-h5-core/sdk";

import { createFavoriteService } from "./FavoriteService";

interface CmsSdkOverrides {
  favorites?: {
    create?: CmsAppSdkClient["favorites"]["create"];
    list?: CmsAppSdkClient["favorites"]["list"];
    delete?: CmsAppSdkClient["favorites"]["delete"];
  };
}

function createCmsSdk(overrides: CmsSdkOverrides = {}): CmsAppSdkClient {
  return {
    favorites: {
      create: async () => ({ item: favoriteView("1") }),
      list: async () => ({
        items: [],
        pageInfo: { mode: "cursor", nextCursor: null, hasMore: false },
      }),
      delete: async () => ({ deleted: true }),
      ...overrides.favorites,
    },
  } as unknown as CmsAppSdkClient;
}

function favoriteView(
  id: string,
  overrides: Partial<CmsFavoriteView> = {},
): CmsFavoriteView {
  return {
    id,
    favoriteId: `fav-${id}`,
    favoriteType: "chat",
    targetType: "im_message",
    targetId: "msg-1",
    targetUuid: null,
    targetUrl: null,
    title: "Hello",
    summary: "preview text",
    sourceDisplayName: "张三",
    media: null,
    favoritedAt: "2026-08-09T12:00:00Z",
    ...overrides,
  };
}

test("maps favorite views to card items with stable card metadata", async () => {
  const service = createFavoriteService(() =>
    createCmsSdk({
      favorites: {
        list: async () => ({
          items: [
            favoriteView("1", {
              favoriteType: "image",
              title: "公司年度旅游照片合集",
              summary: "[9张图片]",
              sourceDisplayName: "HR 部门",
              favoritedAt: "2026-07-01T12:00:00Z",
            }),
            favoriteView("2", {
              favoriteType: "chat",
              title: "关于系统架构升级的讨论",
              summary: "李四: 我们需要重构网关...",
              sourceDisplayName: "研发一组",
            }),
            favoriteView("3", { favoriteType: "voice" }),
          ],
          pageInfo: { mode: "cursor", nextCursor: null, hasMore: false },
        }),
      },
    }),
  );

  const items = await service.getFavorites();
  assert.equal(items.length, 3);
  assert.deepEqual(items[0], {
    id: "fav-1",
    title: "公司年度旅游照片合集",
    type: "image",
    typeLabel: "相册",
    time: "2026-07-01",
    source: "HR 部门",
    preview: "[9张图片]",
    icon: "Image",
    color: "text-green-500",
  });
  assert.equal(items[1].typeLabel, "聊天记录");
  assert.equal(items[1].icon, "MessageCircle");
  assert.equal(items[1].color, "text-emerald-500");
  assert.equal(items[2].typeLabel, "语音");
  assert.equal(items[2].icon, "Mic");
  assert.equal(items[2].color, "text-orange-500");
});

test("passes the favoriteType filter to the CMS list call", async () => {
  let receivedType: CmsFavoriteType | undefined;
  const service = createFavoriteService(() =>
    createCmsSdk({
      favorites: {
        list: async (params) => {
          receivedType = params?.favoriteType;
          return {
            items: [favoriteView("1", { favoriteType: "link" })],
            pageInfo: { mode: "cursor", nextCursor: null, hasMore: false },
          };
        },
      },
    }),
  );

  const items = await service.getFavorites("link");
  assert.equal(receivedType, "link");
  assert.equal(items.length, 1);
  assert.equal(items[0].type, "link");
});

test("collects cursor pages until the list is exhausted", async () => {
  let listCalls = 0;
  const service = createFavoriteService(() =>
    createCmsSdk({
      favorites: {
        list: async (params) => {
          listCalls += 1;
          if (!params?.cursor) {
            return {
              items: [favoriteView("1")],
              pageInfo: { mode: "cursor", nextCursor: "next", hasMore: true },
            };
          }
          return {
            items: [favoriteView("2")],
            pageInfo: { mode: "cursor", nextCursor: null, hasMore: false },
          };
        },
      },
    }),
  );

  const items = await service.getFavorites();
  assert.equal(listCalls, 2);
  assert.deepEqual(items.map((item) => item.id), ["fav-1", "fav-2"]);
});

test("throws when the CMS list returns a repeated cursor", async () => {
  const service = createFavoriteService(() =>
    createCmsSdk({
      favorites: {
        list: async () => ({
          items: [],
          pageInfo: { mode: "cursor", nextCursor: "next", hasMore: true },
        }),
      },
    }),
  );

  await assert.rejects(service.getFavorites(), /repeated cursor/);
});

test("deletes a favorite by its stable favorite id", async () => {
  let deletedId: string | undefined;
  const service = createFavoriteService(() =>
    createCmsSdk({
      favorites: {
        delete: async (favoriteId) => {
          deletedId = favoriteId;
          return { deleted: true };
        },
      },
    }),
  );

  await service.removeFavorite("fav-42");
  assert.equal(deletedId, "fav-42");
});

test("formats yesterday favorites as 昨天", async () => {
  const yesterday = new Date();
  yesterday.setDate(yesterday.getDate() - 1);
  yesterday.setHours(12, 0, 0, 0);

  const service = createFavoriteService(() =>
    createCmsSdk({
      favorites: {
        list: async () => ({
          items: [favoriteView("1", { favoritedAt: yesterday.toISOString() })],
          pageInfo: { mode: "cursor", nextCursor: null, hasMore: false },
        }),
      },
    }),
  );

  const [item] = await service.getFavorites();
  assert.equal(item.time, "昨天");
});
