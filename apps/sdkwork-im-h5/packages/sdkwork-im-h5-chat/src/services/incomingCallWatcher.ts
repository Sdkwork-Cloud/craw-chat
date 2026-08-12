import type { RtcCallSessionInfo, RtcCallSignalingPort } from "@sdkwork/rtc-h5-call";
import { imLiveService } from "@sdkwork/im-h5-core/realtime";

import {
  imH5CallSignaling,
  resolveImH5CallDeviceId,
  resolveImH5CallParticipantId,
} from "./imH5CallSignaling";

/**
 * Global incoming-call watcher.
 *
 * Watches the IM call signaling stream (user-scope WebSocket events through
 * the shared H5 realtime connection) and lifts ringing incoming sessions so
 * the app can auto-navigate to the full-screen call page. The page consumes
 * the pending call through `consumePendingIncomingCall`.
 */

export interface PendingIncomingCall {
  rtcSessionId: string;
  conversationId?: string;
  rtcMode?: string;
  type: "video" | "voice";
}

export interface IncomingCallWatcherOptions {
  signaling?: RtcCallSignalingPort;
  onIncoming?: (call: PendingIncomingCall) => void;
}

function toPendingIncomingCall(session: RtcCallSessionInfo): PendingIncomingCall | null {
  const state = session.state;
  const isRinging =
    state === "started" || state === "initiating" || state === "ringing";
  if (!isRinging) {
    return null;
  }
  return {
    rtcSessionId: session.rtcSessionId,
    conversationId: session.conversationId,
    rtcMode: session.rtcMode,
    type: session.rtcMode === "video" || session.rtcMode === "video_call" ? "video" : "voice",
  };
}

class IncomingCallWatcher {
  private readonly listeners = new Set<(call: PendingIncomingCall) => void>();
  private pending: PendingIncomingCall | null = null;
  private signaling: RtcCallSignalingPort = imH5CallSignaling;
  private unsubscribeCalls?: () => void;
  private unsubscribeScope?: () => void;
  private started = false;
  private disposed = false;
  private lastEmittedRtcSessionId?: string;

  async start(options: IncomingCallWatcherOptions = {}): Promise<() => void> {
    if (this.started) {
      this.disposed = false;
      if (options.onIncoming) {
        this.listeners.add(options.onIncoming);
      }
      return () => this.stop();
    }
    this.started = true;
    this.disposed = false;
    if (options.signaling) {
      this.signaling = options.signaling;
    }
    if (options.onIncoming) {
      this.listeners.add(options.onIncoming);
    }

    // Live session events (accepted / rejected / ended / new incoming) arrive
    // through the signaling subscription; the user-scope realtime subscription
    // keeps the shared connection demanded so events keep flowing.
    this.unsubscribeCalls = this.signaling.subscribe((session) => {
      const call = toPendingIncomingCall(session);
      if (call) {
        this.emit(call);
      }
    });

    try {
      const principalId = resolveImH5CallParticipantId();
      this.unsubscribeScope = imLiveService.subscribeScopeEvents(
        "user",
        principalId,
        () => undefined,
      );
    } catch {
      // Session may not be ready yet; the signaling subscription still covers
      // direct events and the scope is refreshed on the next start.
    }

    // One-shot query catches a call that was already ringing before subscribe.
    try {
      const session = await this.signaling.watchIncoming({
        conversationIds: [],
        deviceId: resolveImH5CallDeviceId(),
        principalId: resolveImH5CallParticipantId(),
      });
      if (!this.disposed && session) {
        const call = toPendingIncomingCall(session);
        if (call) {
          this.emit(call);
        }
      }
    } catch {
      // Watching is best-effort; live events remain the primary channel.
    }

    return () => this.stop();
  }

  stop(): void {
    this.disposed = true;
    this.started = false;
    this.unsubscribeCalls?.();
    this.unsubscribeCalls = undefined;
    this.unsubscribeScope?.();
    this.unsubscribeScope = undefined;
    this.listeners.clear();
  }

  consumePending(): PendingIncomingCall | null {
    const call = this.pending;
    this.pending = null;
    return call;
  }

  private emit(call: PendingIncomingCall): void {
    if (call.rtcSessionId === this.lastEmittedRtcSessionId) {
      return;
    }
    this.lastEmittedRtcSessionId = call.rtcSessionId;
    this.pending = call;
    for (const listener of this.listeners) {
      try {
        listener(call);
      } catch {
        // One observer must not break the watcher for others.
      }
    }
  }
}

export const incomingCallWatcher = new IncomingCallWatcher();

export function startIncomingCallWatcher(
  options: IncomingCallWatcherOptions,
): Promise<() => void> {
  return incomingCallWatcher.start(options);
}

export function consumePendingIncomingCall(): PendingIncomingCall | null {
  return incomingCallWatcher.consumePending();
}
