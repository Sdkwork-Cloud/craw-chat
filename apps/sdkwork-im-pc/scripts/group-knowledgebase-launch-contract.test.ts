import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

import {
  buildGroupKnowledgebaseBrowserUrlForBaseUrl,
  createGroupKnowledgebaseLaunchIdempotencyKey,
  createGroupKnowledgebaseLaunchService,
  resolveGroupKnowledgebaseAccessMode,
  reserveGroupKnowledgebaseBrowserWindow,
  type GroupKnowledgebaseLaunchClient,
} from '../packages/sdkwork-im-pc-chat/src/services/GroupKnowledgebaseLaunchService';
import {
  isCurrentGroupOwnerMember,
  resolveCurrentGroupKnowledgebaseMemberAccess,
} from '../packages/sdkwork-im-pc-chat/src/services/GroupKnowledgebaseAccessPolicy';

const VALID_TICKET = `gklt_${'a'.repeat(43)}`;

type GroupKnowledgebaseLaunchOperation = GroupKnowledgebaseLaunchClient['chat']['conversations']['knowledgebase']['launch'];
type GroupKnowledgebaseLaunchArguments = Parameters<GroupKnowledgebaseLaunchOperation>;
type GroupKnowledgebaseLaunchResponse = Awaited<ReturnType<GroupKnowledgebaseLaunchOperation>>;
type GroupKnowledgebaseRetrieveOperation = GroupKnowledgebaseLaunchClient['chat']['conversations']['knowledgebase']['retrieve'];
type GroupKnowledgebaseRetrieveResponse = Awaited<ReturnType<GroupKnowledgebaseRetrieveOperation>>;

function activeLaunchResponse(ticket = VALID_TICKET): GroupKnowledgebaseLaunchResponse {
  return {
    upstreamLinkGeneration: '1',
    conversationId: 'conversation-1',
    launchTicket: ticket,
    lifecycleState: 'active',
    membershipEpoch: '1',
  };
}

function lifecycleResponse(
  lifecycleState: GroupKnowledgebaseRetrieveResponse['lifecycleState'],
  conversationId = 'conversation-1',
): GroupKnowledgebaseRetrieveResponse {
  return {
    upstreamLinkGeneration: '1',
    conversationId,
    lifecycleState,
    membershipEpoch: '1',
  };
}

function createLaunchClient(
  launch: GroupKnowledgebaseLaunchOperation,
  retrieve: GroupKnowledgebaseRetrieveOperation = async () => lifecycleResponse('absent'),
): GroupKnowledgebaseLaunchClient {
  return {
    chat: {
      conversations: {
        knowledgebase: { launch, retrieve },
      },
    },
  };
}

{
  assert.match(
    createGroupKnowledgebaseLaunchIdempotencyKey(),
    /^pc-group-knowledgebase-launch-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/u,
  );
}

{
  assert.equal(isCurrentGroupOwnerMember({ state: 'joined', role: 'owner' }), true);
  assert.equal(isCurrentGroupOwnerMember({ state: 'joined', role: 'admin' }), false);
  assert.equal(isCurrentGroupOwnerMember({ state: 'left', role: 'owner' }), false);

  assert.deepEqual(
    resolveCurrentGroupKnowledgebaseMemberAccess({ state: 'joined', role: 'owner' }),
    { canInitialize: true, canOpen: true },
  );
  assert.deepEqual(
    resolveCurrentGroupKnowledgebaseMemberAccess({ state: 'joined', role: 'admin' }),
    { canInitialize: false, canOpen: true },
  );
  assert.deepEqual(
    resolveCurrentGroupKnowledgebaseMemberAccess({ state: 'joined', role: 'member' }),
    { canInitialize: false, canOpen: true },
  );
  assert.deepEqual(
    resolveCurrentGroupKnowledgebaseMemberAccess({ state: 'joined', role: 'guest' }),
    { canInitialize: false, canOpen: false },
  );
  assert.deepEqual(
    resolveCurrentGroupKnowledgebaseMemberAccess({ state: 'left', role: 'owner' }),
    { canInitialize: false, canOpen: false },
  );
}

