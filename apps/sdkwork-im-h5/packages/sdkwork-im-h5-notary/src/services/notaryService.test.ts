import assert from "node:assert/strict";
import test from "node:test";

import {
  appendBoundedUnique,
  createNotaryService,
  NotaryResponseContractError,
  type NotaryApiPort,
} from "./notaryService";

function createCaseRecord(overrides: Record<string, unknown> = {}) {
  return {
    id: "case-1",
    title: "Property authorization",
    type: "authorization",
    createTime: "2026-07-29T08:00:00Z",
    status: "PENDING_REVIEW",
    ...overrides,
  };
}

function createApi(overrides: Partial<NotaryApiPort> = {}): NotaryApiPort {
  return {
    getAccess: async () => ({ data: {} }),
    listMatters: async () => ({
      data: { items: [], pageInfo: { mode: "cursor", hasMore: false } },
    }),
    listStaff: async () => ({
      data: { items: [], pageInfo: { mode: "cursor", hasMore: false } },
    }),
    listCases: async () => ({
      data: { items: [], pageInfo: { mode: "cursor", hasMore: false } },
    }),
    getCase: async () => ({ data: createCaseRecord() }),
    createCase: async () => ({ data: createCaseRecord() }),
    uploadCaseFile: async () => ({ data: { id: "file-1" } }),
    attachPartySignature: async () => ({ data: { id: "party-1" } }),
    createPartyVideoInvite: async () => ({
      data: {
        inviteId: "invite-1",
        conversationId: "conversation-1",
        inviteUrl: "https://notary.example.test/invites/invite-1",
        expiresAt: "2026-07-29T09:00:00Z",
      },
    }),
    getDashboardStatistics: async () => ({ data: {} }),
    ...overrides,
  };
}

test("passes cursor input and preserves cursor page metadata", async () => {
  let receivedInput: unknown;
  const service = createNotaryService(() => createApi({
    listCases: async (input) => {
      receivedInput = input;
      return {
        data: {
          items: [createCaseRecord()],
          pageInfo: {
            mode: "cursor",
            hasMore: true,
            nextCursor: "cursor-2",
          },
        },
      };
    },
  }));

  const page = await service.getNotaryRecords("PENDING_REVIEW", "cursor-1");

  assert.deepEqual(receivedInput, {
    status: "PENDING_REVIEW",
    pageSize: 20,
    cursor: "cursor-1",
  });
  assert.deepEqual(page.pageInfo, {
    mode: "cursor",
    hasMore: true,
    nextCursor: "cursor-2",
  });
  assert.equal(page.records[0]?.status, "PENDING_REVIEW");
});

test("rejects non-cursor list responses", async () => {
  const service = createNotaryService(() => createApi({
    listCases: async () => ({
      data: {
        items: [],
        pageInfo: { mode: "number", hasMore: false },
      },
    }),
  }));

  await assert.rejects(
    service.getNotaryRecords("ALL"),
    NotaryResponseContractError,
  );
});

test("rejects hasMore without nextCursor", async () => {
  const service = createNotaryService(() => createApi({
    listCases: async () => ({
      data: {
        items: [],
        pageInfo: { mode: "cursor", hasMore: true },
      },
    }),
  }));

  await assert.rejects(
    service.getNotaryRecords("ALL"),
    /hasMore=true without pageInfo.nextCursor/,
  );
});

test("rejects unknown notary case statuses", async () => {
  const service = createNotaryService(() => createApi({
    listCases: async () => ({
      data: {
        items: [createCaseRecord({ status: "APPROVED" })],
        pageInfo: { mode: "cursor", hasMore: false },
      },
    }),
  }));

  await assert.rejects(
    service.getNotaryRecords("ALL"),
    /unsupported status: APPROVED/,
  );
});

test("forwards one stable idempotency key and uploads files sequentially", async () => {
  let createInput: Record<string, unknown> | undefined;
  const uploadOrder: string[] = [];
  let activeUploads = 0;
  let maximumActiveUploads = 0;
  const service = createNotaryService(() => createApi({
    createCase: async (input) => {
      createInput = input as unknown as Record<string, unknown>;
      return { data: createCaseRecord() };
    },
    uploadCaseFile: async (input) => {
      activeUploads += 1;
      maximumActiveUploads = Math.max(maximumActiveUploads, activeUploads);
      uploadOrder.push(String((input as { uploadIntentId?: string }).uploadIntentId));
      await Promise.resolve();
      activeUploads -= 1;
      return { data: { id: "file-1" } };
    },
  }));
  const firstFile = new File(["first"], "first.txt", { type: "text/plain" });
  const secondFile = new File(["second"], "second.txt", { type: "text/plain" });

  await service.createCase({
    skuId: "matter-1",
    title: "Property authorization",
    applicantName: "Applicant",
    description: "Notary request",
    parties: [{ name: "Applicant", idCard: "11010519900101234X" }],
    attachments: [
      {
        id: "upload-1",
        name: firstFile.name,
        file: firstFile,
        previewUrl: "blob:first",
        type: "file",
        size: "5 B",
      },
      {
        id: "upload-2",
        name: secondFile.name,
        file: secondFile,
        previewUrl: "blob:second",
        type: "file",
        size: "6 B",
      },
    ],
    idempotencyKey: "stable-key",
  });

  assert.equal(createInput?.idempotencyKey, "stable-key");
  assert.deepEqual(uploadOrder, ["upload-1", "upload-2"]);
  assert.equal(maximumActiveUploads, 1);
});

test("bounded merge de-duplicates and never exceeds the requested limit", () => {
  const merged = appendBoundedUnique(
    [{ id: "1" }, { id: "2" }],
    [{ id: "2" }, { id: "3" }, { id: "4" }],
    (item) => item.id,
    3,
  );

  assert.deepEqual(merged, [{ id: "1" }, { id: "2" }, { id: "3" }]);
});
