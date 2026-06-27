import type { Chat, Message } from '@sdkwork/im-pc-types';

export type NotificationPreviewMode = 'hidden' | 'sender-and-preview' | 'sender-only';

export interface NotificationSettings {
  notificationPreview: NotificationPreviewMode;
  notificationWhenFocused: boolean;
  notifyDesktop: boolean;
}

export interface IncomingMessageNotification {
  body: string;
  conversationId: string;
  icon?: string;
  kind: 'message';
  messageId: string;
  title: string;
}

export interface IncomingCallNotification {
  body: string;
  callId: string;
  conversationId: string;
  icon?: string;
  kind: 'call';
  title: string;
  type: 'voice' | 'video';
}

export type IncomingAppNotification = IncomingCallNotification | IncomingMessageNotification;

export interface NotificationTextProvider {
  callLabels?: Partial<Record<IncomingCallNotification['type'], string>>;
  hiddenBody: string;
  messageTypeLabels: Partial<Record<Message['type'], string>>;
  titleFallback: string;
}

export interface IncomingMessageNotificationDecision {
  activeConversationId?: string;
  chat: Pick<Chat, 'id' | 'isMuted'>;
  currentUserId?: string;
  message: Pick<Message, 'chatId' | 'senderId'>;
  settings: NotificationSettings;
  windowIsFocused: boolean;
}

export interface IncomingMessageNotificationInput {
  chat: Pick<Chat, 'avatar' | 'id' | 'name'>;
  message: Pick<Message, 'chatId' | 'content' | 'fileName' | 'id' | 'type'>;
  previewMode: NotificationPreviewMode;
  texts?: NotificationTextProvider;
}

export interface IncomingCallNotificationInput {
  callId: string;
  callerAvatar?: string;
  callerName: string;
  conversationId: string;
  previewMode: NotificationPreviewMode;
  texts?: NotificationTextProvider;
  type: IncomingCallNotification['type'];
}

export interface NotificationServiceDependencies {
  deliver(notification: IncomingMessageNotification): void | Promise<void>;
  getActiveConversationId(): string | undefined;
  getCurrentUserId(): string | undefined;
  getSettings(): NotificationSettings;
  getTexts?(): NotificationTextProvider;
  isWindowFocused(): boolean;
}

export interface SdkworkNotificationService {
  handleIncomingMessage(chat: Chat, message: Message): void;
}

export type SystemNotificationPermission = 'default' | 'denied' | 'granted' | 'unsupported';
export type SystemNotificationChannel = 'tauri-native' | 'unsupported' | 'web-notification';
export type SystemNotificationOperatingSystem = 'linux' | 'macos' | 'unknown' | 'web' | 'windows';
export type SystemNotificationDeliveryResult = 'native' | 'skipped' | 'web';

export interface SystemNotificationRuntime {
  channel: SystemNotificationChannel;
  operatingSystem: SystemNotificationOperatingSystem;
  permission: SystemNotificationPermission;
}

type BrowserNotificationPermission = 'default' | 'denied' | 'granted';

interface BrowserNotificationInstance {
  close(): void;
  onclick: (() => void) | null;
}

interface BrowserNotificationConstructor {
  new(title: string, options?: NotificationOptions): BrowserNotificationInstance;
  permission?: string;
  requestPermission?: () => Promise<string>;
}

type TauriInvoke = <T = unknown>(command: string, args?: Record<string, unknown>) => Promise<T>;

interface SystemNotificationWindow {
  __TAURI__?: {
    core?: {
      invoke?: TauriInvoke;
    };
  };
  CustomEvent?: typeof CustomEvent;
  dispatchEvent?: (event: Event) => boolean;
  focus?: () => void;
}

interface SystemNotificationNavigator {
  platform?: string;
  userAgent?: string;
}

export interface SystemNotificationEnvironment {
  Notification?: BrowserNotificationConstructor;
  navigator?: SystemNotificationNavigator;
  window?: SystemNotificationWindow;
}

const DEFAULT_APP_NOTIFICATION_TITLE = 'Sdkwork IM';
const DEFAULT_HIDDEN_NOTIFICATION_BODY = 'New message';
const MAX_NOTIFICATION_BODY_LENGTH = 96;
const NOTIFICATION_SOUND_VOLUME = 0.045;
const NOTIFICATION_SOUND_DURATION_MS = 120;

