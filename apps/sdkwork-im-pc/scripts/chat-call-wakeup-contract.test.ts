import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';
import type * as NotificationServiceModule from '../packages/sdkwork-im-pc-chat/src/services/NotificationService.ts';

type NotificationServiceExports = typeof NotificationServiceModule & {
  buildIncomingCallNotification: (input: {
    callId: string;
    conversationId: string;
    callerAvatar?: string;
    callerName: string;
    previewMode: NotificationServiceModule.NotificationPreviewMode;
    type: 'voice' | 'video';
  }) => NotificationServiceModule.IncomingCallNotification;
  dispatchNotificationOpenCall: (
    notification: NotificationServiceModule.IncomingCallNotification,
    environment?: unknown,
  ) => void;
  restoreDesktopMainWindow: (environment?: unknown) => Promise<boolean>;
  showSystemNotification: (
    notification: NotificationServiceModule.IncomingCallNotification,
    environment?: unknown,
  ) => Promise<'native' | 'web' | 'skipped'>;
};

async function loadNotificationServiceModule(): Promise<NotificationServiceExports> {
  const moduleUrl = pathToFileURL(
    './packages/sdkwork-im-pc-chat/src/services/NotificationService.ts',
  ).href;
  return await import(moduleUrl) as NotificationServiceExports;
}

const notificationService = await loadNotificationServiceModule();

assert.equal(
  typeof notificationService.buildIncomingCallNotification,
  'function',
  'NotificationService must build a dedicated incoming call notification instead of treating calls as ordinary messages.',
);
assert.equal(
  typeof notificationService.dispatchNotificationOpenCall,
  'function',
  'NotificationService must expose a typed open-call dispatcher for notification clicks and tray wake-up.',
);
assert.equal(
  typeof notificationService.restoreDesktopMainWindow,
  'function',
  'NotificationService must expose one guarded desktop window restore adapter for call wake-up.',
);

const hiddenPreviewCall = notificationService.buildIncomingCallNotification({
  callId: 'rtc-call-hidden',
  conversationId: 'conversation-video-1',
  callerAvatar: 'https://example.test/bob.png',
  callerName: 'Bob',
  previewMode: 'hidden',
  type: 'video',
});

assert.deepEqual(
  hiddenPreviewCall,
  {
    body: 'Incoming video call',
    callId: 'rtc-call-hidden',
    conversationId: 'conversation-video-1',
    icon: undefined,
    kind: 'call',
    title: 'Sdkwork IM',
    type: 'video',
  },
  'Hidden notification previews must hide caller identity but keep enough call metadata to open the call UI.',
);

const visiblePreviewCall = notificationService.buildIncomingCallNotification({
  callId: 'rtc-call-visible',
  conversationId: 'conversation-video-2',
  callerAvatar: 'https://example.test/alice.png',
  callerName: 'Alice',
  previewMode: 'sender-and-preview',
  type: 'video',
});

assert.deepEqual(
  visiblePreviewCall,
  {
    body: 'Incoming video call',
    callId: 'rtc-call-visible',
    conversationId: 'conversation-video-2',
    icon: 'https://example.test/alice.png',
    kind: 'call',
    title: 'Alice',
    type: 'video',
  },
  'Visible call previews should show the caller name and call type.',
);

const dispatchedEvents: Array<{ detail?: unknown; type: string }> = [];
const focused: string[] = [];
class ContractCustomEvent {
  detail?: unknown;
  type: string;

  constructor(type: string, init?: { detail?: unknown }) {
    this.type = type;
    this.detail = init?.detail;
  }
}

notificationService.dispatchNotificationOpenCall(visiblePreviewCall, {
  window: {
    CustomEvent: ContractCustomEvent,
    dispatchEvent(event: { detail?: unknown; type: string }) {
      dispatchedEvents.push({
        detail: event.detail,
        type: event.type,
      });
      return true;
    },
    focus() {
      focused.push('focus');
    },
  },
});

assert.deepEqual(
  dispatchedEvents,
  [{
    detail: {
      callId: 'rtc-call-visible',
      conversationId: 'conversation-video-2',
      type: 'video',
    },
    type: 'sdkwork-im-pc:open-call',
  }],
  'Open-call dispatch must carry call id, conversation id, and call type so ChatLayout can show the correct CallOverlay.',
);
assert.deepEqual(focused, ['focus']);

const webNotificationClicks: Array<() => void> = [];
class ContractNotification {
  static permission = 'granted';
  onclick: (() => void) | null = null;

  constructor(public title: string, public options?: { body?: string; tag?: string }) {
    webNotificationClicks.push(() => this.onclick?.());
  }

  close() {
    return undefined;
  }
}

