export interface ImH5SdkRegistration<TClient = unknown> {
  readonly id: string;
  readonly client: TClient;
}

export interface ImH5SdkRegistry {
  register<TClient>(registration: ImH5SdkRegistration<TClient>): void;
  resolve<TClient>(id: string): TClient;
  has(id: string): boolean;
  list(): readonly ImH5SdkRegistration[];
}

export function createImH5SdkRegistry(
  initial: readonly ImH5SdkRegistration[] = [],
): ImH5SdkRegistry {
  const registrations = new Map<string, ImH5SdkRegistration>();

  const register = <TClient>(registration: ImH5SdkRegistration<TClient>): void => {
    if (registrations.has(registration.id)) {
      throw new Error(`IM H5 SDK registration already exists: ${registration.id}`);
    }
    registrations.set(registration.id, registration);
  };

  initial.forEach(register);

  return {
    register,
    resolve<TClient>(id: string): TClient {
      const registration = registrations.get(id);
      if (!registration) {
        throw new Error(`IM H5 SDK registration is missing: ${id}`);
      }
      return registration.client as TClient;
    },
    has: (id) => registrations.has(id),
    list: () => Array.from(registrations.values()),
  };
}