const DEFAULT_NOTIFICATION_TEXTS: NotificationTextProvider = {
  callLabels: {
    video: 'Incoming video call',
    voice: 'Incoming voice call',
  },
  hiddenBody: DEFAULT_HIDDEN_NOTIFICATION_BODY,
  messageTypeLabels: {
    applet: '[Mini app]',
    card: '[Card]',
    file: '[File]',
    image: '[Image]',
    link: '[Link]',
    music: '[Music]',
    system: '[System message]',
    video: '[Video]',
    video_call: '[Call]',
    voice: '[Voice message]',
  },
  titleFallback: DEFAULT_APP_NOTIFICATION_TITLE,
};

function isSameIdentifier(left: string | undefined, right: string | undefined): boolean {
  return Boolean(left && right && left.trim() === right.trim());
}

function truncateNotificationBody(value: string): string {
  const normalized = value.replace(/\s+/gu, ' ').trim();
  if (normalized.length <= MAX_NOTIFICATION_BODY_LENGTH) {
    return normalized;
  }
  return `${normalized.slice(0, MAX_NOTIFICATION_BODY_LENGTH - 1)}...`;
}

function resolveNotificationTexts(texts?: NotificationTextProvider): NotificationTextProvider {
  return {
    callLabels: {
      ...DEFAULT_NOTIFICATION_TEXTS.callLabels,
      ...texts?.callLabels,
    },
    hiddenBody: texts?.hiddenBody ?? DEFAULT_NOTIFICATION_TEXTS.hiddenBody,
    messageTypeLabels: {
      ...DEFAULT_NOTIFICATION_TEXTS.messageTypeLabels,
      ...texts?.messageTypeLabels,
    },
    titleFallback: texts?.titleFallback ?? DEFAULT_NOTIFICATION_TEXTS.titleFallback,
  };
}

function resolveSystemNotificationEnvironment(
  environment?: SystemNotificationEnvironment,
): SystemNotificationEnvironment {
  if (environment) {
    return environment;
  }

  const globalNotification = typeof Notification === 'undefined'
    ? undefined
    : Notification as unknown as BrowserNotificationConstructor;
  const globalWindow = typeof window === 'undefined'
    ? undefined
    : window as unknown as SystemNotificationWindow;
  const globalNavigator = typeof navigator === 'undefined'
    ? undefined
    : navigator;

  return {
    Notification: globalNotification,
    navigator: globalNavigator,
    window: globalWindow,
  };
}

function resolveTauriInvoke(environment?: SystemNotificationEnvironment): TauriInvoke | null {
  const resolvedEnvironment = resolveSystemNotificationEnvironment(environment);
  const invoke = resolvedEnvironment.window?.__TAURI__?.core?.invoke;
  return typeof invoke === 'function' ? invoke : null;
}

function normalizeSystemNotificationPermission(value: unknown): SystemNotificationPermission {
  if (value === 'granted' || value === 'denied' || value === 'default') {
    return value;
  }
  if (value === 'prompt' || value === 'prompt-with-rationale') {
    return 'default';
  }
  return 'unsupported';
}

function detectDesktopOperatingSystem(
  environment?: SystemNotificationEnvironment,
): Exclude<SystemNotificationOperatingSystem, 'web'> {
  const resolvedEnvironment = resolveSystemNotificationEnvironment(environment);
  const platform = resolvedEnvironment.navigator?.platform?.toLowerCase() ?? '';
  const userAgent = resolvedEnvironment.navigator?.userAgent?.toLowerCase() ?? '';
  const source = `${platform} ${userAgent}`;

  if (source.includes('win')) {
    return 'windows';
  }
  if (source.includes('mac')) {
    return 'macos';
  }
  if (source.includes('linux') || source.includes('x11')) {
    return 'linux';
  }
  return 'unknown';
}

