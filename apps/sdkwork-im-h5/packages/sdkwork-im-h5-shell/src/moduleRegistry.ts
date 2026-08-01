import type { ImH5CapabilityModule, ImH5ModuleId } from "./contracts";
import {
  DEFAULT_IM_H5_MODULES,
} from "./moduleCatalog";
import {
  requireImH5ShellModule,
  validateImH5ShellModules,
} from "./moduleValidation";
import { chatModule } from "./modules/chatModule";
import { contactsModule } from "./modules/contactsModule";
import { driveModule } from "./modules/driveModule";
import { notaryModule } from "./modules/notaryModule";
import { ordersModule } from "./modules/ordersModule";

export * from "./moduleCatalog";
export { validateImH5ShellModules } from "./moduleValidation";

export const BUILTIN_IM_H5_MODULE_REGISTRY: Readonly<Partial<Record<ImH5ModuleId, ImH5CapabilityModule>>> = {
  chat: chatModule,
  contacts: contactsModule,
  drive: driveModule,
  notary: notaryModule,
  orders: ordersModule,
};

export function resolveImH5ShellModules(
  moduleIds: readonly ImH5ModuleId[] = DEFAULT_IM_H5_MODULES,
  registry: Readonly<Partial<Record<ImH5ModuleId, ImH5CapabilityModule>>> = BUILTIN_IM_H5_MODULE_REGISTRY,
): ImH5CapabilityModule[] {
  const modules = moduleIds.map((moduleId) => requireImH5ShellModule(moduleId, registry));
  validateImH5ShellModules(modules);
  return modules;
}
