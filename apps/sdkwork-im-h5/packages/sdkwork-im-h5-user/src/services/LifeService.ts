import { UserCapabilityUnavailableError } from "./UserCapabilityUnavailableError";

export interface LifeServiceItem {
  iconName: string;
  label: string;
  color: string;
}

export const LifeService = {
  async getLifeServices(): Promise<LifeServiceItem[]> {
    throw new UserCapabilityUnavailableError("Life services");
  },
};
