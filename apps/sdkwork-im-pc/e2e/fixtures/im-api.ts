import type { Page, Route } from '@playwright/test';

export const PLAYWRIGHT_CONVERSATION_ID = 'conversation.playwright.1';
export const PLAYWRIGHT_PEER_DISPLAY_NAME = 'Playwright Peer';
export const PLAYWRIGHT_INITIAL_MESSAGE = 'Hello from Playwright';

export interface PlaywrightImApiFixtureOptions {
  conversationId?: string;
  peerDisplayName?: string;
  initialMessage?: string;
  tenantId?: string;
  userId?: string;
}

function buildInboxEntry(options: Required<PlaywrightImApiFixtureOptions>) {
  const now = new Date().toISOString();
  return {
    tenantId: options.tenantId,
    conversationId: options.conversationId,
    agentHandoff: false,
    conversationType: 'single',
    displayName: options.peerDisplayName,
    avatarUrl: null,
    displaySource: 'inbox',
    peer: {
      userId: 'user.playwright.peer.1',
      displayName: options.peerDisplayName,
      avatarUrl: null,
    },
    preferences: {
      isPinned: false,
      isMuted: false,
      isMarkedUnread: false,
      isHidden: false,
    },
    lastActivityAt: now,
    lastMessageId: 'message.playwright.1',
    lastSenderId: 'user.playwright.peer.1',
    messageCount: 1,
    lastMessageSeq: 1,
    lastSummary: options.initialMessage,
    lastMessageAt: now,
    unreadCount: 0,
  };
}

function buildTimelineEntry(options: Required<PlaywrightImApiFixtureOptions>) {
  const now = new Date().toISOString();
  return {
    tenantId: options.tenantId,
    conversationId: options.conversationId,
    messageId: 'message.playwright.1',
    messageSeq: 1,
    summary: options.initialMessage,
    sender: {
      id: 'user.playwright.peer.1',
      kind: 'user',
    },
    body: {
      text: options.initialMessage,
      summary: options.initialMessage,
      parts: [],
    },
    messageType: 'text',
    deliveryMode: 'normal',
    occurredAt: now,
  };
}

function emptyPagedResponse() {
  return {
    items: [],
    hasMore: false,
    nextCursor: null,
  };
}

function emptyTimelineResponse() {
  return {
    items: [],
    hasMore: false,
    nextAfterSeq: null,
  };
}

export async function installPlaywrightImApiMocks(
  page: Page,
  fixtureOptions: PlaywrightImApiFixtureOptions = {},
): Promise<void> {
  const options: Required<PlaywrightImApiFixtureOptions> = {
    conversationId: fixtureOptions.conversationId ?? PLAYWRIGHT_CONVERSATION_ID,
    peerDisplayName: fixtureOptions.peerDisplayName ?? PLAYWRIGHT_PEER_DISPLAY_NAME,
    initialMessage: fixtureOptions.initialMessage ?? PLAYWRIGHT_INITIAL_MESSAGE,
    tenantId: fixtureOptions.tenantId ?? 'tenant.playwright.1',
    userId: fixtureOptions.userId ?? 'user.playwright.100',
  };

  const inboxEntry = buildInboxEntry(options);
  const timelineEntry = buildTimelineEntry(options);
  let nextMessageSeq = timelineEntry.messageSeq;

  await page.route('**/im/v3/api/**', async (route: Route) => {
    const request = route.request();
    const url = new URL(request.url());
    const pathname = url.pathname;

    if (request.method() === 'GET' && pathname.endsWith('/chat/inbox')) {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          items: [inboxEntry],
          hasMore: false,
          nextCursor: null,
        }),
      });
      return;
    }

    if (
      request.method() === 'GET'
      && pathname.includes('/chat/conversations/')
      && pathname.endsWith('/messages')
    ) {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          items: [timelineEntry],
          hasMore: false,
          nextAfterSeq: null,
        }),
      });
      return;
    }

    if (request.method() === 'GET' && pathname.endsWith('/chat/contacts')) {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify(emptyPagedResponse()),
      });
      return;
    }

    if (request.method() === 'GET' && pathname.endsWith('/social/friend_requests')) {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify(emptyPagedResponse()),
      });
      return;
    }

    if (
      request.method() === 'POST'
      && pathname.includes('/chat/conversations/')
      && pathname.endsWith('/messages')
    ) {
      nextMessageSeq += 1;
      const requestBody = request.postDataJSON() as {
        summary?: string;
        text?: string;
        clientMsgId?: string;
      } | null;
      const outboundText = requestBody?.text ?? requestBody?.summary ?? 'E2E outbound message';
      const now = new Date().toISOString();
      const messageId = `message.playwright.${nextMessageSeq}`;

      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          conversationId: options.conversationId,
          messageId,
          messageSeq: nextMessageSeq,
          body: {
            text: outboundText,
            summary: outboundText,
            parts: [],
          },
          occurredAt: now,
        }),
      });
      return;
    }

    if (request.method() === 'GET') {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify(
          pathname.includes('/messages') ? emptyTimelineResponse() : emptyPagedResponse(),
        ),
      });
      return;
    }

    await route.continue();
  });
}
