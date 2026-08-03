import {
  ALL_IM_H5_MODULES,
  COMPOSABLE_IM_H5_MODULES,
  DEFAULT_IM_H5_MODULES,
} from "@sdkwork/im-h5-shell/moduleCatalog";
import type { ImH5ModuleId } from "@sdkwork/im-h5-shell";

const MODULE_SELECTION_CONFIG_KEY = "VITE_SDKWORK_IM_H5_MODULES";
const knownModuleIds = new Set<string>(ALL_IM_H5_MODULES);

function readModuleSelection(): string | undefined {
  // Static access so vite inlines the value in both dev and production
  // builds; dynamic `import.meta.env?.[key]` lookups stay undefined at
  // runtime because vite only rewrites literal `import.meta.env` reads.
  const value = import.meta.env?.VITE_SDKWORK_IM_H5_MODULES as
    | string
    | undefined;
  return typeof value === "string" && value.trim() ? value : undefined;
}

export function parseImH5ModuleSelection(value?: string): readonly ImH5ModuleId[] {
  if (value === undefined || value.trim().length === 0) {
    return DEFAULT_IM_H5_MODULES;
  }

  const rawModuleIds = value.split(",").map((moduleId) => moduleId.trim());
  if (rawModuleIds.some((moduleId) => moduleId.length === 0)) {
    throw new Error(`${MODULE_SELECTION_CONFIG_KEY} contains an empty module id.`);
  }

  const duplicateModuleId = rawModuleIds.find(
    (moduleId, index) => rawModuleIds.indexOf(moduleId) !== index,
  );
  if (duplicateModuleId) {
    throw new Error(`${MODULE_SELECTION_CONFIG_KEY} contains duplicate module ${duplicateModuleId}.`);
  }

  const unknownModuleId = rawModuleIds.find((moduleId) => !knownModuleIds.has(moduleId));
  if (unknownModuleId) {
    throw new Error(`${MODULE_SELECTION_CONFIG_KEY} contains unknown module ${unknownModuleId}.`);
  }

  const moduleIds = rawModuleIds as ImH5ModuleId[];
  const pendingModuleId = moduleIds.find((moduleId) => !COMPOSABLE_IM_H5_MODULES.has(moduleId));
  if (pendingModuleId) {
    throw new Error(`H5 module ${pendingModuleId} does not have a composed runtime contract.`);
  }

  return moduleIds;
}

export function resolveConfiguredImH5ModuleIds(): readonly ImH5ModuleId[] {
  return parseImH5ModuleSelection(readModuleSelection());
}