const webEvents: Array<{ detail?: unknown; type: string }> = [];
assert.equal(
  await notificationService.showSystemNotification(visiblePreviewCall, {
    Notification: ContractNotification,
    window: {
      CustomEvent: ContractCustomEvent,
      dispatchEvent(event: { detail?: unknown; type: string }) {
        webEvents.push({ detail: event.detail, type: event.type });
        return true;
      },
      focus() {
        return undefined;
      },
    },
  }),
  'web',
  'Web notification delivery must stay available for browser and WebView runtimes.',
);
webNotificationClicks[0]?.();
assert.deepEqual(
  webEvents,
  [{
    detail: {
      callId: 'rtc-call-visible',
      conversationId: 'conversation-video-2',
      type: 'video',
    },
    type: 'sdkwork-im-pc:open-call',
  }],
  'Clicking a call notification must open the call UI instead of only opening the conversation.',
);

const tauriInvocations: Array<{ args?: Record<string, unknown>; command: string }> = [];
assert.equal(
  await notificationService.restoreDesktopMainWindow({
    window: {
      __TAURI__: {
        core: {
          invoke: async (command: string, args?: Record<string, unknown>) => {
            tauriInvocations.push({ args, command });
            return null;
          },
        },
      },
    },
  }),
  true,
  'Desktop call wake-up must restore the main window through the narrow Tauri window-control command.',
);
assert.deepEqual(
  tauriInvocations.map((entry) => ({
    args: entry.args,
    command: entry.command,
  })),
  [{
    args: { action: 'show' },
    command: 'sdkwork_chat_pc_window_control',
  }],
);

tauriInvocations.length = 0;
await notificationService.showSystemNotification(visiblePreviewCall, {
  window: {
    __TAURI__: {
      core: {
        invoke: async (command: string, args?: Record<string, unknown>) => {
          tauriInvocations.push({ args, command });
          return null;
        },
      },
    },
  },
});

assert.equal(
  tauriInvocations.some((entry) => {
    const notificationPayload = entry.args?.notification as { callId?: string; kind?: string } | undefined;
    return entry.command === 'sdkwork_chat_pc_show_notification'
      && notificationPayload?.kind === 'call'
      && notificationPayload.callId === 'rtc-call-visible';
  }),
  true,
  'Native notification payloads must include call metadata for diagnostics and future OS action support.',
);

const chatLayoutSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/pages/ChatLayout.tsx',
  'utf8',
);
assert.match(
  chatLayoutSource,
  /buildIncomingCallNotification/,
  'ChatLayout incoming call flow must build a dedicated call notification.',
);
assert.match(
  chatLayoutSource,
  /restoreDesktopMainWindow\(\)/,
  'ChatLayout incoming call flow must restore the desktop window when a call arrives while hidden to tray.',
);
assert.match(
  chatLayoutSource,
  /sdkwork-im-pc:open-call/,
  'ChatLayout must listen for call notification and tray wake-up events.',
);
assert.match(
  chatLayoutSource,
  /openIncomingCallOverlay/u,
  'ChatLayout must route incoming snapshot, notification click, and tray show-call events through one open-call UI path.',
);
assert.match(
  chatLayoutSource,
  /openActiveCallOverlay/u,
  'ChatLayout tray wake-up must show the existing active call without forcing outgoing calls into incoming-call UI.',
);
assert.doesNotMatch(
  chatLayoutSource,
  /handleShowActiveCall[\s\S]*openIncomingCallOverlay/u,
  'Tray show-active-call must not reuse the incoming-call wake-up path because it loses the current call direction.',
);

const callOverlaySource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/components/CallOverlay.tsx',
  'utf8',
);
assert.match(
  callOverlaySource,
  /mode === 'outgoing' && !rtcSessionId/u,
  'CallOverlay must not start a new outgoing RTC session when reopening an existing active call from the tray.',
);

const appSource = readFileSync('./src/App.tsx', 'utf8');
assert.match(
  appSource,
  /sdkwork-im-pc:\/\/tray\/show-active-call/u,
  'The app-level Tauri tray bridge must listen for a stable show-active-call event.',
);
assert.match(
  appSource,
  /sdkwork-im-pc:show-active-call/u,
  'The tray bridge must dispatch a renderer event that ChatLayout can use to show the active call.',
);

const traySource = readFileSync(
  './packages/sdkwork-im-pc-desktop/src-tauri/src/tray.rs',
  'utf8',
);
assert.match(
  traySource,
  /TRAY_MENU_CALL_ID/u,
  'Desktop tray must define a stable menu id for showing the active call.',
);
assert.match(
  traySource,
  /TRAY_EVENT_SHOW_ACTIVE_CALL/u,
  'Desktop tray must define a stable event for showing the active call UI.',
);
assert.match(
  traySource,
  /show_main_window\(app\)[\s\S]*TRAY_EVENT_SHOW_ACTIVE_CALL/u,
  'Tray show-call action must restore the main window before telling the renderer to show the active call.',
);

console.log('sdkwork im pc call wake-up contract passed.');
