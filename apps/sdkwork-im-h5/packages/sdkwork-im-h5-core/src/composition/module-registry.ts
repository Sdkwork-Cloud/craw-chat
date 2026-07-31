export interface ImH5CoreModuleRegistration {
  readonly id: string;
  readonly packageName: string;
  readonly enabledByDefault: boolean;
}

export const DEFAULT_IM_H5_CORE_MODULES = [
  {
    id: "chat",
    packageName: "@sdkwork/im-h5-chat",
    enabledByDefault: true,
  },
  {
    id: "notary",
    packageName: "@sdkwork/im-h5-notary",
    enabledByDefault: true,
  },
] as const satisfies readonly ImH5CoreModuleRegistration[];

export interface ImH5CoreModuleRegistry {
  register(module: ImH5CoreModuleRegistration): void;
  resolve(id: string): ImH5CoreModuleRegistration | undefined;
  list(): readonly ImH5CoreModuleRegistration[];
}

export function createImH5CoreModuleRegistry(
  initial: readonly ImH5CoreModuleRegistration[] = DEFAULT_IM_H5_CORE_MODULES,
): ImH5CoreModuleRegistry {
  const modules = new Map<string, ImH5CoreModuleRegistration>();

  const register = (module: ImH5CoreModuleRegistration): void => {
    if (modules.has(module.id)) {
      throw new Error(`IM H5 module registration already exists: ${module.id}`);
    }
    modules.set(module.id, module);
  };

  initial.forEach(register);

  return {
    register,
    resolve: (id) => modules.get(id),
    list: () => Array.from(modules.values()),
  };
}
