import type {
  DriveUploaderClient,
  DriveUploaderRequest,
  DriveUploaderUploadResult,
} from "@sdkwork/drive-app-sdk";
import { getDriveAppSdkClientWithSession } from "@sdkwork/im-h5-core/sdk";
import { readImH5IamSessionTokens } from "@sdkwork/im-h5-core/session";

export type ChatMediaUploadType = "image" | "audio" | "video" | "attachment";

export interface ChatDriveReference {
  driveUri: string;
  spaceId: string;
  nodeId: string;
}

const CHAT_DRIVE_APP_RESOURCE_TYPE = "im_conversation";
const CHAT_DRIVE_SCENE = "im";
const CHAT_DRIVE_SOURCE = "chat_message";

function resolveUploadUserId(): string {
  const session = readImH5IamSessionTokens();
  const userId =
    session?.context?.userId?.trim()
    || session?.user?.userId?.trim()
    || session?.user?.id?.trim();
  if (!userId) {
    throw new Error("Chat media upload requires user_id in the authenticated session.");
  }
  return userId;
}

function normalizeDriveUploadResult(result: DriveUploaderUploadResult): ChatDriveReference {
  const spaceId = result.uploadItem.spaceId || result.uploadSession.spaceId;
  const nodeId = result.uploadItem.nodeId || result.uploadSession.nodeId;
  if (!spaceId || !nodeId) {
    throw new Error("Drive uploader result is missing spaceId or nodeId.");
  }
  return {
    driveUri: `drive://spaces/${spaceId}/nodes/${nodeId}`,
    spaceId,
    nodeId,
  };
}

function getDriveUploader(): Pick<
  DriveUploaderClient,
  "uploadAudio" | "uploadAttachment" | "uploadImage" | "uploadVideo"
> {
  return getDriveAppSdkClientWithSession().uploader;
}

export async function uploadChatMediaFile({
  conversationId,
  file,
  type,
  originalFileName,
  contentType,
}: {
  conversationId: string;
  file: Blob;
  type: ChatMediaUploadType;
  originalFileName?: string;
  contentType?: string;
}): Promise<{ drive: ChatDriveReference; uploadResult: DriveUploaderUploadResult }> {
  const uploadRequest: DriveUploaderRequest = {
    file,
    userId: resolveUploadUserId(),
    appResourceType: CHAT_DRIVE_APP_RESOURCE_TYPE,
    appResourceId: conversationId,
    scene: CHAT_DRIVE_SCENE,
    source: CHAT_DRIVE_SOURCE,
    ...(originalFileName ? { originalFileName } : {}),
    ...(contentType ? { contentType } : {}),
  };

  const uploader = getDriveUploader();
  const uploadResult =
    type === "image"
      ? await uploader.uploadImage(uploadRequest)
      : type === "audio"
        ? await uploader.uploadAudio(uploadRequest)
        : type === "video"
          ? await uploader.uploadVideo(uploadRequest)
          : await uploader.uploadAttachment(uploadRequest);

  return {
    drive: normalizeDriveUploadResult(uploadResult),
    uploadResult,
  };
}
