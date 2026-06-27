/**
 * Canonical PC sidebar module catalog.
 * Capability packages register views; the shell owns module identity and defaults.
 */

export const ALL_APP_MODULES = [
  "chat",
  "workspace",
  "contacts",
  "knowledge",
  "drive",
  "agent",
  "favorites",
  "orders",
  "shop",
  "calendar",
  "notary",
  "mail",
  "approval",
  "report",
  "attendance",
  "enterprise",
  "devices",
  "community",
  "voice",
  "course",
  "videogen",
  "imagegen",
  "voicegen",
  "musicgen",
  "writing",
] as const;

export type AppModuleId = (typeof ALL_APP_MODULES)[number];

export const DEFAULT_SIDEBAR_MODULES: AppModuleId[] = [
  "chat",
  "workspace",
  "contacts",
  "knowledge",
  "drive",
  "agent",
  "favorites",
];

/**
 * Modules with verified app/backend SDK contracts for commercial runtime.
 * Contract-pending modules stay in ALL_APP_MODULES for future enablement but
 * must not appear in production navigation until their SDK surfaces ship.
 */
export const COMMERCIAL_RUNTIME_MODULES = new Set<AppModuleId>([
  "chat",
  "workspace",
  "contacts",
  "knowledge",
  "drive",
  "agent",
  "favorites",
  "notary",
  "voice",
  "shop",
  "orders",
  "mail",
  "community",
  "course",
  "devices",
  "enterprise",
]);

export const CONTRACT_PENDING_MODULES = new Set<AppModuleId>(
  ALL_APP_MODULES.filter((moduleId) => !COMMERCIAL_RUNTIME_MODULES.has(moduleId)),
);

export function isCommercialRuntimeModule(
  moduleId: string,
): moduleId is AppModuleId {
  return COMMERCIAL_RUNTIME_MODULES.has(moduleId as AppModuleId);
}

export function listCommercialRuntimeModules(): AppModuleId[] {
  return ALL_APP_MODULES.filter((moduleId) =>
    COMMERCIAL_RUNTIME_MODULES.has(moduleId),
  );
}

export const ALWAYS_CONFIGURABLE_MODULES = new Set<AppModuleId>(["notary"]);

/** Maps workspace launcher app ids to sidebar module tabs. */
export const WORKSPACE_APP_TAB_MAP: Record<string, AppModuleId> = {
  notary: "notary",
  mail: "mail",
  drive: "drive",
  calendar: "calendar",
  approval: "approval",
  report: "report",
  attendance: "attendance",
  knowledge: "knowledge",
  devices: "devices",
  community: "community",
  videogen: "videogen",
  imagegen: "imagegen",
  voicegen: "voicegen",
  musicgen: "musicgen",
  writing: "writing",
};

export function resolveWorkspaceAppTab(appId: string): AppModuleId | undefined {
  return WORKSPACE_APP_TAB_MAP[appId];
}
