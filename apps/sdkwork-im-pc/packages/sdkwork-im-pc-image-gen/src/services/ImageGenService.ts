export const PC_IMAGEGEN_CONTRACT_UNAVAILABLE = 'pc imagegen contract is not available';

export interface ImageGenService {
  generate(prompt: string): Promise<void>;
}

class SdkworkImageGenService implements ImageGenService {
  async generate(_prompt: string): Promise<void> {
    throw new Error(PC_IMAGEGEN_CONTRACT_UNAVAILABLE);
  }
}

export const imageGenService = new SdkworkImageGenService();
