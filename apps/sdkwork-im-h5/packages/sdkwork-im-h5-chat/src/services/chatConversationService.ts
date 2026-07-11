import type { ContentPart, ConversationMessageListResponse, ConversationProfileView, PostMessageResult } from "@sdkwork/im-sdk";
import { getImSdkClient } from "@sdkwork/im-h5-core";

import { uploadChatMediaFile } from "./chatMediaUploadService";

export interface FetchConversationMessagesOptions {
  pageSize?: number;
  cursor?: string;
}

const DEFAULT_MESSAGE_PAGE_SIZE = 20;
const MAX_MESSAGE_PAGE_SIZE = 200;

function normalizeMessagePageSize(pageSize: number | undefined): number {
  if (pageSize === undefined) {
    return DEFAULT_MESSAGE_PAGE_SIZE;
  }
  const normalized = Math.floor(pageSize);
  if (!Number.isFinite(normalized) || normalized <= 0) {
    return DEFAULT_MESSAGE_PAGE_SIZE;
  }
  return Math.min(normalized, MAX_MESSAGE_PAGE_SIZE);
}

function buildMediaMessageParts(
  drive: { driveUri: string; spaceId: string; nodeId: string },
  fileName: string,
  mimeType: string,
  sizeBytes: number,
): ContentPart[] {
  return [
    {
      kind: "media",
      drive: {
        driveUri: drive.driveUri,
        spaceId: drive.spaceId,
        nodeId: drive.nodeId,
      },
      resource: {
        source: "drive",
        uri: drive.driveUri,
        fileName,
        mimeType,
        sizeBytes: String(Math.max(0, sizeBytes)),
        kind: "image",
      },
      mediaRole: "attachment",
    },
  ];
}

export async function fetchConversationMessages(
  conversationId: string,
  options?: FetchConversationMessagesOptions,
): Promise<ConversationMessageListResponse> {
  return getImSdkClient().conversations.listMessages(conversationId, {
    pageSize: normalizeMessagePageSize(options?.pageSize),
    ...(options?.cursor ? { cursor: options.cursor } : {}),
  });
}

export async function fetchConversationMessageDelta(
  conversationId: string,
  pageSize = DEFAULT_MESSAGE_PAGE_SIZE,
): Promise<ConversationMessageListResponse> {
  // The cursor contract walks backwards through older history. Live refreshes
  // must request the latest page without reusing that older-history cursor.
  return getImSdkClient().conversations.listMessages(conversationId, {
    pageSize: normalizeMessagePageSize(pageSize),
  });
}

export async function fetchConversationProfile(conversationId: string): Promise<ConversationProfileView> {
  return getImSdkClient().conversations.getProfile(conversationId);
}

export async function sendConversationText(
  conversationId: string,
  text: string,
  options?: { clientMsgId?: string },
): Promise<PostMessageResult> {
  return getImSdkClient().conversations.postText(conversationId, text.trim(), {
    clientMsgId: options?.clientMsgId,
  });
}

export async function sendConversationImage(
  conversationId: string,
  file: File,
): Promise<PostMessageResult> {
  const { drive, uploadResult } = await uploadChatMediaFile({
    conversationId,
    file,
    type: "image",
    originalFileName: file.name,
    contentType: file.type || "application/octet-stream",
  });
  const fileName = uploadResult.uploadItem.originalFileName ?? file.name;
  const mimeType = uploadResult.uploadItem.contentType ?? file.type ?? "application/octet-stream";
  const sizeBytes = Number(uploadResult.uploadItem.contentLength ?? file.size);

  return getImSdkClient().conversations.postMessage(conversationId, {
    clientMsgId: `h5-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
    summary: fileName,
    parts: buildMediaMessageParts(drive, fileName, mimeType, sizeBytes),
  });
}
