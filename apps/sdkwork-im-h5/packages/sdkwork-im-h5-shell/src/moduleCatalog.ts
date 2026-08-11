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
 * Default product composition: the five main tabs (chat / agents / workspace /
 * discover / me) plus every capability surface with a real owner SDK or an
 * approved canonical owner-repo UI composed in this app root.
 *
 * Fail-closed rule (PRD): capabilities without an owner SDK / end-to-end
 * evidence must not be registered by default. approval / attendance /
 * calendar / report / recruitment / enterprise were audited as pure
 * localStorage mocks with no backend SDK — they are removed from the default
 * composition. Their route entries stay in the shell registry for opt-in via
 * `VITE_SDKWORK_IM_H5_MODULES`, where their services now fail closed with
 * typed `*CapabilityUnavailableError`s instead of fabricating data.
 */
export const DEFAULT_IM_H5_MODULES = [
  "chat",
  "contacts",
  "user",
  "agents",
  "notary",
  "orders",
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
