import type { ImH5ModuleId } from "./contracts";

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

export const DEFAULT_IM_H5_MODULES = ["chat", "notary", "orders"] as const satisfies readonly ImH5ModuleId[];

export const COMPOSABLE_IM_H5_MODULES = new Set<ImH5ModuleId>([
  ...DEFAULT_IM_H5_MODULES,
  "contacts",
  "drive",
]);

export const CONTRACT_PENDING_IM_H5_MODULES = new Set<ImH5ModuleId>(
  ALL_IM_H5_MODULES.filter((moduleId) => !COMPOSABLE_IM_H5_MODULES.has(moduleId)),
);
