import {
  getDriveAppSdkClientWithSession,
  type SdkworkDriveAppClient,
  type DriveUploaderRequest,
  type DriveUploaderUploadResult,
} from '@sdkwork/im-h5-core';

export interface ChatMediaUploadRequest {
  blob: Blob;
  filename: string;
  mimeType: string;
  conversationId: string;
}

export interface ChatMediaUploadResult {
  fileId: string;
  url: string;
  filename: string;
  mimeType: string;
  size: number;
}

function toUploaderRequest(request: ChatMediaUploadRequest): DriveUploaderRequest {
  return {
    blob: request.blob,
    filename: request.filename,
    mimeType: request.mimeType,
  } as unknown as DriveUploaderRequest;
}

function toChatMediaUploadResult(
  uploadResult: DriveUploaderUploadResult,
  request: ChatMediaUploadRequest,
): ChatMediaUploadResult {
  const record = uploadResult as unknown as Record<string, unknown>;
  return {
    fileId: String(record.fileId ?? record.id ?? record.nodeId ?? ''),
    url: String(record.url ?? record.downloadUrl ?? record.location ?? ''),
    filename: request.filename,
    mimeType: request.mimeType,
    size: request.blob.size,
  };
}

export async function uploadChatMedia(request: ChatMediaUploadRequest): Promise<ChatMediaUploadResult> {
  const driveClient = getDriveAppSdkClientWithSession();
  const uploadResult = await driveClient.uploader.uploadAttachment(toUploaderRequest(request));
  return toChatMediaUploadResult(uploadResult, request);
}

export function getChatMediaUploadClient(): SdkworkDriveAppClient {
  return getDriveAppSdkClientWithSession();
}
