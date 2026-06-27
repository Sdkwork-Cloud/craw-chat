import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';
import type * as NotificationServiceModule from '../packages/sdkwork-im-pc-chat/src/services/NotificationService.ts';

type NotificationServiceExports = typeof NotificationServiceModule & {
  getSystemNotificationPermission: () => NotificationServiceModule.SystemNotificationPermission;
  querySystemNotificationPermission: (environment?: unknown) => Promise<NotificationServiceModule.SystemNotificationPermission>;
  requestSystemNotificationPermission: () => Promise<NotificationServiceModule.SystemNotificationPermission>;
  resolveSystemNotificationRuntime: (environment: unknown) => {
    channel: 'tauri-native' | 'web-notification' | 'unsupported';
    operatingSystem: 'windows' | 'macos' | 'linux' | 'web' | 'unknown';
    permission: NotificationServiceModule.SystemNotificationPermission;
  };
  showSystemNotification: (
    notification: NotificationServiceModule.IncomingMessageNotification,
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
  typeof notificationService.resolveSystemNotificationRuntime,
  'function',
  'Notification service must expose a deterministic runtime resolver for desktop and web system notifications.',
);
assert.equal(
  typeof notificationService.showSystemNotification,
  'function',
  'Notification service must expose one guarded system notification entrypoint instead of scattering OS-specific logic in UI.',
);
assert.equal(
  typeof notificationService.querySystemNotificationPermission,
  'function',
  'Settings center must be able to asynchronously query native OS notification permission on desktop.',
);

const tauriInvocations: Array<{ args?: Record<string, unknown>; command: string }> = [];
const tauriEnvironment = {
  navigator: {
    platform: 'Win32',
    userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)',
  },
  window: {
    __TAURI__: {
      core: {
        invoke: async (command: string, args?: Record<string, unknown>) => {
          tauriInvocations.push({ args, command });
          if (command === 'sdkwork_chat_pc_notification_permission') {
            return 'granted';
          }
          return null;
        },
      },
    },
  },
};

assert.deepEqual(
  notificationService.resolveSystemNotificationRuntime(tauriEnvironment),
  {
    channel: 'tauri-native',
    operatingSystem: 'windows',
    permission: 'granted',
  },
  'Windows desktop runtime should prefer the native Tauri notification bridge over WebView browser notifications.',
);

assert.deepEqual(
  notificationService.resolveSystemNotificationRuntime({
    navigator: {
      platform: 'MacIntel',
      userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 14_0)',
    },
    window: tauriEnvironment.window,
  }).operatingSystem,
  'macos',
  'macOS desktop runtime should be detected explicitly for notification diagnostics and permission guidance.',
);

assert.deepEqual(
  notificationService.resolveSystemNotificationRuntime({
    navigator: {
      platform: 'Linux x86_64',
      userAgent: 'Mozilla/5.0 (X11; Linux x86_64)',
    },
    window: tauriEnvironment.window,
  }).operatingSystem,
  'linux',
  'Linux desktop runtime should be detected explicitly for notification diagnostics and fallback behavior.',
);

const webNotifications: Array<{ body?: string; tag?: string; title: string }> = [];
class ContractNotification {
  static permission = 'granted';
  onclick: (() => void) | null = null;

  constructor(title: string, options?: { body?: string; icon?: string; tag?: string }) {
    webNotifications.push({ body: options?.body, tag: options?.tag, title });
  }

  close() {
    return undefined;
  }
}

assert.deepEqual(
  notificationService.resolveSystemNotificationRuntime({
    Notification: ContractNotification,
    navigator: {
      platform: 'Linux x86_64',
      userAgent: 'Mozilla/5.0 (X11; Linux x86_64)',
    },
    window: {
      dispatchEvent() {
        return true;
      },
      focus() {
        return undefined;
      },
    },
  }),
  {
    channel: 'web-notification',
    operatingSystem: 'web',
    permission: 'granted',
  },
  'Web runtime should use browser notifications when permission is granted and no native host bridge exists.',
);

