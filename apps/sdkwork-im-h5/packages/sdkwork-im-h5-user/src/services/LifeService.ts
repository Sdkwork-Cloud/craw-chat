/**
 * Life services — fail-closed (PRD).
 *
 * Audited as a pure in-memory mock with no owner backend SDK. The fake list
 * is removed: `getLifeServices` throws a typed
 * `LifeServiceCapabilityUnavailableError` so consumers surface a typed
 * unavailable state instead of fabricated entries.
 */

export interface LifeServiceItem {
  iconName: string;
  label: string;
  color: string;
}

export class LifeServiceCapabilityUnavailableError extends Error {
  constructor(capability: string) {
    super(`${capability} is unavailable because its owner SDK is not composed.`);
    this.name = "LifeServiceCapabilityUnavailableError";
  }
}

export const LifeService = {
  getLifeServices: async (): Promise<LifeServiceItem[]> => {
    throw new LifeServiceCapabilityUnavailableError("Life services list");
  },
};
