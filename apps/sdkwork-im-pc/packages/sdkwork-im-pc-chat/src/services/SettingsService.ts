import {
  getAiotAppSdkClientWithSession,
  type SdkworkAiotAppClient,
} from "@sdkwork/im-pc-core/sdk/aiotAppSdkClient";
import {
  getAppSdkClientWithSession,
  type SdkworkImAppClient,
} from "@sdkwork/im-pc-core/sdk/appSdkClient";
import { resolveSdkworkChatPcClientId } from "./ClientIdentityService";
import {
  ALL_APP_MODULES,
  ALWAYS_CONFIGURABLE_MODULES,
  DEFAULT_SIDEBAR_MODULES,
  isCommercialRuntimeModule,
  listCommercialRuntimeModules,
} from "@sdkwork/im-pc-shell/moduleRegistry";
import {
  applyHostAppearanceTheme,
  readPersistedSettingsRecord,
  SDKWORK_IM_PC_SETTINGS_CHANGED_EVENT,
  SDKWORK_IM_PC_SETTINGS_STORAGE_KEY,
} from "@sdkwork/im-pc-commons";

export {
  ALL_APP_MODULES,
  DEFAULT_SIDEBAR_MODULES,
  ALWAYS_CONFIGURABLE_MODULES,
  listCommercialRuntimeModules,
  isCommercialRuntimeModule,
};

const SUPPORTED_LANGUAGES = new Set(["zh-CN", "en-US"]);
const NOTIFICATION_PREVIEW_MODES = new Set([
  "sender-and-preview",
  "sender-only",
  "hidden",
]);
const SETTINGS_CHANGED_EVENT = SDKWORK_IM_PC_SETTINGS_CHANGED_EVENT;
const LEGACY_AUTOFILLED_SIDEBAR_MODULES = [
  "chat",
  "workspace",
  "orders",
  "shop",
  "calendar",
  "notary",
  "knowledge",
  "enterprise",
  "devices",
  "community",
  "voice",
  "agent",
  "course",
  "contacts",
  "favorites",
];

function normalizeSettingsLanguage(lang: unknown) {
  return typeof lang === "string" && SUPPORTED_LANGUAGES.has(lang)
    ? lang
    : "zh-CN";
}

function normalizeNotificationPreviewMode(value: unknown): AppSettings["notificationPreview"] {
  return typeof value === "string" && NOTIFICATION_PREVIEW_MODES.has(value)
    ? value as AppSettings["notificationPreview"]
    : "sender-and-preview";
}

function getLocalStorage(): Storage | undefined {
  return typeof localStorage === "undefined" ? undefined : localStorage;
}

function notifySettingsChanged(settings: AppSettings) {
  if (typeof window === "undefined") return;
  window.dispatchEvent(new CustomEvent(SETTINGS_CHANGED_EVENT, {
    detail: { settings },
  }));
}

export interface AppSettings {
  lang: string;
  autoStart: boolean;
  notifySound: boolean;
  notifyDesktop: boolean;
  notifySystem: boolean;
  notificationPreview: "hidden" | "sender-and-preview" | "sender-only";
  notificationWhenFocused: boolean;
  privacyShowOnline: boolean;
  theme: "system" | "dark" | "light";
  sidebarModules: string[];
}

export interface DeviceInfo {
  id: string;
  name: string;
  time: string;
  isCurrent?: boolean;
}

export interface SettingsService {
  getSettings(): Promise<AppSettings>;
  updateSettings(updates: Partial<AppSettings>): Promise<AppSettings>;
  getDevices(): Promise<DeviceInfo[]>;
  removeDevice(deviceId: string): Promise<void>;
  clearCache(): Promise<void>;
  getServerModules(): Promise<string[]>;
}

type RecordLike = Record<string, unknown>;