{
  const owner = { canInitialize: true, canOpen: true, hasAuthenticatedSession: true };
  const member = { canInitialize: false, canOpen: true, hasAuthenticatedSession: true };
  const guest = { canInitialize: false, canOpen: false, hasAuthenticatedSession: true };

  assert.equal(resolveGroupKnowledgebaseAccessMode('absent', owner), 'initialize');
  assert.equal(resolveGroupKnowledgebaseAccessMode('failed', owner), 'initialize');
  assert.equal(resolveGroupKnowledgebaseAccessMode('absent', member), 'contact-owner');
  assert.equal(resolveGroupKnowledgebaseAccessMode('provisioning', owner), 'provisioning');
  assert.equal(resolveGroupKnowledgebaseAccessMode('provisioning', member), 'contact-owner');
  assert.equal(resolveGroupKnowledgebaseAccessMode('active', member), 'open');
  assert.equal(resolveGroupKnowledgebaseAccessMode('active', guest), 'unavailable');
  assert.equal(
    resolveGroupKnowledgebaseAccessMode('absent', { ...owner, hasLifecycleLoadError: true }),
    'retry',
  );
  assert.equal(
    resolveGroupKnowledgebaseAccessMode('active', { ...member, hasLifecycleLoadError: true }),
    'unavailable',
  );
  assert.equal(
    resolveGroupKnowledgebaseAccessMode('active', { ...guest, hasMemberAccessLoadError: true }),
    'retry',
  );
  assert.equal(
    resolveGroupKnowledgebaseAccessMode('active', { ...owner, hasAuthenticatedSession: false }),
    'unavailable',
  );
  assert.equal(
    resolveGroupKnowledgebaseAccessMode('absent', { ...owner, hasLifecycleUnavailable: true }),
    'unavailable',
  );
  assert.equal(
    resolveGroupKnowledgebaseAccessMode(null, { ...guest, isLoading: true }),
    'loading',
  );
}

{
  const service = createGroupKnowledgebaseLaunchService({
    getClient: () => createLaunchClient(
      async () => activeLaunchResponse(),
      async () => {
        throw Object.assign(new Error('organization scope is unavailable'), { httpStatus: 403 });
      },
    ),
  });

  assert.deepEqual(await service.retrieveLifecycle('conversation-1'), { kind: 'unavailable' });
}

{
  const service = createGroupKnowledgebaseLaunchService({
    getClient: () => createLaunchClient(
      async () => activeLaunchResponse(),
      async () => lifecycleResponse('active'),
    ),
  });

  assert.equal(await service.retrieveLifecycleState('conversation-1'), 'active');
  assert.deepEqual(await service.retrieveLifecycle('conversation-1'), {
    kind: 'resolved',
    lifecycleState: 'active',
  });
}

{
  const service = createGroupKnowledgebaseLaunchService({
    getClient: () => createLaunchClient(
      async () => activeLaunchResponse(),
      async () => lifecycleResponse('active', 'another-conversation'),
    ),
  });

  assert.equal(await service.retrieveLifecycleState('conversation-1'), null);
  assert.deepEqual(await service.retrieveLifecycle('conversation-1'), { kind: 'failed' });
}

{
  const service = createGroupKnowledgebaseLaunchService({
    getClient: () => createLaunchClient(
      async () => activeLaunchResponse(),
      async () => {
        throw new Error('transient lifecycle lookup failure');
      },
    ),
  });

  assert.deepEqual(await service.retrieveLifecycle('conversation-1'), { kind: 'failed' });
}

