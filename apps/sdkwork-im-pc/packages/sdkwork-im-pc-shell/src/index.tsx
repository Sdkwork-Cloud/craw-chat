export { AppShellFrame, FULLSCREEN_MODULE_TABS } from './AppShellFrame';
export type { AppShellFrameProps } from './AppShellFrame';
export { ModuleRenderHost } from './ModuleRenderHost';
export type { ModuleRenderHostProps } from './ModuleRenderHost';
export { isChatModule, isFullscreenModule, CHAT_MODULE_ID } from './moduleLayout';
export {
  ALL_APP_MODULES,
  ALWAYS_CONFIGURABLE_MODULES,
  COMMERCIAL_RUNTIME_MODULES,
  CONTRACT_PENDING_MODULES,
  DEFAULT_SIDEBAR_MODULES,
  MODULE_REQUIRED_PERMISSIONS,
  WORKSPACE_APP_TAB_MAP,
  isCommercialRuntimeModule,
  listCommercialRuntimeModules,
  resolveModuleRequiredPermission,
  resolveWorkspaceAppTab,
} from './moduleRegistry';
export type { AppModuleId } from './moduleRegistry';
export {
  isShellCapabilityModule,
  resolveLazyCapabilityModule,
  SHELL_CAPABILITY_MODULE_LOADERS,
} from './capabilityModuleLoaders';
export type { CapabilityModuleLoader, DriveOpenRequest } from './capabilityModuleLoaders';
export { LazyCapabilityModuleRenderer } from './LazyCapabilityModuleRenderer';
export type { LazyCapabilityModuleRendererProps } from './LazyCapabilityModuleRenderer';