function formatMessagePreview(
  message: Pick<Message, 'content' | 'fileName' | 'type'>,
  texts: NotificationTextProvider,
): string {
  const labels = texts.messageTypeLabels;
  switch (message.type) {
    case 'image':
      return labels.image ?? DEFAULT_NOTIFICATION_TEXTS.messageTypeLabels.image ?? '[Image]';
    case 'video':
      return labels.video ?? DEFAULT_NOTIFICATION_TEXTS.messageTypeLabels.video ?? '[Video]';
    case 'voice':
      return labels.voice ?? DEFAULT_NOTIFICATION_TEXTS.messageTypeLabels.voice ?? '[Voice message]';
    case 'file':
      return message.fileName
        ? `${labels.file ?? DEFAULT_NOTIFICATION_TEXTS.messageTypeLabels.file} ${message.fileName}`
        : labels.file ?? DEFAULT_NOTIFICATION_TEXTS.messageTypeLabels.file ?? '[File]';
    case 'video_call':
      return labels.video_call ?? DEFAULT_NOTIFICATION_TEXTS.messageTypeLabels.video_call ?? '[Call]';
    case 'card':
      return labels.card ?? DEFAULT_NOTIFICATION_TEXTS.messageTypeLabels.card ?? '[Card]';
    case 'applet':
      return labels.applet ?? DEFAULT_NOTIFICATION_TEXTS.messageTypeLabels.applet ?? '[Mini app]';
    case 'link':
      return message.content
        ? truncateNotificationBody(message.content)
        : labels.link ?? DEFAULT_NOTIFICATION_TEXTS.messageTypeLabels.link ?? '[Link]';
    case 'music':
      return message.content
        ? truncateNotificationBody(message.content)
        : labels.music ?? DEFAULT_NOTIFICATION_TEXTS.messageTypeLabels.music ?? '[Music]';
    case 'system':
      return message.content
        ? truncateNotificationBody(message.content)
        : labels.system ?? DEFAULT_NOTIFICATION_TEXTS.messageTypeLabels.system ?? '[System message]';
    default:
      return truncateNotificationBody(message.content || texts.hiddenBody);
  }
}

export function shouldNotifyIncomingMessage(decision: IncomingMessageNotificationDecision): boolean {
  if (!decision.settings.notifyDesktop) {
    return false;
  }
  if (decision.chat.isMuted) {
    return false;
  }
  if (isSameIdentifier(decision.currentUserId, decision.message.senderId)) {
    return false;
  }
  if (
    decision.windowIsFocused
    && !decision.settings.notificationWhenFocused
    && isSameIdentifier(decision.activeConversationId, decision.message.chatId)
  ) {
    return false;
  }
  return true;
}

export function resolveSystemNotificationRuntime(
  environment?: SystemNotificationEnvironment,
): SystemNotificationRuntime {
  const resolvedEnvironment = resolveSystemNotificationEnvironment(environment);
  if (resolveTauriInvoke(resolvedEnvironment)) {
    return {
      channel: 'tauri-native',
      operatingSystem: detectDesktopOperatingSystem(resolvedEnvironment),
      permission: 'granted',
    };
  }

  const browserNotification = resolvedEnvironment.Notification;
  if (browserNotification) {
    return {
      channel: 'web-notification',
      operatingSystem: 'web',
      permission: normalizeSystemNotificationPermission(browserNotification.permission),
    };
  }

  return {
    channel: 'unsupported',
    operatingSystem: 'unknown',
    permission: 'unsupported',
  };
}

export function buildIncomingMessageNotification(
  input: IncomingMessageNotificationInput,
): IncomingMessageNotification {
  const texts = resolveNotificationTexts(input.texts);
  if (input.previewMode === 'hidden') {
    return {
      body: texts.hiddenBody,
      conversationId: input.chat.id,
      kind: 'message',
      messageId: input.message.id,
      title: texts.titleFallback,
    };
  }

  return {
    body: input.previewMode === 'sender-only'
      ? texts.hiddenBody
      : formatMessagePreview(input.message, texts),
    conversationId: input.chat.id,
    icon: input.chat.avatar,
    kind: 'message',
    messageId: input.message.id,
    title: input.chat.name || texts.titleFallback,
  };
}

export function buildIncomingCallNotification(
  input: IncomingCallNotificationInput,
): IncomingCallNotification {
  const texts = resolveNotificationTexts(input.texts);
  const body = texts.callLabels?.[input.type]
    ?? DEFAULT_NOTIFICATION_TEXTS.callLabels?.[input.type]
    ?? 'Incoming call';

  if (input.previewMode === 'hidden') {
    return {
      body,
      callId: input.callId,
      conversationId: input.conversationId,
      icon: undefined,
      kind: 'call',
      title: texts.titleFallback,
      type: input.type,
    };
  }

  return {
    body,
    callId: input.callId,
    conversationId: input.conversationId,
    icon: input.callerAvatar,
    kind: 'call',
    title: input.callerName || texts.titleFallback,
    type: input.type,
  };
}

