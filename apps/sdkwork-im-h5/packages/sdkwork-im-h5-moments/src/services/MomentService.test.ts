import assert from "node:assert/strict";
import test from "node:test";

import type { SdkworkCommunityEntry } from "@sdkwork/community-contracts";
import { createInMemoryCommunityAppSdkPort } from "@sdkwork/community-sdk-ports";
import {
  MomentCapabilityUnavailableError,
  configureMomentsFeedsPort,
  configureMomentsRuntimePort,
  resetMomentsRuntimePort,
} from "./momentsRuntimePort";
import { MomentService, resetMomentsSessionState } from "./MomentService";

function seedEntry(overrides: Partial<SdkworkCommunityEntry> = {}): SdkworkCommunityEntry {
  return {
    id: "entry-1",
    categoryId: "circle-1",
    author: { id: "user-1", name: "Alex Chen" },
    kind: "discussion",
    title: "今天天气真好",
    reviewState: "approved",
    stats: { commentCount: 0, reactionCount: 0 },
    body: "今天天气真好！出去走走~",
    publishedAt: "2026-08-01T10:00:00.000Z",
    ...overrides,
  };
}

function seedCategory(overrides: Record<string, unknown> = {}) {
  return {
    id: "circle-1",
    tenantId: "local",
    slug: "tech",
    title: "技术交流圈",
    enabled: true,
    priority: 0,
    description: "技术分享",
    ...overrides,
  };
}


function seedFeedsClient(items: Array<Record<string, unknown>> = defaultFeedItems(), options: { hasMore?: boolean; nextCursor?: string } = {}): { feeds: { streams: { items: { list: any } } } } {
  return {
    feeds: {
      streams: {
        items: {
          list: async () => ({
            items,
            pageInfo: { mode: "cursor", pageSize: 20, hasMore: options.hasMore ?? false, ...(options.nextCursor ? { nextCursor: options.nextCursor } : {}) },
          }),
        },
      },
    },
  };
}

function defaultFeedItems(): Array<Record<string, unknown>> {
  return [
    {
      id: "entry-1",
      streamKey: "moments-global",
      sourceType: "community.entry",
      sourceId: "entry-1",
      title: "今天天气真好",
      excerpt: "今天天气真好！出去走走~",
      author: { id: "user-1", name: "Alex Chen", avatarUrl: "https://cdn.example/a.png" },
      reactionCount: 3,
      commentCount: 2,
      publishedAt: "2026-08-01T10:00:00.000Z",
      createdAt: "2026-08-01T10:00:00.000Z",
      updatedAt: "2026-08-01T10:00:00.000Z",
      isPinned: false,
      status: "active",
      tenantId: "local",
      streamId: "s1",
    },
    {
      id: "entry-2",
      streamKey: "moments-global",
      sourceType: "community.entry",
      sourceId: "entry-2",
      title: "标题",
      excerpt: "摘要",
      author: { id: "user-2", name: "Bo Li" },
      reactionCount: 0,
      commentCount: 0,
      publishedAt: "2026-08-02T10:00:00.000Z",
      createdAt: "2026-08-02T10:00:00.000Z",
      updatedAt: "2026-08-02T10:00:00.000Z",
      isPinned: false,
      status: "active",
      tenantId: "local",
      streamId: "s1",
    },
  ];
}

test("fails closed with a typed error before the host binds the runtime port", async () => {
  resetMomentsRuntimePort();
  await assert.rejects(() => MomentService.getFeed(), MomentCapabilityUnavailableError);
});

test("maps the global feed to moment view models", async () => {
  resetMomentsRuntimePort();
  resetMomentsSessionState();
  configureMomentsFeedsPort(seedFeedsClient() as never);
  configureMomentsRuntimePort(createInMemoryCommunityAppSdkPort({ entries: [seedEntry()] }));

  const { moments, hasMore } = await MomentService.getFeed(1, 20);

  assert.equal(moments.length, 2);
  assert.equal(hasMore, false);

  const first = moments[0];
  assert.equal(first.id, "entry-1");
  assert.equal(first.author.name, "Alex Chen");
  assert.equal(first.author.avatar, "https://cdn.example/a.png");
  assert.equal(first.content, "今天天气真好！出去走走~");
  // Feed stream snapshots do not carry the circle category id.
  assert.equal(first.categoryId, "");
  assert.equal(first.likeCount, 3);
  assert.equal(first.commentCount, 2);
  assert.equal(first.isLiked, false);
  assert.equal(first.timestamp, Date.parse("2026-08-01T10:00:00.000Z"));

  const second = moments[1];
  assert.equal(second.content, "摘要");
});

