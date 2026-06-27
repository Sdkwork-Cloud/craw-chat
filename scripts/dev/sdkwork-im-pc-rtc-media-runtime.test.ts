import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkCallService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/CallService';
import type {
  SdkworkRtcMediaJoinOptions,
  SdkworkRtcMediaPublishOptions,
  SdkworkRtcMediaService,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/RtcMediaService';
import { createSdkworkRtcMediaService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/RtcMediaService';

type CallSessionListener = Parameters<NonNullable<ImSdkClient['calls']['subscribe']>>[0];

const mediaCalls: Array<{
  audioMuted?: boolean;
  kind?: string;
  method: string;
  participantId?: string;
  providerKey?: string;
  providerRegion?: string;
  roomId?: string;
  rtcSessionId?: string;
  token?: string;
  videoMuted?: boolean;
}> = [];

function createDeferred<T>(): {
  promise: Promise<T>;
  reject: (error: unknown) => void;
  resolve: (value: T) => void;
} {
  let rejectPromise: (error: unknown) => void = () => undefined;
  let resolvePromise: (value: T) => void = () => undefined;
  const promise = new Promise<T>((resolve, reject) => {
    rejectPromise = reject;
    resolvePromise = resolve;
  });
  return {
    promise,
    reject: rejectPromise,
    resolve: resolvePromise,
  };
}

const rtcMediaService: SdkworkRtcMediaService = {
  async join(options: SdkworkRtcMediaJoinOptions) {
    mediaCalls.push({
      method: 'rtc.join',
      participantId: options.participantId,
      providerKey: options.providerKey,
      providerRegion: options.providerRegion,
      roomId: options.roomId,
      rtcSessionId: options.rtcSessionId,
      token: options.credential.credential,
    });
  },
  async publish(options: SdkworkRtcMediaPublishOptions) {
    for (const kind of options.kinds) {
      mediaCalls.push({
        method: 'rtc.publish',
        kind,
        rtcSessionId: options.rtcSessionId,
      });
    }
  },
  async muteAudio(muted: boolean) {
    mediaCalls.push({
      method: 'rtc.muteAudio',
      audioMuted: muted,
    });
  },
  async muteVideo(muted: boolean) {
    mediaCalls.push({
      method: 'rtc.muteVideo',
      videoMuted: muted,
    });
  },
  async bindLocalVideoElement() {},
  async leave() {
    mediaCalls.push({
      method: 'rtc.leave',
    });
  },
};

function createSession(rtcSessionId: string, state: 'accepted' | 'ended' | 'started') {
  return {
    tenantId: '100001',
    rtcSessionId,
    conversationId: 'conversation-rtc-media-1',
    rtcMode: 'video',
    initiatorId: 'u_bob',
    initiatorKind: 'user',
    providerPluginId: 'volcengine',
    providerSessionId: `room-${rtcSessionId}`,
    accessEndpoint: 'rtc:volcengine',
    providerRegion: 'cn-north-1',
    startedAt: '2026-06-09T10:00:00.000Z',
    state,
  };
}

async function waitForCondition(predicate: () => boolean, label: string): Promise<void> {
  for (let attempt = 0; attempt < 10; attempt += 1) {
    if (predicate()) {
      return;
    }
    await Promise.resolve();
  }
  assert.equal(predicate(), true, label);
}

async function main(): Promise<void> {
  const listeners: CallSessionListener[] = [];
  const imCalls: Array<{ method: string; participantId?: string; rtcSessionId?: string }> = [];
  const imClient = {
    calls: {
      subscribe(handler: CallSessionListener) {
        listeners.push(handler);
        return () => undefined;
      },
      async watchIncoming() {
        return null;
      },
      async issueParticipantCredential(rtcSessionId: string, body: { participantId: string }) {
        imCalls.push({
          method: 'calls.issueParticipantCredential',
          participantId: body.participantId,
          rtcSessionId,
        });
        return {
          tenantId: '100001',
          rtcSessionId,
          participantId: body.participantId,
          credential: 'provider-token-1',
          expiresAt: '2026-06-09T10:10:00.000Z',
        };
      },
      async end(rtcSessionId: string) {
        imCalls.push({
          method: 'calls.end',
          rtcSessionId,
        });
        return createSession(rtcSessionId, 'ended');
      },
    },
  } as unknown as ImSdkClient;

  const service = createSdkworkCallService({
    getClient: () => imClient,
    rtcMediaService,
    readSession: () => ({
      authToken: 'auth-token-1',
      sessionId: 'session-1',
      context: {
        appId: 'app-1',
        tenantId: '100001',
        userId: 'u_alice',
        sessionId: 'session-1',
        environment: 'dev',
        deploymentMode: 'saas',
        authLevel: 'password',
        dataScope: [],
        permissionScope: [],
      },
      user: {
        userId: 'u_alice',
        displayName: 'Alice',
      },
    }),
  });

  await service.watchIncomingCalls(['conversation-rtc-media-1']);
  listeners[0]?.(createSession('rtc-media-1', 'started'));
  assert.equal(service.getSnapshot().state, 'ringing');
  listeners[0]?.(createSession('rtc-media-1', 'accepted'));

  await waitForCondition(
    () => mediaCalls.some((call) => call.method === 'rtc.publish' && call.kind === 'video'),
    'Connected video calls must join the RTC media runtime and publish local media.',
  );

  assert.deepEqual(imCalls, [
    {
      method: 'calls.issueParticipantCredential',
      participantId: 'u_alice',
      rtcSessionId: 'rtc-media-1',
    },
  ]);
  assert.deepEqual(mediaCalls, [
    {
      method: 'rtc.join',
      participantId: 'u_alice',
      providerKey: 'volcengine',
      providerRegion: 'cn-north-1',
      roomId: 'room-rtc-media-1',
      rtcSessionId: 'rtc-media-1',
      token: 'provider-token-1',
    },
    {
      method: 'rtc.publish',
      kind: 'audio',
      rtcSessionId: 'rtc-media-1',
    },
    {
      method: 'rtc.publish',
      kind: 'video',
      rtcSessionId: 'rtc-media-1',
    },
  ]);

  await service.setAudioMuted(true);
  await service.setVideoMuted(true);
  assert.deepEqual(mediaCalls.slice(-2), [
    {
      method: 'rtc.muteAudio',
      audioMuted: true,
    },
    {
      method: 'rtc.muteVideo',
      videoMuted: true,
    },
  ]);

  await service.endCall({ reason: 'test_hangup' });
  assert.deepEqual(mediaCalls.slice(-1), [
    {
      method: 'rtc.leave',
    },
  ]);

  const muteRollbackCalls: string[] = [];
  const muteRollbackListeners: CallSessionListener[] = [];
  const muteRollbackClient = {
    calls: {
      subscribe(handler: CallSessionListener) {
        muteRollbackListeners.push(handler);
        return () => undefined;
      },
      async watchIncoming() {
        return null;
      },
      async issueParticipantCredential(rtcSessionId: string, body: { participantId: string }) {
        return {
          tenantId: '100001',
          rtcSessionId,
          participantId: body.participantId,
          credential: 'provider-token-mute-rollback',
          expiresAt: '2026-06-09T10:12:00.000Z',
        };
      },
    },
  } as unknown as ImSdkClient;
  const muteRollbackMediaService: SdkworkRtcMediaService = {
    async join() {
      muteRollbackCalls.push('join');
    },
    async publish() {
      muteRollbackCalls.push('publish');
    },
    async muteAudio() {
      muteRollbackCalls.push('muteAudio');
      throw new Error('volcengine mute audio failed');
    },
    async muteVideo() {
      muteRollbackCalls.push('muteVideo');
      throw new Error('volcengine mute video failed');
    },
    async bindLocalVideoElement() {},
    async leave() {
      muteRollbackCalls.push('leave');
    },
  };
  const muteRollbackService = createSdkworkCallService({
    getClient: () => muteRollbackClient,
    rtcMediaService: muteRollbackMediaService,
    readSession: () => ({
      authToken: 'auth-token-1',
      sessionId: 'session-1',
      context: {
        appId: 'app-1',
        tenantId: '100001',
        userId: 'u_alice',
        sessionId: 'session-1',
        environment: 'dev',
        deploymentMode: 'saas',
        authLevel: 'password',
        dataScope: [],
        permissionScope: [],
      },
      user: {
        userId: 'u_alice',
        displayName: 'Alice',
      },
    }),
  });
  await muteRollbackService.watchIncomingCalls(['conversation-rtc-media-1']);
  muteRollbackListeners[0]?.(createSession('rtc-mute-rollback-1', 'started'));
  muteRollbackListeners[0]?.(createSession('rtc-mute-rollback-1', 'accepted'));
  await waitForCondition(
    () => muteRollbackCalls.includes('publish'),
    'CallService must publish media before exercising mute rollback.',
  );
  await assert.rejects(
    () => muteRollbackService.setAudioMuted(true),
    /volcengine mute audio failed/u,
  );
  assert.equal(
    muteRollbackService.getSnapshot().isAudioMuted,
    false,
    'CallService must roll back audio mute state when the RTC provider rejects the mute operation.',
  );
  await assert.rejects(
    () => muteRollbackService.setVideoMuted(true),
    /volcengine mute video failed/u,
  );
  assert.equal(
    muteRollbackService.getSnapshot().isVideoMuted,
    false,
    'CallService must roll back video mute state when the RTC provider rejects the mute operation.',
  );
  assert.deepEqual(muteRollbackCalls, ['join', 'publish', 'muteAudio', 'muteVideo']);

  const hangupSignalFailureCalls: string[] = [];
  const hangupSignalFailureListeners: CallSessionListener[] = [];
  const hangupSignalFailureClient = {
    calls: {
      subscribe(handler: CallSessionListener) {
        hangupSignalFailureListeners.push(handler);
        return () => undefined;
      },
      async watchIncoming() {
        return null;
      },
      async issueParticipantCredential(rtcSessionId: string, body: { participantId: string }) {
        return {
          tenantId: '100001',
          rtcSessionId,
          participantId: body.participantId,
          credential: 'provider-token-hangup-signal-fail',
          expiresAt: '2026-06-09T10:13:00.000Z',
        };
      },
      async end() {
        hangupSignalFailureCalls.push('calls.end');
        throw new Error('IM end signaling failed');
      },
    },
  } as unknown as ImSdkClient;
  const hangupSignalFailureMediaService: SdkworkRtcMediaService = {
    async join() {
      hangupSignalFailureCalls.push('join');
    },
    async publish() {
      hangupSignalFailureCalls.push('publish');
    },
    async muteAudio() {},
    async muteVideo() {},
    async bindLocalVideoElement() {},
    async leave() {
      hangupSignalFailureCalls.push('leave');
    },
  };
  const hangupSignalFailureService = createSdkworkCallService({
    getClient: () => hangupSignalFailureClient,
    rtcMediaService: hangupSignalFailureMediaService,
    readSession: () => ({
      authToken: 'auth-token-1',
      sessionId: 'session-1',
      context: {
        appId: 'app-1',
        tenantId: '100001',
        userId: 'u_alice',
        sessionId: 'session-1',
        environment: 'dev',
        deploymentMode: 'saas',
        authLevel: 'password',
        dataScope: [],
        permissionScope: [],
      },
      user: {
        userId: 'u_alice',
        displayName: 'Alice',
      },
    }),
  });
  await hangupSignalFailureService.watchIncomingCalls(['conversation-rtc-media-1']);
  hangupSignalFailureListeners[0]?.(createSession('rtc-hangup-signal-fail-1', 'started'));
  hangupSignalFailureListeners[0]?.(createSession('rtc-hangup-signal-fail-1', 'accepted'));
  await waitForCondition(
    () => hangupSignalFailureCalls.includes('publish'),
    'CallService must publish media before exercising hangup signaling failure cleanup.',
  );
  await hangupSignalFailureService.endCall({ reason: 'test_hangup_signal_failure' });
  assert.equal(
    hangupSignalFailureService.getSnapshot().state,
    'errored',
    'CallService must still surface IM hangup signaling failures.',
  );
  assert.deepEqual(
    hangupSignalFailureCalls,
    ['join', 'publish', 'calls.end', 'leave'],
    'CallService must leave RTC media when hangup signaling fails after local media is active.',
  );
  assert.equal(
    hangupSignalFailureService.getSnapshot().isParticipantCredentialReady,
    false,
    'CallService must clear participant credential readiness after hangup signaling failure cleanup.',
  );
  assert.equal(
    hangupSignalFailureService.getSnapshot().participantCredentialExpiresAt,
    undefined,
    'CallService must clear participant credential expiry after hangup signaling failure cleanup.',
  );

  const hangupLeaveFailureCalls: string[] = [];
  const hangupLeaveFailureListeners: CallSessionListener[] = [];
  const hangupLeaveFailureClient = {
    calls: {
      subscribe(handler: CallSessionListener) {
        hangupLeaveFailureListeners.push(handler);
        return () => undefined;
      },
      async watchIncoming() {
        return null;
      },
      async issueParticipantCredential(rtcSessionId: string, body: { participantId: string }) {
        return {
          tenantId: '100001',
          rtcSessionId,
          participantId: body.participantId,
          credential: 'provider-token-hangup-leave-fail',
          expiresAt: '2026-06-09T10:15:00.000Z',
        };
      },
      async end(rtcSessionId: string) {
        hangupLeaveFailureCalls.push('calls.end');
        return createSession(rtcSessionId, 'ended');
      },
    },
  } as unknown as ImSdkClient;
  const hangupLeaveFailureMediaService: SdkworkRtcMediaService = {
    async join() {
      hangupLeaveFailureCalls.push('join');
    },
    async publish() {
      hangupLeaveFailureCalls.push('publish');
    },
    async muteAudio() {},
    async muteVideo() {},
    async bindLocalVideoElement() {},
    async leave() {
      hangupLeaveFailureCalls.push('leave');
      throw new Error('volcengine leave failed after hangup');
    },
  };
  const hangupLeaveFailureService = createSdkworkCallService({
    getClient: () => hangupLeaveFailureClient,
    rtcMediaService: hangupLeaveFailureMediaService,
    readSession: () => ({
      authToken: 'auth-token-1',
      sessionId: 'session-1',
      context: {
        appId: 'app-1',
        tenantId: '100001',
        userId: 'u_alice',
        sessionId: 'session-1',
        environment: 'dev',
        deploymentMode: 'saas',
        authLevel: 'password',
        dataScope: [],
        permissionScope: [],
      },
      user: {
        userId: 'u_alice',
        displayName: 'Alice',
      },
    }),
  });
  await hangupLeaveFailureService.watchIncomingCalls(['conversation-rtc-media-1']);
  hangupLeaveFailureListeners[0]?.(createSession('rtc-hangup-leave-fail-1', 'started'));
  hangupLeaveFailureListeners[0]?.(createSession('rtc-hangup-leave-fail-1', 'accepted'));
  await waitForCondition(
    () => hangupLeaveFailureCalls.includes('publish'),
    'CallService must publish media before exercising hangup cleanup failure.',
  );
  await hangupLeaveFailureService.endCall({ reason: 'test_hangup_leave_failure' });
  assert.equal(
    hangupLeaveFailureService.getSnapshot().state,
    'ended',
    'RTC leave cleanup failures after successful hangup signaling must not overwrite the terminal ended state.',
  );
  assert.deepEqual(hangupLeaveFailureCalls, ['join', 'publish', 'calls.end', 'leave']);

  const failingListeners: CallSessionListener[] = [];
  const failingImClient = {
    calls: {
      subscribe(handler: CallSessionListener) {
        failingListeners.push(handler);
        return () => undefined;
      },
      async watchIncoming() {
        return null;
      },
      async issueParticipantCredential(rtcSessionId: string, body: { participantId: string }) {
        return {
          tenantId: '100001',
          rtcSessionId,
          participantId: body.participantId,
          credential: 'provider-token-fail',
          expiresAt: '2026-06-09T10:20:00.000Z',
        };
      },
    },
  } as unknown as ImSdkClient;
  const failingMediaService: SdkworkRtcMediaService = {
    async join() {
      throw new Error('volcengine app id is not configured');
    },
    async publish() {},
    async muteAudio() {},
    async muteVideo() {},
    async bindLocalVideoElement() {},
    async leave() {
      mediaCalls.push({
        method: 'rtc.leave',
        rtcSessionId: 'failing-media',
      });
    },
  };
  const failingService = createSdkworkCallService({
    getClient: () => failingImClient,
    rtcMediaService: failingMediaService,
    readSession: () => ({
      authToken: 'auth-token-1',
      sessionId: 'session-1',
      context: {
        appId: 'app-1',
        tenantId: '100001',
        userId: 'u_alice',
        sessionId: 'session-1',
        environment: 'dev',
        deploymentMode: 'saas',
        authLevel: 'password',
        dataScope: [],
        permissionScope: [],
      },
      user: {
        userId: 'u_alice',
        displayName: 'Alice',
      },
    }),
  });
  await failingService.watchIncomingCalls(['conversation-rtc-media-1']);
  failingListeners[0]?.(createSession('rtc-media-fail-1', 'started'));
  failingListeners[0]?.(createSession('rtc-media-fail-1', 'accepted'));
  await waitForCondition(
    () => failingService.getSnapshot().errorMessage === 'volcengine app id is not configured',
    'CallService must surface RTC media runtime errors.',
  );
  assert.equal(
    failingService.getSnapshot().state,
    'errored',
    'CallService must not report a connected video call when the RTC media runtime failed to join.',
  );
  assert.equal(
    failingService.getSnapshot().isParticipantCredentialReady,
    false,
    'CallService must clear participant credential readiness when RTC media join fails.',
  );
  assert.equal(
    failingService.getSnapshot().participantCredentialExpiresAt,
    undefined,
    'CallService must not keep participant credential expiry metadata after RTC media join fails.',
  );
  assert.deepEqual(
    mediaCalls.slice(-1),
    [{
      method: 'rtc.leave',
      rtcSessionId: 'failing-media',
    }],
    'CallService must ask the RTC media runtime to leave after a failed join attempt.',
  );

  const joinFailureLeaveFailureCalls: string[] = [];
  const joinFailureLeaveFailureListeners: CallSessionListener[] = [];
  const joinFailureLeaveFailureClient = {
    calls: {
      subscribe(handler: CallSessionListener) {
        joinFailureLeaveFailureListeners.push(handler);
        return () => undefined;
      },
      async watchIncoming() {
        return null;
      },
      async issueParticipantCredential(rtcSessionId: string, body: { participantId: string }) {
        return {
          tenantId: '100001',
          rtcSessionId,
          participantId: body.participantId,
          credential: 'provider-token-join-fail-leave-fail',
          expiresAt: '2026-06-09T10:25:00.000Z',
        };
      },
    },
  } as unknown as ImSdkClient;
  const joinFailureLeaveFailureMediaService: SdkworkRtcMediaService = {
    async join() {
      joinFailureLeaveFailureCalls.push('join');
      throw new Error('volcengine join failed before publish');
    },
    async publish() {},
    async muteAudio() {},
    async muteVideo() {},
    async bindLocalVideoElement() {},
    async leave() {
      joinFailureLeaveFailureCalls.push('leave');
      throw new Error('volcengine leave failed during join cleanup');
    },
  };
  const joinFailureLeaveFailureService = createSdkworkCallService({
    getClient: () => joinFailureLeaveFailureClient,
    rtcMediaService: joinFailureLeaveFailureMediaService,
    readSession: () => ({
      authToken: 'auth-token-1',
      sessionId: 'session-1',
      context: {
        appId: 'app-1',
        tenantId: '100001',
        userId: 'u_alice',
        sessionId: 'session-1',
        environment: 'dev',
        deploymentMode: 'saas',
        authLevel: 'password',
        dataScope: [],
        permissionScope: [],
      },
      user: {
        userId: 'u_alice',
        displayName: 'Alice',
      },
    }),
  });
  await joinFailureLeaveFailureService.watchIncomingCalls(['conversation-rtc-media-1']);
  joinFailureLeaveFailureListeners[0]?.(createSession('rtc-join-fail-leave-fail-1', 'started'));
  joinFailureLeaveFailureListeners[0]?.(createSession('rtc-join-fail-leave-fail-1', 'accepted'));
  await waitForCondition(
    () => joinFailureLeaveFailureService.getSnapshot().state === 'errored',
    'CallService must still enter errored state when RTC join fails and cleanup also fails.',
  );
  assert.equal(
    joinFailureLeaveFailureService.getSnapshot().errorMessage,
    'volcengine join failed before publish',
    'RTC cleanup failures must not mask the original join failure.',
  );
  assert.deepEqual(joinFailureLeaveFailureCalls, ['join', 'leave']);

  const publishFailureCalls: string[] = [];
  const publishFailingListeners: CallSessionListener[] = [];
  const publishFailingClient = {
    calls: {
      subscribe(handler: CallSessionListener) {
        publishFailingListeners.push(handler);
        return () => undefined;
      },
      async watchIncoming() {
        return null;
      },
      async issueParticipantCredential(rtcSessionId: string, body: { participantId: string }) {
        return {
          tenantId: '100001',
          rtcSessionId,
          participantId: body.participantId,
          credential: 'provider-token-publish-fail',
          expiresAt: '2026-06-09T10:30:00.000Z',
        };
      },
    },
  } as unknown as ImSdkClient;
  const publishFailingMediaService: SdkworkRtcMediaService = {
    async join() {
      publishFailureCalls.push('join');
    },
    async publish() {
      publishFailureCalls.push('publish');
      throw new Error('volcengine publish failed');
    },
    async muteAudio() {},
    async muteVideo() {},
    async bindLocalVideoElement() {},
    async leave() {
      publishFailureCalls.push('leave');
    },
  };
  const publishFailingService = createSdkworkCallService({
    getClient: () => publishFailingClient,
    rtcMediaService: publishFailingMediaService,
    readSession: () => ({
      authToken: 'auth-token-1',
      sessionId: 'session-1',
      context: {
        appId: 'app-1',
        tenantId: '100001',
        userId: 'u_alice',
        sessionId: 'session-1',
        environment: 'dev',
        deploymentMode: 'saas',
        authLevel: 'password',
        dataScope: [],
        permissionScope: [],
      },
      user: {
        userId: 'u_alice',
        displayName: 'Alice',
      },
    }),
  });
  await publishFailingService.watchIncomingCalls(['conversation-rtc-media-1']);
  publishFailingListeners[0]?.(createSession('rtc-media-publish-fail-1', 'started'));
  publishFailingListeners[0]?.(createSession('rtc-media-publish-fail-1', 'accepted'));
  await waitForCondition(
    () => publishFailingService.getSnapshot().errorMessage === 'volcengine publish failed',
    'CallService must surface RTC media publish errors.',
  );
  assert.deepEqual(
    publishFailureCalls,
    ['join', 'publish', 'leave'],
    'CallService must leave the RTC media runtime after publish fails so camera and room resources are released.',
  );

  const latePublish = createDeferred<void>();
  const lateMediaCalls: string[] = [];
  const lateMediaListeners: CallSessionListener[] = [];
  const lateMediaClient = {
    calls: {
      subscribe(handler: CallSessionListener) {
        lateMediaListeners.push(handler);
        return () => undefined;
      },
      async watchIncoming() {
        return null;
      },
      async issueParticipantCredential(rtcSessionId: string, body: { participantId: string }) {
        return {
          tenantId: '100001',
          rtcSessionId,
          participantId: body.participantId,
          credential: 'provider-token-late-media',
          expiresAt: '2026-06-09T10:40:00.000Z',
        };
      },
    },
  } as unknown as ImSdkClient;
  const lateMediaService: SdkworkRtcMediaService = {
    async join() {
      lateMediaCalls.push('join');
    },
    publish() {
      lateMediaCalls.push('publish');
      return latePublish.promise;
    },
    async muteAudio() {},
    async muteVideo() {},
    async bindLocalVideoElement() {},
    async leave() {
      lateMediaCalls.push('leave');
    },
  };
  const lateMediaServiceInstance = createSdkworkCallService({
    getClient: () => lateMediaClient,
    rtcMediaService: lateMediaService,
    readSession: () => ({
      authToken: 'auth-token-1',
      sessionId: 'session-1',
      context: {
        appId: 'app-1',
        tenantId: '100001',
        userId: 'u_alice',
        sessionId: 'session-1',
        environment: 'dev',
        deploymentMode: 'saas',
        authLevel: 'password',
        dataScope: [],
        permissionScope: [],
      },
      user: {
        userId: 'u_alice',
        displayName: 'Alice',
      },
    }),
  });
  await lateMediaServiceInstance.watchIncomingCalls(['conversation-rtc-media-1']);
  lateMediaListeners[0]?.(createSession('rtc-media-late-fail-1', 'started'));
  lateMediaListeners[0]?.(createSession('rtc-media-late-fail-1', 'accepted'));
  await waitForCondition(
    () => lateMediaCalls.includes('publish'),
    'CallService must start publishing media before this late-failure race can be exercised.',
  );
  lateMediaListeners[0]?.(createSession('rtc-media-late-fail-1', 'ended'));
  assert.equal(lateMediaServiceInstance.getSnapshot().state, 'ended');
  latePublish.reject(new Error('late media publish failed'));
  await new Promise((resolve) => setTimeout(resolve, 0));
  assert.equal(
    lateMediaServiceInstance.getSnapshot().state,
    'ended',
    'A late RTC media failure must not overwrite a remotely ended call with errored state.',
  );
  assert.ok(
    lateMediaCalls.includes('leave'),
    'CallService must leave the RTC media runtime when the remote side ends during local media startup.',
  );

  const localVideoCalls: Array<{
    method: string;
    playerId?: string;
    renderDom?: HTMLElement;
    renderMode?: number;
    streamIndex?: number;
  }> = [];
  const localPreviewElement = { nodeType: 1 } as HTMLElement;
  const fakeEngine = {
    setLocalVideoPlayer(streamIndex: number, options?: { playerId?: string; renderDom?: HTMLElement; renderMode?: number }) {
      localVideoCalls.push({
        method: 'setLocalVideoPlayer',
        playerId: options?.playerId,
        renderDom: options?.renderDom,
        renderMode: options?.renderMode,
        streamIndex,
      });
      return undefined;
    },
    async play(_userId?: string, _mediaType?: unknown, streamIndex?: number, playerId?: string) {
      localVideoCalls.push({
        method: 'play',
        playerId,
        streamIndex,
      });
    },
    stop(_userId?: string, _mediaType?: unknown, streamIndex?: number, playerId?: string) {
      localVideoCalls.push({
        method: 'stop',
        playerId,
        streamIndex,
      });
    },
  };
  const serviceBackedRtcMedia = createSdkworkRtcMediaService({
    createDataSource: () => ({
      async createClient() {
        return {
          async join() {},
          async publish() {
            return {
              trackId: 'video-track-1',
              kind: 'video',
              muted: false,
            };
          },
          async muteAudio() {
            return {
              kind: 'audio',
              muted: false,
            };
          },
          async muteVideo() {
            return {
              kind: 'video',
              muted: false,
            };
          },
          async leave() {
            return {
              sessionId: 'rtc-local-preview-1',
              roomId: 'room-local-preview-1',
              participantId: 'u_alice',
              providerKey: 'volcengine',
              connectionState: 'left',
            };
          },
          supportsProviderExtension(extensionKey: string) {
            return extensionKey === 'volcengine.native-client';
          },
          unwrap() {
            return {
              engine: fakeEngine,
            };
          },
        };
      },
    } as never),
  });
  await serviceBackedRtcMedia.bindLocalVideoElement(localPreviewElement);
  assert.deepEqual(
    localVideoCalls,
    [],
    'RtcMediaService must defer local video binding until the provider client has joined.',
  );
  await serviceBackedRtcMedia.join({
    credential: {
      tenantId: '100001',
      rtcSessionId: 'rtc-local-preview-1',
      participantId: 'u_alice',
      credential: 'provider-token-local-preview',
      expiresAt: '2026-06-09T10:50:00.000Z',
    },
    participantId: 'u_alice',
    providerKey: 'volcengine',
    roomId: 'room-local-preview-1',
    rtcMode: 'video',
    rtcSessionId: 'rtc-local-preview-1',
  });
  assert.deepEqual(localVideoCalls, [
    {
      method: 'setLocalVideoPlayer',
      playerId: 'sdkwork-im-pc-local-video-preview',
      renderDom: localPreviewElement,
      renderMode: 0,
      streamIndex: 0,
    },
    {
      method: 'play',
      playerId: 'sdkwork-im-pc-local-video-preview',
      streamIndex: 0,
    },
  ]);
  await serviceBackedRtcMedia.publish({
    kinds: ['video'],
    rtcSessionId: 'rtc-local-preview-1',
  });
  assert.deepEqual(
    localVideoCalls,
    [
      {
        method: 'setLocalVideoPlayer',
        playerId: 'sdkwork-im-pc-local-video-preview',
        renderDom: localPreviewElement,
        renderMode: 0,
        streamIndex: 0,
      },
      {
        method: 'play',
        playerId: 'sdkwork-im-pc-local-video-preview',
        streamIndex: 0,
      },
    ],
    'RtcMediaService must not duplicate local preview binding during video publish.',
  );
  await serviceBackedRtcMedia.bindLocalVideoElement(null);
  assert.deepEqual(localVideoCalls.slice(-2), [
    {
      method: 'stop',
      playerId: 'sdkwork-im-pc-local-video-preview',
      streamIndex: 0,
    },
    {
      method: 'setLocalVideoPlayer',
      playerId: 'sdkwork-im-pc-local-video-preview',
      renderDom: undefined,
      renderMode: undefined,
      streamIndex: 0,
    },
  ]);

  const previewFailureCalls: string[] = [];
  const previewFailureRtcMedia = createSdkworkRtcMediaService({
    createDataSource: () => ({
      async createClient() {
        return {
          async join() {
            previewFailureCalls.push('join');
          },
          async publish() {
            previewFailureCalls.push('publish');
            return {
              trackId: 'video-track-preview-failure',
              kind: 'video',
              muted: false,
            };
          },
          async muteAudio() {
            return {
              kind: 'audio',
              muted: false,
            };
          },
          async muteVideo() {
            return {
              kind: 'video',
              muted: false,
            };
          },
          async leave() {
            previewFailureCalls.push('leave');
            return {
              sessionId: 'rtc-preview-failure-1',
              roomId: 'room-preview-failure-1',
              participantId: 'u_alice',
              providerKey: 'volcengine',
              connectionState: 'left',
            };
          },
          supportsProviderExtension(extensionKey: string) {
            return extensionKey === 'volcengine.native-client';
          },
          unwrap() {
            return {
              engine: {
                setLocalVideoPlayer() {
                  previewFailureCalls.push('setLocalVideoPlayer');
                  return undefined;
                },
                async play() {
                  previewFailureCalls.push('play');
                  throw new Error('browser autoplay blocked local preview');
                },
                stop() {
                  previewFailureCalls.push('stop');
                },
              },
            };
          },
        };
      },
    } as never),
  });
  await previewFailureRtcMedia.bindLocalVideoElement({ nodeType: 1 } as HTMLElement);
  await previewFailureRtcMedia.join({
    credential: {
      tenantId: '100001',
      rtcSessionId: 'rtc-preview-failure-1',
      participantId: 'u_alice',
      credential: 'provider-token-preview-failure',
      expiresAt: '2026-06-09T11:00:00.000Z',
    },
    participantId: 'u_alice',
    providerKey: 'volcengine',
    roomId: 'room-preview-failure-1',
    rtcMode: 'video',
    rtcSessionId: 'rtc-preview-failure-1',
  });
  await previewFailureRtcMedia.publish({
    kinds: ['video'],
    rtcSessionId: 'rtc-preview-failure-1',
  });
  assert.deepEqual(
    previewFailureCalls,
    ['join', 'setLocalVideoPlayer', 'play', 'publish'],
    'Local preview playback failures must not abort RTC room join or local media publish.',
  );

  const previewExtensionFailureCalls: string[] = [];
  const previewExtensionFailureRtcMedia = createSdkworkRtcMediaService({
    createDataSource: () => ({
      async createClient() {
        return {
          async join() {
            previewExtensionFailureCalls.push('join');
          },
          async publish() {
            previewExtensionFailureCalls.push('publish');
            return {
              trackId: 'video-track-preview-extension-failure',
              kind: 'video',
              muted: false,
            };
          },
          async muteAudio() {
            return {
              kind: 'audio',
              muted: false,
            };
          },
          async muteVideo() {
            return {
              kind: 'video',
              muted: false,
            };
          },
          async leave() {
            previewExtensionFailureCalls.push('leave');
            return {
              sessionId: 'rtc-preview-extension-failure-1',
              roomId: 'room-preview-extension-failure-1',
              participantId: 'u_alice',
              providerKey: 'volcengine',
              connectionState: 'left',
            };
          },
          supportsProviderExtension(extensionKey: string) {
            previewExtensionFailureCalls.push(`supports:${extensionKey}`);
            return true;
          },
          unwrap() {
            previewExtensionFailureCalls.push('unwrap');
            throw new Error('volcengine native extension unavailable');
          },
        };
      },
    } as never),
  });
  await previewExtensionFailureRtcMedia.bindLocalVideoElement({ nodeType: 1 } as HTMLElement);
  await previewExtensionFailureRtcMedia.join({
    credential: {
      tenantId: '100001',
      rtcSessionId: 'rtc-preview-extension-failure-1',
      participantId: 'u_alice',
      credential: 'provider-token-preview-extension-failure',
      expiresAt: '2026-06-09T11:05:00.000Z',
    },
    participantId: 'u_alice',
    providerKey: 'volcengine',
    roomId: 'room-preview-extension-failure-1',
    rtcMode: 'video',
    rtcSessionId: 'rtc-preview-extension-failure-1',
  });
  await previewExtensionFailureRtcMedia.publish({
    kinds: ['video'],
    rtcSessionId: 'rtc-preview-extension-failure-1',
  });
  assert.deepEqual(
    previewExtensionFailureCalls,
    ['join', 'supports:volcengine.native-client', 'unwrap', 'publish'],
    'Local preview provider extension failures must not abort RTC room join or local media publish.',
  );

  const joinFailureCleanupCalls: string[] = [];
  const joinFailureCleanupRtcMedia = createSdkworkRtcMediaService({
    createDataSource: () => ({
      async createClient() {
        return {
          async join() {
            joinFailureCleanupCalls.push('join');
            throw new Error('volcengine join failed after client allocation');
          },
          async publish() {
            throw new Error('publish should not be called after join failure');
          },
          async muteAudio() {
            return {
              kind: 'audio',
              muted: false,
            };
          },
          async muteVideo() {
            return {
              kind: 'video',
              muted: false,
            };
          },
          async leave() {
            joinFailureCleanupCalls.push('leave');
            return {
              sessionId: 'rtc-join-failure-cleanup-1',
              roomId: 'room-join-failure-cleanup-1',
              participantId: 'u_alice',
              providerKey: 'volcengine',
              connectionState: 'left',
            };
          },
          supportsProviderExtension() {
            return false;
          },
          unwrap() {
            return {};
          },
        };
      },
    } as never),
  });
  await assert.rejects(
    () => joinFailureCleanupRtcMedia.join({
      credential: {
        tenantId: '100001',
        rtcSessionId: 'rtc-join-failure-cleanup-1',
        participantId: 'u_alice',
        credential: 'provider-token-join-failure-cleanup',
        expiresAt: '2026-06-09T11:08:00.000Z',
      },
      participantId: 'u_alice',
      providerKey: 'volcengine',
      roomId: 'room-join-failure-cleanup-1',
      rtcMode: 'video',
      rtcSessionId: 'rtc-join-failure-cleanup-1',
    }),
    /volcengine join failed after client allocation/u,
  );
  assert.deepEqual(
    joinFailureCleanupCalls,
    ['join', 'leave'],
    'RtcMediaService must leave a provider client when join fails after client allocation.',
  );

  const leaveAfterPreviewUnbindFailureCalls: string[] = [];
  const leaveAfterPreviewUnbindFailureRtcMedia = createSdkworkRtcMediaService({
    createDataSource: () => ({
      async createClient() {
        return {
          async join() {
            leaveAfterPreviewUnbindFailureCalls.push('join');
          },
          async publish() {
            return {
              trackId: 'video-track-preview-unbind-failure',
              kind: 'video',
              muted: false,
            };
          },
          async muteAudio() {
            return {
              kind: 'audio',
              muted: false,
            };
          },
          async muteVideo() {
            return {
              kind: 'video',
              muted: false,
            };
          },
          async leave() {
            leaveAfterPreviewUnbindFailureCalls.push('leave');
            return {
              sessionId: 'rtc-preview-unbind-failure-1',
              roomId: 'room-preview-unbind-failure-1',
              participantId: 'u_alice',
              providerKey: 'volcengine',
              connectionState: 'left',
            };
          },
          supportsProviderExtension(extensionKey: string) {
            return extensionKey === 'volcengine.native-client';
          },
          unwrap() {
            return {
              engine: {
                setLocalVideoPlayer() {
                  leaveAfterPreviewUnbindFailureCalls.push('setLocalVideoPlayer');
                  return undefined;
                },
                async play() {
                  leaveAfterPreviewUnbindFailureCalls.push('play');
                },
                stop() {
                  leaveAfterPreviewUnbindFailureCalls.push('stop');
                  throw new Error('volcengine local preview unbind failed');
                },
              },
            };
          },
        };
      },
    } as never),
  });
  await leaveAfterPreviewUnbindFailureRtcMedia.bindLocalVideoElement({ nodeType: 1 } as HTMLElement);
  await leaveAfterPreviewUnbindFailureRtcMedia.join({
    credential: {
      tenantId: '100001',
      rtcSessionId: 'rtc-preview-unbind-failure-1',
      participantId: 'u_alice',
      credential: 'provider-token-preview-unbind-failure',
      expiresAt: '2026-06-09T11:10:00.000Z',
    },
    participantId: 'u_alice',
    providerKey: 'volcengine',
    roomId: 'room-preview-unbind-failure-1',
    rtcMode: 'video',
    rtcSessionId: 'rtc-preview-unbind-failure-1',
  });
  await leaveAfterPreviewUnbindFailureRtcMedia.leave();
  assert.deepEqual(
    leaveAfterPreviewUnbindFailureCalls,
    ['join', 'setLocalVideoPlayer', 'play', 'stop', 'setLocalVideoPlayer', 'leave'],
    'Local preview unbind failures must not prevent the RTC provider client from leaving the room.',
  );

  const leaveAfterPreviewPlayerUnbindFailureCalls: string[] = [];
  const leaveAfterPreviewPlayerUnbindFailureRtcMedia = createSdkworkRtcMediaService({
    createDataSource: () => ({
      async createClient() {
        return {
          async join() {
            leaveAfterPreviewPlayerUnbindFailureCalls.push('join');
          },
          async publish() {
            return {
              trackId: 'video-track-preview-player-unbind-failure',
              kind: 'video',
              muted: false,
            };
          },
          async muteAudio() {
            return {
              kind: 'audio',
              muted: false,
            };
          },
          async muteVideo() {
            return {
              kind: 'video',
              muted: false,
            };
          },
          async leave() {
            leaveAfterPreviewPlayerUnbindFailureCalls.push('leave');
            return {
              sessionId: 'rtc-preview-player-unbind-failure-1',
              roomId: 'room-preview-player-unbind-failure-1',
              participantId: 'u_alice',
              providerKey: 'volcengine',
              connectionState: 'left',
            };
          },
          supportsProviderExtension(extensionKey: string) {
            return extensionKey === 'volcengine.native-client';
          },
          unwrap() {
            return {
              engine: {
                setLocalVideoPlayer(_streamIndex: number, options?: { renderDom?: HTMLElement }) {
                  leaveAfterPreviewPlayerUnbindFailureCalls.push('setLocalVideoPlayer');
                  if (!options?.renderDom) {
                    throw new Error('volcengine local preview player reset failed');
                  }
                  return undefined;
                },
                async play() {
                  leaveAfterPreviewPlayerUnbindFailureCalls.push('play');
                },
                stop() {
                  leaveAfterPreviewPlayerUnbindFailureCalls.push('stop');
                },
              },
            };
          },
        };
      },
    } as never),
  });
  await leaveAfterPreviewPlayerUnbindFailureRtcMedia.bindLocalVideoElement({ nodeType: 1 } as HTMLElement);
  await leaveAfterPreviewPlayerUnbindFailureRtcMedia.join({
    credential: {
      tenantId: '100001',
      rtcSessionId: 'rtc-preview-player-unbind-failure-1',
      participantId: 'u_alice',
      credential: 'provider-token-preview-player-unbind-failure',
      expiresAt: '2026-06-09T11:20:00.000Z',
    },
    participantId: 'u_alice',
    providerKey: 'volcengine',
    roomId: 'room-preview-player-unbind-failure-1',
    rtcMode: 'video',
    rtcSessionId: 'rtc-preview-player-unbind-failure-1',
  });
  await leaveAfterPreviewPlayerUnbindFailureRtcMedia.leave();
  assert.deepEqual(
    leaveAfterPreviewPlayerUnbindFailureCalls,
    ['join', 'setLocalVideoPlayer', 'play', 'stop', 'setLocalVideoPlayer', 'leave'],
    'Local preview player reset failures must not prevent the RTC provider client from leaving the room.',
  );

  console.log('sdkwork-im-pc RTC media runtime contract passed');
}

void main();
