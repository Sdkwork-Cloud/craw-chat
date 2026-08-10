/**
 * IM H5 AI Voice runtime integration.
 *
 * Composes the voice-owned mobile packages into the IM H5 host:
 * - injects the bootstrap-constructed voice app SDK client into
 *   `@sdkwork/voice-mobile-my-voices` service ports
 * - supplies host media capabilities (SDKWork Drive upload + signed playback
 *   URL grants) used by the my-voices create/preview flows
 * - registers my-voices i18n resources into the host i18next instance
 */

import { i18n } from '@sdkwork/im-h5-commons';
import {
  getDriveAppSdkClient,
  getVoiceAppSdkClient,
} from '@sdkwork/im-h5-core/sdk';
import {
  configureMyVoiceSdkPorts,
  registerMyVoicesI18n,
  type MyVoiceMediaSample,
  type MyVoiceProfilesClient,
} from '@sdkwork/voice-mobile-my-voices';

let configured = false;

function resolveMyVoiceClient(): MyVoiceProfilesClient {
  return getVoiceAppSdkClient() as unknown as MyVoiceProfilesClient;
}

function toMediaSample(uploadResult: {
  uploadItem: {
    spaceId?: string;
    nodeId?: string;
    originalFileName?: string;
    contentType?: string;
    contentLength?: string;
  };
  uploadSession: { spaceId?: string; nodeId?: string };
}): MyVoiceMediaSample {
  const spaceId = uploadResult.uploadItem.spaceId || uploadResult.uploadSession.spaceId;
  const nodeId = uploadResult.uploadItem.nodeId || uploadResult.uploadSession.nodeId;
  const sample: MyVoiceMediaSample = {
    kind: 'audio',
    source: 'drive',
    fileName: uploadResult.uploadItem.originalFileName,
    mimeType: uploadResult.uploadItem.contentType,
    sizeBytes: uploadResult.uploadItem.contentLength,
  };
  if (spaceId && nodeId) {
    sample.spaceId = spaceId;
    sample.nodeId = nodeId;
    sample.uri = `drive://spaces/${spaceId}/nodes/${nodeId}`;
  }
  return sample;
}

export function configureVoiceMyVoicesRuntime(): void {
  if (configured) {
    return;
  }
  const driveClient = getDriveAppSdkClient();
  configureMyVoiceSdkPorts({
    getVoiceClient: () => resolveMyVoiceClient(),
    uploadAudioSample: async (file, options) => {
      const uploadResult = await driveClient.uploader.uploadAudio({
        file,
        appResourceType: 'voice_profile',
        appResourceId: 'my_voices',
        scene: 'voice',
        source: 'voice_profile',
        uploadProfileCode: 'audio',
        ...(options?.fileName ? { originalFileName: options.fileName } : {}),
        ...(options?.mimeType ? { contentType: options.mimeType } : {}),
      });
      const sample = toMediaSample(uploadResult);
      if (options?.durationSeconds !== undefined) {
        sample.durationSeconds = options.durationSeconds;
      }
      return sample;
    },
    resolveMediaPlaybackUrl: async (sample) => {
      if (sample.nodeId) {
        const response = await driveClient.drive.downloadGrants.create(sample.nodeId, {
          requestedTtlSeconds: 900,
        });
        const url = response.downloadUrl || response.signedSourceUrl;
        if (typeof url === 'string' && url) {
          return url;
        }
      }
      return null;
    },
  });
  registerMyVoicesI18n(i18n);
  configured = true;
}
