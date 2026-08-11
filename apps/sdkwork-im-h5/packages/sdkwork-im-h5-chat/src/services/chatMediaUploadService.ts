import {
  getDriveAppSdkClient,
  type DriveUploaderBlobLike,
  type DriveUploaderProfile,
  type DriveUploaderUploadResult,
  type SdkworkDriveAppClient,
} from '@sdkwork/im-h5-core/sdk';

export function getDriveAppSdkClientWithSession(): SdkworkDriveAppClient {
  return getDriveAppSdkClient();
}

export interface ChatMediaUpload {
  drive: { driveUri: string; spaceId: string; nodeId: string };
  resource: {
    id: string;
    kind: "image" | "file" | "audio" | "video" | "voice" | "document";
    source: "drive";
    uri: string;
    fileName?: string;
    mimeType?: string;
    sizeBytes?: string;
    durationSeconds?: number;
  };
  uploadResult: DriveUploaderUploadResult;
}

export async function uploadChatMedia(
  conversationId: string,
  file: DriveUploaderBlobLike,
  kind: ChatMediaUpload["resource"]["kind"],
  options: { durationSeconds?: number; fileName?: string; mimeType?: string } = {},
): Promise<ChatMediaUpload> {
  const client = getDriveAppSdkClientWithSession();
  const profile: DriveUploaderProfile = kind === "image" ? "image" : kind === "video" ? "video" : kind === "voice" || kind === "audio" ? "audio" : "attachment";
  const request = {
    file, appResourceType: "im_conversation", appResourceId: conversationId, scene: "im", source: "chat_message", uploadProfileCode: profile,
    ...(options.fileName ? { originalFileName: options.fileName } : {}), ...(options.mimeType ? { contentType: options.mimeType } : {}),
  };
  const uploadResult = kind === "image" ? await client.uploader.uploadImage(request) : kind === "video" ? await client.uploader.uploadVideo(request) : kind === "voice" || kind === "audio" ? await client.uploader.uploadAudio(request) : await client.uploader.uploadAttachment(request);
  const spaceId = uploadResult.uploadItem.spaceId || uploadResult.uploadSession.spaceId;
  const nodeId = uploadResult.uploadItem.nodeId || uploadResult.uploadSession.nodeId;
  if (!spaceId || !nodeId) throw new Error("Drive upload did not return a space or node id.");
  const driveUri = `drive://spaces/${spaceId}/nodes/${nodeId}`;
  return { drive: { driveUri, spaceId, nodeId }, resource: { id: nodeId, kind, source: "drive", uri: driveUri, ...(uploadResult.uploadItem.originalFileName ? { fileName: uploadResult.uploadItem.originalFileName } : {}), ...(uploadResult.uploadItem.contentType ? { mimeType: uploadResult.uploadItem.contentType } : {}), ...(uploadResult.uploadItem.contentLength ? { sizeBytes: uploadResult.uploadItem.contentLength } : {}), ...(options.durationSeconds !== undefined ? { durationSeconds: options.durationSeconds } : {}) }, uploadResult };
}

// Download grants are requested with a 900s TTL; caching below that bound
// means a page reload or a realtime merge for an already-rendered node never
// re-issues a grant (fix: repeated full-page loads used to mint a new grant
// per message per load).
const DOWNLOAD_URL_CACHE_TTL_MS = 8 * 60 * 1000;
const DOWNLOAD_URL_CACHE_MAX_ENTRIES = 500;
const downloadUrlCache = new Map<string, { url: string; expiresAt: number }>();

export async function createChatMediaDownloadUrl(nodeId: string): Promise<string> {
  const cached = downloadUrlCache.get(nodeId);
  if (cached && cached.expiresAt > Date.now()) {
    return cached.url;
  }
  const response = await getDriveAppSdkClientWithSession().drive.downloadGrants.create(nodeId, { requestedTtlSeconds: 900 });
  const url = response.downloadUrl || response.signedSourceUrl;
  if (!url) throw new Error("Drive download grant did not return a URL.");
  // Bound the cache: evict the oldest entry when it grows past the cap so a
  // busy conversation cannot accumulate grants indefinitely.
  if (downloadUrlCache.size >= DOWNLOAD_URL_CACHE_MAX_ENTRIES) {
    const oldestKey = downloadUrlCache.keys().next().value as string | undefined;
    if (oldestKey) downloadUrlCache.delete(oldestKey);
  }
  downloadUrlCache.set(nodeId, { url, expiresAt: Date.now() + DOWNLOAD_URL_CACHE_TTL_MS });
  return url;
}