function isRecord(value: unknown): value is RecordLike {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function parseJsonRecord(value: string | undefined): RecordLike {
  if (!value) return {};
  try {
    const parsed = JSON.parse(value) as unknown;
    return isRecord(parsed) ? parsed : {};
  } catch {
    return {};
  }
}

function pickString(value: unknown): string | undefined {
  return typeof value === "string" && value.trim().length > 0
    ? value.trim()
    : undefined;
}

function pickStringArray(value: unknown): string[] {
  if (!Array.isArray(value)) return [];
  return value
    .map((item) => pickString(item))
    .filter((item): item is string => Boolean(item));
}

function uniqueKnownModules(modules: string[]): string[] {
  const known = new Set<string>(ALL_APP_MODULES as readonly string[]);
  const seen = new Set<string>();
  const result: string[] = [];
  for (const moduleId of modules) {
    if (!known.has(moduleId) || seen.has(moduleId) || !isCommercialRuntimeModule(moduleId)) {
      continue;
    }
    seen.add(moduleId);
    result.push(moduleId);
  }
  return result;
}

function hasSameModuleSet(left: string[], right: string[]): boolean {
  if (left.length !== right.length) return false;
  const leftSet = new Set(left);
  return right.every((moduleId) => leftSet.has(moduleId));
}

function normalizeSidebarModules(
  value: unknown,
  options: { migrateLegacyAllModules?: boolean } = {},
): string[] {
  const modules = uniqueKnownModules(pickStringArray(value));
  if (modules.length === 0) {
    return [...DEFAULT_SIDEBAR_MODULES];
  }
  if (options.migrateLegacyAllModules && (
    hasSameModuleSet(modules, [...ALL_APP_MODULES]) ||
    hasSameModuleSet(modules, LEGACY_AUTOFILLED_SIDEBAR_MODULES)
  )) {
    return [...DEFAULT_SIDEBAR_MODULES];
  }
  return modules.includes("chat") ? modules : ["chat", ...modules];
}

function collectModules(snapshot: unknown): string[] {
  if (!isRecord(snapshot)) return [];
  for (const key of [
    "enabledModules",
    "enabled_modules",
    "sidebarModules",
    "sidebar_modules",
    "modules",
  ]) {
    const modules = pickStringArray(snapshot[key]);
    if (modules.length > 0) return modules;
    if (isRecord(snapshot[key])) {
      const nestedModules = pickStringArray((snapshot[key] as RecordLike).items);
      if (nestedModules.length > 0) return nestedModules;
    }
  }
  return [];
}

function collectDeviceInfo(...states: RecordLike[]): DeviceInfo[] {
  const devices = new Map<string, DeviceInfo>();
  for (const state of states) {
    const candidates = [
      state.loginDevices,
      state.login_devices,
      state.devices,
      isRecord(state.sessions) ? state.sessions.items : undefined,
    ];
    for (const candidate of candidates) {
      if (!Array.isArray(candidate)) continue;
      for (const item of candidate) {
        if (!isRecord(item)) continue;
        const id = pickString(item.id)
          ?? pickString(item.deviceId)
          ?? pickString(item.device_id);
        if (!id) continue;
        const isCurrent = item.isCurrent === true || item.current === true;
        devices.set(id, {
          id,
          name: pickString(item.name)
            ?? pickString(item.deviceName)
            ?? pickString(item.device_name)
            ?? id,
          time: pickString(item.time)
            ?? pickString(item.lastSeenAt)
            ?? pickString(item.last_seen_at)
            ?? pickString(item.updatedAt)
            ?? "",
          ...(isCurrent ? { isCurrent } : {}),
        });
      }
    }
  }
  return Array.from(devices.values());
}

function collectTwinStateRecords(twin: unknown): RecordLike[] {
  const envelope = isRecord(twin) && "data" in twin ? twin.data : twin;
  const record = isRecord(envelope) ? envelope : {};
  return [
    record.reported,
    record.reportedState,
    record.reported_state,
    parseJsonRecord(pickString(record.reportedStateJson)),
    parseJsonRecord(pickString(record.reported_state_json)),
    record.desired,
    record.desiredState,
    record.desired_state,
    parseJsonRecord(pickString(record.desiredStateJson)),
    parseJsonRecord(pickString(record.desired_state_json)),
  ].filter(isRecord);
}

class SdkworkSettingsService implements SettingsService {
  private get defaultSettings(): AppSettings {
    return {
      lang: "zh-CN",
      autoStart: true,
      notifySound: true,
      notifyDesktop: true,
      notifySystem: false,
      notificationPreview: "sender-and-preview",
      notificationWhenFocused: false,
      privacyShowOnline: true,
      theme: "system",
      sidebarModules: [...DEFAULT_SIDEBAR_MODULES],
    };
  }

  private settings: AppSettings = this.loadSettings();

  constructor(
    private readonly getClient: () => SdkworkImAppClient = getAppSdkClientWithSession,
    private readonly getAiotClient: () => SdkworkAiotAppClient = getAiotAppSdkClientWithSession,
    private readonly resolveClientId: () => string = resolveSdkworkChatPcClientId,
  ) {
    setTimeout(() => this.initTheme(), 0);
  }

  private loadSettings(): AppSettings {
    try {
      const parsed = readPersistedSettingsRecord();
      if (parsed) {
        return {
          ...this.defaultSettings,
          ...parsed,
          lang: normalizeSettingsLanguage(parsed.lang),
          notificationPreview: normalizeNotificationPreviewMode(parsed.notificationPreview),
          sidebarModules: normalizeSidebarModules(parsed.sidebarModules, {
            migrateLegacyAllModules: true,
          }),
        };
      }
    } catch {}
    return this.defaultSettings;
  }

  private initTheme() {
    this.applyTheme(this.settings.theme);
  }

  applyTheme(theme: "system" | "dark" | "light") {
    applyHostAppearanceTheme(theme);
  }

  async getSettings(): Promise<AppSettings> {
    return { ...this.settings };
  }

  async updateSettings(updates: Partial<AppSettings>): Promise<AppSettings> {
    const sidebarModules = updates.sidebarModules === undefined
      ? this.settings.sidebarModules
      : normalizeSidebarModules(updates.sidebarModules);
    this.settings = {
      ...this.settings,
      ...updates,
      lang: normalizeSettingsLanguage(updates.lang ?? this.settings.lang),
      notificationPreview: normalizeNotificationPreviewMode(
        updates.notificationPreview ?? this.settings.notificationPreview,
      ),
      sidebarModules,
    };
    try {
      getLocalStorage()?.setItem(
        SDKWORK_IM_PC_SETTINGS_STORAGE_KEY,
        JSON.stringify(this.settings),
      );
    } catch {}
    this.applyTheme(this.settings.theme);
    notifySettingsChanged(this.settings);
    return { ...this.settings };
  }

  async getDevices(): Promise<DeviceInfo[]> {
    try {
      const twin = await this.getAiotClient().iot.devices.twin.retrieve(
        this.resolveClientId(),
      );
      return collectDeviceInfo(...collectTwinStateRecords(twin));
    } catch {
      return [];
    }
  }

  async removeDevice(deviceId: string): Promise<void> {
    const normalizedDeviceId = deviceId.trim();
    if (!normalizedDeviceId) {
      throw new Error("Device id is required");
    }
    await this.getAiotClient().iot.devices.commands.create(
      this.resolveClientId(),
      {
        capabilityName: "login-sessions",
        commandName: "disable-login-device",
        payload: {
          disabledLoginDeviceIds: [normalizedDeviceId],
        },
      },
      { idempotencyKey: `disable-login-device:${normalizedDeviceId}` },
    );
  }

  async clearCache(): Promise<void> {
    if (typeof caches === "undefined") {
      return;
    }
    const keys = await caches.keys();
    await Promise.all(keys.map((key) => caches.delete(key)));
  }

  async getServerModules(): Promise<string[]> {
    try {
      const modules = uniqueKnownModules(
        collectModules(await this.getClient().portal.home.retrieve()),
      );
      if (modules.length > 0) {
        return modules;
      }
    } catch (error) {
      console.warn("Failed to fetch server modules config through app SDK, using defaults", error);
    }
    return listCommercialRuntimeModules();
  }
}

export function createSdkworkSettingsService(
  getClient?: () => SdkworkImAppClient,
  getAiotClient?: () => SdkworkAiotAppClient,
  resolveClientId?: () => string,
): SettingsService {
  return new SdkworkSettingsService(getClient, getAiotClient, resolveClientId);
}

export const settingsService = createSdkworkSettingsService();
