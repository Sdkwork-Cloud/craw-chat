import type { ImH5CapabilityModule, ImH5ModuleId } from "./contracts";
import { chatModule } from "./modules/chatModule";
import { notaryModule } from "./modules/notaryModule";

export const ALL_IM_H5_MODULES = [
  "chat",
  "contacts",
  "knowledge",
  "drive",
  "orders",
  "shop",
  "calendar",
  "notary",
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
  "musicgen",
  "writing",
  "meeting",
  "channels",
  "recruitment",
  "membership",
] as const satisfies readonly ImH5ModuleId[];

export const DEFAULT_IM_H5_MODULES = ["chat", "notary"] as const satisfies readonly ImH5ModuleId[];

export const COMPOSABLE_IM_H5_MODULES = new Set<ImH5ModuleId>(DEFAULT_IM_H5_MODULES);

export const CONTRACT_PENDING_IM_H5_MODULES = new Set<ImH5ModuleId>(
  ALL_IM_H5_MODULES.filter((moduleId) => !COMPOSABLE_IM_H5_MODULES.has(moduleId)),
);

export const BUILTIN_IM_H5_MODULE_REGISTRY: Readonly<Partial<Record<ImH5ModuleId, ImH5CapabilityModule>>> = {
  chat: chatModule,
  notary: notaryModule,
};

export function resolveImH5ShellModules(
  moduleIds: readonly ImH5ModuleId[] = DEFAULT_IM_H5_MODULES,
  registry: Readonly<Partial<Record<ImH5ModuleId, ImH5CapabilityModule>>> = BUILTIN_IM_H5_MODULE_REGISTRY,
): ImH5CapabilityModule[] {
  return moduleIds.flatMap((moduleId) => {
    const module = registry[moduleId];
    return module ? [module] : [];
  });
}