test("pages the feed and reports hasMore on a full page", async () => {
  resetMomentsRuntimePort();
  resetMomentsSessionState();
  configureMomentsRuntimePort(createInMemoryCommunityAppSdkPort({ entries: [seedEntry()] }));
  const firstPageItems = [
    { ...defaultFeedItems()[0], id: "entry-1" },
    { ...defaultFeedItems()[1], id: "entry-2" },
  ];
  configureMomentsFeedsPort(
    seedFeedsClient(firstPageItems, { hasMore: true, nextCursor: "cursor-2" }) as never,
  );

  const firstPage = await MomentService.getFeed(1, 2);
  assert.deepEqual(
    firstPage.moments.map((moment) => moment.id),
    ["entry-1", "entry-2"],
  );
  assert.equal(firstPage.hasMore, true);
  assert.equal(firstPage.nextCursor, "cursor-2");

  configureMomentsFeedsPort(
    seedFeedsClient([{ ...defaultFeedItems()[0], id: "entry-3" }]) as never,
  );
  const secondPage = await MomentService.getFeed(2, 2, "cursor-2");
  assert.deepEqual(
    secondPage.moments.map((moment) => moment.id),
    ["entry-3"],
  );
  assert.equal(secondPage.hasMore, false);
});

test("returns an empty feed without error", async () => {
  resetMomentsRuntimePort();
  resetMomentsSessionState();
  configureMomentsRuntimePort(createInMemoryCommunityAppSdkPort({ entries: [seedEntry()] }));
  configureMomentsFeedsPort(seedFeedsClient([]) as never);

  const { moments, hasMore } = await MomentService.getFeed();
  assert.deepEqual(moments, []);
  assert.equal(hasMore, false);
});

test("falls back through excerpt and title for content", async () => {
  resetMomentsRuntimePort();
  resetMomentsSessionState();
  configureMomentsRuntimePort(createInMemoryCommunityAppSdkPort({ entries: [seedEntry()] }));
  configureMomentsFeedsPort(
    seedFeedsClient([
      { ...defaultFeedItems()[0], id: "with-excerpt", excerpt: "只有摘要", title: "标题" },
      { ...defaultFeedItems()[1], id: "with-title", excerpt: undefined, title: "只有标题" },
    ]) as never,
  );

  const { moments } = await MomentService.getFeed();
  assert.equal(moments[0].content, "只有摘要");
  assert.equal(moments[1].content, "只有标题");
});

test("tolerates entries without a published date", async () => {
  resetMomentsRuntimePort();
  resetMomentsSessionState();
  configureMomentsRuntimePort(createInMemoryCommunityAppSdkPort({ entries: [seedEntry()] }));
  configureMomentsFeedsPort(
    seedFeedsClient([{ ...defaultFeedItems()[0], id: "no-date", publishedAt: undefined }]) as never,
  );

  const { moments } = await MomentService.getFeed();
  assert.ok(Number.isFinite(moments[0].timestamp));
});

test("lists only enabled circles for the publish picker", async () => {
  resetMomentsRuntimePort();
  resetMomentsSessionState();
  configureMomentsRuntimePort(
    createInMemoryCommunityAppSdkPort({
      categories: [
        seedCategory({ id: "circle-1", title: "技术交流圈", memberCount: 12 }),
        seedCategory({ id: "circle-2", title: "已关闭圈子", enabled: false }),
      ],
    }),
  );

  const circles = await MomentService.getCircles();

  assert.deepEqual(
    circles.map((circle) => circle.id),
    ["circle-1"],
  );
  assert.equal(circles[0].name, "技术交流圈");
  assert.equal(circles[0].memberCount, 12);
});