{
  const chatLayoutSource = readFileSync(
    new URL('../packages/sdkwork-im-pc-chat/src/pages/ChatLayout.tsx', import.meta.url),
    'utf8',
  );
  const chatRightPanelSource = readFileSync(
    new URL('../packages/sdkwork-im-pc-chat/src/components/ChatRightPanel.tsx', import.meta.url),
    'utf8',
  );
  const launchServiceSource = readFileSync(
    new URL('../packages/sdkwork-im-pc-chat/src/services/GroupKnowledgebaseLaunchService.ts', import.meta.url),
    'utf8',
  );

  assert.match(chatLayoutSource, /groupService\.retrieveCurrentUserGroupKnowledgebaseAccess\(activeGroupId\)/u);
  assert.match(chatLayoutSource, /groupKnowledgebaseLaunchService\.retrieveLifecycle\(activeGroupId\)/u);
  assert.doesNotMatch(
    chatLayoutSource,
    /resolveAppSdkOrganizationId|isCanonicalGroupKnowledgebaseOrganizationId/u,
    'the PC client must not infer organization authorization from locally visible token claims',
  );
  assert.doesNotMatch(
    launchServiceSource,
    /hasCanonicalGroupKnowledgebaseOrganizationScope|hasOrganizationScope/u,
    'the launch service must defer organization authorization to the IM service',
  );
  assert.match(chatLayoutSource, /resolveGroupKnowledgebaseAccessMode\(/u);
  assert.match(
    chatLayoutSource,
    /const\s+shouldShowGroupKnowledgebaseHeaderAction\s*=\s*localizedActiveChat\?\.type\s*===\s*['"]group['"]/u,
    'every group member must see the knowledgebase header action regardless of lifecycle state',
  );
  assert.match(
    chatLayoutSource,
    /groupKnowledgebaseAccessMode\s*===\s*['"]contact-owner['"][\s\S]*knowledgebaseContactOwner/u,
    'non-owner members must receive the contact-owner guidance when the group knowledgebase is absent',
  );
  assert.match(
    chatLayoutSource,
    /disabled=\{groupKnowledgebaseAccessMode\s*===\s*['"]loading['"][\s\S]*groupKnowledgebaseAccessMode\s*===\s*['"]provisioning['"][\s\S]*isOpeningGroupKnowledgebase\}/u,
    'the knowledgebase icon must block duplicate launch requests while provisioning',
  );
  assert.match(chatLayoutSource, /groupKnowledgebaseAccessConversationId/u);
  assert.match(chatLayoutSource, /groupKnowledgebaseAccessSessionEpoch/u);
  assert.match(chatLayoutSource, /groupKnowledgebaseAccessReloadEpoch/u);
  assert.match(
    chatLayoutSource,
    /chat\.name\.trim\(\) === ['"]Group chat['"][\s\S]*chat\.name\.trim\(\) === ['"]Direct chat['"]/u,
    'technical fallback conversation names must be localized before rendering',
  );
  assert.match(chatLayoutSource, /handleRetryGroupKnowledgebaseAccess/u);
  assert.doesNotMatch(
    chatLayoutSource,
    /handleRetryGroupKnowledgebaseAccess[\s\S]{0,400}isCurrentUserGroupOwner/u,
  );
  assert.match(chatLayoutSource, /groupKnowledgebaseSessionEpoch/u);
  assert.match(chatLayoutSource, /setGroupKnowledgebaseSessionEpoch\(\(epoch\) => epoch \+ 1\)/u);
  assert.match(chatLayoutSource, /groupKnowledgebaseLaunchAbortRef\.current = null/u);
  assert.match(chatRightPanelSource, /canManageKnowledgebase/u);
  assert.match(chatRightPanelSource, /onManageKnowledgebase/u);
  assert.doesNotMatch(
    chatRightPanelSource,
    /activeChat\.type\s*===\s*['"]group['"]\s*&&\s*canManageKnowledgebase\s*&&\s*knowledgebaseActionLabel/u,
    'group details must show the knowledgebase status and action to every group member',
  );
  assert.match(
    chatLayoutSource,
    /chat\.rightPanel\.actions\.knowledgebaseContactOwner/u,
    'group details must explain the owner contact requirement when the knowledgebase is not initialized',
  );
}

function createBrowserWindow() {
  let closed = false;
  const navigatedUrls: string[] = [];

  return {
    browserWindow: {
      close() {
        closed = true;
      },
      navigate(url: string) {
        navigatedUrls.push(url);
        return true;
      },
    },
    isClosed: () => closed,
    navigatedUrls,
  };
}

function withBrowserWindow(fakeWindow: unknown, run: () => void): void {
  const previous = Object.getOwnPropertyDescriptor(globalThis, 'window');
  Object.defineProperty(globalThis, 'window', {
    configurable: true,
    value: fakeWindow,
  });
  try {
    run();
  } finally {
    if (previous) {
      Object.defineProperty(globalThis, 'window', previous);
    } else {
      delete (globalThis as typeof globalThis & { window?: unknown }).window;
    }
  }
}

{
  let closed = false;
  const popup = {
    close() {
      closed = true;
    },
    location: {
      replace() {
        return undefined;
      },
    },
    opener: {} as unknown,
  };
  withBrowserWindow({ open: () => popup }, () => {
    const reserved = reserveGroupKnowledgebaseBrowserWindow();
    assert.ok(reserved);
    assert.equal(popup.opener, null);
    assert.equal(closed, false);
  });
}

{
  let closed = false;
  const popup: {
    close(): void;
    location: { replace(): void };
    opener?: unknown;
  } = {
    close() {
      closed = true;
    },
    location: {
      replace() {
        return undefined;
      },
    },
  };
  Object.defineProperty(popup, 'opener', {
    configurable: true,
    get: () => ({}),
    set: () => undefined,
  });
  withBrowserWindow({ open: () => popup }, () => {
    assert.equal(reserveGroupKnowledgebaseBrowserWindow(), null);
    assert.equal(closed, true);
  });
}

{
  const destination = buildGroupKnowledgebaseBrowserUrlForBaseUrl(
    VALID_TICKET,
    'https://knowledgebase.example.test/apps/knowledgebase',
  );
  assert.ok(destination);

  const url = new URL(destination);
  assert.equal(url.pathname, '/apps/knowledgebase/group-launch');
  assert.equal(url.search, '');
  assert.equal(url.hash, `#ticket=${VALID_TICKET}`);
  assert.equal(
    buildGroupKnowledgebaseBrowserUrlForBaseUrl(
      VALID_TICKET,
      'https://knowledgebase.example.test/apps/knowledgebase?unexpected=value',
    ),
    null,
  );
  assert.equal(
    buildGroupKnowledgebaseBrowserUrlForBaseUrl(
      VALID_TICKET,
      'http://knowledgebase.example.test/apps/knowledgebase',
    ),
    null,
  );
  assert.equal(
    buildGroupKnowledgebaseBrowserUrlForBaseUrl(
      VALID_TICKET,
      'http://knowledgebase.example.test/apps/knowledgebase',
      { allowInsecureLoopback: true },
    ),
    null,
  );
  assert.equal(
    buildGroupKnowledgebaseBrowserUrlForBaseUrl(
      VALID_TICKET,
      'http://localhost:4173/apps/knowledgebase',
      { allowInsecureLoopback: true },
    ),
    `http://localhost:4173/apps/knowledgebase/group-launch#ticket=${VALID_TICKET}`,
  );
}

{
  let clientCalls = 0;
  const popup = createBrowserWindow();
  const service = createGroupKnowledgebaseLaunchService({
    isBrowserDestinationConfigured: () => false,
    getClient: () => {
      clientCalls += 1;
      return createLaunchClient(async () => activeLaunchResponse());
    },
    isDesktopRuntime: () => false,
    reserveBrowserWindow: () => popup.browserWindow,
  });

  assert.deepEqual(await service.open('conversation-1'), { kind: 'failed' });
  assert.equal(clientCalls, 0);
  assert.equal(popup.isClosed(), true);
}

{
  let clientCalls = 0;
  const service = createGroupKnowledgebaseLaunchService({
    getClient: () => {
      clientCalls += 1;
      return createLaunchClient(async () => activeLaunchResponse());
    },
    isDesktopRuntime: () => true,
    isDesktopHostAvailable: () => false,
  });

  assert.deepEqual(await service.open('conversation-1'), { kind: 'failed' });
  assert.equal(clientCalls, 0);
}

{
  const popup = createBrowserWindow();
  const launchCalls: GroupKnowledgebaseLaunchArguments[] = [];
  const service = createGroupKnowledgebaseLaunchService({
    createIdempotencyKey: () => 'pc-group-knowledgebase-launch-test',
    getClient: () => createLaunchClient(async (...args) => {
      launchCalls.push(args);
      return activeLaunchResponse();
    }),
    isDesktopRuntime: () => false,
    isBrowserDestinationConfigured: () => true,
    reserveBrowserWindow: () => popup.browserWindow,
    resolveBrowserUrl: (ticket) => (
      `https://knowledgebase.example.test/apps/knowledgebase/group-launch#ticket=${ticket}`
    ),
  });

  assert.deepEqual(await service.open('conversation-1'), { kind: 'opened' });
  assert.equal(popup.isClosed(), false);
  assert.equal(popup.navigatedUrls.length, 1);

  const destination = new URL(popup.navigatedUrls[0]);
  assert.equal(destination.pathname, '/apps/knowledgebase/group-launch');
  assert.equal(destination.search, '');
  assert.equal(destination.hash, `#ticket=${VALID_TICKET}`);
  assert.deepEqual(launchCalls, [[
    'conversation-1',
    {},
    { idempotencyKey: 'pc-group-knowledgebase-launch-test' },
  ]]);
}

{
  const popup = createBrowserWindow();
  const launchCalls: GroupKnowledgebaseLaunchArguments[] = [];
  const service = createGroupKnowledgebaseLaunchService({
    createIdempotencyKey: () => 'pc-group-knowledgebase-launch-initial',
    getClient: () => createLaunchClient(async (...args) => {
      launchCalls.push(args);
      return {
        upstreamLinkGeneration: '1',
        conversationId: 'conversation-1',
        lifecycleState: 'provisioning',
        membershipEpoch: '1',
      };
    }),
    isDesktopRuntime: () => false,
    isBrowserDestinationConfigured: () => true,
    reserveBrowserWindow: () => popup.browserWindow,
  });

  assert.deepEqual(await service.open('conversation-1'), { kind: 'provisioning' });
  assert.equal(popup.isClosed(), true);
  assert.equal(popup.navigatedUrls.length, 0);
  assert.deepEqual(launchCalls, [[
    'conversation-1',
    {},
    { idempotencyKey: 'pc-group-knowledgebase-launch-initial' },
  ]]);
}

{
  const popup = createBrowserWindow();
  const service = createGroupKnowledgebaseLaunchService({
    getClient: () => createLaunchClient(async () => ({
      upstreamLinkGeneration: '1',
      conversationId: 'conversation-1',
      lifecycleState: 'active',
      membershipEpoch: '1',
    })),
    isDesktopRuntime: () => false,
    isBrowserDestinationConfigured: () => true,
    reserveBrowserWindow: () => popup.browserWindow,
  });

  // The server keeps lifecycleState active while membership ACL convergence is
  // pending, and deliberately withholds the one-time ticket until it finishes.
  assert.deepEqual(await service.open('conversation-1'), { kind: 'provisioning' });
  assert.equal(popup.isClosed(), true);
  assert.equal(popup.navigatedUrls.length, 0);
}

{
  const popup = createBrowserWindow();
  let launchCalls = 0;
  let resolveLaunch: ((response: GroupKnowledgebaseLaunchResponse) => void) | undefined;
  const deferredLaunch = new Promise<GroupKnowledgebaseLaunchResponse>((resolve) => {
    resolveLaunch = resolve;
  });
  const service = createGroupKnowledgebaseLaunchService({
    getClient: () => createLaunchClient(async () => {
      launchCalls += 1;
      return deferredLaunch;
    }),
    isDesktopRuntime: () => false,
    isBrowserDestinationConfigured: () => true,
    reserveBrowserWindow: () => popup.browserWindow,
  });
  const controller = new AbortController();
  const opening = service.open('conversation-1', { signal: controller.signal });

  assert.equal(launchCalls, 1);
  controller.abort();
  assert.deepEqual(await opening, { kind: 'cancelled' });
  assert.equal(popup.isClosed(), true);
  assert.equal(popup.navigatedUrls.length, 0);

  if (!resolveLaunch) {
    throw new Error('The deferred launch must be started before cancellation.');
  }
  resolveLaunch(activeLaunchResponse());
  await Promise.resolve();
}

{
  let desktopRequests = 0;
  const service = createGroupKnowledgebaseLaunchService({
    getClient: () => createLaunchClient(async () => ({
      upstreamLinkGeneration: '1',
      conversationId: 'conversation-1',
      lifecycleState: 'provisioning',
      membershipEpoch: '1',
    })),
    isDesktopRuntime: () => true,
    isDesktopHostAvailable: () => true,
    openDesktop: async () => {
      desktopRequests += 1;
      return true;
    },
  });

  assert.deepEqual(await service.open('conversation-1'), { kind: 'provisioning' });
  assert.equal(desktopRequests, 0);
}

{
  const popup = createBrowserWindow();
  const service = createGroupKnowledgebaseLaunchService({
    getClient: () => createLaunchClient(async () => ({
      upstreamLinkGeneration: '1',
      conversationId: 'conversation-1',
      lifecycleState: 'archived',
      membershipEpoch: '1',
    })),
    isDesktopRuntime: () => false,
    isBrowserDestinationConfigured: () => true,
    reserveBrowserWindow: () => popup.browserWindow,
  });

  assert.deepEqual(await service.open('conversation-1'), { kind: 'unavailable' });
  assert.equal(popup.isClosed(), true);
}

{
  const popup = createBrowserWindow();
  const service = createGroupKnowledgebaseLaunchService({
    getClient: () => createLaunchClient(async () => ({
      ...activeLaunchResponse(),
      conversationId: 'another-conversation',
    })),
    isDesktopRuntime: () => false,
    isBrowserDestinationConfigured: () => true,
    reserveBrowserWindow: () => popup.browserWindow,
  });

  assert.deepEqual(await service.open('conversation-1'), { kind: 'failed' });
  assert.equal(popup.isClosed(), true);
  assert.equal(popup.navigatedUrls.length, 0);
}

{
  let desktopRequest: { launchTicket: string } | undefined;
  const service = createGroupKnowledgebaseLaunchService({
    getClient: () => createLaunchClient(async () => activeLaunchResponse()),
    isDesktopRuntime: () => true,
    isDesktopHostAvailable: () => true,
    openDesktop: async (request) => {
      desktopRequest = request;
      return true;
    },
  });

  assert.deepEqual(await service.open('conversation-1'), { kind: 'opened' });
  assert.deepEqual(desktopRequest, { launchTicket: VALID_TICKET });
}

{
  const controller = new AbortController();
  let desktopRequest: { launchTicket: string } | undefined;
  const service = createGroupKnowledgebaseLaunchService({
    getClient: () => createLaunchClient(async () => activeLaunchResponse()),
    isDesktopRuntime: () => true,
    isDesktopHostAvailable: () => true,
    openDesktop: async (request) => {
      desktopRequest = request;
      controller.abort();
      return true;
    },
  });

  assert.deepEqual(
    await service.open('conversation-1', { signal: controller.signal }),
    { kind: 'cancelled' },
  );
  assert.deepEqual(desktopRequest, { launchTicket: VALID_TICKET });
}

{
  let reserveCalls = 0;
  const popup = createBrowserWindow();
  const service = createGroupKnowledgebaseLaunchService({
    getClient: () => createLaunchClient(async () => ({
      upstreamLinkGeneration: '1',
      conversationId: 'conversation-1',
      lifecycleState: 'absent',
      membershipEpoch: '1',
    })),
    isDesktopRuntime: () => false,
    isBrowserDestinationConfigured: () => true,
    reserveBrowserWindow: () => {
      reserveCalls += 1;
      return popup.browserWindow;
    },
  });

  assert.deepEqual(await service.open('conversation-1'), { kind: 'unavailable' });
  assert.equal(reserveCalls, 1);
  assert.equal(popup.isClosed(), true);
}

{
  const popup = createBrowserWindow();
  const service = createGroupKnowledgebaseLaunchService({
    getClient: () => createLaunchClient(async () => activeLaunchResponse('gklt_invalid')),
    isDesktopRuntime: () => false,
    isBrowserDestinationConfigured: () => true,
    reserveBrowserWindow: () => popup.browserWindow,
  });

  assert.deepEqual(await service.open('conversation-1'), { kind: 'failed' });
  assert.equal(popup.isClosed(), true);
}

{
  let clientCalls = 0;
  const popup = createBrowserWindow();
  const service = createGroupKnowledgebaseLaunchService({
    createIdempotencyKey: () => {
      throw new Error('Web Crypto is unavailable');
    },
    getClient: () => {
      clientCalls += 1;
      return createLaunchClient(async () => activeLaunchResponse());
    },
    isDesktopRuntime: () => false,
    isBrowserDestinationConfigured: () => true,
    reserveBrowserWindow: () => popup.browserWindow,
  });

  assert.deepEqual(await service.open('conversation-1'), { kind: 'failed' });
  assert.equal(clientCalls, 0);
  assert.equal(popup.isClosed(), true);
}

{
  const malformedResponse = activeLaunchResponse();
  Object.defineProperty(malformedResponse, 'lifecycleState', { value: undefined });
  const popup = createBrowserWindow();
  const service = createGroupKnowledgebaseLaunchService({
    getClient: () => createLaunchClient(async () => malformedResponse),
    isDesktopRuntime: () => false,
    isBrowserDestinationConfigured: () => true,
    reserveBrowserWindow: () => popup.browserWindow,
  });

  assert.deepEqual(await service.open('conversation-1'), { kind: 'failed' });
  assert.equal(popup.isClosed(), true);
}

console.log('group knowledgebase launch contract passed.');
