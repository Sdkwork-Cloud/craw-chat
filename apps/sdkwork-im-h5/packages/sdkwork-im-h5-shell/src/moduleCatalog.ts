import type { ImH5ModuleId } from "./contracts";

export const ALL_IM_H5_MODULES = [
  "chat",
  "contacts",
  "user",
  "agents",
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
  "moments",
  "music",
  "channels",
  "recruitment",
  "membership",
] as const satisfies readonly ImH5ModuleId[];

/**
 * Full product composition restored to the original sdkwork-im-h5 UI: the
 * five main tabs (chat / agents / workspace / discover / me) plus every
 * capability surface from the original app.
 */
export const DEFAULT_IM_H5_MODULES = [
  "chat",
  "contacts",
  "user",
  "agents",
  "notary",
  "orders",
  "approval",
  "attendance",
  "calendar",
  "report",
  "recruitment",
  "enterprise",
  "meeting",
  "moments",
  "music",
  "knowledge",
  "drive",
  "voice",
  "videogen",
  "imagegen",
  "musicgen",
  "writing",
  "devices",
  "membership",
  "course",
  "community",
  "shop",
] as const satisfies readonly ImH5ModuleId[];

export const COMPOSABLE_IM_H5_MODULES = new Set<ImH5ModuleId>([...DEFAULT_IM_H5_MODULES]);

export const CONTRACT_PENDING_IM_H5_MODULES = new Set<ImH5ModuleId>(
  ALL_IM_H5_MODULES.filter((moduleId) => !COMPOSABLE_IM_H5_MODULES.has(moduleId)),
);
