const COMMERCIAL_MODULES = new Set([
  'notary',
  'drive',
  'knowledge',
  'community',
  'voice',
  'shop',
  'orders',
]);

export const WORKSPACE_APP_TAB_MAP: Record<string, string> = Object.fromEntries(
  Array.from(COMMERCIAL_MODULES, (moduleId) => [moduleId, moduleId]),
);

export function isCommercialRuntimeModule(moduleId: string): boolean {
  return COMMERCIAL_MODULES.has(moduleId);
}
