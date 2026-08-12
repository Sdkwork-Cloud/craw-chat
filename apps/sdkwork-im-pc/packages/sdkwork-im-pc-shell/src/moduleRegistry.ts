/**
 * Canonical PC sidebar module catalog.
 * Capability packages register views; the shell owns module identity and defaults.
 *
 * `COMMERCIAL_RUNTIME_MODULES` is the only set eligible for sidebar navigation,
 * lazy module rendering, workspace launcher, and settings module picker.
 * `CONTRACT_PENDING_MODULES` remain in the catalog for future sibling SDK wiring.
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
 * Modules with verified read/write SDK contracts for commercial runtime navigation.
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
  "community",
  "shop",
  "orders",
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

/**
 * Optional permission codes required to mount a commercial module's routes.
 *
 * Absent entry means the module is available to any authenticated user.
 * Permission matching follows the backend `AppContext::has_permission`
 * semantics (`*`, `tenant.admin`, exact code, and `<prefix>.*` wildcards all
 * grant access), resolved via `@sdkwork/im-pc-core` `hasAppSdkPermission`.
 *
 * The commercial sidebar modules (chat, contacts, drive, …) are user-facing
 * and therefore intentionally unlisted: they require only authentication.
 * Entries below gate modules that surface privileged operator/admin data and
 * must not mount when the session token lacks the declared permission claim.
 */
export const MODULE_REQUIRED_PERMISSIONS: Partial<Record<AppModuleId, string>> = {
  // Commercial sidebar modules are user-facing; no admin/control permission
  // is required beyond authentication. Add entries here when a commercial
  // module begins surfacing privileged operator/admin data.
};

export function resolveModuleRequiredPermission(
  moduleId: string,
): string | undefined {
  return MODULE_REQUIRED_PERMISSIONS[moduleId as AppModuleId];
}

export const WORKSPACE_APP_TAB_MAP: Record<string, AppModuleId> = Object.fromEntries(
  Object.entries({
    notary: 'notary',
    drive: 'drive',
    knowledge: 'knowledge',
    community: 'community',
    voice: 'voice',
    shop: 'shop',
    orders: 'orders',
  }).filter(([appId]) => COMMERCIAL_RUNTIME_MODULES.has(appId as AppModuleId)),
) as Record<string, AppModuleId>;

export function resolveWorkspaceAppTab(appId: string): AppModuleId | undefined {
  return WORKSPACE_APP_TAB_MAP[appId];
}
