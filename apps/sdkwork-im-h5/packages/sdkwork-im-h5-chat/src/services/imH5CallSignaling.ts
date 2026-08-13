import type { ImCallSession, ImLiveConnection } from "@sdkwork/im-h5-core/sdk";
import { useAppStore } from "@sdkwork/im-h5-core";
import { imLiveService } from "@sdkwork/im-h5-core/realtime";
import type {
  RtcCallParticipantCredential,
  RtcCallSessionInfo,
  RtcCallSignalingPort,
  RtcCallStartOptions,
  RtcCallWatchOptions,
} from "@sdkwork/rtc-h5-call";

import { getChatImSdkClient } from "./chatConversationService";

/**
 * IM H5 signaling adapter for the RTC call surface.
 *
 * Implements the `RtcCallSignalingPort` defined by `@sdkwork/rtc-h5-call`
 * (the RTC authority package) with the IM-owned call signaling stack:
 * `@sdkwork/im-sdk` `.calls.*` (start/invite/accept/reject/end/retrieve/
 * issueParticipantCredential/watchIncoming/subscribe) plus the shared H5
 * realtime connection. Per the RTC↔IM boundary this is the *only* place
 * where IM signaling touches the call surface — the RTC package stays free
 * of any IM dependency.
 */

const IM_H5_DEVICE_ID_STORAGE_KEY = "sdkwork-im-h5-device-id";

export function resolveImH5CallParticipantId(): string {
  const user = useAppStore.getState().currentUser;
  if (user?.id) {
    return user.id;
  }
  throw new Error("Sdkwork IM login session does not include a user id for calls.");
}

export function resolveImH5CallDeviceId(): string {
  try {
    let deviceId = window.localStorage.getItem(IM_H5_DEVICE_ID_STORAGE_KEY);
    if (!deviceId) {
      deviceId =
        typeof crypto !== "undefined" && typeof crypto.randomUUID === "function"
          ? `h5-${crypto.randomUUID()}`
          : `h5-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`;
      window.localStorage.setItem(IM_H5_DEVICE_ID_STORAGE_KEY, deviceId);
    }
    return deviceId;
  } catch {
    return "im-h5-browser";
  }
}

export async function ensureImH5CallConnection(): Promise<ImLiveConnection> {
  return imLiveService.ensureImLiveConnection();
}

function mapSession(session: ImCallSession): RtcCallSessionInfo {
  return {
    rtcSessionId: session.rtcSessionId,
    conversationId: session.conversationId ?? undefined,
    initiatorId: session.initiatorId ?? undefined,
    initiatorKind: session.initiatorKind ?? undefined,
    providerPluginId: session.providerPluginId ?? undefined,
    providerSessionId: session.providerSessionId ?? undefined,
    accessEndpoint: session.accessEndpoint ?? undefined,
    providerRegion: session.providerRegion ?? undefined,
    rtcMode: session.rtcMode ?? undefined,
    state: session.state,
    signalingStreamId: session.signalingStreamId ?? undefined,
    artifactMessageId: session.artifactMessageId ?? undefined,
    startedAt: session.startedAt ?? undefined,
    endedAt: session.endedAt ?? undefined,
  };
}

function mapCredential(credential: {
  tenantId: string;
  rtcSessionId: string;
  participantId: string;
  credential: string;
  expiresAt: string;
}): RtcCallParticipantCredential {
  return {
    tenantId: credential.tenantId,
    rtcSessionId: credential.rtcSessionId,
    participantId: credential.participantId,
    credential: credential.credential,
    expiresAt: credential.expiresAt,
  };
}

export class ImH5CallSignaling implements RtcCallSignalingPort {
  async startOutgoingCall(options: RtcCallStartOptions): Promise<RtcCallSessionInfo> {
    const client = getChatImSdkClient();
    const created = await client.calls.start({
      conversationId: options.conversationId,
      rtcMode: options.rtcMode,
      rtcSessionId: options.rtcSessionId,
    });
    const invited = await client.calls.invite(created.rtcSessionId, {
      signalingStreamId: options.signalingStreamId,
    });
    return mapSession(invited);
  }

  async retrieve(rtcSessionId: string): Promise<RtcCallSessionInfo> {
    return mapSession(await getChatImSdkClient().calls.retrieve(rtcSessionId));
  }

  async invite(
    rtcSessionId: string,
    options: { signalingStreamId?: string } = {},
  ): Promise<RtcCallSessionInfo> {
    return mapSession(
      await getChatImSdkClient().calls.invite(rtcSessionId, {
        signalingStreamId: options.signalingStreamId,
      }),
    );
  }

  async accept(rtcSessionId: string): Promise<RtcCallSessionInfo> {
    return mapSession(await getChatImSdkClient().calls.accept(rtcSessionId));
  }

  async reject(rtcSessionId: string): Promise<RtcCallSessionInfo> {
    return mapSession(await getChatImSdkClient().calls.reject(rtcSessionId));
  }

  async end(rtcSessionId: string): Promise<RtcCallSessionInfo> {
    return mapSession(await getChatImSdkClient().calls.end(rtcSessionId));
  }

  async issueParticipantCredential(
    rtcSessionId: string,
    options: { participantId: string },
  ): Promise<RtcCallParticipantCredential> {
    const credential = await getChatImSdkClient().calls.issueParticipantCredential(rtcSessionId, {
      participantId: options.participantId || resolveImH5CallParticipantId(),
    });
    return mapCredential(credential);
  }

  async watchIncoming(options: RtcCallWatchOptions): Promise<RtcCallSessionInfo | null> {
    const client = getChatImSdkClient();
    const connection = await ensureImH5CallConnection();
    const principalId = options.principalId || resolveImH5CallParticipantId();
    const session = await client.calls.watchIncoming({
      connection,
      conversationIds: options.conversationIds,
      deviceId: resolveImH5CallDeviceId(),
      principalId,
    });
    return session ? mapSession(session) : null;
  }

  subscribe(handler: (session: RtcCallSessionInfo) => void): () => void {
    return getChatImSdkClient().calls.subscribe((session) => {
      handler(mapSession(session));
    });
  }
}

export function createImH5CallSignaling(): RtcCallSignalingPort {
  return new ImH5CallSignaling();
}

export const imH5CallSignaling: RtcCallSignalingPort = createImH5CallSignaling();
