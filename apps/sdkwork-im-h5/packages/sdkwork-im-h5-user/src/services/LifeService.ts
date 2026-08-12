/**
 * Life services — fail-closed (PRD).
 *
 * Audited as a pure in-memory mock with no owner backend SDK. The fake list
 * is removed: `getLifeServices` throws a typed
 * `UserCapabilityUnavailableError` so consumers surface a typed
 * unavailable state instead of fabricated entries.
 */

import { UserCapabilityUnavailableError } from "./UserCapabilityUnavailableError";

export interface LifeServiceItem {
  iconName: string;
  label: string;
  color: string;
}

export const LifeService = {
  getLifeServices: async (): Promise<LifeServiceItem[]> => {
    throw new UserCapabilityUnavailableError("Life services list");
  },
};
