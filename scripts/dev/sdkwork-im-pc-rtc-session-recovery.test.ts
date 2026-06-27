import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkCallService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/CallService';
import type { SdkworkRtcMediaService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/RtcMediaService';

const calls: Array<{
  method: string;
  participantId?: string;
  rtcSessionId?: string;
}> = [];

const noopRtcMediaService: SdkworkRtcMediaService = {
  async bindLocalVideoElement() {},
  async join() {},
  async publish() {},
  async muteAudio() {},
  async muteVideo() {},
  async leave() {},
};

const fakeClient = {
  calls: {
    async retrieve(rtcSessionId: string) {
      calls.push({ method: 'calls.retrieve', rtcSessionId });
      return {
        tenantId: '100001',
        rtcSessionId,
        conversationId: 'conversation-rtc-1',
        rtcMode: 'voice',
        initiatorId: 'u_bob',
        providerPluginId: 'volcengine',
        providerSessionId: 'room-provider-1',
        startedAt: '2026-06-04T10:00:00.000Z',
        state: 'accepted',
      };
    },
    async start() {
      calls.push({ method: 'calls.start' });
      throw new Error('recoverRtcSession must not create a duplicate RTC session');
    },
    async invite() {
      calls.push({ method: 'calls.invite' });
      throw new Error('recoverRtcSession must not invite a duplicate RTC session');
    },
    async issueParticipantCredential(rtcSessionId: string, body: { participantId: string }) {
      calls.push({
        method: 'calls.issueParticipantCredential',
        participantId: body.participantId,
        rtcSessionId,
      });
      return {
        tenantId: '100001',
        rtcSessionId,
        participantId: body.participantId,
        credential: 'secret-recovered-rtc-credential',
        expiresAt: '2026-06-04T10:10:00.000Z',
      };
    },
  },
} as unknown as ImSdkClient;

type CallSessionListener = Parameters<NonNullable<ImSdkClient['calls']['subscribe']>>[0];

function createSession(rtcSessionId: string, state: 'accepted' | 'ended' | 'rejected' | 'started') {
  return {
    tenantId: '100001',
    rtcSessionId,
    conversationId: 'conversation-rtc-1',
    rtcMode: 'video',
    initiatorId: 'u_bob',
    providerPluginId: null,
    providerSessionId: `room-${rtcSessionId}`,
    startedAt: '2026-06-04T10:00:00.000Z',
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

async function main(): Promise<void> {
  const service = createSdkworkCallService({
    getClient: () => fakeClient,
    rtcMediaService: noopRtcMediaService,
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

  assert.equal(typeof service.recoverRtcSession, 'function');

  const snapshot = await service.recoverRtcSession('rtc-recover-1', {
    targetName: 'Bob',
  });

  assert.deepEqual(calls, [
    { method: 'calls.retrieve', rtcSessionId: 'rtc-recover-1' },
    {
      method: 'calls.issueParticipantCredential',
      participantId: 'u_alice',
      rtcSessionId: 'rtc-recover-1',
    },
  ]);
  assert.deepEqual(
    {
      accessEndpoint: snapshot.accessEndpoint,
      state: snapshot.state,
      controllerState: snapshot.controllerState,
      conversationId: snapshot.conversationId,
      isParticipantCredentialReady: snapshot.isParticipantCredentialReady,
      isAudioMuted: snapshot.isAudioMuted,
      isVideoMuted: snapshot.isVideoMuted,
      participantCredentialExpiresAt: snapshot.participantCredentialExpiresAt,
      participantId: snapshot.participantId,
      providerKey: snapshot.providerKey,
      roomId: snapshot.roomId,
      rtcMode: snapshot.rtcMode,
      rtcSessionId: snapshot.rtcSessionId,
      initiatorId: snapshot.initiatorId,
      peerUserId: snapshot.peerUserId,
      targetName: snapshot.targetName,
      type: snapshot.type,
    },
    {
      accessEndpoint: undefined,
      state: 'connected',
      controllerState: 'connected',
      conversationId: 'conversation-rtc-1',
      isParticipantCredentialReady: true,
      isAudioMuted: false,
      isVideoMuted: true,
      participantCredentialExpiresAt: '2026-06-04T10:10:00.000Z',
      participantId: 'u_alice',
      providerKey: 'volcengine',
      roomId: 'room-provider-1',
      rtcMode: 'voice',
      rtcSessionId: 'rtc-recover-1',
      initiatorId: 'u_bob',
      peerUserId: 'u_bob',
      targetName: 'Bob',
      type: 'voice',
    },
  );
  assert.equal(
    JSON.stringify(snapshot).includes('secret-recovered-rtc-credential'),
    false,
    'CallService snapshots must expose credential readiness metadata without leaking raw RTC credentials into UI state',
  );

  const callSessionListeners: CallSessionListener[] = [];
  const watchCalls: Array<{
    method: string;
    participantId?: string;
    rtcSessionId?: string;
  }> = [];
  const watchClient = {
    calls: {
      subscribe(handler: CallSessionListener) {
        callSessionListeners.push(handler);
        return () => undefined;
      },
      async watchIncoming() {
        return createSession('rtc-watch-1', 'started');
      },
      async issueParticipantCredential(rtcSessionId: string, body: { participantId: string }) {
        watchCalls.push({
          method: 'calls.issueParticipantCredential',
          participantId: body.participantId,
          rtcSessionId,
        });
        return {
          tenantId: '100001',
          rtcSessionId,
          participantId: body.participantId,
          credential: 'secret-watch-credential',
          expiresAt: '2026-06-04T10:11:00.000Z',
        };
      },
    },
  } as unknown as ImSdkClient;
  const watchService = createSdkworkCallService({
    getClient: () => watchClient,
    rtcMediaService: noopRtcMediaService,
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
  const snapshots: string[] = [];
  const unsubscribeWatchService = watchService.subscribe((nextSnapshot) => {
    snapshots.push(nextSnapshot.state);
  });
  await watchService.watchIncomingCalls(['conversation-rtc-1']);
  assert.equal(watchService.getSnapshot().state, 'ringing');
  callSessionListeners[0]?.(createSession('rtc-watch-1', 'rejected'));
  assert.equal(
    watchService.getSnapshot().state,
    'rejected',
    'CallService must sync reject signals for the active RTC session through the IM calls subscription',
  );
  assert.ok(
    snapshots.includes('rejected'),
    'CallService subscribers must be notified when the active RTC session is rejected remotely',
  );
  assert.deepEqual(
    watchCalls,
    [],
    'CallService must not issue RTC participant credentials for rejected calls',
  );
  unsubscribeWatchService();

  const liveIncomingCalls: Array<{
    method: string;
    participantId?: string;
    rtcSessionId?: string;
  }> = [];
  const liveIncomingClient = {
    calls: {
      subscribe(handler: CallSessionListener) {
        callSessionListeners.push(handler);
        return () => undefined;
      },
      async watchIncoming() {
        return null;
      },
      async issueParticipantCredential(rtcSessionId: string, body: { participantId: string }) {
        liveIncomingCalls.push({
          method: 'calls.issueParticipantCredential',
          participantId: body.participantId,
          rtcSessionId,
        });
        return {
          tenantId: '100001',
          rtcSessionId,
          participantId: body.participantId,
          credential: 'secret-live-incoming-credential',
          expiresAt: '2026-06-04T10:12:00.000Z',
        };
      },
    },
  } as unknown as ImSdkClient;
  const liveIncomingService = createSdkworkCallService({
    getClient: () => liveIncomingClient,
    rtcMediaService: noopRtcMediaService,
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
  const liveIncomingSnapshots: string[] = [];
  const unsubscribeLiveIncomingService = liveIncomingService.subscribe((nextSnapshot) => {
    liveIncomingSnapshots.push(nextSnapshot.state);
  });
  await liveIncomingService.watchIncomingCalls(['conversation-rtc-1']);
  assert.equal(liveIncomingService.getSnapshot().controllerState, 'watching');

  callSessionListeners.at(-1)?.(createSession('rtc-live-incoming-1', 'started'));
  assert.equal(
    liveIncomingService.getSnapshot().state,
    'ringing',
    'CallService must open the incoming ringing state when the calls subscription receives an invite',
  );
  assert.equal(liveIncomingService.getSnapshot().direction, 'incoming');
  assert.equal(liveIncomingService.getSnapshot().rtcSessionId, 'rtc-live-incoming-1');
  assert.equal(
    liveIncomingService.getSnapshot().targetName,
    undefined,
    'CallService must not expose raw initiator ids as target display names; ChatLayout should resolve the friendly chat/contact name',
  );
  assert.equal(
    liveIncomingService.getSnapshot().initiatorId,
    'u_bob',
    'CallService must preserve the session initiator id so the UI can identify who started the call',
  );
  assert.equal(
    liveIncomingService.getSnapshot().peerUserId,
    'u_bob',
    'CallService must expose the peer user id for incoming calls so ChatLayout can hydrate a friendly display name',
  );
  callSessionListeners.at(-1)?.(createSession('rtc-live-incoming-1', 'accepted'));
  assert.equal(
    liveIncomingService.getSnapshot().state,
    'connected',
    'CallService must sync accept signals for the active incoming RTC session',
  );
  assert.deepEqual(
    liveIncomingCalls,
    [{
      method: 'calls.issueParticipantCredential',
      participantId: 'u_alice',
      rtcSessionId: 'rtc-live-incoming-1',
    }],
    'CallService must issue the RTC participant credential once a call becomes connected',
  );
  await waitForCondition(
    () => liveIncomingService.getSnapshot().isParticipantCredentialReady === true,
    'CallService must mark the RTC participant credential ready after async connected-session credential issuance',
  );
  assert.equal(liveIncomingService.getSnapshot().isParticipantCredentialReady, true);
  assert.equal(liveIncomingService.getSnapshot().participantCredentialExpiresAt, '2026-06-04T10:12:00.000Z');
  assert.equal(
    JSON.stringify(liveIncomingService.getSnapshot()).includes('secret-live-incoming-credential'),
    false,
    'CallService must not place raw RTC credentials in browser-visible call snapshots',
  );
  callSessionListeners.at(-1)?.(createSession('rtc-live-incoming-1', 'ended'));
  assert.equal(
    liveIncomingService.getSnapshot().state,
    'ended',
    'CallService must sync hangup/end signals for the active incoming RTC session',
  );
  assert.deepEqual(
    liveIncomingSnapshots
      .filter((state) => state === 'ringing' || state === 'connected' || state === 'ended')
      .filter((state, index, states) => index === 0 || states[index - 1] !== state),
    ['ringing', 'connected', 'ended'],
    'CallService subscribers must be notified for invite, accept, and hangup state transitions',
  );
  unsubscribeLiveIncomingService();

  const closingOnlyService = createSdkworkCallService({
    getClient: () => liveIncomingClient,
    rtcMediaService: noopRtcMediaService,
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
  await closingOnlyService.watchIncomingCalls(['conversation-rtc-1']);
  callSessionListeners.at(-1)?.(createSession('rtc-closing-only-1', 'ended'));
  assert.equal(
    closingOnlyService.getSnapshot().controllerState,
    'watching',
    'CallService must ignore closing signals for calls that were not active instead of opening a false incoming overlay',
  );
  assert.equal(closingOnlyService.getSnapshot().rtcSessionId, undefined);

  const watchRaceDeferred = createDeferred<null>();
  const watchRaceListeners: CallSessionListener[] = [];
  const watchRaceClient = {
    calls: {
      subscribe(handler: CallSessionListener) {
        watchRaceListeners.push(handler);
        return () => undefined;
      },
      watchIncoming() {
        return watchRaceDeferred.promise;
      },
    },
  } as unknown as ImSdkClient;
  const watchRaceService = createSdkworkCallService({
    getClient: () => watchRaceClient,
    rtcMediaService: noopRtcMediaService,
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
  const watchRaceTask = watchRaceService.watchIncomingCalls(['conversation-rtc-1']);
  watchRaceListeners[0]?.(createSession('rtc-watch-race-1', 'started'));
  assert.equal(
    watchRaceService.getSnapshot().state,
    'ringing',
    'CallService must apply subscription call invites while watchIncoming is still connecting.',
  );
  watchRaceDeferred.resolve(null);
  await watchRaceTask;
  assert.equal(
    watchRaceService.getSnapshot().state,
    'ringing',
    'A late empty watchIncoming result must not overwrite a subscription-delivered incoming call.',
  );
  assert.equal(watchRaceService.getSnapshot().rtcSessionId, 'rtc-watch-race-1');

  const outgoingSessionListeners: CallSessionListener[] = [];
  const outgoingCalls: Array<{
    method: string;
    participantId?: string;
    rtcSessionId?: string;
  }> = [];
  const outgoingClient = {
    calls: {
      subscribe(handler: CallSessionListener) {
        outgoingSessionListeners.push(handler);
        return () => undefined;
      },
      async watchIncoming() {
        return null;
      },
      async start(body: { conversationId?: string; rtcMode: string; rtcSessionId: string }) {
        outgoingCalls.push({ method: 'calls.start', rtcSessionId: body.rtcSessionId });
        return {
          tenantId: '100001',
          rtcSessionId: body.rtcSessionId,
          conversationId: body.conversationId,
          rtcMode: body.rtcMode,
          initiatorId: 'u_alice',
          initiatorKind: 'user',
          providerPluginId: null,
          providerSessionId: `room-${body.rtcSessionId}`,
          startedAt: '2026-06-04T10:20:00.000Z',
          state: 'started',
          requestKey: 'request-start-1',
          deliveryStatus: 'applied',
          proofVersion: 'v1',
        };
      },
      async invite(rtcSessionId: string) {
        outgoingCalls.push({ method: 'calls.invite', rtcSessionId });
        return {
          tenantId: '100001',
          rtcSessionId,
          conversationId: 'conversation-rtc-1',
          rtcMode: 'video',
          initiatorId: 'u_alice',
          initiatorKind: 'user',
          providerPluginId: null,
          providerSessionId: `room-${rtcSessionId}`,
          startedAt: '2026-06-04T10:20:00.000Z',
          state: 'started',
          requestKey: 'request-invite-1',
          deliveryStatus: 'applied',
          proofVersion: 'v1',
        };
      },
      async issueParticipantCredential(rtcSessionId: string, body: { participantId: string }) {
        outgoingCalls.push({
          method: 'calls.issueParticipantCredential',
          participantId: body.participantId,
          rtcSessionId,
        });
        return {
          tenantId: '100001',
          rtcSessionId,
          participantId: body.participantId,
          credential: 'secret-outgoing-credential',
          expiresAt: '2026-06-04T10:30:00.000Z',
        };
      },
    },
  } as unknown as ImSdkClient;
  const outgoingService = createSdkworkCallService({
    getClient: () => outgoingClient,
    rtcMediaService: noopRtcMediaService,
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
  const outgoingSnapshots: string[] = [];
  outgoingService.subscribe((nextSnapshot) => {
    outgoingSnapshots.push(nextSnapshot.state);
  });
  await outgoingService.watchIncomingCalls(['conversation-rtc-1']);
  const outgoingSnapshot = await outgoingService.startOutgoingCall({
    conversationId: 'conversation-rtc-1',
    targetName: 'Bob',
    type: 'video',
  });
  assert.equal(outgoingSnapshot.state, 'ringing');
  assert.equal(outgoingSnapshot.direction, 'outgoing');
  assert.equal(outgoingSnapshot.rtcSessionId?.startsWith('call-pc-conversation-rtc-1-'), true);
  const outgoingRtcSessionId = outgoingSnapshot.rtcSessionId;
  assert.equal(typeof outgoingRtcSessionId, 'string');
  outgoingSessionListeners.at(-1)?.({
    tenantId: '100001',
    rtcSessionId: outgoingRtcSessionId,
    conversationId: 'conversation-rtc-1',
    rtcMode: 'video',
    initiatorId: 'u_alice',
    initiatorKind: 'user',
    providerPluginId: null,
    providerSessionId: `room-${outgoingRtcSessionId}`,
    startedAt: '2026-06-04T10:20:00.000Z',
    state: 'accepted',
  });
  await waitForCondition(
    () => outgoingService.getSnapshot().isParticipantCredentialReady === true,
    'Outgoing caller must issue participant credentials after the callee accepts through realtime signaling',
  );
  assert.equal(outgoingService.getSnapshot().state, 'connected');
  outgoingSessionListeners.at(-1)?.({
    tenantId: '100001',
    rtcSessionId: outgoingRtcSessionId,
    conversationId: 'conversation-rtc-1',
    rtcMode: 'video',
    initiatorId: 'u_alice',
    initiatorKind: 'user',
    providerPluginId: null,
    providerSessionId: `room-${outgoingRtcSessionId}`,
    startedAt: '2026-06-04T10:20:00.000Z',
    state: 'ended',
  });
  assert.equal(
    outgoingService.getSnapshot().state,
    'ended',
    'Outgoing caller CallService state must end automatically when the peer hangs up through realtime signaling',
  );
  assert.deepEqual(
    outgoingSnapshots
      .filter((state) => state === 'ringing' || state === 'connected' || state === 'ended')
      .filter((state, index, states) => index === 0 || states[index - 1] !== state),
    ['ringing', 'connected', 'ended'],
    'Outgoing caller subscribers must receive ringing, accepted, and remote-ended transitions for the same call',
  );
  assert.deepEqual(
    outgoingCalls.filter((call) => call.method === 'calls.issueParticipantCredential'),
    [{
      method: 'calls.issueParticipantCredential',
      participantId: 'u_alice',
      rtcSessionId: outgoingRtcSessionId,
    }],
    'Outgoing caller must prepare RTC participant credentials exactly once after the call is accepted',
  );

  const racingCredential = createDeferred<{
    credential: string;
    expiresAt: string;
    participantId: string;
    rtcSessionId: string;
    tenantId: string;
  }>();
  const racingListeners: CallSessionListener[] = [];
  const racingClient = {
    calls: {
      subscribe(handler: CallSessionListener) {
        racingListeners.push(handler);
        return () => undefined;
      },
      async watchIncoming() {
        return null;
      },
      issueParticipantCredential() {
        return racingCredential.promise;
      },
    },
  } as unknown as ImSdkClient;
  const racingService = createSdkworkCallService({
    getClient: () => racingClient,
    rtcMediaService: noopRtcMediaService,
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
  await racingService.watchIncomingCalls(['conversation-rtc-1']);
  racingListeners.at(-1)?.(createSession('rtc-racing-credential-1', 'started'));
  racingListeners.at(-1)?.(createSession('rtc-racing-credential-1', 'accepted'));
  assert.equal(racingService.getSnapshot().state, 'connected');
  racingListeners.at(-1)?.(createSession('rtc-racing-credential-1', 'ended'));
  assert.equal(racingService.getSnapshot().state, 'ended');
  racingCredential.resolve({
    tenantId: '100001',
    rtcSessionId: 'rtc-racing-credential-1',
    participantId: 'u_alice',
    credential: 'secret-late-credential',
    expiresAt: '2026-06-04T10:35:00.000Z',
  });
  await Promise.resolve();
  assert.equal(
    racingService.getSnapshot().isParticipantCredentialReady,
    false,
    'A late RTC participant credential must not mark an already-ended call as credential-ready',
  );
  assert.equal(racingService.getSnapshot().participantCredentialExpiresAt, undefined);

  const failingCredential = createDeferred<{
    credential: string;
    expiresAt: string;
    participantId: string;
    rtcSessionId: string;
    tenantId: string;
  }>();
  const failingCredentialListeners: CallSessionListener[] = [];
  const failingCredentialClient = {
    calls: {
      subscribe(handler: CallSessionListener) {
        failingCredentialListeners.push(handler);
        return () => undefined;
      },
      async watchIncoming() {
        return null;
      },
      issueParticipantCredential() {
        return failingCredential.promise;
      },
    },
  } as unknown as ImSdkClient;
  const failingCredentialService = createSdkworkCallService({
    getClient: () => failingCredentialClient,
    rtcMediaService: noopRtcMediaService,
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
  await failingCredentialService.watchIncomingCalls(['conversation-rtc-1']);
  failingCredentialListeners.at(-1)?.(createSession('rtc-failing-credential-1', 'started'));
  failingCredentialListeners.at(-1)?.(createSession('rtc-failing-credential-1', 'accepted'));
  assert.equal(failingCredentialService.getSnapshot().state, 'connected');
  failingCredentialListeners.at(-1)?.(createSession('rtc-failing-credential-1', 'ended'));
  assert.equal(failingCredentialService.getSnapshot().state, 'ended');
  failingCredential.reject(new Error('credential service unavailable'));
  await new Promise((resolve) => setTimeout(resolve, 0));
  assert.equal(
    failingCredentialService.getSnapshot().state,
    'ended',
    'A late RTC credential failure must not overwrite a remotely ended call with errored state',
  );

  console.log('sdkwork-im-pc RTC session recovery contract passed');
}

void main();