export function getSystemNotificationPermission(
  environment?: SystemNotificationEnvironment,
): SystemNotificationPermission {
  return resolveSystemNotificationRuntime(environment).permission;
}

export async function querySystemNotificationPermission(
  environment?: SystemNotificationEnvironment,
): Promise<SystemNotificationPermission> {
  const resolvedEnvironment = resolveSystemNotificationEnvironment(environment);
  const invoke = resolveTauriInvoke(resolvedEnvironment);
  if (invoke) {
    try {
      return normalizeSystemNotificationPermission(
        await invoke('sdkwork_chat_pc_notification_permission'),
      );
    } catch {
      return 'unsupported';
    }
  }

  return getSystemNotificationPermission(resolvedEnvironment);
}

export async function requestSystemNotificationPermission(
  environment?: SystemNotificationEnvironment,
): Promise<SystemNotificationPermission> {
  const resolvedEnvironment = resolveSystemNotificationEnvironment(environment);
  const invoke = resolveTauriInvoke(resolvedEnvironment);
  if (invoke) {
    try {
      return normalizeSystemNotificationPermission(
        await invoke('sdkwork_chat_pc_request_notification_permission'),
      );
    } catch {
      return 'unsupported';
    }
  }

  const browserNotification = resolvedEnvironment.Notification;
  if (!browserNotification) {
    return 'unsupported';
  }
  const currentPermission = normalizeSystemNotificationPermission(browserNotification.permission);
  if (currentPermission !== 'default') {
    return currentPermission;
  }
  if (typeof browserNotification.requestPermission !== 'function') {
    return currentPermission;
  }
  return normalizeSystemNotificationPermission(await browserNotification.requestPermission());
}

function dispatchNotificationOpenConversation(
  notification: IncomingMessageNotification,
  environment?: SystemNotificationEnvironment,
): void {
  const resolvedEnvironment = resolveSystemNotificationEnvironment(environment);
  const targetWindow = resolvedEnvironment.window;
  if (!targetWindow) {
    return;
  }

  targetWindow.focus?.();
  const CustomEventClass = targetWindow.CustomEvent
    ?? (typeof CustomEvent === 'undefined' ? undefined : CustomEvent);
  if (!CustomEventClass || typeof targetWindow.dispatchEvent !== 'function') {
    return;
  }
  targetWindow.dispatchEvent(new CustomEventClass('sdkwork-im-pc:open-conversation', {
    detail: { conversationId: notification.conversationId },
  }));
}

export function dispatchNotificationOpenCall(
  notification: IncomingCallNotification,
  environment?: SystemNotificationEnvironment,
): void {
  const resolvedEnvironment = resolveSystemNotificationEnvironment(environment);
  const targetWindow = resolvedEnvironment.window;
  if (!targetWindow) {
    return;
  }

  targetWindow.focus?.();
  const CustomEventClass = targetWindow.CustomEvent
    ?? (typeof CustomEvent === 'undefined' ? undefined : CustomEvent);
  if (!CustomEventClass || typeof targetWindow.dispatchEvent !== 'function') {
    return;
  }
  targetWindow.dispatchEvent(new CustomEventClass('sdkwork-im-pc:open-call', {
    detail: {
      callId: notification.callId,
      conversationId: notification.conversationId,
      type: notification.type,
    },
  }));
}

export async function restoreDesktopMainWindow(
  environment?: SystemNotificationEnvironment,
): Promise<boolean> {
  const invoke = resolveTauriInvoke(resolveSystemNotificationEnvironment(environment));
  if (!invoke) {
    return false;
  }

  try {
    await invoke('sdkwork_chat_pc_window_control', {
      action: 'show',
    });
    return true;
  } catch {
    return false;
  }
}

function dispatchNotificationOpen(
  notification: IncomingAppNotification,
  environment?: SystemNotificationEnvironment,
): void {
  if (notification.kind === 'call') {
    dispatchNotificationOpenCall(notification, environment);
    return;
  }
  dispatchNotificationOpenConversation(notification, environment);
}