test("publishes a text moment into the selected circle", async () => {
  resetMomentsRuntimePort();
  resetMomentsSessionState();
  configureMomentsRuntimePort(
    createInMemoryCommunityAppSdkPort({ categories: [seedCategory()] }),
  );

  const moment = await MomentService.publish({
    categoryId: "circle-1",
    content: "刚看完一本好书，推荐给大家《设计心理学》。",
  });

  assert.equal(moment.categoryId, "circle-1");
  assert.equal(moment.content, "刚看完一本好书，推荐给大家《设计心理学》。");
  assert.equal(moment.author.name, "Local User");
  assert.ok(moment.id.length > 0);

  // The published moment must be visible on the first feed page afterwards
  // (the feeds stream carries the snapshot).
  configureMomentsFeedsPort(
    seedFeedsClient([{ ...defaultFeedItems()[0], id: moment.id, excerpt: "刚看完一本好书，推荐给大家《设计心理学》。" }]) as never,
  );
  const { moments } = await MomentService.getFeed(1, 20);
  assert.equal(moments.length, 1);
  assert.equal(moments[0].id, moment.id);
  assert.equal(moments[0].content, "刚看完一本好书，推荐给大家《设计心理学》。");
});

test("publishes long content with a truncated required title", async () => {
  resetMomentsRuntimePort();
  resetMomentsSessionState();
  configureMomentsRuntimePort(
    createInMemoryCommunityAppSdkPort({ categories: [seedCategory()] }),
  );

  const longContent = "字".repeat(120);
  const moment = await MomentService.publish({ categoryId: "circle-1", content: longContent });

  assert.equal(moment.content, longContent);
});

test("toggles the viewer like and returns the reaction count", async () => {
  resetMomentsRuntimePort();
  resetMomentsSessionState();
  configureMomentsRuntimePort(
    createInMemoryCommunityAppSdkPort({ entries: [seedEntry({ id: "like-target" })] }),
  );

  const first = await MomentService.toggleLike("like-target");
  assert.equal(first.isLiked, true);
  assert.equal(first.likeCount, 1);

  const second = await MomentService.toggleLike("like-target");
  assert.equal(second.isLiked, false);
  assert.equal(second.likeCount, 0);

  configureMomentsFeedsPort(
    seedFeedsClient([{ ...defaultFeedItems()[0], id: "like-target", reactionCount: 0 }]) as never,
  );
  const { moments } = await MomentService.getFeed();
  assert.equal(moments[0].isLiked, false);
  assert.equal(moments[0].likeCount, 0);
});

test("clears the viewer like memory on reset (logout)", async () => {
  resetMomentsRuntimePort();
  resetMomentsSessionState();
  configureMomentsRuntimePort(
    createInMemoryCommunityAppSdkPort({ entries: [seedEntry({ id: "reset-target" })] }),
  );

  configureMomentsFeedsPort(
    seedFeedsClient([{ ...defaultFeedItems()[0], id: "reset-target", reactionCount: 1 }]) as never,
  );
  await MomentService.toggleLike("reset-target");
  const likedFeed = await MomentService.getFeed();
  assert.equal(likedFeed.moments[0].isLiked, true);

  resetMomentsSessionState();

  const resetFeed = await MomentService.getFeed();
  assert.equal(resetFeed.moments[0].isLiked, false);
  // The server-side reaction count is untouched by the client-side reset.
  assert.equal(resetFeed.moments[0].likeCount, 1);
});

test("fetches and posts comments", async () => {
  resetMomentsRuntimePort();
  resetMomentsSessionState();
  configureMomentsRuntimePort(
    createInMemoryCommunityAppSdkPort({ entries: [seedEntry({ id: "comment-target" })] }),
  );

  const added = await MomentService.addComment("comment-target", "Nice view!");
  assert.equal(added.authorName, "Local User");
  assert.equal(added.content, "Nice view!");

  const comments = await MomentService.getComments("comment-target");
  assert.equal(comments.length, 1);
  assert.equal(comments[0].content, "Nice view!");
  assert.equal(comments[0].authorId, "local-user");
});

test("deletes a moment", async () => {
  resetMomentsRuntimePort();
  resetMomentsSessionState();
  configureMomentsRuntimePort(
    createInMemoryCommunityAppSdkPort({ entries: [seedEntry({ id: "delete-target" })] }),
  );

  configureMomentsFeedsPort(seedFeedsClient([]) as never);
  await MomentService.deleteMoment("delete-target");

  const { moments } = await MomentService.getFeed();
  assert.equal(moments.length, 0);
});
