export const PC_VIDEOGEN_CONTRACT_UNAVAILABLE = 'pc videogen contract is not available';

export interface VideoGenService {
  generate(prompt: string): Promise<void>;
}

class SdkworkVideoGenService implements VideoGenService {
  async generate(_prompt: string): Promise<void> {
    throw new Error(PC_VIDEOGEN_CONTRACT_UNAVAILABLE);
  }
}

export const videoGenService = new SdkworkVideoGenService();