const baseNotification = {
  body: 'Contract message',
  conversationId: 'conversation.platform',
  kind: 'message',
  messageId: 'message.platform',
  title: 'Platform Contract',
} as const;

assert.equal(
  await notificationService.showSystemNotification(baseNotification, tauriEnvironment),
  'native',
  'Desktop system notifications should use the native host command.',
);
assert.equal(
  tauriInvocations.some((entry) => entry.command === 'sdkwork_chat_pc_show_notification'),
  true,
  'Desktop system notifications must invoke the narrow Tauri notification command.',
);
assert.equal(
  await notificationService.querySystemNotificationPermission(tauriEnvironment),
  'granted',
  'Desktop settings must query the native host permission command instead of relying only on a synchronous browser API snapshot.',
);
assert.equal(
  tauriInvocations.some((entry) => entry.command === 'sdkwork_chat_pc_notification_permission'),
  true,
  'Desktop permission refresh must invoke the narrow Tauri permission command.',
);

assert.equal(
  await notificationService.showSystemNotification(baseNotification, {
    Notification: ContractNotification,
    navigator: {
      platform: 'Win32',
      userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)',
    },
    window: {
      dispatchEvent() {
        return true;
      },
      focus() {
        return undefined;
      },
    },
  }),
  'web',
  'Browser system notifications should remain available for the web version.',
);
assert.equal(webNotifications.length, 1);

assert.equal(
  await notificationService.showSystemNotification(baseNotification, {
    Notification: undefined,
    navigator: {
      platform: 'Unknown',
      userAgent: 'test',
    },
    window: {},
  }),
  'skipped',
  'Unsupported or denied runtimes must fail closed so the in-app notification center remains the baseline.',
);

const chatLayoutSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/pages/ChatLayout.tsx',
  'utf8',
);
assert.match(
  chatLayoutSource,
  /showSystemNotification/,
  'ChatLayout must call the unified cross-platform system notification adapter.',
);
assert.doesNotMatch(
  chatLayoutSource,
  /showBrowserSystemNotification/,
  'ChatLayout should not call the browser-only notification helper directly.',
);

const settingsModalSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/components/SettingsModal.tsx',
  'utf8',
);
assert.match(
  settingsModalSource,
  /querySystemNotificationPermission/,
  'SettingsModal must refresh system notification permission through the asynchronous cross-platform adapter.',
);

const desktopManifestSource = readFileSync(
  './packages/sdkwork-im-pc-desktop/src-tauri/Cargo.toml',
  'utf8',
);
assert.match(
  desktopManifestSource,
  /tauri-plugin-notification/,
  'Desktop host must include the Tauri notification plugin for Windows, macOS, and Linux native notifications.',
);

const desktopLibSource = readFileSync(
  './packages/sdkwork-im-pc-desktop/src-tauri/src/lib.rs',
  'utf8',
);
assert.match(
  desktopLibSource,
  /sdkwork_chat_pc_show_notification/,
  'Desktop host must register a narrow notification command.',
);

const desktopNotificationSource = readFileSync(
  './packages/sdkwork-im-pc-desktop/src-tauri/src/notification.rs',
  'utf8',
);
assert.match(
  desktopNotificationSource,
  /starts_with\("https:\/\/"\)/,
  'Native desktop notifications should not pass remote avatar URLs as OS notification icons on Windows, macOS, or Linux.',
);

const desktopCapabilitySource = readFileSync(
  './packages/sdkwork-im-pc-desktop/src-tauri/capabilities/default.json',
  'utf8',
);
assert.doesNotMatch(
  desktopCapabilitySource,
  /notification:default/,
  'Desktop notification permissions should stay least-privilege instead of enabling the whole notification plugin command set.',
);
assert.match(
  desktopCapabilitySource,
  /notification:allow-notify/,
  'Desktop notification permissions must allow only the notification commands needed by the native host bridge.',
);

console.log('sdkwork im pc notification platform compatibility contract passed.');