function resolveSystemNotificationTag(notification: IncomingAppNotification): string {
  return notification.kind === 'call'
    ? `sdkwork-chat:call:${notification.conversationId}:${notification.callId}`
    : `sdkwork-chat:message:${notification.conversationId}:${notification.messageId}`;
}

function showBrowserSystemNotificationWithResult(
  notification: IncomingAppNotification,
  environment?: SystemNotificationEnvironment,
): boolean {
  const resolvedEnvironment = resolveSystemNotificationEnvironment(environment);
  const browserNotification = resolvedEnvironment.Notification;
  if (
    !browserNotification
    || normalizeSystemNotificationPermission(browserNotification.permission) !== 'granted'
  ) {
    return false;
  }

  const systemNotification = new browserNotification(notification.title, {
    body: notification.body,
    icon: notification.icon,
    tag: resolveSystemNotificationTag(notification),
  });
  systemNotification.onclick = () => {
    dispatchNotificationOpen(notification, resolvedEnvironment);
    systemNotification.close();
  };
  return true;
}

export function showBrowserSystemNotification(notification: IncomingAppNotification): void {
  showBrowserSystemNotificationWithResult(notification);
}

export async function showSystemNotification(
  notification: IncomingAppNotification,
  environment?: SystemNotificationEnvironment,
): Promise<SystemNotificationDeliveryResult> {
  const resolvedEnvironment = resolveSystemNotificationEnvironment(environment);
  const invoke = resolveTauriInvoke(resolvedEnvironment);
  if (invoke) {
    try {
      await invoke('sdkwork_chat_pc_show_notification', {
        notification: {
          body: notification.body,
          callId: notification.kind === 'call' ? notification.callId : undefined,
          conversationId: notification.conversationId,
          icon: notification.icon,
          kind: notification.kind,
          messageId: notification.kind === 'message' ? notification.messageId : undefined,
          title: notification.title,
          type: notification.kind === 'call' ? notification.type : undefined,
        },
      });
      return 'native';
    } catch {
      if (showBrowserSystemNotificationWithResult(notification, resolvedEnvironment)) {
        return 'web';
      }
      return 'skipped';
    }
  }

  return showBrowserSystemNotificationWithResult(notification, resolvedEnvironment)
    ? 'web'
    : 'skipped';
}

export function playMessageNotificationSound(): void {
  const AudioContextClass = globalThis.AudioContext
    ?? (globalThis as typeof globalThis & { webkitAudioContext?: typeof AudioContext }).webkitAudioContext;
  if (!AudioContextClass) {
    return;
  }

  try {
    const audioContext = new AudioContextClass();
    const oscillator = audioContext.createOscillator();
    const gainNode = audioContext.createGain();
    oscillator.type = 'sine';
    oscillator.frequency.value = 880;
    gainNode.gain.value = NOTIFICATION_SOUND_VOLUME;
    oscillator.connect(gainNode);
    gainNode.connect(audioContext.destination);
    oscillator.start();
    oscillator.stop(audioContext.currentTime + NOTIFICATION_SOUND_DURATION_MS / 1000);
    oscillator.onended = () => {
      void audioContext.close().catch(() => undefined);
    };
  } catch {
    // Browsers and WebViews can block audio until the user interacts with the app.
  }
}

class SdkworkNotificationServiceImpl implements SdkworkNotificationService {
  private readonly deliveredMessageIds = new Set<string>();

  constructor(private readonly dependencies: NotificationServiceDependencies) {}

  handleIncomingMessage(chat: Chat, message: Message): void {
    if (this.deliveredMessageIds.has(message.id)) {
      return;
    }
    const settings = this.dependencies.getSettings();
    if (!shouldNotifyIncomingMessage({
      activeConversationId: this.dependencies.getActiveConversationId(),
      chat,
      currentUserId: this.dependencies.getCurrentUserId(),
      message,
      settings,
      windowIsFocused: this.dependencies.isWindowFocused(),
    })) {
      return;
    }

    this.deliveredMessageIds.add(message.id);
    void this.dependencies.deliver(buildIncomingMessageNotification({
      chat,
      message,
      previewMode: settings.notificationPreview,
      texts: this.dependencies.getTexts?.(),
    }));
  }
}

export function createSdkworkNotificationService(
  dependencies: NotificationServiceDependencies,
): SdkworkNotificationService {
  return new SdkworkNotificationServiceImpl(dependencies);
}
