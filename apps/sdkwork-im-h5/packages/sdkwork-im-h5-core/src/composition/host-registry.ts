import type { ImH5HostPort } from "../host";

export interface ImH5HostRegistration {
  readonly id: string;
  readonly adapter: ImH5HostPort;
}

export interface ImH5HostRegistry {
  register(registration: ImH5HostRegistration): void;
  resolve(id: string): ImH5HostPort | undefined;
  list(): readonly ImH5HostRegistration[];
}

export function createImH5HostRegistry(
  initial: readonly ImH5HostRegistration[] = [],
): ImH5HostRegistry {
  const registrations = new Map<string, ImH5HostRegistration>();

  const register = (registration: ImH5HostRegistration): void => {
    if (registrations.has(registration.id)) {
      throw new Error(`IM H5 host registration already exists: ${registration.id}`);
    }
    registrations.set(registration.id, registration);
  };

  initial.forEach(register);

  return {
    register,
    resolve: (id) => registrations.get(id)?.adapter,
    list: () => Array.from(registrations.values()),
  };
}
