import type { ContentPart, ConversationMessageListResponse, ConversationProfileView, PostMessageResult } from "@sdkwork/im-sdk";
import { getImSdkClient } from "@sdkwork/im-h5-core";

import { uploadChatMediaFile } from "./chatMediaUploadService";

export interface FetchConversationMessagesOptions {
  pageSize?: number;
  afterSeq?: number;
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
    pageSize: options?.pageSize ?? 20,
    afterSeq: options?.afterSeq ?? 0,
  });
}

export async function fetchConversationMessageDelta(
  conversationId: string,
  afterSeq: number,
  pageSize = 20,
): Promise<ConversationMessageListResponse> {
  return getImSdkClient().conversations.listMessages(conversationId, {
    afterSeq,
    pageSize,
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
